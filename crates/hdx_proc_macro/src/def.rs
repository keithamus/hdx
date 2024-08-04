use hdx_atom::{atom, Atom};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::{
	fmt::Display,
	ops::{Range, RangeFrom, RangeTo},
};
use syn::{
	braced, bracketed,
	ext::IdentExt,
	parenthesized,
	parse::{Parse, ParseStream},
	parse2, token, Error, Ident, LitFloat, LitInt, LitStr, Result, Token, Visibility,
};

use crate::{kebab, pascal};

pub(crate) struct StrWrapped<T: Parse>(pub T);
impl<T: Parse> Parse for StrWrapped<T> {
	fn parse(input_raw: ParseStream) -> Result<Self> {
		Ok(Self(parse2::<T>(input_raw.parse::<LitStr>()?.value().replace("'", "\"").parse::<TokenStream>()?)?))
	}
}

pub trait GenerateDefinition {
	fn generate_definition(&self, vis: &Visibility, ident: &Ident) -> TokenStream;
}

pub trait GeneratePeekImpl {
	fn peek_steps(&self) -> TokenStream;
}

pub trait GenerateParseImpl: GeneratePeekImpl {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream;
}

pub trait GenerateWriteImpl {
	fn write_steps(&self, capture: TokenStream) -> TokenStream;
	fn will_write_cond_steps(&self, _capture: TokenStream) -> Option<TokenStream> {
		None
	}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Def {
	Ident(DefIdent),
	Type(DefType),
	Optional(Box<Def>), // ?
	Combinator(Vec<Def>, DefCombinatorStyle),
	Group(Box<Def>, DefGroupStyle),
	Multiplier(Box<Def>, DefMultiplierStyle),
	Punct(char),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefGroupStyle {
	None,            // [ ] - regular group notation
	OneMustOccur,    // [ ]! - at least one in the group must occur
	OneOrMore,       // [ ]#
	Range(DefRange), // [ ]{A,B}
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub(crate) enum DefCombinatorStyle {
	Ordered,      // <space>
	AllMustOccur, // && - all must occur
	Options,      // || - one or more must occur
	Alternatives, // | - exactly one must occur
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefMultiplierStyle {
	ZeroOrMore,                        // *
	OneOrMore,                         // +
	OneOrMoreCommaSeparated(DefRange), // # or #{,}
	Range(DefRange),                   // {,}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefRange {
	None,
	Range(Range<f32>),         // {A,B}
	RangeFrom(RangeFrom<f32>), // {A,}
	RangeTo(RangeTo<f32>),     // {,B}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct DefIdent(pub Atom);

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefType {
	Length(DefRange),
	LengthPercentage(DefRange),
	Angle(DefRange),
	Time(DefRange),
	Resolution(DefRange),
	Integer(DefRange),
	Number(DefRange),
	Percentage(DefRange),
	Color,
	String,
	Custom(DefIdent, FunctionNotation),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum FunctionNotation {
	Yes,
	No,
}

impl Parse for Def {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut root = if input.peek(Token![<]) {
			Self::Type(input.parse::<DefType>()?)
		} else if input.peek(token::Bracket) {
			let content;
			bracketed!(content in input);
			let inner = Box::new(content.parse::<Def>()?);
			let style = if input.peek(Token![!]) {
				input.parse::<Token![!]>()?;
				DefGroupStyle::OneMustOccur
			} else if input.peek(Token![#]) {
				input.parse::<Token![#]>()?;
				DefGroupStyle::OneOrMore
			} else if input.peek(token::Brace) {
				let content;
				braced!(content in input);
				DefGroupStyle::Range(content.parse::<DefRange>()?)
			} else {
				DefGroupStyle::None
			};
			Self::Group(inner, style)
		} else if input.peek(Ident::peek_any) {
			Self::Ident(input.parse::<DefIdent>()?)
		} else {
			input.step(|cursor| {
				if let Some((p, next)) = cursor.punct() {
					return Ok((Self::Punct(p.as_char()), next));
				}
				Err(Error::new(input.span(), "unknown token!"))?
			})?
		};
		loop {
			if input.is_empty() {
				return Ok(root);
			} else if input.peek(Token![?]) {
				input.parse::<Token![?]>()?;
				let inner = root;
				root = Self::Optional(Box::new(inner));
			} else if input.peek(Token![+])
				|| input.peek(Token![#])
				|| input.peek(token::Brace)
				|| input.peek(Token![*])
			{
				let inner = root;
				let style = input.parse::<DefMultiplierStyle>()?;
				root = Self::Multiplier(Box::new(inner), style);
			} else {
				let style = if input.peek(Token![||]) {
					input.parse::<Token![||]>()?;
					DefCombinatorStyle::Options
				} else if input.peek(Token![|]) {
					input.parse::<Token![|]>()?;
					DefCombinatorStyle::Alternatives
				} else if input.peek(Token![&&]) {
					input.parse::<Token![&&]>()?;
					DefCombinatorStyle::AllMustOccur
				} else {
					DefCombinatorStyle::Ordered
				};
				let mut next = input.parse::<Def>()?;
				match (&mut root, &mut next) {
					(_, Self::Combinator(ref mut children, ref s)) if s == &style => {
						children.insert(0, root);
						root = next;
					}
					(Self::Combinator(ref mut children, ref s), _) if s == &style => {
						children.push(next);
					}
					(_, Self::Combinator(ref mut children, other_style)) if &style < other_style => {
						let options = Self::Combinator(vec![root, children.remove(0)], style);
						children.insert(0, options);
						root = next;
					}
					_ => {
						let children = vec![root, next];
						root = Self::Combinator(children, style);
					}
				}
			}
		}
	}
}

impl Parse for DefMultiplierStyle {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Token![*]) {
			input.parse::<Token![*]>()?;
			Ok(Self::ZeroOrMore)
		} else if input.peek(Token![+]) {
			input.parse::<Token![+]>()?;
			Ok(Self::OneOrMore)
		} else if input.peek(Token![#]) {
			input.parse::<Token![#]>()?;
			let range = if input.peek(token::Brace) {
				let content;
				braced!(content in input);
				content.parse::<DefRange>()?
			} else {
				DefRange::None
			};
			Ok(Self::OneOrMoreCommaSeparated(range))
		} else if input.peek(token::Brace) {
			let content;
			braced!(content in input);
			Ok(Self::Range(content.parse::<DefRange>()?))
		} else {
			Err(Error::new(input.span(), "Unknown token!"))?
		}
	}
}

impl Parse for DefIdent {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut str = "".to_owned();
		let mut last_was_ident = false;
		loop {
			if input.peek(Token![>]) || input.peek(token::Bracket) {
				return Ok(Self(Atom::from(str)));
			} else if input.peek(Ident::peek_any) && !last_was_ident {
				last_was_ident = true;
				let ident = input.call(Ident::parse_any)?;
				str.push_str(&ident.to_string());
			} else if input.peek(Token![-]) {
				last_was_ident = false;
				input.parse::<Token![-]>()?;
				str.push('-');
			} else {
				return Ok(Self(Atom::from(str)));
			}
		}
	}
}

impl Parse for DefType {
	fn parse(input: ParseStream) -> Result<Self> {
		input.parse::<Token![<]>()?;
		let ident =
			if input.peek(LitStr) { input.parse::<StrWrapped<DefIdent>>()?.0 } else { input.parse::<DefIdent>()? };
		let mut checks = DefRange::None;
		if input.peek(token::Bracket) {
			let content;
			bracketed!(content in input);
			checks = content.parse::<DefRange>()?;
		}
		let ty = match ident.0 {
			atom!("length") => Self::Length(checks),
			atom!("length-percentage") => Self::LengthPercentage(checks),
			atom!("angle") => Self::Angle(checks),
			atom!("time") => Self::Time(checks),
			atom!("resolution") => Self::Resolution(checks),
			atom!("integer") => Self::Integer(checks),
			atom!("number") => Self::Number(checks),
			atom!("percentage") => Self::Percentage(checks),
			atom!("string") => Self::String,
			atom!("color") => Self::Color,
			atom => {
				let notation = if input.peek(token::Paren) {
					let content;
					parenthesized!(content in input);
					if !content.is_empty() {
						Err(Error::new(input.span(), "unknown token!"))?
					}
					FunctionNotation::Yes
				} else {
					FunctionNotation::No
				};
				let str = pascal(atom.to_string());
				Self::Custom(DefIdent(Atom::from(str)), notation)
			}
		};
		input.parse::<Token![>]>()?;
		Ok(ty)
	}
}

impl Parse for DefRange {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut lhs = None;
		let mut rhs = None;
		if input.peek(LitFloat) {
			lhs = Some(input.parse::<LitFloat>()?.base10_parse()?);
		} else if input.peek(LitInt) {
			lhs = Some(input.parse::<LitInt>()?.base10_parse::<f32>()?);
		}
		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			if input.peek(LitFloat) {
				rhs = Some(input.parse::<LitFloat>()?.base10_parse()?);
			} else if input.peek(LitInt) {
				rhs = Some(input.parse::<LitInt>()?.base10_parse::<f32>()?);
			}
		}
		Ok(match (lhs, rhs) {
			(Some(start), Some(end)) => Self::Range(Range { start, end }),
			(None, Some(end)) => Self::RangeTo(RangeTo { end }),
			(Some(start), None) => Self::RangeFrom(RangeFrom { start }),
			(None, None) => Self::None,
		})
	}
}

pub enum DataType {
	SingleUnnamedStruct,
	Enum,
}

impl DataType {
	pub fn is_struct(&self) -> bool {
		matches!(self, Self::SingleUnnamedStruct)
	}

	pub fn is_enum(&self) -> bool {
		matches!(self, Self::Enum)
	}
}

impl Def {
	pub fn to_variant_name(&self, capture: Option<Ident>) -> TokenStream {
		match self {
			Self::Ident(v) => v.to_variant_name(),
			Self::Type(v) => v.to_variant_name(capture),
			_ => {
				dbg!("TODO variant name", self);
				todo!("variant name")
			}
		}
	}

	pub fn generated_data_type(&self) -> DataType {
		match self {
			Self::Combinator(_, DefCombinatorStyle::Alternatives) => DataType::Enum,
			_ => DataType::SingleUnnamedStruct,
		}
	}

	pub fn generate_peek_trait_implementation(&self, ident: &Ident) -> TokenStream {
		let steps = self.peek_steps();
		quote! {
			#[automatically_derived]
			impl<'a> ::hdx_parser::Peek<'a> for #ident {
				fn peek(parser: &::hdx_parser::Parser<'a>) -> Option<::hdx_lexer::Token> {
					use ::hdx_parser::Peek;
					#steps
				}
			}
		}
	}

	pub fn generate_parse_trait_implementation(&self, ident: &Ident) -> TokenStream {
		let steps = match self {
			Self::Ident(_) => quote! { compile_error!("cannot generate top level singular keyword") },
			Self::Type(ty) => {
				let steps = ty.parse_steps(Some(format_ident!("val")));
				quote! {
					#steps
					Ok(Self(val))
				}
			}
			Self::Optional(_) => quote! { compile_error!("cannot generate top level optional") },
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let (keywords, others): (Vec<&Def>, Vec<&Def>) =
					opts.iter().partition(|def| matches!(def, Def::Ident(_)));
				let other_if: Vec<TokenStream> = others
					.into_iter()
					.enumerate()
					.map(|(i, def)| {
						let peek = def.peek_steps();
						let parse = def.parse_steps(Some(format_ident!("val")));
						let var = def.to_variant_name(Some(format_ident!("val")));
						if i == 0 && keywords.is_empty() {
							quote! { if #peek { #parse } }
						} else {
							quote! { else if #peek.is_some() { #parse; Ok(Self::#var) } }
						}
					})
					.collect();
				let keyword_if = if keywords.is_empty() {
					None
				} else {
					let keyword_arms = keywords.into_iter().map(|def| {
						if let Def::Ident(ident) = def {
							let atom = ident.to_atom_macro();
							let variant_name = ident.to_variant_name();
							quote! { #atom => Self::#variant_name, }
						} else {
							quote! {}
						}
					});
					Some(quote! {
						if let Some(token) = parser.peek::<::hdx_parser::token::Ident>() {
						  parser.hop(token);
						  Ok(match parser.parse_atom_lower(token) {
							#(#keyword_arms)*
							atom => Err(::hdx_parser::diagnostics::UnexpectedIdent(atom, token.span()))?
						  })
						}
					})
				};
				let q = quote! {
					#keyword_if
					#(#other_if)*
					else {
						let token = parser.peek::<::hdx_parser::token::Any>().unwrap();
					  Err(::hdx_parser::diagnostics::Unexpected(token, token.span()))?
					}
				};
				q
			}
			Self::Combinator(_, _) => {
				quote! {
					todo!()
				}
			}
			Self::Group(_, _) => {
				dbg!("generate_parse_trait_implementation", self);
				todo!("generate_parse_trait_implementation")
			}
			Self::Multiplier(_, DefMultiplierStyle::ZeroOrMore) => {
				quote! { compile_error!("cannot generate top level multiplier of zero-or-more") }
			}
			Self::Multiplier(_, _) => {
				let parse_steps = self.parse_steps(Some(format_ident!("items")));
				quote! {
					#parse_steps
					Ok(Self(items))
				}
			}
			Self::Punct(_) => todo!(),
		};
		quote! {
			#[automatically_derived]
			impl<'a> ::hdx_parser::Parse<'a> for #ident {
				fn parse(parser: &mut ::hdx_parser::Parser<'a>) -> ::hdx_parser::Result<Self> {
		  use ::hdx_parser::Parse;
					#steps
				}
			}
		}
	}

	pub fn generate_writecss_trait_implementation(&self, ident: &Ident) -> TokenStream {
		let steps = match self {
			Self::Ident(_) => quote! { compile_error!("cannot generate top level singular keyword") },
			Self::Type(w) => w.write_steps(quote! { self.0 }),
			Self::Optional(_) => self.write_steps(quote! { self.0 }),
			Self::Combinator(_, DefCombinatorStyle::Ordered) => {
				dbg!("generate_writecss_trait_implementation Ordered TODO", self);
				todo!("generate_writecss_trait_implementation Ordered TODO")
			}
			Self::Combinator(_, DefCombinatorStyle::AllMustOccur) => {
				dbg!("generate_writecss_trait_implementation AllMustOccur TODO", self);
				todo!("generate_writecss_trait_implementation AllMustOccur TODO")
			}
			Self::Combinator(_, DefCombinatorStyle::Options) => {
				dbg!("generate_writecss_trait_implementation Options TODO", self);
				todo!("generate_writecss_trait_implementation Options TODO")
			}
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let arms: Vec<TokenStream> = opts
					.iter()
					.map(|def| {
						let name = format_ident!("inner");
						let var = def.to_variant_name(Some(name.clone()));
						let write = def.write_steps(quote! { #name });
						quote! { Self::#var => { #write } }
					})
					.collect();
				quote! {
					match self {
						#(#arms),*
					}
				}
			}
			Self::Group(_, _) => {
				dbg!("generate_writecss_trait_implementation Group TODO", self);
				todo!("generate_writecss_trait_implementation Group TODO")
			}
			Self::Multiplier(_, DefMultiplierStyle::ZeroOrMore) => {
				quote! { compile_error!("cannot generate top level multiplier of zero-or-more") }
			}
			Self::Multiplier(_, _) => self.write_steps(quote! { self.0 }),
			Self::Punct(_) => todo!(),
		};
		quote! {
			#[automatically_derived]
			impl<'a> ::hdx_writer::WriteCss<'a> for #ident {
				fn write_css<W: ::hdx_writer::CssWriter>(&self, sink: &mut W) -> ::hdx_writer::Result {
		  use ::hdx_writer::WriteCss;
					#steps
					Ok(())
				}
			}
		}
	}
}

impl GenerateDefinition for Def {
	fn generate_definition(&self, vis: &Visibility, ident: &Ident) -> TokenStream {
		match self.generated_data_type() {
			DataType::SingleUnnamedStruct => match self {
				Self::Type(ty) => {
					let modname = ty.to_type_name();
					quote! { #vis struct #ident(pub #modname); }
				}
				Self::Ident(_) => {
					Error::new(ident.span(), "cannot generate top level singular keyword").into_compile_error()
				}
				Self::Combinator(_, DefCombinatorStyle::Alternatives) => {
					Error::new(ident.span(), "cannot generate alternative combinators in struct").into_compile_error()
				}
				Self::Combinator(opts, _) => {
					let members: Vec<TokenStream> = opts
						.iter()
						.map(|def| match def {
							Self::Type(deftype) => {
								let ty = deftype.to_type_name();
								quote! { pub #ty }
							}
							Self::Multiplier(x, style) => match x.as_ref() {
								Def::Type(ty) => {
									let modname = ty.to_type_name();
									let n = style.smallvec_size_hint();
									quote! { pub ::smallvec::SmallVec<[#modname; #n]> }
								}
								_ => {
									dbg!("TODO Multiplier() variant", self);
									todo!("generated data type")
								}
							},
							Self::Optional(_) => {
								dbg!("todo combinator() field", self);
								todo!("generated data type")
							}
							_ => {
								dbg!("todo combinator() field", self);
								todo!("generated data type")
							}
						})
						.collect();
					quote! { #vis struct #ident(#(#members),*); }
				}
				Self::Multiplier(x, style) => match x.as_ref() {
					Def::Type(ty) => {
						let modname = ty.to_type_name();
						let n = style.smallvec_size_hint();
						quote! { #vis struct #ident(pub ::smallvec::SmallVec<[#modname; #n]>); }
					}
					_ => {
						dbg!("TODO Multiplier() variant", self);
						todo!("generated data type")
					}
				},
				_ => {
					dbg!("TODO variant", self);
					todo!("generate_definition match generated_data_type")
				}
			},
			DataType::Enum => match self {
				Self::Combinator(children, DefCombinatorStyle::Alternatives) => {
					let variants: Vec<TokenStream> = children.iter().map(|d| d.to_variant_name(None)).collect();
					quote! { #vis enum #ident { #(#variants),* } }
				}
				Self::Combinator(_, _) => {
					Error::new(ident.span(), "cannot generate non-Alternatives combinators in enum")
						.into_compile_error()
				}
				_ => {
					dbg!("TODO non union enum", self);
					todo!("non union enum")
				}
			},
		}
	}
}

impl GenerateWriteImpl for Def {
	fn write_steps(&self, capture: TokenStream) -> TokenStream {
		match self {
			Self::Type(ty) => ty.write_steps(capture),
			Self::Ident(ident) => ident.write_steps(capture),
			Self::Optional(option) => {
				let name = quote! { inner };
				let w = option.write_steps(name.clone());
				quote! {
					if let Some(#name) = #capture {
						#w
					}
				}
			}
			Self::Combinator(_, DefCombinatorStyle::Ordered) => {
				dbg!("generate_writecss_trait_implementation Ordered TODO", self);
				todo!("generate_writecss_trait_implementation Ordered TODO")
			}
			Self::Combinator(_, DefCombinatorStyle::AllMustOccur) => {
				dbg!("generate_writecss_trait_implementation AllMustOccur TODO", self);
				todo!("generate_writecss_trait_implementation AllMustOccur TODO")
			}
			Self::Combinator(_, DefCombinatorStyle::Options) => {
				dbg!("generate_writecss_trait_implementation Options TODO", self);
				todo!("generate_writecss_trait_implementation Options TODO")
			}
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let arms: Vec<TokenStream> = opts
					.iter()
					.map(|def| {
						let name = format_ident!("inner");
						let var = def.to_variant_name(Some(name.clone()));
						let write = def.write_steps(quote! { #name });
						quote! { Self::#var => { #write } }
					})
					.collect();
				quote! {
					match self {
						#(#arms),*
					}
				}
			}
			Self::Group(_, _) => {
				dbg!("generate_writecss_trait_implementation Group TODO", self);
				todo!("generate_writecss_trait_implementation Group TODO")
			}
			Self::Multiplier(def, style) => {
				let name = format_ident!("item");
				let post_write = match style {
					DefMultiplierStyle::OneOrMoreCommaSeparated(_) => quote! {
						::hdx_parser::token::Comma.write_css(sink)?;
						sink.write_whitespace(sink)?;
					},
					_ => quote! { sink.write_char(' ')?; },
				};
				let write = def.write_steps(quote! { #name });
				let do_write = def
					.will_write_cond_steps(quote! { #name })
					.map(|cond| {
						quote! {
							if #cond {
								#write
								if iter.peek().is_some() {
									#post_write
								}
							}
						}
					})
					.or_else(|| {
						Some(quote! {
							#write
							if iter.peek().is_some() {
								#post_write
							}
						})
					});
				quote! {
					let mut iter = #capture.iter().peekable();
					while let Some(#name) = iter.next() { #do_write }
				}
			}
			Self::Punct(_) => todo!(),
		}
	}
}

impl GeneratePeekImpl for Def {
	fn peek_steps(&self) -> TokenStream {
		match self {
			Self::Type(p) => p.peek_steps(),
			Self::Ident(p) => p.peek_steps(),
			Self::Optional(p) => p.peek_steps(),
			Self::Combinator(p, DefCombinatorStyle::Ordered) => p[0].peek_steps(),
			Self::Combinator(p, _) => {
				let peeks: Vec<TokenStream> = p
					.iter()
					.enumerate()
					.map(|(i, p)| {
						let steps = p.peek_steps();
						let expr = if i == 0 {
							quote! { #steps }
						} else {
							quote! { (|| #steps ) }
						};
						quote! { #expr.or_else }
					})
					.collect();
				quote! { #(#peeks)*(||None) }
			}
			Self::Group(p, _) => p.peek_steps(),
			Self::Multiplier(p, _) => p.peek_steps(),
			Self::Punct(_) => todo!(),
		}
	}
}

impl GenerateParseImpl for Def {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream {
		match self {
			Self::Type(p) => p.parse_steps(capture),
			Self::Ident(p) => p.parse_steps(capture),
			Self::Multiplier(def, DefMultiplierStyle::Range(range)) => {
				let peek_steps = def.peek_steps();
				let steps = def.parse_steps(Some(format_ident!("item")));
				let max_check = match range {
					DefRange::Range(Range { end, .. }) => {
						let n = *end as usize;
						quote! {
							if i > #n {
								break;
							} else
						}
					}
					_ => quote! {},
				};
				let min_check = match range {
					DefRange::None => quote! {},
					DefRange::RangeTo(_) => quote! { compile_error!("invalid range expression on multipler") },
					DefRange::RangeFrom(_) => quote! { compile_error!("from range multiplier is todo") },
					DefRange::Range(Range { start, .. }) => {
						let n = *start as usize;
						quote! {
							if i < #n {
								let token = parser.peek::<::hdx_parser::token::Any>().unwrap();
								return Err(::hdx_parser::diagnostics::Unexpected(token, token.span()))?
							}
						}
					}
				};
				let capture_name = capture.unwrap_or_else(|| format_ident!("items"));
				quote! {
					let mut i = 0;
					let mut #capture_name = ::smallvec::smallvec![];
					loop {
						#max_check
						if #peek_steps.is_some() {
							#steps
							i += 1;
							#capture_name.push(item)
						} else {
							break;
						}
					}
					#min_check
				}
			}
			_ => {
				dbg!("peek_steps", self);
				todo!("peek_steps");
			}
		}
	}
}

impl DefType {
	pub fn to_variant_name(&self, capture: Option<Ident>) -> TokenStream {
		let inner = if let Some(ident) = capture {
			quote! { #ident }
		} else {
			self.to_type_name()
		};
		match self {
			Self::Length(_) => quote! { Length(#inner) },
			Self::LengthPercentage(_) => quote! { LengthPercentage(#inner) },
			Self::Angle(_) => quote! { Angle(#inner) },
			Self::Time(_) => quote! { Time(#inner) },
			Self::Resolution(_) => quote! { Resolution(#inner) },
			Self::Integer(_) => quote! { Integer(#inner) },
			Self::Number(_) => quote! { Number(#inner) },
			Self::Percentage(_) => quote! { Percentage(#inner) },
			Self::String => quote! { String(#inner) },
			Self::Color => quote! { Color(#inner) },
			Self::Custom(ident, _) => quote! { #ident(#inner) },
		}
	}

	pub fn to_type_name(&self) -> TokenStream {
		match self {
			Self::Length(_) => quote! { Length },
			Self::LengthPercentage(_) => quote! { LengthPercentage },
			Self::Angle(_) => quote! { Angle },
			Self::Time(_) => quote! { Time },
			Self::Resolution(_) => quote! { Resolution },
			Self::Integer(_) => quote! { CSSInt },
			Self::Number(_) => quote! { CSSFloat },
			Self::Percentage(_) => quote! { Percentage },
			Self::Color => quote! { Color },
			Self::String => quote! { &'a str },
			Self::Custom(ty, _) => quote! { #ty },
		}
	}

	pub fn checks(&self) -> &DefRange {
		match self {
			Self::Length(c)
			| Self::LengthPercentage(c)
			| Self::Angle(c)
			| Self::Time(c)
			| Self::Resolution(c)
			| Self::Integer(c)
			| Self::Number(c)
			| Self::Percentage(c) => c,
			_ => &DefRange::None,
		}
	}
}

impl GenerateWriteImpl for DefType {
	fn write_steps(&self, capture: TokenStream) -> TokenStream {
		quote! { #capture.write_css(sink)?; }
	}
}

impl GeneratePeekImpl for DefType {
	fn peek_steps(&self) -> TokenStream {
		let name = self.to_type_name();
		quote! { parser.peek::<#name>() }
	}
}

impl GenerateParseImpl for DefType {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream {
		let capture_name = capture.unwrap_or_else(|| format_ident!("val"));
		let name = self.to_type_name();
		let checks = self.checks();
		let check_code = match checks {
			DefRange::RangeTo(RangeTo { end }) => {
				quote! {
					if #end < #capture_name.into() {
						return Err(::hdx_parser::diagnostics::NumberTooLarge(#end, ::hdx_lexer::Span::new(start, parser.offset())))?
					}
				}
			}
			DefRange::Range(Range { start, end }) => {
				quote! {
					if !(#start..#end).contains(#capture_name) {
						return Err(::hdx_parser::diagnostics::NumberOutOfBounds(#capture_name, "#start..#end", ::hdx_lexer::Span::new(start, parser.offset())))?
					}
				}
			}
			DefRange::RangeFrom(RangeFrom { start }) => {
				quote! {
					if #start > #capture_name.into() {
						return Err(::hdx_parser::diagnostics::NumberTooSmall(#start, ::hdx_lexer::Span::new(start, parser.offset())))?
					}
				}
			}
			DefRange::None => quote! {},
		};
		quote! {
			let start = parser.offset();
			let #capture_name = parser.parse::<#name>()?;
			#check_code
		}
	}
}

impl Display for DefIdent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl ToTokens for DefIdent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Ident::new(&self.to_string(), Span::call_site()));
	}
}

impl DefIdent {
	pub fn to_atom_macro(&self) -> TokenStream {
		let name = kebab(self.to_string());
		quote! { ::hdx_atom::atom!(#name) }
	}

	pub fn to_variant_name(&self) -> TokenStream {
		let variant_str = pascal(self.0.to_string());
		let ident = format_ident!("{}", variant_str);
		quote! { #ident }
	}
}

impl GenerateWriteImpl for DefIdent {
	fn write_steps(&self, _: TokenStream) -> TokenStream {
		let atom = self.to_atom_macro();
		quote! { #atom.write_css(sink)?; }
	}
}

impl GeneratePeekImpl for DefIdent {
	fn peek_steps(&self) -> TokenStream {
		quote! { parser.peek::<::hdx_parser::token::Ident>() }
	}
}

impl GenerateParseImpl for DefIdent {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream {
		let atom = self.to_atom_macro();
		quote! {
			let #capture = parser.parse::<Token![Ident]>().map_or(false, |t| parser.parse_atom_lower(t) == #atom);
		}
	}
}

impl DefMultiplierStyle {
	fn smallvec_size_hint(&self) -> usize {
		match self {
			Self::ZeroOrMore => 0,
			Self::OneOrMore => 1,
			Self::OneOrMoreCommaSeparated(_) => 1,
			Self::Range(DefRange::None) => 0,
			Self::Range(DefRange::Range(Range { start, .. }))
			| Self::Range(DefRange::RangeFrom(RangeFrom { start, .. })) => *start as usize,
			Self::Range(DefRange::RangeTo(RangeTo { end, .. })) => *end as usize,
		}
	}
}
