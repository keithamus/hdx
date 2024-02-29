use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Value};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Value, Default)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextDecorationLine {
	#[default]
	None = 0b0000,
	Underline = 0b0001,
	Overline = 0b0010,
	LineThrough = 0b0100,
	Blink = 0b1000,
}

impl<'a> Parse<'a> for TextDecorationLine {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self::none();
		loop {
			if value.is_all() {
				break;
			}
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("none") if value.is_none() => {
						return Ok(Self::None);
					}
					atom!("underline") if !value.contains(Self::Underline) => value |= Self::Underline,
					atom!("overline") if !value.contains(Self::Overline) => value |= Self::Overline,
					atom!("line-through") if !value.contains(Self::LineThrough) => value |= Self::LineThrough,
					atom!("blink") if !value.contains(Self::Blink) => value |= Self::Blink,
					_ => break,
				},
				token => unexpected!(parser, token),
			}
			parser.advance();
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for TextDecorationLine {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.is_none() {
			atom!("none").write_css(sink)?;
		} else {
			if self.contains(Self::Underline) {
				atom!("underline").write_css(sink)?;
			}
			if self.contains(Self::Overline) {
				if self.intersects(Self::Overline) {
					sink.write_char(' ')?;
				}
				atom!("overline").write_css(sink)?;
			}
			if self.contains(Self::LineThrough) {
				if self.intersects(Self::Underline | Self::Overline) {
					sink.write_char(' ')?;
				}
				atom!("line-through").write_css(sink)?;
			}
			if self.contains(Self::Blink) {
				if self.intersects(Self::Underline | Self::Overline | Self::LineThrough) {
					sink.write_char(' ')?;
				}
				atom!("blink").write_css(sink)?;
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
		assert_eq!(::std::mem::size_of::<TextDecorationLine>(), 1);
	}
}
