use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Attribute, DeriveInput, Error, Ident, Meta, Token};

use crate::{err, kebab};

#[derive(Clone, Debug)]
enum ValueArg {
	Inherits(bool),
}

impl Parse for ValueArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		match input.parse::<Ident>()? {
			i if i == "Inherits" => {
				input.parse::<Token![::]>()?;
				match input.parse::<Ident>()? {
					i if i == "No" => Ok(Self::Inherits(true)),
					i if i == "Float" => Ok(Self::Inherits(false)),
					ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg Inherits::{:?}", ident)))?,
				}
			}
			ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {:?}", ident)))?,
		}
	}
}

#[derive(Debug)]
struct ValueArgs {
	inherits: bool,
}

impl ValueArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		let mut ret = Self { inherits: false };
		if let Some(Attribute { meta: Meta::List(meta), .. }) = &attrs.iter().find(|a| a.path().is_ident("parsable")) {
			let args = meta.parse_args_with(Punctuated::<ValueArg, Token![,]>::parse_terminated).unwrap();
			for arg in args {
				match arg {
					ValueArg::Inherits(b) => ret.inherits = b,
				}
			}
		}
		ret
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let input_args = ValueArgs::parse(&input.attrs);
	let inherits = if input_args.inherits {
		Some(quote!{
			fn inherits() -> bool { true }
		})
	} else {
		None
	};
	quote! {
		#[automatically_derived]
		impl hdx_ast::traits::Value for #ident {
			#inherits
		}
	}
}
