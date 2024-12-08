use proc_macro::TokenStream;

mod string_transform;

mod value;
mod visitable;

use proc_macro2::Span;
pub(crate) use string_transform::*;
use syn::Error;

#[proc_macro_derive(Value, attributes(value))]
pub fn derive_value(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	value::derive(input).into()
}

#[proc_macro_derive(Visitable, attributes(visitable))]
pub fn derive_visitable(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	visitable::derive(input).into()
}

fn err(span: Span, msg: &str) -> proc_macro2::TokenStream {
	let err = Error::new(span, msg).into_compile_error();
	quote::quote! {#err}
}
