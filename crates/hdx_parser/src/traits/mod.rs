use hdx_atom::Atom;
use hdx_lexer::{Span, Spanned, Token as LexerToken};

mod declarations;
mod rules;
mod selectors;

pub use declarations::*;
pub use rules::*;
pub use selectors::*;

use crate::{diagnostics, discard, parser::Parser, Comparison, Delim, Result, State, Token, Vec};

impl<'a, T: Parse<'a>> Parse<'a> for Vec<'a, T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Vec<'a, T>> {
		loop {
			let mut vec = parser.new_vec();
			if let Ok(t) = T::parse(parser) {
				vec.push(t);
			} else {
				return Ok(vec);
			}
		}
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for Spanned<T> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<T>> {
		T::parse_spanned(parser)
	}
}

pub trait Parse<'a>: Sized {
	fn parse(parser: &mut Parser<'a>) -> Result<Self>;

	fn try_parse(parser: &mut Parser<'a>) -> Result<Self> {
		let checkpoint = parser.checkpoint();
		Self::parse(parser).inspect_err(|_| parser.rewind(checkpoint))
	}

	fn parse_spanned(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let start = parser.offset();
		let node = Self::parse(parser)?;
		Ok(Spanned { node, span: Span::new(start, parser.offset()) })
	}

	fn try_parse_spanned(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let start = parser.offset();
		let node = Self::try_parse(parser)?;
		Ok(Spanned { node, span: Span::new(start, parser.offset()) })
	}
}

pub trait Peek<'a>: Sized {
	fn peek(parser: &Parser<'a>) -> Option<LexerToken>;
}

pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type Rule: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-block-contents
	fn parse_block(
		parser: &mut Parser<'a>,
	) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::Rule>>)> {
		parser.parse::<Token![LeftCurly]>()?;
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			if parser.at_end() {
				break;
			}
			discard!(parser, Semicolon);
			if discard!(parser, RightCurly) {
				break;
			}
			let old_state = parser.set_state(State::Nested);
			if parser.peek::<Token![AtKeyword]>().is_some() {
				rules.push(parser.parse_spanned::<Self::Rule>().inspect_err(|_| {
					parser.set_state(old_state);
				})?);
			} else {
				let checkpoint = parser.checkpoint();

				if let Ok(decl) = parser.parse_spanned::<Self::Declaration>() {
					declarations.push(decl);
				} else {
					parser.rewind(checkpoint);
					rules.push(parser.parse_spanned::<Self::Rule>()?);
				}
			}
			parser.set_state(old_state);
		}
		Ok((declarations, rules))
	}
}

pub trait StyleSheet<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_stylesheet(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		let mut rules: Vec<'a, Spanned<Self::Rule>> = parser.new_vec();
		loop {
			if parser.at_end() {
				return Ok(rules);
			}
			discard!(parser, CdcOrCdo);
			rules.push(Self::Rule::parse_spanned(parser)?);
		}
	}
}

pub trait DiscreteMediaFeature<'a>: Sized + Default {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_descrete_media_feature(name: Atom, parser: &mut Parser<'a>) -> Result<Self> {
		let token = *parser.parse::<Token![Ident]>()?;
		let atom = parser.parse_atom_lower(token);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, token.span()))?
		}
		if let Some(token) = parser.peek::<Delim![:]>() {
			parser.hop(token);
			Ok(Self::parse_media_feature_value(parser)?)
		} else {
			Ok(Self::default())
		}
	}
}

pub trait RangedMediaFeature<'a>: Sized {
	type Type: Parse<'a>;

	fn new(left: (Comparison, Self::Type), right: Option<(Comparison, Self::Type)>, legacy: bool) -> Self;

	fn parse_ranged_media_feature(name: Atom, parser: &mut Parser<'a>) -> Result<Self> {
		let left = if let Some(token) = parser.peek::<Token![Ident]>() {
			parser.hop(token);
			let mut legacy = false;
			let legacy_cmp = match parser.parse_atom_lower(token) {
				atom if atom == name => {
					legacy = parser.peek::<Delim![:]>().is_some();
					Comparison::Equal
				}
				atom if atom.strip_prefix("max-").unwrap_or("") == name.as_ref() => {
					legacy = true;
					Comparison::GreaterThanEqual
				}
				atom if atom.strip_prefix("min-").unwrap_or("") == name.as_ref() => {
					legacy = true;
					Comparison::LessThanEqual
				}
				atom => Err(diagnostics::ExpectedIdentOf(name, atom, token.span()))?,
			};
			if legacy {
				parser.parse::<Delim![:]>()?;
				return Ok(Self::new((legacy_cmp, Self::Type::parse(parser)?), None, true));
			} else {
				let cmp = parser.parse::<Comparison>()?;
				return Ok(Self::new((cmp, Self::Type::parse(parser)?), None, false));
			}
		} else {
			Self::Type::parse(parser)?
		};
		let offset = parser.offset();
		let left_cmp = parser.parse::<Comparison>()?;
		let token = *parser.parse::<Token![Ident]>()?;
		let atom = parser.parse_atom_lower(token);
		if atom != name {
			Err(diagnostics::ExpectedIdentOf(name, atom, token.span()))?
		}
		if parser.peek::<Token![Delim]>().is_none() {
			return Ok(Self::new((left_cmp, left), None, false));
		}
		let right_cmp = Comparison::parse(parser)?;
		if left_cmp == Comparison::Equal && right_cmp == Comparison::Equal {
			Err(diagnostics::UnexpectedMediaRangeComparisonEqualsTwice(Span::new(offset, parser.offset())))?
		}
		Ok(Self::new((left_cmp, left), Some((right_cmp, Self::Type::parse(parser)?)), false))
	}
}
