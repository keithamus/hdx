use hdx_derive::Value;
use hdx_lexer::{Kind, Token};
use hdx_parser::{discard, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::units::Time;

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AnimationDelay(pub SmallVec<[Time; 2]>);

impl<'a> Parse<'a> for AnimationDelay {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek() {
			Token::Dimension(_, _, _) => {
				let mut values = smallvec![];
				loop {
					values.push(Time::parse(parser)?);
					if !discard!(parser, Kind::Comma) {
						break;
					}
				}
				AnimationDelay(values)
			}
			token => unexpected!(parser, token),
		})
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
