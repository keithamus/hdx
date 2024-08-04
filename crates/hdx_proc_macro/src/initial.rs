use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	ext::IdentExt,
	parse::{Parse, ParseStream},
	spanned::Spanned,
	Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, Result, Type, TypeArray,
	TypePath,
};

use crate::def::DefIdent;

#[derive(Debug, PartialEq)]
pub(crate) enum Args {
	Ident(DefIdent),
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Ident::peek_any) {
			Ok(Self::Ident(input.parse::<DefIdent>()?))
		} else {
			Err(Error::new(input.span(), "unknown token!"))?
		}
	}
}

fn unpack_smallvec_type(type_path: &TypePath) -> Option<Type> {
	if let Some(syn::PathArguments::AngleBracketed(args)) = &type_path.path.segments.last().map(|s| &s.arguments) {
		if let Some(syn::GenericArgument::Type(Type::Array(TypeArray { elem, .. }))) = args.args.first() {
			return Some(*elem.clone());
		}
	}
	None
}

pub fn generate(args: Args, ast: DeriveInput) -> TokenStream {
	let ident = &ast.ident;
	let steps = if let Args::Ident(ident) = args {
		match &ast.data {
			Data::Enum(DataEnum { variants, .. }) => {
				let var = ident.to_variant_name();
				// The initial is one of the root level variants
				if variants.iter().any(|v| v.ident == var.to_string()) {
					quote! { Self::#var }
				// The initial must be one of the sub-enum variants
				} else {
					let first_deep = variants.iter().find(|v| v.fields != Fields::Unit);
					let ty = first_deep.expect("Cannot find variant").ident.clone();
					quote! { Self::#ty(#ty::#var) }
				}
			}
			Data::Struct(DataStruct { fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }), .. }) => {
				if unnamed.len() == 1 {
					let var = ident.to_variant_name();
					if let Type::Path(type_path) = &unnamed[0].ty {
						// Handle the easy case of an in-scope enum
						if type_path.path.get_ident().is_some() {
							quote! { Self(#type_path::#var) }
						} else if type_path.path.segments.first().map(|s| s.ident == "smallvec").unwrap_or(false) {
							if let Some(type_path) = unpack_smallvec_type(type_path) {
								quote! { Self(::smallvec::smallvec![#type_path::#var]) }
							} else {
								return Error::new(ident.span(), "cannot create initial on Unit struct")
									.into_compile_error();
							}
						} else {
							todo!("initial with struct field path of {:?}", type_path);
						}
					} else {
						todo!("initial with struct field of {:?}", &unnamed[0]);
					}
				} else {
					todo!("initial of struct with multiple unnamed");
				}
			}
			Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) => {
				todo!("initial of fieldsnamed");
			}
			Data::Struct(DataStruct { fields: Fields::Unit, .. }) => {
				return Error::new(ident.span(), "cannot create initial on Unit struct").into_compile_error();
			}
			Data::Union(_) => {
				return Error::new(ident.span(), "cannot create initial on Union").into_compile_error();
			}
		}
	} else {
		todo!();
	};
	quote! {
		#ast

		#[automatically_derived]
		impl Default for #ident {
			fn default() -> Self {
				#steps
			}
		}
	}
}
