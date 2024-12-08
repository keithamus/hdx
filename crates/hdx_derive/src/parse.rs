use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsUnnamed};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			if fields.unnamed.len() != 1 {
				err(ident.span(), "Cannot derive Parse on a struct with multiple unnamed fields")
			} else {
				let field = fields.unnamed.first().unwrap();
				let field_ty = &field.ty;
				quote! {
					#[automatically_derived]
					impl<'a> hdx_parser::Parse<'a> for #ident {
						fn parse(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
							use hdx_parser::{Parse};
							p.parse::<#field_ty>().map(Self)
						}
					}
				}
			}
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive Parse on a struct with named fields"),

		Data::Union(_) => err(ident.span(), "Cannot derive Parse on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps = vec![];
			let mut first = true;
			for var in variants {
				let var_ident = var.ident;
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
									p.parse_if_peek::<#field_ty>().ok().flatten().map(Self::#var_ident)
								});
							} else {
								steps.push(quote! {
									.or_else(|| p.parse_if_peek::<#field_ty>().ok().flatten().map(Self::#var_ident))
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
				impl<'a> hdx_parser::Parse<'a> for #ident {
					fn parse(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
						use hdx_parser::{Parse};
						#(#steps)*
							.ok_or_else(|| {
								let cursor = p.peek::<hdx_parser::T![Any]>().unwrap();
								hdx_parser::diagnostics::Unexpected(cursor.token(), cursor.span()).into()
							})
					}
				}
			}
		}
	}
}
