use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	ext::IdentExt,
	parse::{Parse, ParseStream},
	Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, FieldsUnnamed, Ident, LitFloat, LitInt,
	Result, Type, TypeArray, TypePath,
};

use crate::{def::DefIdent, pascal};

#[derive(Debug, PartialEq)]
pub(crate) enum Args {
	Ident(DefIdent),
	Dimension(LitFloat),
	Individual,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Ident::peek_any) {
			Ok(Self::Ident(input.parse::<DefIdent>()?))
		} else if input.peek(LitFloat) || input.peek(LitInt) {
			let f = if input.peek(LitFloat) {
				input.parse::<LitFloat>()?
			} else {
				let int = input.parse::<LitInt>()?;
				LitFloat::new(&format!("{}.0{}", int.base10_digits(), int.suffix()), int.span())
			};
			Ok(Self::Dimension(f))
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
	let generics = &ast.generics;
	let steps = match &ast.data {
		Data::Enum(DataEnum { variants, .. }) => {
			match args {
				Args::Ident(ident) => {
					let var = ident.to_variant_name(0);
					// The initial is one of the root level variants
					if variants.iter().any(|v| v.ident == var.to_string()) {
						quote! { Self::#var }
					// The initial must be one of the sub-enum variants
					} else {
						let first_deep =
							variants.iter().find(|v| v.fields != Fields::Unit).expect("Cannot find variant");
						let variant = first_deep.ident.clone();
						let values: Vec<TokenStream> = (&first_deep.fields)
							.into_iter()
							.map(|f| {
								let ty = &f.ty;
								quote! { #ty::#var }
							})
							.collect();
						quote! { Self::#variant(#(#values),*) }
					}
				}
				Args::Dimension(f) => {
					let first_deep = variants.iter().find(|v| v.fields != Fields::Unit).expect("Cannot find variant");
					let num = f.base10_digits();
					let variant_ident = first_deep.ident.clone();
					let mut vec = false;
					let ty = if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = &first_deep.fields {
						if let Type::Path(type_path) = &unnamed.first().unwrap().ty {
							vec = type_path.path.segments.first().map(|s| s.ident == "smallvec").unwrap_or(false);
							quote! { #type_path }
						} else {
							quote! { #variant_ident }
						}
					} else {
						quote! { #variant_ident }
					};
					let val = if let Ok(0) = f.base10_parse() {
						quote! { #ty::Zero }
					} else if let Ok(0.0) = f.base10_parse() {
						quote! { #ty::Zero }
					} else if !f.suffix().is_empty() {
						let var = format_ident!("{}", pascal(f.suffix().to_lowercase()));
						let num = LitFloat::new(&format!("{}f32", f.base10_digits()), f.span());
						quote! { #ty::#var::from(#num) }
					} else {
						quote! { #ty::from(#num) }
					};
					if vec {
						quote! { Self::#variant_ident(::smallvec::smallvec![#val]) }
					} else {
						quote! { Self::#variant_ident(#val) }
					}
				}
				Args::Individual => {
					dbg!("cannot auto a default impl for enum");
					return quote! { #ast };
				}
			}
		}
		Data::Struct(DataStruct { fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }), .. }) => {
			if unnamed.len() == 1 {
				if let Type::Path(type_path) = &unnamed[0].ty {
					// Handle vectors of properties
					if type_path.path.segments.first().map(|s| s.ident == "smallvec").unwrap_or(false) {
						if let Some(type_path) = unpack_smallvec_type(type_path) {
							match args {
								Args::Ident(ident) => {
									let var = ident.to_variant_name(0);
									quote! { Self(::smallvec::smallvec![#type_path::#var]) }
								}
								Args::Dimension(f) => {
									let num = LitFloat::new(&format!("{}f32", f.base10_digits()), f.span());
									let val = if let Ok(0) = f.base10_parse() {
										quote! { #type_path::Zero }
									} else if let Ok(0.0) = f.base10_parse() {
										quote! { #type_path::Zero }
									} else if !f.suffix().is_empty() {
										let var = format_ident!("{}", pascal(f.suffix().to_lowercase()));
										quote! { #type_path::#var::from(#num) }
									} else {
										quote! { #type_path::from(#num) }
									};
									quote! { Self(::smallvec::smallvec![#val]) }
								}
								Args::Individual => {
									return quote! {
										#[derive(Default)]
										#ast
									}
								}
							}
						} else {
							return Error::new(ident.span(), "cannot create initial on Unit struct")
								.into_compile_error();
						}
					} else {
						match args {
							Args::Ident(ident) => {
								let var = ident.to_variant_name(0);
								quote! { Self(#type_path::#var) }
							}
							Args::Dimension(f) => {
								// If it's a dimension it must be an enum variant like Angle or Length
								if let Ok(0) = f.base10_parse() {
									quote! { Self(#type_path::Zero) }
								} else if let Ok(0.0) = f.base10_parse() {
									quote! { Self(#type_path::Zero) }
								} else if !f.suffix().is_empty() {
									let var = format_ident!("{}", pascal(f.suffix().to_lowercase()));
									let num = LitFloat::new(&format!("{}f32", f.base10_digits()), f.span());
									quote! { Self(#type_path::#var::from(#num)) }
								} else {
									quote! { Self(#type_path::from(#f)) }
								}
							}
							Args::Individual => {
								return quote! {
									#[derive(Default)]
									#ast
								}
							}
						}
					}
				} else {
					todo!("initial of struct with type of {:?}", &unnamed[0].ty);
				}
			} else {
				return quote! {
					#[derive(Default)]
					#ast
				};
			}
		}
		Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { .. }), .. }) => {
			dbg!("initial of fieldsnamed");
			return quote! {
				#[derive(Default)]
				#ast
			};
		}
		Data::Struct(DataStruct { fields: Fields::Unit, .. }) => {
			return Error::new(ident.span(), "cannot create initial on Unit struct").into_compile_error();
		}
		Data::Union(_) => {
			return Error::new(ident.span(), "cannot create initial on Union").into_compile_error();
		}
	};
	quote! {
		#ast

		#[automatically_derived]
		impl #generics Default for #ident #generics {
			fn default() -> Self {
				#steps
			}
		}
	}
}
