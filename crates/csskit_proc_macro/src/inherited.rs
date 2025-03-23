use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse::{Parse, ParseStream},
	DeriveInput, LitStr, Result,
};

#[derive(Debug, PartialEq)]
pub(crate) enum Args {
	Yes,
	No,
	SeeIndividualProperties,
	Unknown,
}

impl Parse for Args {
	fn parse(input: ParseStream) -> Result<Self> {
		match input.parse::<LitStr>()?.value().as_str() {
			"yes" => Ok(Self::Yes),
			// TODO(keithamus): drop 'invidual properties' after https://github.com/w3c/csswg-drafts/pull/11106
			"see individual properties" | "invidual properties" => Ok(Self::SeeIndividualProperties),
			"no" => Ok(Self::No),
			s => {
				dbg!(format!("WARNING:: inherited value {} not recognised. Property will need to impl StyleValue trait manually.", s));
				Ok(Self::Unknown)
			}
		}
	}
}

pub fn generate(args: Args, ast: DeriveInput) -> TokenStream {
	let ident = &ast.ident;
	let generics = &ast.generics;
	let bool = args == Args::Yes;
	quote! {
		#ast
		#[automatically_derived]
		impl #generics StyleValue for #ident #generics {
			fn inherits() -> bool {
				#bool
			}
		}
	}
}
