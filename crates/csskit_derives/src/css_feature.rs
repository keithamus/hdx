use heck::ToKebabCase;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use std::fmt::Display;
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, LitStr, Meta, Result,
	parse::{Parse, ParseBuffer},
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct CSSFeatureName(pub LitStr);

impl Display for CSSFeatureName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.value())
	}
}

impl ToTokens for CSSFeatureName {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens);
	}
}

impl Parse for CSSFeatureName {
	fn parse(input: &ParseBuffer) -> Result<Self> {
		input.parse::<LitStr>().map(Self)
	}
}

impl TryFrom<&[Attribute]> for CSSFeatureName {
	type Error = Error;
	fn try_from(attrs: &[Attribute]) -> Result<Self> {
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("css_feature")) {
			match meta {
				// #[css_feature("foo")]
				Meta::List(meta) => meta.parse_args::<Self>(),
				// #[visit]
				_ => Err(Error::new(Span::call_site(), "`css_feature` attribute has no value")),
			}
		} else {
			Err(Error::new(Span::call_site(), "Missing `css_feature` attribute"))
		}
	}
}

fn by_feature_name(feature: String) -> TokenStream {
	quote! { ::css_feature_data::CSSFeature::by_feature_name(#feature) }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let ident = input.ident;
	let generics = &input.generics;
	let (impl_generics, type_generics, _) = generics.split_for_impl();
	let feature: CSSFeatureName = CSSFeatureName::try_from(input.attrs.as_slice())?;
	let steps = match &input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive on a Union")),

		Data::Struct(DataStruct { fields, .. }) => {
			if fields.is_empty() {
				return Err(Error::new(ident.span(), "Cannot derive on empty Struct"));
			} else {
				quote! {}
			}
		}

		Data::Enum(DataEnum { variants, .. }) => {
			if variants.is_empty() {
				return Err(Error::new(ident.span(), "Cannot derive on empty Enum"));
			} else {
				let variants = variants
					.iter()
					.map(|variant| {
						let variant_ident = &variant.ident;
						let pattern = match &variant.fields {
							Fields::Named(fields) => {
								let ignores = fields.named.iter().map(|f| {
									let fname = f.ident.as_ref().unwrap();
									quote! { #fname: _ }
								});
								quote! { Self::#variant_ident { #(#ignores),* } }
							}
							_ => {
								let members = variant.fields.members().map(|_| quote! { _ });
								quote! { Self::#variant_ident(#(#members),*) }
							}
						};
						if let Ok(feature) = CSSFeatureName::try_from(variant.attrs.as_slice()) {
							let step = by_feature_name(feature.to_string());
							quote! { #pattern => { #step }, }
						} else {
							let str = feature.to_string();
							let guessed_step =
								by_feature_name(format!("{}.{}", str, variant_ident.to_string().to_kebab_case()));
							let or_step = by_feature_name(feature.to_string());
							quote! { #pattern => { #guessed_step.or_else(|| #or_step) }, }
						}
					})
					.collect::<Vec<_>>();
				let by_feature_name = by_feature_name(feature.to_string());
				quote! {
					match self {
						#(#variants)*
						_ => { #by_feature_name },
					}
				}
			}
		}
	};
	let steps = if steps.is_empty() { by_feature_name(feature.to_string()) } else { steps };
	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_feature_data::ToCSSFeature for #ident #type_generics {
			fn to_css_feature(&self) -> Option<&'static ::css_feature_data::CSSFeature> {
				#steps
			}
		}
	})
}
