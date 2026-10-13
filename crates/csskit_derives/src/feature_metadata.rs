use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Meta, Path, Result};

/// Reads `#[feature_metadata(CssAtomSet::Foo)]` from the type's attributes.
pub(crate) fn atom_from_attrs(attrs: &[syn::Attribute]) -> Option<Path> {
	attrs.iter().find(|a| a.path().is_ident("feature_metadata")).and_then(|attr| match &attr.meta {
		Meta::List(meta) => meta.parse_args::<Path>().ok(),
		_ => None,
	})
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let atom = atom_from_attrs(&input.attrs).ok_or_else(|| {
		Error::new(input.ident.span(), "#[derive(FeatureMetadata)] requires #[feature_metadata(CssAtomSet::Atom)]")
	})?;

	let ident = &input.ident;
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

	let variants = match &input.data {
		Data::Enum(e) => &e.variants,
		_ => return Err(Error::new(ident.span(), "#[derive(FeatureMetadata)] requires an enum")),
	};

	let mut arms = Vec::new();

	for variant in variants {
		let vname = &variant.ident;
		match vname.to_string().as_str() {
			// discrete_feature! / boolean_feature! colon form
			"WithValue" => {
				arms.push(quote! {
					Self::WithValue(_, _, _, value, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Plain {
							name: #atom,
							value: Some(value.to_span()),
						}
					}
				});
			}
			// discrete_feature! / boolean_feature! bare form
			"Bare" => {
				arms.push(quote! {
					Self::Bare(_, _, _) => crate::ConditionalFeature::Plain {
						name: #atom,
						value: None,
					}
				});
			}
			// ranged_feature! exact colon form
			"Exact" => {
				arms.push(quote! {
					Self::Exact(_, _, _, value, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Ranged {
							name: #atom,
							form: crate::RangedForm::Exact { value: value.to_span() },
						}
					}
				});
			}
			// ranged_feature! legacy min form
			"Min" => {
				arms.push(quote! {
					Self::Min(_, ident, colon, value, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Ranged {
							name: #atom,
							form: crate::RangedForm::LegacyMin { name: ident.to_span(), colon: colon.to_span(), value: value.to_span() },
						}
					}
				});
			}
			// ranged_feature! legacy max form
			"Max" => {
				arms.push(quote! {
					Self::Max(_, ident, colon, value, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Ranged {
							name: #atom,
							form: crate::RangedForm::LegacyMax { name: ident.to_span(), colon: colon.to_span(), value: value.to_span() },
						}
					}
				});
			}
			// ranged_feature! left form: (feature op value)
			"Left" => {
				arms.push(quote! {
					Self::Left(_, _, comparison, value, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Ranged {
							name: #atom,
							form: crate::RangedForm::Left {
								comparison: *comparison,
								value: value.to_span(),
							},
						}
					}
				});
			}
			// ranged_feature! right form: (value op feature)
			"Right" => {
				arms.push(quote! {
					Self::Right(_, value, comparison, _, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Ranged {
							name: #atom,
							form: crate::RangedForm::Right {
								value: value.to_span(),
								comparison: *comparison,
							},
						}
					}
				});
			}
			// ranged_feature! range form: (v1 op feature op v2)
			"Range" => {
				arms.push(quote! {
					Self::Range(_, left, left_cmp, _, right_cmp, right, _) => {
						use css_lexer::ToSpan;
						crate::ConditionalFeature::Ranged {
							name: #atom,
							form: crate::RangedForm::Range {
								left: left.to_span(),
								left_cmp: *left_cmp,
								right_cmp: *right_cmp,
								right: right.to_span(),
							},
						}
					}
				});
			}
			_ => {}
		}
	}

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics crate::FeatureMetadata for #ident #type_generics #where_clause {
			fn feature_metadata(&self) -> crate::ConditionalFeature {
				match self {
					#(#arms,)*
				}
			}
		}
	})
}
