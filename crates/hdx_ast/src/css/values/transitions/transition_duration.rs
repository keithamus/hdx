use hdx_derive::{Value, Writable};
use hdx_lexer::Token;
use hdx_parser::{discard, expect, Parse, Parser, Result as ParserResult};
use smallvec::{smallvec, SmallVec};

use crate::css::units::Time;

// https://drafts.csswg.org/css-transitions-1/#propdef-transition-duration
#[derive(Value, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TransitionDuration(pub SmallVec<[Time; 2]>);

impl<'a> Parse<'a> for TransitionDuration {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser.peek(), Token::Dimension(_, _, _));
		let mut values = smallvec![];
		loop {
			values.push(Time::parse(parser)?);
			if !discard!(parser, Token::Comma) {
				return Ok(TransitionDuration(values));
			}
		}
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
