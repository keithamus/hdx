use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error,
	Fields, FieldsUnnamed, Ident, Index, LitStr, Meta, Token,
};

use crate::{err, kebab};

#[derive(Clone, Debug)]
pub enum WritableArg {
	AsFunction(String),
	Suffix(String),
	Prefix(String),
	Rename(String),
	String,
}

impl Parse for WritableArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let ident = input.parse::<Ident>()?;
		if ident == "as_function" {
			input.parse::<Token![=]>()?;
			Ok(Self::AsFunction(input.parse::<LitStr>()?.value()))
		} else if ident == "String" {
			Ok(Self::String)
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

pub enum WritableKind {
	String,
	Function(String),
	None,
}

pub struct WritableArgs {
	kind: WritableKind,
	suffix: Option<String>,
	prefix: Option<String>,
	rename: Option<String>,
}

impl WritableArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		let mut ret = Self { kind: WritableKind::None, suffix: None, prefix: None, rename: None };
		if let Some(Attribute { meta: Meta::List(meta), .. }) = &attrs.iter().find(|a| a.path().is_ident("writable")) {
			let args = meta.parse_args_with(Punctuated::<WritableArg, Token![,]>::parse_terminated).unwrap();
			for arg in args {
				match arg {
					WritableArg::AsFunction(s) => ret.kind = WritableKind::Function(s),
					WritableArg::String => ret.kind = WritableKind::String,
					WritableArg::Suffix(s) => ret.suffix = Some(s),
					WritableArg::Prefix(s) => ret.prefix = Some(s),
					WritableArg::Rename(s) => ret.rename = Some(s),
				}
			}
		}
		ret
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
				match var.fields {
					Fields::Unit => {
						let str = LitStr::new(
							&args.rename.unwrap_or_else(|| kebab(format!("{}", var_ident))),
							var_ident.span(),
						);
						let mut head = None;
						let mut tail = None;
						let mut prefix = None;
						let mut suffix = None;
						match args.kind {
							WritableKind::Function(str) => {
								head = Some(quote! {
									hdx_atom::atom!(#str).write_css(sink)?;
									sink.write_char('(')?;
								});
								tail = Some(quote! {
									sink.write_char(')')?;
								});
							},
							WritableKind::String => {
								head = Some(quote! { sink.write_char('"')?; });
								tail = Some(quote! { sink.write_char('"')?; });
							},
							WritableKind::None => {},
						}
						if let Some(str) = args.prefix {
							prefix = Some(quote! {
								sink.write_str(#str)?;
							});
						}
						if let Some(str) = args.suffix {
							suffix = Some(quote! {
								sink.write_str(#str)?;
							});
						}
						matchers.push(quote! {
							Self::#var_ident => {
								#head
								#prefix
								// This is the write of the UNIT
								hdx_atom::atom!(#str).write_css(sink)?;
								#suffix
								#tail
							}
						});
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
						let mut head = None;
						let mut tail = None;
						let mut prefix = None;
						let mut suffix = None;
						match args.kind {
							WritableKind::Function(str) => {
								head = Some(quote! {
									hdx_atom::atom!(#str).write_css(sink)?;
									sink.write_char('(')?;
								});
								tail = Some(quote! {
									sink.write_char(')')?;
								});
							}
							WritableKind::String => {
								head = Some(quote! { sink.write_char('"')?; });
								tail = Some(quote! { sink.write_char('"')?; });
							}
							WritableKind::None => {},
						}
						if let Some(str) = args.prefix {
							prefix = Some(quote! {
								sink.write_str(#str)?;
							});
						}
						if let Some(str) = args.suffix {
							suffix = Some(quote! {
								sink.write_str(#str)?;
							});
						}
						matchers.push(quote! {
							Self::#var_ident(#(#field_extract)*) => {
								#head
								#prefix
								#(#field_writes)*
								#suffix
								#tail
							}
						});
					}
					Fields::Named(_) => {
						matchers.push(err(var.fields.span(), "Cannot derive on Writable on named fields"))
					}
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
			let mut field_writes = vec![];
			for i in 0..fields.unnamed.len() {
				let idx = Index::from(i);
				field_writes.push(quote! {
					self.#idx.write_css(sink)?;
				});
			}
			let mut head = None;
			let mut tail = None;
			let mut prefix = None;
			let mut suffix = None;
			match input_args.kind {
				WritableKind::Function(str) => {
					head = Some(quote! {
						hdx_atom::atom!(#str).write_css(sink)?;
						sink.write_char('(')?;
					});
					tail = Some(quote! {
						sink.write_char(')')?;
					});
				}
				WritableKind::String => {
					head = Some(quote! { sink.write_char('"')?; });
					tail = Some(quote! { sink.write_char('"')?; });
				}
				WritableKind::None => {},
			}
			if let Some(str) = input_args.prefix {
				prefix = Some(quote! {
					sink.write_str(#str)?;
				});
			}
			if let Some(str) = input_args.suffix {
				suffix = Some(quote! {
					sink.write_str(#str)?;
				});
			}
			quote! {
				#[automatically_derived]
				impl<'a> ::hdx_writer::WriteCss<'a> for #ident {
					fn write_css<W: ::hdx_writer::CssWriter>(&self, sink: &mut W) -> ::hdx_writer::Result {
						use ::hdx_writer::{WriteCss, CssWriter};
						#head
						#prefix
						#(#field_writes)*
						#suffix
						#tail
						Ok(())
					}
				}
			}
		}
		Data::Struct(_) => err(ident.span(), "Cannot derive Writable on a Struct with named or no fields"),
		Data::Union(_) => err(ident.span(), "Cannot derive Writable on a Union"),
	}
}
