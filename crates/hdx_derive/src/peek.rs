use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsUnnamed};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			let field = fields.unnamed.first().unwrap();
			let field_ty = &field.ty;
			quote! {
				#[automatically_derived]
				impl<'a> hdx_parser::Peek<'a> for #ident {
					fn peek(p: &hdx_parser::Parser<'a>) -> bool {
						use hdx_parser::{Peek};
						p.peek::<#field_ty>()
					}
				}
			}
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive Peek on a struct with named fields"),

		Data::Union(_) => err(ident.span(), "Cannot derive Peek on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps = vec![];
			let mut first = true;
			for var in variants {
				match var.fields {
					Fields::Unit => {
						steps.push(err(ident.span(), "Cannot derive Parse on a Field Unit Variant"));
					}
					Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
						if unnamed.len() != 1 {
							steps
								.push(err(ident.span(), "Cannot derive Parse on a struct with multiple unnamed fields"))
						} else {
							let field = unnamed.first().unwrap();
							let field_ty = &field.ty;
							if first {
								steps.push(quote! {
									p.peek::<#field_ty>()
								});
							} else {
								steps.push(quote! {
									|| p.peek::<#field_ty>()
								});
							}
						}
					}
					Fields::Named(_) => {
						steps.push(err(var.fields.span(), "Cannot derive on Parse on a Named Field Variant"));
					}
				}
				first = false;
			}
			quote! {
				#[automatically_derived]
				impl<'a> hdx_parser::Peek<'a> for #ident {
					fn peek(p: &hdx_parser::Parser<'a>) -> bool {
						use hdx_parser::{Peek};
						#(#steps)*
					}
				}
			}
		}
	}
}
