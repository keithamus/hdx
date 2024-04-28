use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{discard, expect, unexpected, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::css::units::Time;
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-transitions-1/#propdef-transition-delay
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TransitionDelay(pub SmallVec<[Time; 2]>);

impl<'a> Parse<'a> for TransitionDelay {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser.peek(), Token::Dimension(_, _, _));
		let mut values = smallvec![];
		loop {
			if let Some(time) = Time::from_token(&parser.next()) {
				values.push(time);
			} else {
				unexpected!(parser);
			}
			if !discard!(parser, Token::Comma) {
				return Ok(TransitionDelay(values));
			}
		}
	}
}

impl<'a> WriteCss<'a> for TransitionDelay {
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
		assert_size!(TransitionDelay, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TransitionDelay, "0s");
		assert_parse!(TransitionDelay, "1ms");
		assert_parse!(TransitionDelay, "1ms, 400ms, 8s");
	}

	#[test]
	fn test_minify() {
		assert_minify!(TransitionDelay, "1ms, 400ms, 8s", "1ms,400ms,8s");
	}
}
