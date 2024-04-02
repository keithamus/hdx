use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, expect, unexpected, unexpected_function, unexpected_ident, Parse, Parser, Result as ParserResult,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::{bitmask, Parsable, Value, Writable};

// https://drafts.csswg.org/css-fonts/#font-variant-east-asian-prop
#[derive(Value, Default)]
#[bitmask(u16)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontVariantEastAsian {
	#[default]
	Normal = 0b0000_0000,

	// <east-asian-variant-values>
	Jis78 = 0b0000_0001,
	Jis83 = 0b0000_0010,
	Jis90 = 0b0000_0100,
	Jis04 = 0b0000_1000,
	Simplified = 0b0001_0000,
	Traditional = 0b0010_0000,

	// <east-asian-width-values>
	FullWidth = 0b0100_0000,
	ProportionalWidth = 0b1000_0000,

	Ruby = 0b0001_0000_0000,
}

impl FontVariantEastAsian {
	#[inline]
	fn has_variant_values(&self) -> bool {
		self.bits & 0b0011_1111 > 0
	}

	#[inline]
	fn has_width_values(&self) -> bool {
		self.bits & 0b1100_0000 > 0
	}
}

impl<'a> Parse<'a> for FontVariantEastAsian {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self::Normal;
		match parser.cur() {
			Token::Ident(atom) => {
				if atom.to_ascii_lowercase() == atom!("normal") {
					parser.advance();
					return Ok(Self::Normal);
				}
			},
			token => unexpected!(parser, token),
		}
		loop {
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("jis78") if !value.has_variant_values() => value |= Self::Jis78,
					atom!("jis83") if !value.has_variant_values() => value |= Self::Jis83,
					atom!("jis90") if !value.has_variant_values() => value |= Self::Jis90,
					atom!("jis04") if !value.has_variant_values() => value |= Self::Jis04,
					atom!("simplified") if !value.has_variant_values() => value |= Self::Simplified,
					atom!("traditional") if !value.has_variant_values() => value |= Self::Traditional,
					atom!("full-width") if !value.has_width_values() => value |= Self::FullWidth,
					atom!("proportional-width") if !value.has_width_values() => value |= Self::ProportionalWidth,
					_ => break,
				},
				_ => break,
			}
			parser.advance();
		}
		match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("ruby") => {
					parser.advance();
					value |= Self::Ruby
				}
				_ => {
					if value == Self::Normal {
						unexpected_ident!(parser, atom);
					}
				}
			},
			_ => {},
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for FontVariantEastAsian {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match *self {
			Self::Normal => atom!("normal").write_css(sink),
			val => {
				if val.contains(Self::Jis78) {
					atom!("jis78").write_css(sink)?;
					if val != Self::Jis78 {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::Jis83) {
					atom!("jis83").write_css(sink)?;
					if val != Self::Jis83 {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::Jis90) {
					atom!("jis90").write_css(sink)?;
					if val != Self::Jis90 {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::Jis04) {
					atom!("jis04").write_css(sink)?;
					if val != Self::Jis04 {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::Simplified) {
					atom!("simplified").write_css(sink)?;
					if val != Self::Simplified {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::Traditional) {
					atom!("traditional").write_css(sink)?;
					if val != Self::Traditional {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::FullWidth) {
					atom!("full-width").write_css(sink)?;
					if val.contains(Self::Ruby) {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::ProportionalWidth) {
					atom!("proportional-width").write_css(sink)?;
					if val.contains(Self::Ruby) {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::Ruby) {
					atom!("ruby").write_css(sink)?;
				}
				Ok(())
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
		assert_size!(FontVariantEastAsian, 2);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantEastAsian, "normal");
		assert_parse!(FontVariantEastAsian, "simplified");
		assert_parse!(FontVariantEastAsian, "ruby");
		assert_parse!(FontVariantEastAsian, "jis83 proportional-width ruby");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(FontVariantEastAsian, "normal ruby");
		assert_parse_error!(FontVariantEastAsian, "full-width proportional-width");
		assert_parse_error!(FontVariantEastAsian, "jis83 jis04");
		assert_parse_error!(FontVariantEastAsian, "no-common-ligatures");
	}
}
