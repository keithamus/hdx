// https://drafts.csswg.org/css-text/#text-transform
use bitmask_enum::bitmask;
use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-text/#text-align-property
#[derive(Value, Default)]
#[value(Inherits)]
#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum TextTransform {
	#[default]
	None = 0b0000,
	Capitalize = 0b0001,
	Uppercase = 0b0010,
	Lowercase = 0b0100,
	FullWidth = 0b0001_0000,
	FullSizeKana = 0b0010_0000,
}

impl TextTransform {
	#[inline]
	pub fn has_case_transform(&self) -> bool {
		self.bits & 0b0000_1111 > 0
	}
}

impl<'a> Parse<'a> for TextTransform {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self::none();
		loop {
			if value.is_all() {
				break;
			}
			match parser.peek() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("none") if value.is_none() => {
						parser.next();
						return Ok(Self::None);
					}
					atom!("capitalize") if !value.has_case_transform() => value |= Self::Capitalize,
					atom!("uppercase") if !value.has_case_transform() => value |= Self::Uppercase,
					atom!("lowercase") if !value.has_case_transform() => value |= Self::Lowercase,
					atom!("full-width") if !value.contains(Self::FullWidth) => value |= Self::FullWidth,
					atom!("full-size-kana") if !value.contains(Self::FullSizeKana) => value |= Self::FullSizeKana,
					_ => break,
				},
				_ => {
					break;
				}
			}
			parser.next();
		}
		// Explicit "none" is handled above, so if there are no other collected values this is a parse error
		if value == Self::none() {
			unexpected!(parser);
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for TextTransform {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if self.is_none() {
			atom!("none").write_css(sink)?;
		} else {
			if self.contains(Self::Capitalize) {
				atom!("capitalize").write_css(sink)?;
			} else if self.contains(Self::Lowercase) {
				atom!("lowercase").write_css(sink)?;
			} else if self.contains(Self::Uppercase) {
				atom!("uppercase").write_css(sink)?;
			}
			if self.contains(Self::FullWidth) {
				if self.has_case_transform() {
					sink.write_char(' ')?;
				}
				atom!("full-width").write_css(sink)?;
			}
			if self.contains(Self::FullSizeKana) {
				if self.has_case_transform() || self.contains(Self::FullWidth) {
					sink.write_char(' ')?;
				}
				atom!("full-size-kana").write_css(sink)?;
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
		assert_size!(TextTransform, 2);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TextTransform, "none");
		assert_parse!(TextTransform, "capitalize");
		assert_parse!(TextTransform, "lowercase full-width");
		assert_parse!(TextTransform, "uppercase full-width full-size-kana");
		assert_parse!(TextTransform, "uppercase full-size-kana");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(TextTransform, "");
		assert_parse_error!(TextTransform, "lowercase uppercase");
		assert_parse_error!(TextTransform, "capitalize uppercase");
		assert_parse_error!(TextTransform, "none capitalize");
		assert_parse_error!(TextTransform, "none full-size-kana");
		assert_parse_error!(TextTransform, "full-width-kana");
	}
}
