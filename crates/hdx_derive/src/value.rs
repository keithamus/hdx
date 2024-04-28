use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parse, punctuated::Punctuated, Attribute, DeriveInput, Error, Ident, Meta, Token};

#[derive(Clone, Debug)]
enum ValueArg {
	Inherits,
}

impl Parse for ValueArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		match input.parse::<Ident>()? {
			i if i == "Inherits" => Ok(Self::Inherits),
			ident => {
				if ident.to_string().to_ascii_lowercase().starts_with("inherit") {
					Err(Error::new(ident.span(), format!("You wrote {:?} but you probably want 'Inherits'", ident.to_string())))?
				} else {
					Err(Error::new(ident.span(), format!("Unrecognized Value arg {:?}", ident)))?
				}
			}
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
		if let Some(Attribute { meta: Meta::List(meta), .. }) = &attrs.iter().find(|a| a.path().is_ident("value")) {
			let args = meta.parse_args_with(Punctuated::<ValueArg, Token![,]>::parse_terminated).unwrap();
			for arg in args {
				match arg {
					ValueArg::Inherits => ret.inherits = true,
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
		Some(quote! {
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
