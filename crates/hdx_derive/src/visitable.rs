use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse::Parse, punctuated::Punctuated, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident,
	Index, LitStr, Meta, Token,
};

use crate::{err, snake};

#[derive(Clone, Debug)]
pub enum VisitableArg {
	CallAuto,
	Call(String),
	Skip,
}

impl Parse for VisitableArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let ident = input.parse::<Ident>()?;
		if ident == "skip" {
			Ok(Self::Skip)
		} else if ident == "call" {
			if input.parse::<Token![=]>().is_ok() {
				Ok(Self::Call(input.parse::<Ident>()?.to_string()))
			} else {
				Ok(Self::CallAuto)
			}
		} else {
			Err(Error::new(ident.span(), "Unrecognized Visitable arg"))?
		}
	}
}

#[derive(Default, PartialEq)]
enum AutoOrNamed {
	Named(String),
	Auto,
	#[default]
	None,
}

#[derive(Default)]
pub struct VisitableArgs {
	call: AutoOrNamed,
	skip: bool,
}

impl VisitableArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		let mut ret = Self::default();
		if let Some(Attribute { meta: Meta::List(meta), .. }) = &attrs.iter().find(|a| a.path().is_ident("visitable")) {
			let args = meta.parse_args_with(Punctuated::<VisitableArg, Token![,]>::parse_terminated).unwrap();
			for arg in args {
				match arg {
					VisitableArg::Skip => ret.skip = true,
					VisitableArg::CallAuto => ret.call = AutoOrNamed::Auto,
					VisitableArg::Call(s) => ret.call = AutoOrNamed::Named(s),
				}
			}
		}
		ret
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let gen = input.generics;
	let input_args = VisitableArgs::parse(&input.attrs);
	let call = match input_args.call {
		AutoOrNamed::None => None,
		AutoOrNamed::Named(s) => Some(quote! { v.#s(self); }),
		AutoOrNamed::Auto => {
			let s = Ident::new(&snake(format!("visit{}", ident)), ident.span());
			Some(quote! { v.#s(self); })
		}
	};
	if input_args.skip {
		return err(ident.span(), "#[visitable(skip)] is not valid on the input");
	}
	match input.data {
		Data::Enum(DataEnum { variants, .. }) => {
			let mut field_accepts = vec![];
			let mut field_accept_muts = vec![];
			let mut accepts = call.is_some();
			for field in variants {
				let field_ident = field.ident;
				let args = VisitableArgs::parse(&field.attrs);
				let call = match args.call {
					AutoOrNamed::None => None,
					AutoOrNamed::Named(s) => {
						accepts = true;
						Some(quote! { Self::#field_ident(f) => v.#s(f), })
					}
					AutoOrNamed::Auto => {
						accepts = true;
						let s = Ident::new(&snake(format!("visit{}", ident)), ident.span());
						Some(quote! { Self::#field_ident(f) => v.#s(f), })
					}
				};
				field_accepts.push(if args.skip {
					quote! { Self::#field_ident(f) => {}, }
				} else {
					accepts = true;
					quote! { Self::#field_ident(f) => f.accept(v), }
				});
				field_accept_muts.push(if args.skip {
					quote! { Self::#field_ident(f) => {}, }
				} else {
					accepts = true;
					quote! { Self::#field_ident(f) => f.accept_mut(v), }
				});
			}
			if !accepts {
				return err(ident.span(), "Visitable does not accept any variants");
			}
			quote! {
				#[automatically_derived]
				impl<'a> hdx_ast::css::visit::Visitable<'a> for #ident #gen {
					fn accept<V: hdx_ast::css::visit::Visit<'a>>(&self, v: &mut V) {
						#call
						match self {
							#(#field_accepts)*
						}
					}
				}
				#[automatically_derived]
				impl<'a> hdx_ast::css::visit::VisitableMut<'a> for #ident #gen {
					fn accept_mut<V: hdx_ast::css::visit::VisitMut<'a>>(&mut self, v: &mut V) {
						#call
						match self {
							#(#field_accept_muts)*
						}
					}
				}
			}
		}
		Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
			let mut field_accepts = vec![];
			let mut field_accept_muts = vec![];
			for field in fields.named {
				let args = VisitableArgs::parse(&field.attrs);
				if args.call != AutoOrNamed::None {
					return err(ident.span(), "#[visitable(call = ...)] is not valid a struct field");
				}
				let field_ident = field.ident;
				if !args.skip {
					field_accepts.push(quote! {
						self.#field_ident.accept(v);
					});
					field_accept_muts.push(quote! {
						self.#field_ident.accept_mut(v);
					});
				}
			}
			if call.is_none() && field_accepts.is_empty() {
				return err(
					ident.span(),
					"This visitable does nothing, it doesn't call and it doesn't accept any fields",
				);
			}
			quote! {
				#[automatically_derived]
				impl<'a> hdx_ast::css::visit::Visitable<'a> for #ident #gen {
					fn accept<V: hdx_ast::css::visit::Visit<'a>>(&self, v: &mut V) {
						#call
						#(#field_accepts)*
					}
				}
				#[automatically_derived]
				impl<'a> hdx_ast::css::visit::VisitableMut<'a> for #ident #gen {
					fn accept_mut<V: hdx_ast::css::visit::VisitMut<'a>>(&mut self, v: &mut V) {
						#call
						#(#field_accept_muts)*
					}
				}
			}
		}
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			let mut field_accepts = vec![];
			let mut field_accept_muts = vec![];
			for i in 0..fields.unnamed.len() {
				let idx = Index::from(i);
				let args = VisitableArgs::parse(&fields.unnamed[i].attrs);
				if args.call != AutoOrNamed::None {
					return err(ident.span(), "#[visitable(call = ...)] is not valid a struct field");
				}
				if !args.skip {
					field_accepts.push(quote! {
						self.#idx.accept(v);
					});
					field_accept_muts.push(quote! {
						self.#idx.accept_mut(v);
					});
				}
			}
			if call.is_none() && field_accepts.is_empty() {
				return err(
					ident.span(),
					"This visitable does nothing, it doesn't call and it doesn't accept any fields",
				);
			}
			quote! {
				#[automatically_derived]
				impl<'a> hdx_ast::css::visit::Visitable<'a> for #ident #gen {
					fn accept<V: hdx_ast::css::visit::Visit<'a>>(&self, v: &mut V) {
						#call;
						#(#field_accepts)*
					}
				}

				#[automatically_derived]
				impl<'a> hdx_ast::css::visit::VisitableMut<'a> for #ident #gen {
					fn accept_mut<V: hdx_ast::css::visit::VisitMut<'a>>(&mut self, v: &mut V) {
						#call;
						#(#field_accept_muts)*
					}
				}
			}
		}
		Data::Struct(_) => err(ident.span(), "Cannot derive Visitable on a Unit struct"),
		Data::Union(_) => err(ident.span(), "Cannot derive Visitable on a Union"),
	}
}
