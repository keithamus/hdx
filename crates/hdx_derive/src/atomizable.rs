use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse::Parse, punctuated::Punctuated, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident,
	LitStr, Meta, Token,
};

use crate::{err, kebab};

#[derive(Clone, Debug)]
enum AtomizableArg {
	Atom(String),
}

impl Parse for AtomizableArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		match input.parse::<Ident>() {
			Ok(ident) => Err(Error::new(ident.span(), format!("Unrecognized Parsable arg {:?}", ident)))?,
			Err(_) => Ok(Self::Atom(input.parse::<LitStr>()?.value())),
		}
	}
}

#[derive(Debug)]
pub struct AtomizableArgs {
	atom: Option<String>,
}

impl AtomizableArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		let mut ret = Self { atom: None };
		if let Some(Attribute { meta: Meta::List(meta), .. }) = &attrs.iter().find(|a| a.path().is_ident("atomizable"))
		{
			let args = meta.parse_args_with(Punctuated::<AtomizableArg, Token![,]>::parse_terminated).unwrap();
			for arg in args {
				match arg {
					AtomizableArg::Atom(s) => ret.atom = Some(s),
				}
			}
		}
		ret
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let input_args = AtomizableArgs::parse(&input.attrs);
	match input.data {
		Data::Enum(DataEnum { variants, .. }) => {
			let mut match_atom_to_enum_variant = Vec::new();
			let mut match_enum_variant_to_atom = Vec::new();
			for var in variants {
				let var_ident = var.ident;
				let var_args = AtomizableArgs::parse(&var.attrs);
				let ident = if let Some(name) = var_args.atom { name } else { kebab(format!("{}", var_ident)) };
				let str = LitStr::new(&ident, var_ident.span());
				match_atom_to_enum_variant.push(quote! {
					hdx_atom::atom!(#str) => Some(Self::#var_ident),
				});
				// To allow for bitmasks, we match with equality
				match_enum_variant_to_atom.push(quote! {
					s if *s == Self::#var_ident => hdx_atom::atom!(#str),
				});
			}
			let from_atom_match = quote! {
				match atom.to_ascii_lowercase() {
					#(#match_atom_to_enum_variant)*
					_ => None
				}
			};
			let to_atom_match = quote! {
				match self {
					#(#match_enum_variant_to_atom)*
					s => unreachable!(),
				}
			};
			quote! {
				#[automatically_derived]
				impl hdx_atom::Atomizable for #ident {
					fn from_atom(atom: &hdx_atom::Atom) -> Option<Self> {
						#from_atom_match
					}
					fn to_atom(&self) -> hdx_atom::Atom {
						#to_atom_match
					}
				}
			}
		}
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			if fields.unnamed.len() != 1 {
				return err(ident.span(), "Cannot drive Writable on struct with multiple fields");
			}
			let str = LitStr::new(kebab(format!("{}", ident)).as_str(), ident.span());
			quote! {
				#[automatically_derived]
				impl hdx_atom::Atomizable for #ident {
					fn from_atom(atom: &hdx_atom::Atom) -> Option<Self> {
						if atom == hdx_atom::atom!(#str) {
							Some(Self(::core::default::Default::default()))
						} else {
							None
						}
					}
					fn to_atom(&self) -> hdx_atom::Atom {
						hdx_atom::atom!(#str)
					}
				}
			}
		}
		Data::Struct(_) => err(ident.span(), "Cannot derive Atomizable on a struct with named or no fields"),
		Data::Union(_) => err(ident.span(), "Cannot derive Atomizable on a Union"),
	}
}
