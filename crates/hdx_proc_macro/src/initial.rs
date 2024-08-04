use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	ext::IdentExt,
	parse::{Parse, ParseStream},
	DeriveInput, Error, Ident, Result,
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

pub fn generate(args: Args, ast: DeriveInput) -> TokenStream {
	let ident = &ast.ident;
	let steps = if let Args::Ident(ident) = args {
		let var = ident.to_variant_name();
		quote! { Self::#var }
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
