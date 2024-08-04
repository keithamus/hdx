use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
	parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DataStruct, DeriveInput, Error,
	Expr, ExprParen, ExprRange, Fields, FieldsUnnamed, Ident, LitStr, Meta, Token,
};

use crate::{err, kebab};

#[derive(Clone, Debug)]
enum Kind {
	Ident,
	Number,
	String,
	Function,
	Dimension,
	DimensionOrNumber,
	DimensionOrZero,
	AtKeyword,
}

#[derive(Clone, Debug)]
enum Check {
	Int,
	Float,
	Signed,
	Unsigned,
	Zero,
	Range(ExprRange),
}

#[derive(Clone, Debug)]
enum ParsableArg {
	Kind(Kind),
	Atom(String),
	Check(Check),
	ParseInner,
}

impl Parse for ParsableArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		match input.parse::<Ident>()? {
			i if i == "Number" => Ok(Self::Kind(Kind::Number)),
			i if i == "String" => Ok(Self::Kind(Kind::String)),
			i if i == "Function" => Ok(Self::Kind(Kind::Function)),
			i if i == "Dimension" => Ok(Self::Kind(Kind::Dimension)),
			i if i == "DimensionOrZero" => Ok(Self::Kind(Kind::DimensionOrZero)),
			i if i == "DimensionOrNumber" => Ok(Self::Kind(Kind::DimensionOrNumber)),
			i if i == "AtKeyword" => Ok(Self::Kind(Kind::AtKeyword)),
			i if i == "atom" => {
				input.parse::<Token![=]>()?;
				Ok(Self::Atom(input.parse::<LitStr>()?.value()))
			}
			i if i == "Check" => {
				input.parse::<Token![::]>()?;
				match input.parse::<Ident>()? {
					i if i == "Int" => Ok(Self::Check(Check::Int)),
					i if i == "Float" => Ok(Self::Check(Check::Float)),
					i if i == "Signed" => Ok(Self::Check(Check::Signed)),
					i if i == "Unsigned" => Ok(Self::Check(Check::Unsigned)),
					i if i == "Zero" => Ok(Self::Check(Check::Zero)),
					i if i == "Range" => match input.parse::<ExprParen>()?.expr.deref() {
						Expr::Range(range) => Ok(Self::Check(Check::Range(range.clone()))),
						expr => Err(Error::new(
							expr.span(),
							format!("Unrecognized Parsable value Check::Range({:?})", expr),
						))?,
					},
					ident => Err(Error::new(ident.span(), format!("Unrecognized Parsable value Check::{:?}", ident)))?,
				}
			}
			i if i == "parse_inner" => Ok(Self::ParseInner),
			ident => Err(Error::new(ident.span(), format!("Unrecognized Parsable arg {:?}", ident)))?,
		}
	}
}

pub struct ParsableArgs {
	kind: Kind,
	parse_inner: bool,
	atom: Option<String>,
	checks: Vec<Check>,
}

impl ParsableArgs {
	fn parse(attrs: &[Attribute]) -> Self {
		let mut ret = Self { kind: Kind::Ident, parse_inner: false, atom: None, checks: vec![] };
		if let Some(Attribute { meta: Meta::List(meta), .. }) = &attrs.iter().find(|a| a.path().is_ident("parsable")) {
			let args = meta.parse_args_with(Punctuated::<ParsableArg, Token![,]>::parse_terminated).unwrap();
			for arg in args {
				match arg {
					ParsableArg::Kind(k) => ret.kind = k,
					ParsableArg::Check(c) => ret.checks.push(c),
					ParsableArg::Atom(s) => ret.atom = Some(s),
					ParsableArg::ParseInner => ret.parse_inner = true,
				}
			}
		}
		ret
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = input.generics;
	match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			if fields.unnamed.len() != 1 {
				err(ident.span(), "Cannot derive Parsable on a struct with multiple unnamed fields")
			} else {
				let field = fields.unnamed.first().unwrap();
				let field_ty = &field.ty;
				let value = quote! {
					Self(parser.parse::<#field_ty>(parser)?)
				};
				quote! {
					#[automatically_derived]
					impl<'a> hdx_parser::Parse<'a> for #ident #generics {
						fn parse(parser: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
							use hdx_parser::{Parse};
							Ok(#value)
						}
					}
				}
			}
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive Parsable on a struct with named fields"),

		Data::Union(_) => err(ident.span(), "Cannot derive Parsable on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut ident_matchers = vec![];
			let mut function_matchers = vec![];
			let mut at_matchers = vec![];
			let mut string_matcher = None;
			let mut number_matcher = None;
			let mut dimension_matcher = None;
			let mut dimension_matchers = vec![];

			// Each variant in the Enum will be handled in its own way
			for var in variants {
				let var_ident = var.ident;
				let args = ParsableArgs::parse(&var.attrs);
				let str = LitStr::new(&args.atom.unwrap_or_else(|| kebab(format!("{}", var_ident))), var_ident.span());

				// The fields in each variant need to be handled differently
				match var.fields {
					// Named Struct Variants are simply too complex to support
					// enum Foo { A({ b, c }) } will fail
					Fields::Named(_) => {
						ident_matchers.push(err(var.fields.span(), "Cannot derive on Parsable on named fields"))
					}

					// Unit fields can only take an ident. E.g.
					// enum Foo { A, B } will only match the matching idents "a" and "b"
					Fields::Unit => ident_matchers.push(match args.kind {
						Kind::Ident => quote! {
							hdx_atom::atom!(#str) => Ok(Self::#var_ident),
						},
						_ => err(ident.span(), "Parsable only matches Unit variants to Kind::Ident arms"),
					}),

					// Unnamed structs can be handled but each one must must be annotated to deal with diffetent token
					// types, and each token type can support a limited set of fields.
					Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => match args.kind {
						// Dimensions can be assigned to a single unnamed field:
						// enum Foo {
						//   #[parsable(Dimension)]
						//   A(the_dimension_f32)
						//   #[parsable(Dimension)]
						//   B(the_dimension_f32)
						// }
						//
						// Multiple match arms can exist, one for each named unit of the dimension.
						// Alternatively the `parse_inner` flag can be used, in which case only one march arm can exist:
						//
						// enum Foo {
						//   #[parsable(Dimension, parse_inner)]
						//   A(TheParsable)
						// }
						Kind::Dimension | Kind::DimensionOrNumber | Kind::DimensionOrZero => {
							let checks: Vec<TokenStream> = args
								.checks
								.iter()
								.map(|check| match check {
									Check::Float => quote! {
										if !token.is_float() {
											Err(hdx_parser::diagnostics::ExpectedFloat(parser.parse_number(token), token.span()))?
										}
									},
									Check::Int => quote! {
										if !token.is_int() {
											Err(hdx_parser::diagnostics::ExpectedInt(parser.parse_number(token), token.span()))?
										}
									},
									Check::Signed => quote! {
										if !token.is_signed() {
											Err(hdx_parser::diagnostics::ExpectedSign(parser.parse_number(token), token.span()))?
										}
									},
									Check::Unsigned => quote! {
										if token.is_signed() {
											Err(hdx_parser::diagnostics::ExpectedUnsigned(parser.parse_number(token), token.span()))?
										}
									},
									Check::Zero => quote! {
										let val = parser.parse_number(token);
										if val != 0.0 {
											Err(hdx_parser::diagnostics::ExpectedZero(val, token.span()))?
										}
									},
									Check::Range(r) => quote! {
										let val = parser.parse_number(token);
										if !(#r).contains(&val) {
											Err(hdx_parser::diagnostics::NumberOutOfBounds(val, "#r".to_string(), token.span()))?
										}
									},
								})
								.collect();
							if dimension_matcher.is_some() {
								dimension_matcher =
									Some(err(ident.span(), "Cannot have multiple fields match Kind::Dimension"));
							} else if matches!(args.kind, Kind::DimensionOrNumber | Kind::DimensionOrZero)
								&& number_matcher.is_some()
							{
								dimension_matcher = Some(err(
									ident.span(),
									"Cannot have multiple fields match Kind::DimensionOrNumber and a Number",
								));
							} else if unnamed.len() > 1 {
								dimension_matcher = Some(err(
									ident.span(),
									"The match arm for Kind::Dimension can only have a single unnamed value",
								));
							} else {
								let field = unnamed[0].clone().ty;
								if args.parse_inner {
									dimension_matcher = Some(quote! {
										hdx_lexer::Kind::Dimension => {
											#(#checks)*
											Ok(Self::#var_ident(#field::parse(parser)?))
										},
									});
									number_matcher = match args.kind {
										Kind::DimensionOrZero => Some(quote! {
											hdx_lexer::Kind::Number if parser.parse_number(token) == 0.0 => {
												#(#checks)*
												Ok(Self::#var_ident(#field::parse(parser)?))
											}
										}),
										Kind::DimensionOrNumber => Some(quote! {
											hdx_lexer::Kind::Number {
												#(#checks)*
												Ok(Self::#var_ident(#field::parse(parser)?))
											}
										}),
										_ => number_matcher,
									};
								} else {
									dimension_matchers.push(quote! {
										hdx_atom::atom!(#str) => {
											parser.hop(token);
											#(#checks)*
											Ok(Self::#var_ident(parser.parse_number(token).into()))
										},
									});
								}
							}
						}
						// Numbers can be assigned to a single unnamed field:
						// enum Foo {
						//   #[parsable(Number)]
						//   A(the_number_f32)
						// }
						//
						// Only one enum variant can be a Number, if multiple are present it'll error.
						Kind::Number => {
							number_matcher = Some(if number_matcher.is_some() {
								err(ident.span(), "Cannot have multiple fields match Kind::Number")
							} else if unnamed.len() > 1 {
								err(ident.span(), "The match arm for Kind::Number can only have a single unnamed value")
							} else {
								let field = unnamed[0].clone().ty;
								let checks: Vec<TokenStream> = args
									.checks
									.iter()
									.map(|check| match check {
										Check::Float => quote! {
											if !ty.is_float() {
												Err(hdx_parser::diagnostics::ExpectedFloat(val, token.span()))?
											}
										},
										Check::Int => quote! {
											if !ty.is_int() {
												Err(hdx_parser::diagnostics::ExpectedInt(val, token.span()))?
											}
										},
										Check::Signed => quote! {
											if !ty.is_signed() {
												Err(hdx_parser::diagnostics::ExpectedSign(val, token.span()))?
											}
										},
										Check::Unsigned => quote! {
											if ty.is_signed() {
												Err(hdx_parser::diagnostics::ExpectedUnsigned(val, token.span()))?
											}
										},
										Check::Zero => quote! {
											if val != 0.0 {
												Err(hdx_parser::diagnostics::ExpectedZero(val, token.span()))?
											}
										},
										Check::Range(r) => quote! {
											if !(#r).contains(&val) {
												Err(hdx_parser::diagnostics::NumberOutOfBounds(val, "#r".to_string(), token.span()))?
											}
										},
									})
									.collect();
								if args.parse_inner {
									quote! {
										hdx_lexer::Kind::Number => {
											#(#checks)*
											Ok(Self::#var_ident(parser.parse::<#field>()?))
										},
									}
								} else {
									quote! {
										hdx_lexer::Kind::Number => {
											parser.hop(token);
											#(#checks)*
											Ok(Self::#var_ident(val.into()))
										},
									}
								}
							})
						}
						// Strings can be assigned to a single unnamed field:
						// enum Foo {
						//   #[parsable(String)]
						//   A(the_atom)
						// }
						//
						// Only one enum variant can be a String, if multiple are present it'll error.
						Kind::String => {
							string_matcher = Some(if string_matcher.is_some() {
								err(ident.span(), "Cannot have multiple fields match Kind::String")
							} else if unnamed.len() > 1 {
								err(ident.span(), "The match arm for Kind::String can only have a single unnamed value")
							} else {
								let field = unnamed[0].clone().ty;
								if args.parse_inner {
									quote! {
										hdx_lexer::Kind::String => {
											Ok(Self::#var_ident(#field::parse(parser)?))
										},
									}
								} else {
									quote! {
										hdx_lexer::Kind::String => {
											Ok(Self::#var_ident(val.into()))
										},
									}
								}
							});
						}
						// Strings can be assigned to a single unnamed field:
						// enum Foo {
						//   #[parsable(Function)]
						//   Bar(the_atom)
						//   #[parsable(Function)]
						//   Baz(the_atom)
						// }
						//
						// Multiple match arms can exist, one for each named function
						Kind::Function => {
							function_matchers.push(if unnamed.len() > 1 {
								err(
									ident.span(),
									"The match arm for a Kind::Function can only have a single unnamed value",
								)
							} else {
								let field = unnamed[0].clone().ty;
								quote! {
									hdx_atom::atom!(#str) => {
										let val = parser.parse::<#field>()?;
										parser.parse::<Token![RightParen]>()?;
										Ok(Self::#var_ident(val))
									}
								}
							});
						}
						// AtKeywords can be assigned to a single unnamed field:
						// enum Foo {
						//   #[parsable(AtKeyword)]
						//   Bar(the_atom)
						//   #[parsable(AtKeyword)]
						//   Baz(the_atom)
						// }
						//
						// Multiple match arms can exist, one for each named keyword
						Kind::AtKeyword => {
							at_matchers.push(if unnamed.len() > 1 {
								err(
									ident.span(),
									"The match arm for a Kind::AtKeyword can only have a single unnamed value",
								)
							} else {
								let field = unnamed[0].clone().ty;
								quote! {
									hdx_atom::atom!(#str) => {
										let val = parser.parse::<#field>(parser)?;
										Ok(Self::#var_ident(val))
									}
								}
							});
						}
						k => {
							ident_matchers.push(err(
								ident.span(),
								&format!("Parsable cannot match Unnamed fields in a {:?} arm", k),
							));
						}
					},
				}
			}
			let ident_match_arm = if ident_matchers.is_empty() {
				quote! {}
			} else {
				quote! {
					hdx_lexer::Kind::Ident => {
						parser.hop(token);
						let atom = parser.parse_atom_lower(token);
						match atom {
							#(#ident_matchers)*
							_ => Err(hdx_parser::diagnostics::UnexpectedIdent(atom, token.span()))?
						}
					}
				}
			};
			let function_match_arm = if function_matchers.is_empty() {
				quote! {}
			} else {
				quote! {
					hdx_lexer::Kind::Function => {
						parser.hop(token);
						let atom = parser.parse_atom_lower(token);
						match atom {
							#(#function_matchers)*
							_ => Err(hdx_parser::diagnostics::UnexpectedFunction(atom, token.span()))?
						}
					}
				}
			};
			let at_match_arm = if at_matchers.is_empty() {
				quote! {}
			} else {
				quote! {
					hdx_lexer::Kind::AtKeyword => match parser.parse_atom_lower(token) {
						#(#at_matchers)*
						atom => Err(hdx_parser::diagnostics::UnexpectedAtRule(atom, token.span()))?
					}
				}
			};
			let dimension_match_arm = if dimension_matchers.is_empty() && dimension_matcher.is_none() {
				quote! {}
			} else if let Some(quote) = dimension_matcher {
				quote
			} else {
				quote! {
					::hdx_lexer::Kind::Dimension => match parser.parse_atom_lower(token) {
						#(#dimension_matchers)*
						atom => Err(::hdx_parser::diagnostics::UnexpectedDimension(atom, token.span()))?
					}
				}
			};
			quote! {
				#[automatically_derived]
				impl<'a> ::hdx_parser::Parse<'a> for #ident #generics {
					fn parse(parser: &mut ::hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
						use hdx_parser::{Parse};
						let token = parser.peek::<::hdx_parser::token::Any>().unwrap();
						match token.kind() {
							#ident_match_arm
							#string_matcher
							#number_matcher
							#dimension_match_arm
							#at_match_arm
							#function_match_arm
							_ => Err(::hdx_parser::diagnostics::Unexpected(token, token.span()))?
						}
					}
				}
			}
		}
	}
}
