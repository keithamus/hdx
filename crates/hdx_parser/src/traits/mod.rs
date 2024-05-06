use hdx_atom::Atom;
use hdx_lexer::Token;

mod declarations;
mod rules;
mod selectors;

pub use declarations::*;
pub use rules::*;
pub use selectors::*;

use crate::{
	expect, expect_ignore_case, parser::Parser, peek, span::Spanned, unexpected, unexpected_ident, Comparison, Result,
	State, Vec,
};

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
		Self::parse(parser).map_err(|e| {
			parser.rewind(checkpoint);
			e
		})
	}

	fn parse_with_state(parser: &mut Parser<'a>, state: State) -> Result<Self> {
		let old = parser.state;
		parser.state = old | state;
		let value = Self::parse(parser);
		parser.state = old;
		value
	}

	fn parse_spanned(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let node = Self::parse(parser)?;
		Ok(Spanned { node, span: span.end(parser.pos()) })
	}

	fn try_parse_spanned(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let node = Self::try_parse(parser)?;
		Ok(Spanned { node, span: span.end(parser.prev_pos) })
	}

	fn parse_spanned_with_state(parser: &mut Parser<'a>, state: State) -> Result<Spanned<Self>> {
		let old = parser.state;
		parser.state = old | state;
		let value = Self::parse_spanned(parser);
		parser.state = old;
		value
	}
}

pub trait Block<'a>: Sized + Parse<'a> {
	type Declaration: Parse<'a>;
	type Rule: Parse<'a>;

	// https://drafts.csswg.org/css-syntax-3/#consume-block-contents
	fn parse_block(
		parser: &mut Parser<'a>,
	) -> Result<(Vec<'a, Spanned<Self::Declaration>>, Vec<'a, Spanned<Self::Rule>>)> {
		expect!(parser.next(), Token::LeftCurly);
		let mut declarations = parser.new_vec();
		let mut rules = parser.new_vec();
		loop {
			match parser.peek() {
				Token::Semicolon => {
					parser.advance();
				}
				Token::Eof | Token::RightCurly => {
					parser.advance();
					break;
				}
				Token::AtKeyword(_) => {
					rules.push(Self::Rule::parse_spanned_with_state(parser, State::Nested)?);
				}
				_ => {
					let checkpoint = parser.checkpoint();
					if let Ok(decl) = Self::Declaration::parse_spanned_with_state(parser, State::Nested) {
						declarations.push(decl);
					} else {
						parser.rewind(checkpoint);
						rules.push(Self::Rule::parse_spanned(parser)?);
					}
				}
			}
		}
		Ok((declarations, rules))
	}
}

pub trait StyleSheet<'a>: Sized + Parse<'a> {
	type Rule: Parse<'a>;

	fn parse_stylesheet(parser: &mut Parser<'a>) -> Result<Vec<'a, Spanned<Self::Rule>>> {
		let mut rules: Vec<'a, Spanned<Self::Rule>> = parser.new_vec();
		loop {
			match parser.peek() {
				Token::Eof => {
					return Ok(rules);
				}
				Token::Cdc | Token::Cdo => {
					parser.advance();
				}
				_ => {
					rules.push(Self::Rule::parse_spanned(parser)?);
				}
			}
		}
	}
}

pub trait DiscreteMediaFeature<'a>: Sized + Default {
	fn parse_media_feature_value(parser: &mut Parser<'a>) -> Result<Self>;

	fn parse_descrete_media_feature(name: Atom, parser: &mut Parser<'a>) -> Result<Self> {
		expect_ignore_case!(parser.next(), Token::Ident(name));
		let value = match parser.peek() {
			Token::Colon => {
				parser.advance();
				Self::parse_media_feature_value(parser)?
			}
			_ => Self::default(),
		};
		Ok(value)
	}
}

pub trait RangedMediaFeature<'a>: Sized {
	type Type: Parse<'a>;

	fn new(left: (Comparison, Self::Type), right: Option<(Comparison, Self::Type)>, legacy: bool) -> Self;

	fn parse_ranged_media_feature(name: Atom, parser: &mut Parser<'a>) -> Result<Self> {
		let left = match parser.peek().clone() {
			Token::Ident(atom) => {
				parser.next();
				let mut legacy = false;
				let legacy_cmp = match atom.to_ascii_lowercase() {
					atom if atom == name => {
						legacy = peek!(parser, Token::Colon);
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
					_ => unexpected_ident!(parser, atom),
				};
				if legacy {
					expect!(parser.next(), Token::Colon);
					return Ok(Self::new((legacy_cmp, Self::Type::parse(parser)?), None, true));
				} else {
					let cmp = Comparison::parse(parser)?;
					return Ok(Self::new((cmp, Self::Type::parse(parser)?), None, false));
				}
			}
			_ => {
				Self::Type::parse(parser)?
			}
		};
		let left_cmp = Comparison::parse(parser)?;
		expect_ignore_case!(parser.next(), Token::Ident(name));
		if !peek!(parser, Token::Delim(_)) {
			return Ok(Self::new((left_cmp, left), None, false));
		}
		let right_cmp = Comparison::parse(parser)?;
		if left_cmp == Comparison::Equal && right_cmp == Comparison::Equal {
			unexpected!(parser)
		}
		Ok(Self::new((left_cmp, left), Some((right_cmp, Self::Type::parse(parser)?)), false))
	}
}
