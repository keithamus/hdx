use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::super::units::Time;
use crate::Value;
use smallvec::{SmallVec, smallvec};

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum AnimationDuration {
	#[default]
	Auto,
	Absolute(SmallVec<[Time; 2]>),
}

impl<'a> Value for AnimationDuration {}

impl<'a> Parse<'a> for AnimationDuration {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let value = match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("auto") => {
					parser.advance();
					AnimationDuration::Auto
				}
				atom => unexpected_ident!(parser, atom),
			},
			Token::Dimension(_, _, _) => {
				let mut values = smallvec![];
				loop {
					if let Some(time) = Time::from_token(parser.cur()) {
						parser.advance();
						values.push(time);
					} else {
						unexpected!(parser);
					}
					match parser.cur() {
						Token::Comma => {
							parser.advance();
						}
						_ => {
							break;
						}
					}
				}
				AnimationDuration::Absolute(values)
			},
			token => unexpected!(parser, token),
		};
		Ok(value.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for AnimationDuration {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Auto => atom!("auto").write_css(sink),
			Self::Absolute(vals) => {
				let mut iter = vals.iter().peekable();
				while let Some(time) = iter.next() {
					time.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_char(',')?;
						sink.write_trivia_char(' ')?;
					}
				}
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<AnimationDuration>(), 32);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AnimationDuration>(&allocator, "auto", "auto");
		test_write::<AnimationDuration>(&allocator, "0s", "0s");
		test_write::<AnimationDuration>(&allocator, "1ms", "1ms");
		test_write::<AnimationDuration>(&allocator, "1ms, 400ms, 8s", "1ms,400ms,8s");
	}
}
