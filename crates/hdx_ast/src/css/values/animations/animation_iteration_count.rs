use hdx_atom::atom;
use hdx_derive::{Value, Writable};
use hdx_lexer::Kind;
use hdx_parser::{discard, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::units::CSSFloat;

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AnimationIterationCount(pub SmallVec<[SingleAnimationIterationCount; 1]>);

#[derive(Writable, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleAnimationIterationCount {
	Infinite, // atom!("infinite")
	Number(CSSFloat),
}

impl Default for SingleAnimationIterationCount {
	fn default() -> Self {
		Self::Number(1.0.into())
	}
}

impl<'a> Parse<'a> for AnimationIterationCount {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			let token = parser.next();
			match token.kind() {
				Kind::Ident => match parser.parse_atom_lower(token) {
					atom!("infinite") => values.push(SingleAnimationIterationCount::Infinite),
					atom => unexpected_ident!(parser, atom),
				},
				Kind::Number if token.is_int() && !token.has_sign() => {
					values.push(SingleAnimationIterationCount::Number(parser.parse_number(token).into()))
				}
				_ => unexpected!(parser, token),
			}
			if !discard!(parser, Kind::Comma) {
				break;
			}
		}
		Ok(Self(values))
	}
}

impl<'a> WriteCss<'a> for AnimationIterationCount {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(fill) = iter.next() {
			fill.write_css(sink)?;
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
		assert_size!(AnimationIterationCount, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnimationIterationCount, "infinite");
		assert_parse!(AnimationIterationCount, "1, infinite, 7, 800");
	}

	#[test]
	fn test_minify() {
		assert_minify!(AnimationIterationCount, "1, infinite, 7, 800", "1,infinite,7,800");
	}
}
