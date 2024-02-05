use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
	DeriveInput, Error, Fields, FieldsUnnamed, Ident, Index, LitStr, Meta, Token,
};

use crate::{err, kebab};

#[derive(Clone, Debug)]
pub enum WritableArg {
	AsFunction(String),
	Suffix(String),
	Prefix(String),
	Rename(String),
}

impl Parse for WritableArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let ident = input.parse::<Ident>()?;
		if ident == "as_function" {
			input.parse::<Token![=]>()?;
			Ok(Self::AsFunction(input.parse::<LitStr>()?.value()))
		} else if ident == "suffix" {
			input.parse::<Token![=]>()?;
			Ok(Self::Suffix(input.parse::<LitStr>()?.value()))
		} else if ident == "prefix" {
			input.parse::<Token![=]>()?;
			Ok(Self::Prefix(input.parse::<LitStr>()?.value()))
		} else if ident == "rename" {
			input.parse::<Token![=]>()?;
			Ok(Self::Rename(input.parse::<LitStr>()?.value()))
		} else {
			Err(Error::new(ident.span(), "Unrecognized Writable arg"))?
		}
	}
}

pub struct WritableArgs(Vec<WritableArg>);

impl WritableArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		if let Some(Attribute { meta: Meta::List(meta), .. }) =
			&attrs.iter().find(|a| a.path().is_ident("writable"))
		{
			return Self(
				meta.parse_args_with(Punctuated::<WritableArg, Token![,]>::parse_terminated)
					.unwrap()
					.iter()
					.cloned()
					.collect(),
			);
		}
		Self(vec![])
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let input_args = WritableArgs::parse(&input.attrs);
	match input.data {
		Data::Enum(DataEnum { variants, .. }) => {
			let mut matchers = vec![];
			for var in variants {
				let var_ident = var.ident;
				let args = WritableArgs::parse(&var.attrs);
				if args.0.len() > 1 {
					return err(var_ident.span(), "#[writable] can only have one argument");
				}
				match var.fields {
					Fields::Unit => {
						let str =
							LitStr::new(kebab(format!("{}", var_ident)).as_str(), var_ident.span());
						match args.0.first() {
							Some(WritableArg::AsFunction(name)) => {
								matchers.push(quote! {
									Self::#var_ident => {
										hdx_atom::atom!(#name).write_css(sink)?;
										sink.write_char('(')?;
										hdx_atom::atom!(#str).write_css(sink)?;
										sink.write_char(')')?;
									}
								});
							}
							Some(WritableArg::Prefix(prefix)) => {
								matchers.push(quote! {
									Self::#var_ident => {
										sink.write_str(#prefix)?;
										hdx_atom::atom!(#str).write_css(sink)?;
									}
								});
							}
							Some(WritableArg::Suffix(suffix)) => {
								matchers.push(quote! {
									Self::#var_ident => {
										hdx_atom::atom!(#str).write_css(sink)?;
										sink.write_str(#suffix)?;
									}
								});
							}
							_ => {
								matchers.push(quote! {
									Self::#var_ident => hdx_atom::atom!(#str).write_css(sink)?,
								});
							}
						}
					}
					Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
						let mut field_extract = vec![];
						let mut field_writes = vec![];
						for i in 0..unnamed.len() {
							let fname = Ident::new(&format!("f{:?}", i), Span::call_site());
							field_extract.push(fname.clone());
							field_writes.push(quote! {
								#fname.write_css(sink)?;
							});
						}
						match args.0.first() {
							Some(WritableArg::AsFunction(name)) => {
								matchers.push(quote! {
									Self::#var_ident(#(#field_extract)*) => {
										hdx_atom::atom!(#name).write_css(sink)?;
										sink.write_char('(')?;
										#(#field_writes)*
										sink.write_char(')')?;
									},
								});
							}
							Some(WritableArg::Prefix(prefix)) => {
								matchers.push(quote! {
									Self::#var_ident(#(#field_extract)*) => {
										hdx_atom::atom!(#prefix).write_css(sink)?;
										#(#field_writes)*
									}
								});
							}
							Some(WritableArg::Suffix(suffix)) => {
								matchers.push(quote! {
									Self::#var_ident(#(#field_extract)*) => {
										#(#field_writes)*
										hdx_atom::atom!(#suffix).write_css(sink)?;
									}
								});
							}
							_ => matchers.push(quote! {
								Self::#var_ident(#(#field_extract)*) => {
									#(#field_writes)*
								}
							}),
						};
					}
					Fields::Named(_) => matchers
						.push(err(var.fields.span(), "Cannot derive on Writable on named fields")),
				}
			}
			let match_block = quote! {
				match self {
					#(#matchers)*
				}
			};
			quote! {
				#[automatically_derived]
				impl<'a> ::hdx_writer::WriteCss<'a> for #ident {
					fn write_css<W: ::hdx_writer::CssWriter>(&self, sink: &mut W) -> ::hdx_writer::Result {
						use ::hdx_writer::{WriteCss, CssWriter};
						#match_block
						Ok(())
					}
				}
			}
		}
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			if input_args.0.len() > 1 {
				return err(ident.span(), "#[writable] can only have one argument");
			}
			let mut field_writes = vec![];
			for i in 0..fields.unnamed.len() {
				let idx = Index::from(i);
				field_writes.push(quote! {
					self.#idx.write_css(sink)?;
				});
			}
			let write = match input_args.0.first() {
				Some(WritableArg::AsFunction(name)) => {
					quote! {
						hdx_atom::atom!(#name).write_css(sink)?;
						sink.write_char('(')?;
						#(#field_writes)*
						sink.write_char(')')?;
					}
				}
				Some(WritableArg::Prefix(prefix)) => {
					quote! {
						hdx_atom::atom!(#prefix).write_css(sink)?;
						#(#field_writes)*
					}
				}
				Some(WritableArg::Suffix(suffix)) => {
					quote! {
						#(#field_writes)*
						hdx_atom::atom!(#suffix).write_css(sink)?;
					}
				}
				_ => quote! {
					#(#field_writes)*
				},
			};
			quote! {
				#[automatically_derived]
				impl<'a> ::hdx_writer::WriteCss<'a> for #ident {
					fn write_css<W: ::hdx_writer::CssWriter>(&self, sink: &mut W) -> ::hdx_writer::Result {
						use ::hdx_writer::{WriteCss, CssWriter};
						#write
						Ok(())
					}
				}
			}
		}
		Data::Struct(_) => {
			err(ident.span(), "Cannot derive Writable on a Struct with named or no fields")
		}
		Data::Union(_) => err(ident.span(), "Cannot derive Writable on a Union"),
	}
}
