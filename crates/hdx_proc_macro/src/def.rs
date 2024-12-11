use hdx_atom::{atom, Atom};
use itertools::{Itertools, Position};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::{
	fmt::Display,
	ops::{Deref, Range, RangeFrom, RangeTo},
};
use syn::{
	braced, bracketed,
	ext::IdentExt,
	parenthesized,
	parse::{Parse, ParseStream},
	parse2, token, Error, GenericParam, Generics, Ident, Index, Lifetime, LifetimeParam, LitFloat, LitInt, LitStr,
	Result, Token, Visibility,
};

use crate::{kebab, pascal};

pub(crate) struct StrWrapped<T: Parse>(pub T);
impl<T: Parse> Parse for StrWrapped<T> {
	fn parse(input_raw: ParseStream) -> Result<Self> {
		Ok(Self(parse2::<T>(
			input_raw.parse::<LitStr>()?.value().replace("'", "\"").replace("∞", "").parse::<TokenStream>()?,
		)?))
	}
}

pub trait GenerateDefinition {
	fn generate_definition(&self, vis: &Visibility, ident: &Ident, generics: &mut Generics) -> TokenStream;
}

pub trait GeneratePeekImpl {
	fn peek_steps(&self) -> TokenStream;
}

pub trait GenerateParseImpl: GeneratePeekImpl {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream;
}

pub trait GenerateToCursorsImpl {
	fn to_cursors_steps(&self, capture: TokenStream) -> TokenStream;
	fn will_write_cond_steps(&self, _capture: TokenStream) -> Option<TokenStream> {
		None
	}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Def {
	Ident(DefIdent),
	Function(DefIdent, Box<Def>),
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
	Image,
	Image1D,
	DashedIdent,
	CustomIdent,
	Custom(DefIdent, DefIdent),
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
			let ident = input.parse::<DefIdent>()?;
			if input.peek(token::Paren) {
				let content;
				parenthesized!(content in input);
				Self::Function(ident, Box::new(content.parse::<Def>()?))
			} else {
				Self::Ident(ident)
			}
		} else {
			input.step(|cursor| {
				if let Some((p, next)) = cursor.punct() {
					return Ok((Self::Punct(p.as_char()), next));
				}
				Err(Error::new(input.span(), "unknown token in Def parse"))?
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
			Err(Error::new(input.span(), "Unknown token in DefMultiplierStyle parse!"))?
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
			// LitInt might pick up identifier parts like "3d"
			} else if input.peek(LitInt) && last_was_ident {
				last_was_ident = true;
				let int = input.parse::<LitInt>()?;
				str.push_str(&int.to_string());
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
			atom!("image") => Self::Image,
			atom!("image-1D") => Self::Image1D,
			atom!("dashed-ident") => Self::DashedIdent,
			atom!("custom-ident") => Self::CustomIdent,
			atom => {
				let iden = DefIdent(Atom::from(pascal(atom.to_string())));
				let mut str = pascal(atom.to_string()).to_owned();
				if input.peek(token::Paren) {
					let content;
					parenthesized!(content in input);
					if !content.is_empty() {
						Err(Error::new(input.span(), "disallowed content inside deftype function"))?
					}
					str.push_str("Function");
				}
				Self::Custom(iden, DefIdent(Atom::from(str)))
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
	pub fn to_variant_name(&self, size_hint: usize) -> TokenStream {
		match self {
			Self::Ident(v) => v.to_variant_name(size_hint),
			Self::Type(v) => v.to_variant_name(size_hint),
			Self::Function(v, _) => {
				let variant_str = pascal(v.0.to_string());
				let ident = format_ident!("{}Function", variant_str);
				quote! { #ident }
			}
			Self::Multiplier(v, _) => v.deref().to_variant_name(2),
			Self::Group(def, _) => def.deref().to_variant_name(size_hint),
			_ => {
				dbg!("TODO variant name", self);
				todo!("variant name")
			}
		}
	}

	pub fn to_variant_type(&self, size_hint: usize, extra: Option<TokenStream>) -> TokenStream {
		let name = self.to_variant_name(size_hint);
		match self {
			Self::Ident(_) => quote! { #name(::hdx_parser::T![Ident]) },
			Self::Type(v) => v.to_variant_type(size_hint, extra),
			Self::Function(_, ty) => {
				let life = if self.requires_allocator_lifetime() { Some(quote! { <'a> }) } else { None };
				match ty.deref() {
					Def::Type(ty) => {
						let inner = ty.to_type_name();
						quote! { #name(::hdx_parser::T![Function], #inner #life, Option<::hdx_parser::T![')']>) }
					}
					Def::Multiplier(def, style) => {
						let extra = if matches!(style, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) {
							Some(quote! { Option<::hdx_parser::T![,]> })
						} else {
							None
						};
						let inner = match def.deref() {
							Def::Type(v) => v.to_inner_variant_type(2, extra),
							_ => {
								dbg!("TODO function multiplier inner variant", self);
								todo!("function multiplier inner variant")
							}
						};
						quote! { #name(::hdx_parser::T![Function], #inner, Option<::hdx_parser::T![')']>) }
					}
					_ => {
						dbg!("TODO function variant", self);
						todo!("function variant")
					}
				}
			}
			Self::Combinator(_def, _) => {
				dbg!("TODO variant name", self);
				todo!("variant name")
			}
			Self::Multiplier(def, style) => {
				let extra = if matches!(style, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) {
					Some(quote! { Option<::hdx_parser::T![,]> })
				} else {
					None
				};
				def.deref().to_variant_type(2, extra)
			}
			_ => {
				dbg!("TODO variant name", self);
				todo!("variant name")
			}
		}
	}

	pub fn requires_allocator_lifetime(&self) -> bool {
		match self {
			Self::Ident(_) => false,
			Self::Function(_, d) => d.requires_allocator_lifetime(),
			Self::Type(d) => d.requires_allocator_lifetime(),
			Self::Optional(d) => d.requires_allocator_lifetime(),
			Self::Combinator(ds, _) => ds.iter().any(|d| d.requires_allocator_lifetime()),
			Self::Group(d, _) => d.requires_allocator_lifetime(),
			Self::Multiplier(_, style) => {
				// Bounded multipliers get optimized into struct of options
				if let DefMultiplierStyle::Range(range) = style {
					matches!(range, DefRange::RangeFrom(_))
				} else {
					true
				}
			}
			Self::Punct(_) => false,
		}
	}

	pub fn generated_data_type(&self) -> DataType {
		match self {
			Self::Combinator(_, DefCombinatorStyle::Alternatives) => DataType::Enum,
			_ => DataType::SingleUnnamedStruct,
		}
	}

	pub fn generate_peek_trait_implementation(&self, ident: &Ident, generics: &mut Generics) -> TokenStream {
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (gen, _, _) = generics.split_for_impl();
		let steps = self.peek_steps();
		quote! {
			#[automatically_derived]
			impl<'a> ::hdx_parser::Peek<'a> for #ident #gen {
				fn peek(p: &::hdx_parser::Parser<'a>) -> bool {
					use ::hdx_parser::Peek;
					#steps
				}
			}
		}
	}

	pub fn generate_parse_trait_implementation(&self, ident: &Ident, generics: &mut Generics) -> TokenStream {
		let steps = match self {
			Self::Ident(_) => quote! { compile_error!("cannot generate top level singular keyword") },
			Self::Type(ty) => {
				let steps = ty.parse_steps(Some(format_ident!("val")));
				quote! {
					#steps
					Ok(Self(val))
				}
			}
			Self::Function(_, _) => quote! { compile_error!("cannot generate top level function") },
			Self::Optional(_) => quote! { compile_error!("cannot generate top level optional") },
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let (keywords, others): (Vec<&Def>, Vec<&Def>) = opts.iter().partition(|def| {
					matches!(def, Def::Ident(_) | Def::Type(DefType::CustomIdent) | Def::Type(DefType::DashedIdent))
				});
				let other_if: Vec<TokenStream> = others
					.into_iter()
					.with_position()
					.map(|(p, def)| {
						let peek = def.peek_steps();
						let parse = def.parse_steps(Some(format_ident!("val")));
						let var = def.to_variant_name(0);
						let val = match def {
							Def::Group(def, DefGroupStyle::None) => match def.deref() {
								Def::Combinator(opts, DefCombinatorStyle::Options) => {
									let idents: Vec<Ident> =
										(0..opts.len()).map(|i| format_ident!("val{}", i)).collect();
									quote! { #(#idents),* }
								}
								_ => quote! { val },
							},
							Def::Function(_, _) => quote! { function, val, close },
							_ => quote! { val },
						};
						// If it's the only parse block we don't need to peek, just return it.
						if p == Position::Only {
							quote! { #parse; Ok(Self::#var(#val)) }
						} else {
							quote! { if #peek { #parse; return Ok(Self::#var(#val)); } }
						}
					})
					.collect();
				let keyword_if = if keywords.is_empty() {
					None
				} else {
					let mut last_arm = if other_if.is_empty() {
						quote! {
							atom => Err(::hdx_parser::diagnostics::UnexpectedIdent(atom, c.into()))?
						}
					} else {
						// likely cant Err as other Alternatives might use idents
						quote! { atom => {} }
					};
					let keyword_arms = keywords.into_iter().map(|def| {
						if let Def::Ident(ident) = def {
							let atom = ident.to_atom_macro();
							let variant_name = ident.to_variant_name(0);
							quote! { #atom => {
								return Ok(Self::#variant_name(p.parse::<::hdx_parser::T![Ident]>()?));
							} }
						} else if def == &Def::Type(DefType::CustomIdent) {
							last_arm = quote! {
								_ => { return Ok(Self::CustomIdent(p.parse::<::hdx_parser::T![Ident]>()?)); }
							};
							quote! {}
						} else {
							quote! {}
						}
					});
					let error = if other_if.is_empty() {
						Some(quote! {
							let c: ::hdx_lexer::Cursor = p.parse::<::hdx_parser::T![Any]>()?.into();
							Err(::hdx_parser::diagnostics::Unexpected(c.into(), c.into()))?
						})
					} else {
						None
					};
					Some(quote! {
						if p.peek::<::hdx_parser::T![Ident]>() {
							let c = p.peek_n(1);
							match p.parse_atom_lower(c) {
								#(#keyword_arms)*
								#last_arm
							}
						}
						#error
					})
				};
				if other_if.is_empty() {
					quote! { #keyword_if }
				} else if other_if.len() == 1 {
					quote! {
						#keyword_if
						#(#other_if)*
					}
				} else {
					quote! {
						#keyword_if
						#(#other_if)*;
							let c: ::hdx_lexer::Cursor = p.parse::<::hdx_parser::T![Any]>()?.into();
							Err(::hdx_parser::diagnostics::Unexpected(c.into(), c.into()))?
					}
				}
			}
			Self::Combinator(opts, DefCombinatorStyle::Options) => {
				let idents: Vec<Ident> = (0..opts.len()).map(|i| format_ident!("val{}", i)).collect();
				let steps: Vec<TokenStream> = opts
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let ident = format_ident!("val{}", i);
						let ty = match def {
							Def::Type(ty) => ty.to_type_name(),
							_ => {
								dbg!("generate_parse_trait_implementation type on group options", self);
								todo!("generate_parse_trait_implementation type on group options")
							}
						};
						quote! {
							if #ident.is_none() && p.peek::<#ty>() {
								#ident = Some(p.parse::<#ty>()?);
								continue;
							}
						}
					})
					.collect();
				quote! {
					#(let mut #idents = None);*;
					loop {
						#(#steps)*
						if #(#idents.is_none())&&* {
							let c: ::hdx_lexer::Cursor = p.parse::<::hdx_parser::T![Any]>()?.into();
							Err(::hdx_parser::diagnostics::Unexpected(c.into(), c.into()))?
						} else {
							return Ok(Self(#(#idents),*));
						}
					}
				}
			}
			Self::Combinator(defs, DefCombinatorStyle::Ordered) => {
				let idents: Vec<Ident> = (0..defs.len()).map(|i| format_ident!("val{}", i)).collect();
				let steps: Vec<TokenStream> =
					defs.iter().enumerate().map(|(i, def)| def.parse_steps(Some(format_ident!("val{}", i)))).collect();
				quote! {
					#(#steps)*
					Ok(Self(#(#idents),*))
				}
			}
			Self::Combinator(_, DefCombinatorStyle::AllMustOccur) => {
				dbg!("generate_parse_trait_implementation", self);
				todo!("generate_parse_trait_implementation")
			}
			Self::Group(_, _) => {
				dbg!("generate_parse_trait_implementation", self);
				todo!("generate_parse_trait_implementation")
			}
			Self::Multiplier(_, DefMultiplierStyle::ZeroOrMore) => {
				quote! { compile_error!("cannot generate top level multiplier of zero-or-more") }
			}
			Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Range(Range { start, end }))) => {
				// Optimize for bounded ranges like `<foo>{1,2}` which could be expressed as `Foo, Option<Foo>`
				let opts: Vec<Def> = (1..=*end as i32)
					.map(|i| if i <= (*start as i32) { def.deref().clone() } else { Self::Optional(def.clone()) })
					.collect();
				return Self::Combinator(opts, DefCombinatorStyle::Ordered)
					.generate_parse_trait_implementation(ident, generics);
			}
			Self::Multiplier(_, _) => {
				let parse_steps = self.parse_steps(Some(format_ident!("items")));
				quote! {
					#parse_steps
					return Ok(Self(items));
				}
			}
			Self::Punct(_) => todo!(),
		};
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (gen, _, _) = generics.split_for_impl();
		quote! {
			#[automatically_derived]
			impl<'a> ::hdx_parser::Parse<'a> for #ident #gen {
				fn parse(p: &mut ::hdx_parser::Parser<'a>) -> ::hdx_parser::Result<Self> {
					use ::hdx_parser::Parse;
					#steps
				}
			}
		}
	}

	pub fn generate_tocursors_trait_implementation(&self, ident: &Ident, generics: &mut Generics) -> TokenStream {
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (gen, _, _) = generics.split_for_impl();
		let steps = match self {
			Self::Ident(_) => quote! { compile_error!("cannot generate top level singular keyword") },
			Self::Type(_) => quote! { ::hdx_parser::ToCursors::to_cursors(&self.0, s); },
			Self::Optional(_) => {
				let steps = self.to_cursors_steps(quote! { inner });
				quote! {
					if let Some(inner) = &self.0 {
						#steps
					}
				}
			}
			Self::Function(_, _) => quote! { compile_error!("cannot generate top level singular keyword") },
			Self::Combinator(opts, DefCombinatorStyle::Ordered) => {
				let steps: Vec<TokenStream> = opts
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let index = Index { index: i as u32, span: Span::call_site() };
						match def {
							Def::Optional(_) => {
								quote! {
									if let Some(inner) = &self.#index {
										::hdx_parser::ToCursors::to_cursors(inner, s);
									}
								}
							}
							_ => {
								quote! {
									::hdx_parser::ToCursors::to_cursors(&self.#index, s);
								}
							}
						}
					})
					.collect();
				quote! { #(#steps)* }
			}
			Self::Combinator(_, DefCombinatorStyle::AllMustOccur) => {
				dbg!("generate_tocursors_trait_implementation AllMustOccur TODO", self);
				todo!("generate_tocursors_trait_implementation AllMustOccur TODO")
			}
			Self::Combinator(opts, DefCombinatorStyle::Options) => {
				let steps: Vec<TokenStream> = opts
					.iter()
					.enumerate()
					.map(|(i, _)| {
						let index = Index { index: i as u32, span: Span::call_site() };
						quote! {
							if let Some(inner) = &self.#index {
								::hdx_parser::ToCursors::to_cursors(inner, s);
							}
						}
					})
					.collect();
				quote! {
					#(#steps)*
				}
			}
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let arms: Vec<TokenStream> = opts
					.iter()
					.map(|def| {
						let name = match def {
							Self::Group(def, DefGroupStyle::None) => match def.deref() {
								Self::Combinator(opts, DefCombinatorStyle::Options) => {
									let idents: Vec<Ident> =
										(0..opts.len()).map(|i| format_ident!("inner{}", i)).collect();
									quote! { #(#idents),* }
								}
								_ => {
									let ident = format_ident!("inner");
									quote! { #ident }
								}
							},
							Self::Function(_, _) => quote! { function, val, close },
							_ => {
								let ident = format_ident!("inner");
								quote! { #ident }
							}
						};
						let var = def.to_variant_name(0);
						let step = if matches!(def, Self::Function(_, _)) {
							def.to_cursors_steps(quote! { val })
						} else {
							def.to_cursors_steps(quote! { #name })
						};
						quote! { Self::#var(#name) => { #step } }
					})
					.collect();
				quote! {
					match self {
						#(#arms),*
					}
				}
			}
			Self::Group(_, _) => {
				dbg!("generate_tocursors_trait_implementation Group TODO", self);
				todo!("generate_tocursors_trait_implementation Group TODO")
			}
			Self::Multiplier(_, DefMultiplierStyle::ZeroOrMore) => {
				quote! { compile_error!("cannot generate top level multiplier of zero-or-more") }
			}
			Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Range(Range { start, end }))) => {
				// Optimize for bounded ranges like `<foo>{1,2}` which could be expressed as `Foo, Option<Foo>`
				let opts: Vec<Def> = (1..=*end as i32)
					.map(|i| if i <= (*start as i32) { def.deref().clone() } else { Self::Optional(def.clone()) })
					.collect();
				return Self::Combinator(opts, DefCombinatorStyle::Ordered)
					.generate_tocursors_trait_implementation(ident, generics);
			}
			Self::Multiplier(_, _) => self.to_cursors_steps(quote! { &self.0 }),
			Self::Punct(_) => todo!(),
		};
		quote! {
			#[automatically_derived]
			impl<'a> ::hdx_parser::ToCursors<'a> for #ident #gen {
				fn to_cursors(&self, s: &mut ::hdx_parser::CursorStream<'a>) {
					#steps
				}
			}
		}
	}
}

impl GenerateDefinition for Def {
	fn generate_definition(&self, vis: &Visibility, ident: &Ident, generics: &mut Generics) -> TokenStream {
		let life = if self.requires_allocator_lifetime() { Some(quote! { <'a> }) } else { None };
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (_, gen, _) = generics.split_for_impl();
		match self.generated_data_type() {
			DataType::SingleUnnamedStruct => match self {
				Self::Type(ty) => {
					let modname = ty.to_type_name();
					quote! { #vis struct #ident #gen(pub #modname #life); }
				}
				Self::Ident(_) => {
					Error::new(ident.span(), "cannot generate top level singular keyword").into_compile_error()
				}
				Self::Combinator(_, DefCombinatorStyle::Alternatives) => {
					Error::new(ident.span(), "cannot generate alternative combinators in struct").into_compile_error()
				}
				Self::Combinator(opts, DefCombinatorStyle::Options) => {
					let members: Vec<TokenStream> = opts
						.iter()
						.map(|def| match def {
							Self::Type(deftype) => {
								let ty = deftype.to_type_name();
								let life =
									if deftype.requires_allocator_lifetime() { Some(quote! { <'a> }) } else { None };
								quote! { pub Option<#ty #life> }
							}
							Self::Multiplier(x, style) => match x.as_ref() {
								Def::Type(ty) => {
									let modname = if matches!(style, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) {
										let modname = ty.to_type_name();
										quote! { (#modname, Option<::hdx_parser::T![,]>) }
									} else {
										ty.to_type_name()
									};

									quote! { pub ::bumpalo::collections::Vec<'a, #modname> }
								}
								_ => {
									dbg!("TODO Multiplier() variant", self);
									todo!("generated data type")
								}
							},
							Self::Optional(b) => match b.deref() {
								Def::Type(def_type) => {
									let ty = def_type.to_type_name();
									let life = if def_type.requires_allocator_lifetime() {
										Some(quote! { <'a> })
									} else {
										None
									};
									quote! { pub Option<#ty #life> }
								}
								_ => {
									dbg!("todo combinator() optional field", self);
									todo!("generated data type")
								}
							},
							_ => {
								dbg!("todo combinator() field", self);
								todo!("generated data type")
							}
						})
						.collect();
					quote! { #vis struct #ident #gen(#(#members),*); }
				}
				Self::Combinator(opts, _) => {
					let members: Vec<TokenStream> = opts
						.iter()
						.map(|def| match def {
							Self::Type(deftype) => {
								let ty = deftype.to_type_name();
								let life =
									if deftype.requires_allocator_lifetime() { Some(quote! { <'a> }) } else { None };
								quote! { pub #ty #life }
							}
							Self::Multiplier(x, style) => match x.as_ref() {
								Def::Type(ty) => {
									let modname = if matches!(style, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) {
										let modname = ty.to_type_name();
										quote! { (#modname, Option<::hdx_parser::T![,]>) }
									} else {
										ty.to_type_name()
									};
									quote! { pub ::bumpalo::collections::Vec<'a, #modname> }
								}
								_ => {
									dbg!("TODO Multiplier() variant", self);
									todo!("generated data type")
								}
							},
							Self::Optional(b) => match b.deref() {
								Def::Type(def_type) => {
									let ty = def_type.to_type_name();
									let life = if def_type.requires_allocator_lifetime() {
										Some(quote! { <'a> })
									} else {
										None
									};
									quote! { pub Option<#ty #life> }
								}
								_ => {
									dbg!("todo combinator() optional field", self);
									todo!("generated data type")
								}
							},
							_ => {
								dbg!("todo combinator() field", self);
								todo!("generated data type")
							}
						})
						.collect();
					quote! { #vis struct #ident #gen(#(#members),*); }
				}
				Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Range(Range { start, end }))) => {
					// Optimize for bounded ranges like `<foo>{1,2}` which could be expressed as `Foo, Option<Foo>`
					let opts: Vec<Def> = (1..=*end as i32)
						.map(|i| if i <= (*start as i32) { def.deref().clone() } else { Self::Optional(def.clone()) })
						.collect();
					Self::Combinator(opts, DefCombinatorStyle::Ordered).generate_definition(vis, ident, generics)
				}
				Self::Multiplier(
					def,
					DefMultiplierStyle::OneOrMoreCommaSeparated(DefRange::Range(Range { start, end })),
				) => {
					// Optimize for bounded ranges like `<foo>{1,2}` which could be expressed as `Foo, Option<Foo>`
					let opts: Vec<Def> = (1..=*end as i32)
						.map(|i| if i <= (*start as i32) { def.deref().clone() } else { Self::Optional(def.clone()) })
						.collect();
					Self::Combinator(opts, DefCombinatorStyle::Ordered).generate_definition(vis, ident, generics)
				}
				Self::Multiplier(x, style) => match x.as_ref() {
					Def::Type(ty) => {
						let modname = if matches!(style, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) {
							let modname = ty.to_type_name();
							quote! { (#modname, Option<::hdx_parser::T![,]>) }
						} else {
							ty.to_type_name()
						};
						quote! { #vis struct #ident #gen(pub ::bumpalo::collections::Vec<'a, #modname>); }
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
					let variants: Vec<TokenStream> = children.iter().map(|d| d.to_variant_type(0, None)).collect();
					quote! { #vis enum #ident #gen { #(#variants),* } }
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

impl GenerateToCursorsImpl for Def {
	fn to_cursors_steps(&self, capture: TokenStream) -> TokenStream {
		match self {
			Self::Type(ty) => ty.to_cursors_steps(capture),
			Self::Ident(ident) => ident.to_cursors_steps(capture),
			Self::Function(_, def) => {
				let steps = def.to_cursors_steps(capture);
				quote! {
					// let function, arg, close = #capture
					s.append(function.into());
					#steps
					if let Some(close) = close {
						s.append(close.into());
					}
				}
			}
			Self::Optional(option) => {
				let name = quote! { inner };
				let w = option.to_cursors_steps(name.clone());
				quote! {
					if let Some(#name) = #capture {
						#w
					}
				}
			}
			Self::Combinator(opts, DefCombinatorStyle::Ordered) => {
				let exprs: Vec<TokenStream> = (0..opts.len())
					.map(|i| {
						let index = Index { index: i as u32, span: Span::call_site() };
						quote! { ::hdx_parser::ToCursors::to_cursors(&self::#index, s); }
					})
					.collect();
				quote! {
					#(#exprs)*
					Ok(())
				}
			}
			Self::Combinator(_, DefCombinatorStyle::AllMustOccur) => {
				dbg!("generate_tocursors_trait_implementation AllMustOccur TODO", self);
				todo!("generate_tocursors_trait_implementation AllMustOccur TODO")
			}
			Self::Combinator(opts, DefCombinatorStyle::Options) => {
				let arms: Vec<TokenStream> = opts
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let name = format_ident!("inner{}", i);
						if i == 0 {
							def.to_cursors_steps(quote! { #name })
						} else {
							def.to_cursors_steps(quote! {
								s.write_char(' ')?;
								#name
							})
						}
					})
					.collect();
				quote! {
					#(#arms)*
				}
			}
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let arms: Vec<TokenStream> = opts
					.iter()
					.map(|def| {
						let name = format_ident!("inner");
						let var = def.to_variant_name(0);
						let write = def.to_cursors_steps(quote! { #name });
						quote! { Self::#var(#name) => { #write } }
					})
					.collect();
				quote! {
					match self {
						#(#arms),*
					}
				}
			}
			Self::Group(def, DefGroupStyle::None) => def.to_cursors_steps(capture),
			Self::Group(_, _) => {
				dbg!("generate_tocursors_trait_implementation Group TODO", self);
				todo!("generate_tocursors_trait_implementation Group TODO")
			}
			Self::Multiplier(def, style) => {
				let name = format_ident!("item");
				let step = if matches!(style, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) {
					let step = def.to_cursors_steps(quote! { item });
					quote! {
						let (item, comma) = #name;
						#step
						if let Some(comma) = comma {
							s.append(comma.into());
						}
					}
				} else {
					def.to_cursors_steps(quote! { #name })
				};
				let do_step = def
					.will_write_cond_steps(quote! { #name })
					.map(|cond| {
						quote! {
							if #cond {
								#step
							}
						}
					})
					.or_else(|| {
						Some(quote! {
							#step
						})
					});
				quote! {
					for #name in #capture { #do_step }
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
			Self::Function(_, _) => quote! { p.peek::<::hdx_parser::T![Function]>() },
			Self::Optional(p) => p.peek_steps(),
			Self::Combinator(p, DefCombinatorStyle::Ordered) => p[0].peek_steps(),
			Self::Combinator(p, _) => {
				let peeks: Vec<TokenStream> = p
					.iter()
					.map(|p| p.peek_steps())
					.unique_by(|tok| tok.to_string())
					.with_position()
					.map(|(i, steps)| {
						if i == Position::First || i == Position::Only {
							quote! { #steps }
						} else {
							quote! { || #steps }
						}
					})
					.collect();
				quote! { #(#peeks)* }
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
			Self::Function(p, ty) => {
				let atom = p.to_atom_macro();
				let inner = ty.parse_steps(capture);
				quote! {
					let function = p.parse::<::hdx_parser::T![Function]>()?;
					let c: hdx_lexer::Cursor = function.into();
					let atom = p.parse_atom_lower(c.into());
					if atom != #atom {
						return Err(::hdx_parser::diagnostics::UnexpectedFunction(atom, c.into()))?
					}
					#inner
					let close = p.parse_if_peek::<::hdx_parser::T![')']>()?;
				}
			}
			Self::Multiplier(
				def,
				DefMultiplierStyle::Range(range) | DefMultiplierStyle::OneOrMoreCommaSeparated(range),
			) => {
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
								let c: ::hdx_lexer::Cursor = p.parse::<::hdx_parser::T![Any]>()?.into();
								Err(::hdx_parser::diagnostics::Unexpected(c.into(), c.into()))?
							}
						}
					}
				};
				let instantiate_i =
					if matches!(range, DefRange::None) { None } else { Some(quote! { let mut i = 0; }) };
				let increment_i = if matches!(range, DefRange::None) { None } else { Some(quote! { i += 1; }) };
				let capture_name = capture.unwrap_or_else(|| format_ident!("items"));
				let inloop = if matches!(self, Self::Multiplier(_, DefMultiplierStyle::OneOrMoreCommaSeparated(_))) {
					quote! {
						#steps
						let comma = p.parse_if_peek::<::hdx_parser::T![,]>()?;
						#capture_name.push((item, comma));
						#increment_i
						if comma.is_none() {
							break;
						}
					}
				} else {
					quote! {
						if #peek_steps {
							#steps
							#increment_i
							#capture_name.push(item)
						} else {
							break;
						}
					}
				};
				quote! {
					#instantiate_i
					let mut #capture_name = ::bumpalo::collections::Vec::new_in(p.bump());
					loop {
						#max_check
						#inloop
					}
					#min_check
				}
			}
			Self::Optional(def) => match def.deref() {
				Def::Type(ty) => {
					let ty = ty.to_type_name();
					let step = quote! { p.parse_if_peek::<#ty>()?; };
					if let Some(capture_name) = capture {
						quote! { let #capture_name = #step; }
					} else {
						step
					}
				}
				_ => {
					dbg!("parse_steps for Self::Optional def", self);
					todo!("parse_steps for Self::Optional def")
				}
			},
			Self::Combinator(opts, DefCombinatorStyle::Options) => {
				let inner = capture.unwrap_or_else(|| format_ident!("val"));
				let idents: Vec<Ident> = (0..opts.len()).map(|i| format_ident!("{}{}", inner, i)).collect();
				let steps: Vec<TokenStream> = opts
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let ident = format_ident!("{}{}", inner, i);
						let ty = match def {
							Def::Type(ty) => ty.to_type_name(),
							_ => {
								dbg!("generate_parse_trait_implementation type on group options", self);
								todo!("generate_parse_trait_implementation type on group options")
							}
						};
						quote! {
							if #ident.is_none() && p.peek::<#ty>() {
								#ident = Some(p.parse::<#ty>()?);
								continue;
							}
						}
					})
					.collect();
				quote! {
					#(let mut #idents = None);*;
					loop {
						#(#steps)*
						if #(#idents.is_none())&&* {
							let c: ::hdx_lexer::Cursor = p.parse::<::hdx_parser::T![Any]>()?.into();
							Err(::hdx_parser::diagnostics::Unexpected(c.into(), c.into()))?
						} else {
							break;
						}
					}
				}
			}
			Self::Group(def, DefGroupStyle::None) => def.parse_steps(capture),
			_ => {
				dbg!("parse_steps", self);
				todo!("parse_steps");
			}
		}
	}
}

impl DefType {
	pub fn to_variant_name(&self, size_hint: usize) -> TokenStream {
		if size_hint > 0 {
			match self {
				Self::Length(_) => quote! { Lengths },
				Self::LengthPercentage(_) => quote! { LengthPercentages },
				Self::Percentage(_) => quote! { Percentages },
				Self::Angle(_) => quote! { Angles },
				Self::Time(_) => quote! { Times },
				Self::Resolution(_) => quote! { Resolutions },
				Self::Integer(_) => quote! { Integers },
				Self::Number(_) => quote! { Numbers },
				Self::String => quote! { Strings },
				Self::Color => quote! { Colors },
				Self::Image => quote! { Images },
				Self::Image1D => quote! { Images },
				Self::DashedIdent => quote! { DashedIdents },
				Self::CustomIdent => quote! { CustomIdents },
				Self::Custom(_, ident) => {
					let ident = ident.pluralize();
					quote! { #ident }
				}
			}
		} else {
			match self {
				Self::Length(_) => quote! { Length },
				Self::LengthPercentage(_) => quote! { LengthPercentage },
				Self::Percentage(_) => quote! { Percentage },
				Self::Angle(_) => quote! { Angle },
				Self::Time(_) => quote! { Time },
				Self::Resolution(_) => quote! { Resolution },
				Self::Integer(_) => quote! { Integer },
				Self::Number(_) => quote! { Number },
				Self::String => quote! { String },
				Self::Color => quote! { Color },
				Self::Image => quote! { Image },
				Self::Image1D => quote! { Image },
				Self::DashedIdent => quote! { DashedIdent },
				Self::CustomIdent => quote! { CustomIdent },
				Self::Custom(_, ident) => quote! { #ident },
			}
		}
	}

	pub fn to_variant_type(&self, size_hint: usize, extra: Option<TokenStream>) -> TokenStream {
		let inner = self.to_inner_variant_type(size_hint, extra);
		let name = self.to_variant_name(size_hint);
		quote! { #name(#inner) }
	}

	pub fn to_inner_variant_type(&self, size_hint: usize, extra: Option<TokenStream>) -> TokenStream {
		let type_name = self.to_type_name();
		let life = if self.requires_allocator_lifetime() { Some(quote! { <'a> }) } else { None };
		if size_hint > 0 {
			if let Some(extra) = extra {
				quote! { ::bumpalo::collections::Vec::<'a, (#type_name #life, #extra)> }
			} else {
				quote! { ::bumpalo::collections::Vec::<'a, #type_name #life> }
			}
		} else if let Some(extra) = extra {
			quote! { (#type_name #life, #extra) }
		} else {
			quote! { #type_name #life }
		}
	}

	pub fn to_type_name(&self) -> TokenStream {
		match self {
			Self::Length(_) => quote! { types::Length },
			Self::LengthPercentage(_) => quote! { types::LengthPercentage },
			Self::Percentage(_) => quote! { types::CSSFloat },
			Self::Angle(_) => quote! { types::Angle },
			Self::Time(_) => quote! { types::Time },
			Self::Resolution(_) => quote! { types::Resolution },
			Self::Integer(_) => quote! { types::CSSInt },
			Self::Number(_) => quote! { types::CSSFloat },
			Self::Color => quote! { types::Color },
			Self::Image => quote! { types::Image },
			Self::Image1D => quote! { types::Image1D },
			Self::DashedIdent => quote! { ::hdx_parser::T![DashedIdent] },
			Self::CustomIdent => quote! { ::hdx_parser::T![Ident] },
			Self::String => quote! { ::hdx_parser::T![Ident] },
			Self::Custom(ty, _) => quote! { types::#ty },
		}
	}

	pub fn checks(&self) -> &DefRange {
		match self {
			Self::Length(c)
			| Self::LengthPercentage(c)
			| Self::Percentage(c)
			| Self::Angle(c)
			| Self::Time(c)
			| Self::Resolution(c)
			| Self::Integer(c)
			| Self::Number(c) => c,
			_ => &DefRange::None,
		}
	}

	pub fn requires_allocator_lifetime(&self) -> bool {
		if let Self::Custom(DefIdent(ident), _) = self {
			return matches!(ident, &atom!("OutlineColor") | &atom!("BorderTopColor") | &atom!("AnchorName") | &atom!("DynamicRangeLimitMix"));
		} 
		matches!(self, Self::Image | Self::Image1D)
	}
}

impl GenerateToCursorsImpl for DefType {
	fn to_cursors_steps(&self, capture: TokenStream) -> TokenStream {
		quote! { ::hdx_parser::ToCursors::to_cursors(#capture, s); }
	}
}

impl GeneratePeekImpl for DefType {
	fn peek_steps(&self) -> TokenStream {
		match self {
			Self::CustomIdent => quote! { p.peek::<::hdx_parser::T![Ident]>() },
			_ => {
				let name = self.to_type_name();
				quote! { p.peek::<#name>() }
			}
		}
	}
}

impl GenerateParseImpl for DefType {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream {
		let capture_name = capture.unwrap_or_else(|| format_ident!("val"));
		if self == &Self::CustomIdent {
			return quote! {
				let #capture_name = p.parse::<::hdx_parser::T![Ident]>()?;
			};
		}

		let name = self.to_type_name();
		let checks = self.checks();
		let check_code = match checks {
			DefRange::RangeTo(RangeTo { end }) => Some(quote! {
			let valf32: f32 = #capture_name.into();
					if #end < valf32 {
						return Err(::hdx_parser::diagnostics::NumberTooLarge(#end, ::hdx_lexer::Span::new(start, p.offset())))?
					}
				}),
			DefRange::Range(Range { start, end }) => Some(quote! {
			let valf32: f32 = #capture_name.into();
					if !(#start..#end).contains(valf32) {
						return Err(::hdx_parser::diagnostics::NumberOutOfBounds(#capture_name, "#start..#end", ::hdx_lexer::Span::new(start, p.offset())))?
					}
				}),
			DefRange::RangeFrom(RangeFrom { start }) => Some(quote! {
			let valf32: f32 = #capture_name.into();
					if #start > valf32 {
						return Err(::hdx_parser::diagnostics::NumberTooSmall(#start, ::hdx_lexer::Span::new(start, p.offset())))?
					}
				}),
			DefRange::None => None,
		};
		let start_offset = if check_code.is_some() { Some(quote! { let start = p.offset(); }) } else { None };
		quote! {
			#start_offset
			let #capture_name = p.parse::<#name>()?;
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

	pub fn pluralize(&self) -> DefIdent {
		if self.0.ends_with("s") {
			self.clone()
		} else {
			Self(Atom::from(format!("{}s", self.0)))
		}
	}

	pub fn to_variant_name(&self, size_hint: usize) -> TokenStream {
		let variant_str = pascal(self.0.to_lowercase());
		let ident = if size_hint > 0 { format_ident!("{}s", variant_str) } else { format_ident!("{}", variant_str) };
		quote! { #ident }
	}
}

impl GenerateToCursorsImpl for DefIdent {
	fn to_cursors_steps(&self, inner: TokenStream) -> TokenStream {
		quote! { s.append(#inner.into()); }
	}
}

impl GeneratePeekImpl for DefIdent {
	fn peek_steps(&self) -> TokenStream {
		quote! { p.peek::<::hdx_parser::T![Ident]>() }
	}
}

impl GenerateParseImpl for DefIdent {
	fn parse_steps(&self, capture: Option<Ident>) -> TokenStream {
		let atom = self.to_atom_macro();
		quote! {
			let #capture = p.parse::<::hdx_parser::T![Ident]>()?;
			let c: ::hdx_lexer::Cursor = #capture.into();
			let atom = p.parse_atom_lower(t);
			if atom != #atom {
				Err(::hdx_parser::diagnostics::UnexpectedIdent(atom, c.into()))?
			}
		}
	}
}
