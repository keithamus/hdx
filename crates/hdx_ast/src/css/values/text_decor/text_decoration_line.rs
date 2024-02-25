use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Value};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Default)]
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

impl Value for TextDecorationLine {}

impl<'a> Parse<'a> for TextDecorationLine {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		let mut value = Self::none();
		loop {
			if value.is_all() {
				break;
			}
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("none") if value.is_none() => {
						return Ok(Self::None.spanned(span.end(parser.pos())));
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
		Ok(value.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for TextDecorationLine {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.is_none() {
			sink.write_str("none")?;
		} else {
			if self.contains(Self::Underline) {
				sink.write_str("underline")?;
			}
			if self.contains(Self::Overline) {
				if self.intersects(Self::Overline) {
					sink.write_char(' ')?;
				}
				sink.write_str("overline")?;
			}
			if self.contains(Self::LineThrough) {
				if self.intersects(Self::Underline | Self::Overline) {
					sink.write_char(' ')?;
				}
				sink.write_str("line-through")?;
			}
			if self.contains(Self::Blink) {
				if self.intersects(Self::Underline | Self::Overline | Self::LineThrough) {
					sink.write_char(' ')?;
				}
				sink.write_str("blink")?;
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
