use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Atomizable, Value};

// https://drafts.csswg.org/css-text-4/#propdef-white-space-trim
#[derive(Value, Default, Atomizable)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum WhiteSpaceTrim {
	#[default]
	None = 0b0000, // atom!("none")
	DiscardBefore = 0b0001, // atom!("discard-before")
	DiscardAfter = 0b0010,  // atom!("discard-after")
	DiscardInner = 0b0100,  // atom!("discard-inner")
}

impl<'a> Parse<'a> for WhiteSpaceTrim {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self::none();
		loop {
			if value.is_all() {
				break;
			}
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("none") if value.is_none() => return Ok(Self::None),
					atom!("discard-before") if !value.contains(Self::DiscardBefore) => value |= Self::DiscardBefore,
					atom!("discard-after") if !value.contains(Self::DiscardAfter) => value |= Self::DiscardAfter,
					atom!("discard-inner") if !value.contains(Self::DiscardInner) => value |= Self::DiscardInner,
					_ => break,
				},
				token => unexpected!(parser, token),
			}
			parser.advance();
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for WhiteSpaceTrim {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.is_none() {
			atom!("none").write_css(sink)?;
		} else {
			if self.contains(Self::DiscardBefore) {
				atom!("discard-before").write_css(sink)?;
			}
			if self.contains(Self::DiscardAfter) {
				if self.intersects(Self::DiscardBefore) {
					sink.write_char(' ')?;
				}
				atom!("discard-after").write_css(sink)?;
			}
			if self.contains(Self::DiscardInner) {
				if self.intersects(Self::DiscardBefore | Self::DiscardAfter) {
					sink.write_char(' ')?;
				}
				atom!("discard-inner").write_css(sink)?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<WhiteSpaceTrim>(), 1);
	}
}
