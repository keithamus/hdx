use bitmask_enum::bitmask;
use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-fonts/#font-variant-ligatures-prop
#[derive(Value, Default)]
#[value(Inherits)]
#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum FontVariantLigatures {
	#[default]
	Normal = 0b0000_0000_0000,
	None = 0b0000_0000_0001,

	CommonLigatures = 0b0000_0000_0010,
	NoCommonLigatures = 0b0000_0000_0100,

	NoDiscretionaryLigatures = 0b0000_0000_1000,
	DiscretionaryLigatures = 0b0000_0001_0000,

	HistoricalLigatures = 0b0000_0010_0000,
	NoHistoricalLigatures = 0b0000_0100_0000,

	Contextual = 0b0000_1000_0000,
	NoContextual = 0b0001_0000_0000,
}

impl FontVariantLigatures {
	#[inline]
	fn has_common_lig(&self) -> bool {
		self.bits & 0b0000_0110 > 0
	}

	#[inline]
	fn has_discretionary_lig(&self) -> bool {
		self.bits & 0b0001_1000 > 0
	}

	#[inline]
	fn has_historical_lig(&self) -> bool {
		self.bits & 0b0110_0000 > 0
	}

	#[inline]
	fn has_contextual(&self) -> bool {
		self.bits & 0b0001_1000_0000 > 0
	}
}

impl<'a> Parse<'a> for FontVariantLigatures {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("none") => {
					parser.next();
					return Ok(Self::None);
				}
				atom!("normal") => {
					parser.next();
					return Ok(Self::Normal);
				}
				_ => {}
			},
			token => unexpected!(parser, token),
		}
		let mut value = Self::Normal;
		while let Token::Ident(atom) = parser.peek() {
			match atom.to_ascii_lowercase() {
				atom!("common-ligatures") if !value.has_common_lig() => value |= Self::CommonLigatures,
				atom!("no-common-ligatures") if !value.has_common_lig() => value |= Self::NoCommonLigatures,
				atom!("discretionary-ligatures") if !value.has_discretionary_lig() => {
					value |= Self::DiscretionaryLigatures
				}
				atom!("no-discretionary-ligatures") if !value.has_discretionary_lig() => {
					value |= Self::NoDiscretionaryLigatures
				}
				atom!("historical-ligatures") if !value.has_historical_lig() => value |= Self::HistoricalLigatures,
				atom!("no-historical-ligatures") if !value.has_historical_lig() => value |= Self::NoHistoricalLigatures,
				atom!("contextual") if !value.has_contextual() => value |= Self::Contextual,
				atom!("no-contextual") if !value.has_contextual() => value |= Self::NoContextual,
				_ => break,
			}
			parser.next();
		}
		if value == Self::Normal {
			unexpected!(parser)
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for FontVariantLigatures {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match *self {
			Self::None => atom!("none").write_css(sink),
			Self::Normal => atom!("normal").write_css(sink),
			val => {
				if val.contains(Self::CommonLigatures) {
					atom!("common-ligatures").write_css(sink)?;
					if val != Self::CommonLigatures {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::NoCommonLigatures) {
					atom!("no-common-ligatures").write_css(sink)?;
					if val != Self::NoCommonLigatures {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::DiscretionaryLigatures) {
					atom!("discretionary-ligatures").write_css(sink)?;
					if val != Self::DiscretionaryLigatures {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::NoDiscretionaryLigatures) {
					atom!("no-discretionary-ligatures").write_css(sink)?;
					if val != Self::NoDiscretionaryLigatures {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::HistoricalLigatures) {
					atom!("historical-ligatures").write_css(sink)?;
					if val != Self::HistoricalLigatures {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::NoHistoricalLigatures) {
					atom!("no-historical-ligatures").write_css(sink)?;
					if val != Self::NoHistoricalLigatures {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::Contextual) {
					atom!("contextual").write_css(sink)?;
				} else if val.contains(Self::NoContextual) {
					atom!("no-contextual").write_css(sink)?;
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
		assert_size!(FontVariantLigatures, 2);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantLigatures, "normal");
		assert_parse!(FontVariantLigatures, "none");
		assert_parse!(FontVariantLigatures, "common-ligatures contextual");
		assert_parse!(FontVariantLigatures, "no-common-ligatures contextual");
		assert_parse!(FontVariantLigatures, "common-ligatures discretionary-ligatures historical-ligatures contextual");
		assert_parse!(
			FontVariantLigatures,
			"no-common-ligatures discretionary-ligatures no-historical-ligatures contextual"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(FontVariantLigatures, "none normal");
		assert_parse_error!(FontVariantLigatures, "small-caps");
		assert_parse_error!(FontVariantLigatures, "common-ligatures normal");
		assert_parse_error!(FontVariantLigatures, "common-ligatures common-ligatures");
	}
}
