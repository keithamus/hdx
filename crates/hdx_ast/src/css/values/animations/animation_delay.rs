use hdx_lexer::Token;
use hdx_parser::{unexpected, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::super::units::Time;
use crate::Value;
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct AnimationDelay(pub SmallVec<[Time; 2]>);

impl<'a> Value for AnimationDelay {}

impl<'a> Parse<'a> for AnimationDelay {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let value = match parser.cur() {
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
				AnimationDelay(values)
			}
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for AnimationDelay {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(time) = iter.next() {
			time.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
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
		assert_eq!(size_of::<AnimationDelay>(), 32);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AnimationDelay>(&allocator, "0s", "0s");
		test_write::<AnimationDelay>(&allocator, "1ms", "1ms");
		test_write::<AnimationDelay>(&allocator, "1ms, 400ms, 8s", "1ms,400ms,8s");
	}
}
