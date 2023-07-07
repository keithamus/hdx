use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Expr, Lit, LitStr, Meta};

#[derive(Debug, Copy, Clone, PartialEq)]
enum RenameRule {
	PascalCase,
	LowerCase,
	UpperCase,
	CamelCase,
	SnakeCase,
	ScreamingSnakeCase,
	KebabCase,
	ScreamingKebabCase,
}

// Copied from Serde
// https://github.com/serde-rs/serde/blob/6e0b13eedbe24c54d49b421ae1ef36e007e8e592/serde_derive/src/internals/case.rs#L10C1-L84
impl RenameRule {
	pub fn apply_to_variant(&self, variant: &str) -> String {
		match *self {
			Self::PascalCase => variant.to_owned(),
			Self::LowerCase => variant.to_ascii_lowercase(),
			Self::UpperCase => variant.to_ascii_uppercase(),
			Self::CamelCase => variant[..1].to_ascii_lowercase() + &variant[1..],
			Self::SnakeCase => {
				let mut snake = String::new();
				for (i, ch) in variant.char_indices() {
					if i > 0 && ch.is_uppercase() {
						snake.push('_');
					}
					snake.push(ch.to_ascii_lowercase());
				}
				snake
			}
			Self::ScreamingSnakeCase => {
				Self::SnakeCase.apply_to_variant(variant).to_ascii_uppercase()
			}
			Self::KebabCase => Self::SnakeCase.apply_to_variant(variant).replace('_', "-"),
			Self::ScreamingKebabCase => {
				Self::ScreamingSnakeCase.apply_to_variant(variant).replace('_', "-")
			}
		}
	}
}

fn get_rename_rule(input: &DeriveInput) -> Result<RenameRule, Error> {
	for attr in &input.attrs {
		if attr.path().is_ident("atomizable") {
			if let Meta::NameValue(meta) = attr.parse_args().unwrap() {
				if meta.path.is_ident("rename_all") {
					if let Expr::Lit(lit) = meta.value {
						if let Lit::Str(str) = lit.lit {
							return match str.value().as_str() {
								"PascalCase" => Ok(RenameRule::PascalCase),
								"lowercase" => Ok(RenameRule::LowerCase),
								"UPPERCASE" => Ok(RenameRule::UpperCase),
								"camelCase" => Ok(RenameRule::CamelCase),
								"snake_case" => Ok(RenameRule::SnakeCase),
								"SCREAMING_SNAKE_CASE" => Ok(RenameRule::ScreamingSnakeCase),
								"kebab-case" => Ok(RenameRule::KebabCase),
								"SCREAMING-KEBAB-CASE" => Ok(RenameRule::ScreamingKebabCase),
								_ => Err(Error::new_spanned(
									meta.path,
									"expected a valid rename rule",
								)),
							};
						} else {
							Err(Error::new_spanned(meta.path, "expected a string literal"))?;
						}
					} else {
						Err(Error::new_spanned(meta.path, "expected `rename_all`"))?;
					}
				} else {
					Err(Error::new_spanned(meta.path, "expected `rename_all`"))?;
				}
			}
		}
	}
	Ok(RenameRule::KebabCase)
}

#[proc_macro_derive(Atomizable, attributes(atomizable))]
pub fn derive_atomizable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let rename_rule = get_rename_rule(&input).unwrap();
	match input.data {
		Data::Enum(syn::DataEnum { variants, .. }) => {
			let ident = input.ident;
			let mut match_atom_to_enum_variant = Vec::new();
			let mut match_enum_variant_to_atom = Vec::new();
			for var in variants {
				let var_ident = var.ident;
				let str = LitStr::new(
					rename_rule.apply_to_variant(format!("{}", var_ident).as_str()).as_str(),
					var_ident.span(),
				);
				match_atom_to_enum_variant.push(quote! {
					atom!(#str) => Some(Self::#var_ident),
				});
				match_enum_variant_to_atom.push(quote! {
					Self::#var_ident => atom!(#str),
				});
			}
			let from_atom_match = quote! {
				match atom.to_ascii_lowercase() {
					#(#match_atom_to_enum_variant)*
					_ => None
				}
			};
			let to_atom_match = quote! {
				match self {
					#(#match_enum_variant_to_atom)*
				}
			};
			quote! {
				impl Atomizable for #ident {
					fn from_atom(atom: Atom) -> Option<Self> {
						#from_atom_match
					}
					fn to_atom(&self) -> Atom {
						#to_atom_match
					}
				}
			}
		}
		_ => {
			let error = Error::new(input.ident.span(), "can only derive atomizable for enums")
				.into_compile_error();
			quote! {#error}
		}
	}
	.into()
}
