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
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AnimationDelay, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnimationDelay, "0s");
		assert_parse!(AnimationDelay, "1ms");
		assert_parse!(AnimationDelay, "1ms, 400ms, 8s");
	}

	#[test]
	fn test_minify() {
		assert_minify!(AnimationDelay, "1ms, 400ms, 8s", "1ms,400ms,8s");
	}
}
