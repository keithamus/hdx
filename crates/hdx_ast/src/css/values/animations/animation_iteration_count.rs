use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{css::values::units::CSSFloat, Value, Writable};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[derive(Value, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AnimationIterationCount(pub SmallVec<[SingleAnimationIterationCount; 1]>);

#[derive(Writable, Debug, PartialEq, Hash)]
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
			match parser.cur() {
				Token::Ident(ident) => match ident.to_ascii_lowercase() {
					atom!("infinite") => {
						parser.advance();
						values.push(SingleAnimationIterationCount::Infinite);
					}
					atom => unexpected_ident!(parser, atom),
				},
				Token::Number(val, ty) if ty.is_int() && !ty.is_signed() => {
					parser.advance();
					values.push(SingleAnimationIterationCount::Number(val.into()))
				}
				token => unexpected!(parser, token),
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
