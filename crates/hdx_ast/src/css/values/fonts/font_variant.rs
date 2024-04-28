use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use super::{
	FontVariantAlternates, FontVariantCaps, FontVariantEastAsian, FontVariantEmoji, FontVariantLigatures,
	FontVariantNumeric, FontVariantPosition,
};

// https://drafts.csswg.org/css-fonts/#font-variant-prop
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontVariant(
	pub Spanned<FontVariantLigatures>,
	pub Spanned<FontVariantPosition>,
	pub Spanned<FontVariantCaps>,
	pub Spanned<FontVariantNumeric>,
	pub Spanned<FontVariantAlternates>,
	pub Spanned<FontVariantEastAsian>,
	pub Spanned<FontVariantEmoji>,
);

impl<'a> Parse<'a> for FontVariant {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self { ..Default::default() };
		match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("normal") => {
					parser.advance();
					return Ok(Self { ..Default::default() });
				}
				atom!("none") => {
					return Ok(Self { 0: FontVariantLigatures::parse_spanned(parser)?, ..Default::default() });
				}
				_ => {}
			},
			token => unexpected!(parser, token),
		}
		loop {
			if matches!(parser.peek(), Token::Eof | Token::Semicolon) {
				break;
			}
			if value.0.node == FontVariantLigatures::default() {
				if let Ok(prop) = FontVariantLigatures::try_parse_spanned(parser) {
					value.0 = prop;
					continue;
				}
			}
			if value.1.node == FontVariantPosition::default() {
				if let Ok(prop) = FontVariantPosition::try_parse_spanned(parser) {
					value.1 = prop;
					continue;
				}
			}
			if value.2.node == FontVariantCaps::default() {
				if let Ok(prop) = FontVariantCaps::try_parse_spanned(parser) {
					value.2 = prop;
					continue;
				}
			}
			if value.3.node == FontVariantNumeric::default() {
				if let Ok(prop) = FontVariantNumeric::try_parse_spanned(parser) {
					value.3 = prop;
					continue;
				}
			}
			if value.4.node == FontVariantAlternates::default() {
				if let Ok(prop) = FontVariantAlternates::try_parse_spanned(parser) {
					value.4 = prop;
					continue;
				}
			}
			if value.5.node == FontVariantEastAsian::default() {
				if let Ok(prop) = FontVariantEastAsian::try_parse_spanned(parser) {
					value.5 = prop;
					continue;
				}
			}
			if value.6.node == FontVariantEmoji::default() {
				if let Ok(prop) = FontVariantEmoji::try_parse_spanned(parser) {
					value.6 = prop;
					continue;
				}
			}
			break;
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for FontVariant {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut wrote = false;
		if self.0.node != FontVariantLigatures::default() {
			self.0.write_css(sink)?;
			wrote = true
		}
		if self.1.node != FontVariantPosition::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.1.write_css(sink)?;
			wrote = true
		}
		if self.2.node != FontVariantCaps::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.2.write_css(sink)?;
			wrote = true
		}
		if self.3.node != FontVariantNumeric::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.3.write_css(sink)?;
			wrote = true
		}
		if self.4.node != FontVariantAlternates::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.4.write_css(sink)?;
			wrote = true
		}
		if self.5.node != FontVariantEastAsian::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.5.write_css(sink)?;
			wrote = true
		}
		if self.6.node != FontVariantEmoji::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.6.write_css(sink)?;
			wrote = true
		}
		if !wrote {
			self.0.write_css(sink)?;
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
		assert_size!(FontVariant, 112);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariant, "normal");
		assert_parse!(FontVariant, "none");
		assert_parse!(FontVariant, "no-common-ligatures proportional-nums");
		assert_parse!(FontVariant, "common-ligatures tabular-nums");
		assert_parse!(FontVariant, "small-caps slashed-zero");
		assert_parse!(
			FontVariant,
			"no-common-ligatures no-discretionary-ligatures no-historical-ligatures no-contextual"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(FontVariant, "normal normal");
		assert_parse_error!(FontVariant, "none none");
		assert_parse_error!(FontVariant, "no-common-ligatures no-common-ligatures");
	}
}
