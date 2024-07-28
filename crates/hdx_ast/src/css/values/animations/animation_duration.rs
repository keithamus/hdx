use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::{Kind, Token};
use hdx_parser::{discard, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::units::Time;

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AnimationDuration {
	#[default]
	Auto,
	Absolute(SmallVec<[Time; 2]>),
}

impl<'a> Parse<'a> for AnimationDuration {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("auto") => {
					parser.next();
					AnimationDuration::Auto
				}
				atom => unexpected_ident!(parser, atom),
			},
			Token::Dimension(_, _, _) => {
				let mut values = smallvec![];
				loop {
					values.push(Time::parse(parser)?);
					if !discard!(parser, Kind::Comma) {
						break;
					}
				}
				AnimationDuration::Absolute(values)
			}
			token => unexpected!(parser, token),
		})
	}
}

impl<'a> WriteCss<'a> for AnimationDuration {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Auto => atom!("auto").write_css(sink),
			Self::Absolute(vals) => vals.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AnimationDuration, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnimationDuration, "auto");
		assert_parse!(AnimationDuration, "0s");
		assert_parse!(AnimationDuration, "1ms");
		assert_parse!(AnimationDuration, "1ms, 400ms, 8s");
	}

	#[test]
	fn test_minify() {
		assert_minify!(AnimationDuration, "1ms, 400ms, 8s", "1ms,400ms,8s");
	}
}
