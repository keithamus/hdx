use hdx_lexer::Token;
use hdx_parser::{unexpected, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use super::super::units::Time;
use crate::Value;
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-transitions-1/#propdef-transition-duration
#[derive(Value, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TransitionDuration(pub SmallVec<[Time; 2]>);

impl<'a> Parse<'a> for TransitionDuration {
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
				TransitionDuration(values)
			}
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for TransitionDuration {
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
		assert_size!(TransitionDuration, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransitionDuration, "0s");
		assert_parse!(TransitionDuration, "1ms");
		assert_parse!(TransitionDuration, "1ms, 400ms, 8s");
	}

	#[test]
	fn test_minify() {
		assert_minify!(TransitionDuration, "1ms, 400ms, 8s", "1ms,400ms,8s");
	}
}
