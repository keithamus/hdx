use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Value, Writable};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[derive(Value, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct AnimationFillMode(pub SmallVec<[SingleAnimationFillMode; 8]>);

#[derive(Atomizable, Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum SingleAnimationFillMode {
	#[default]
	None, // atom!("none")
	Forwards,  // atom!("forwards")
	Backwards, // atom!("backwards")
	Both,      // atom!("both")
}

impl<'a> Parse<'a> for AnimationFillMode {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			match parser.cur() {
				Token::Ident(atom) => {
					if let Some(fill) = SingleAnimationFillMode::from_atom(atom.to_ascii_lowercase()) {
						parser.advance();
						values.push(fill);
					} else {
						unexpected_ident!(parser, atom);
					}
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

impl<'a> WriteCss<'a> for AnimationFillMode {
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
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::{test_write, test_write_min};

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<AnimationFillMode>(), 24);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AnimationFillMode>(&allocator, "both", "both");
		test_write::<AnimationFillMode>(&allocator, "none, both, backwards, forwards", "none, both, backwards, forwards");
	}

	#[test]
	fn test_minify() {
		let allocator = Allocator::default();
		test_write_min::<AnimationFillMode>(&allocator, "none, both, backwards, forwards", "none,both,backwards,forwards");
	}
}
