use bitmask_enum::bitmask;
use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-fonts/#font-variant-ligatures-prop
#[derive(Value, Default)]
#[value(Inherits)]
#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum FontVariantNumeric {
	#[default]
	Normal = 0b0000_0000_0000,

	LiningNums = 0b0000_0000_0010,
	OldstyleNums = 0b0000_0000_0100,

	ProportionalNums = 0b0000_0000_1000,
	TabularNums = 0b0000_0001_0000,

	DiagonalFractions = 0b0000_0010_0000,
	StackedFractions = 0b0000_0100_0000,

	Ordinal = 0b0000_1000_0000,

	SlashedZero = 0b0001_0000_0000,
}

impl FontVariantNumeric {
	#[inline]
	fn has_figure_values(&self) -> bool {
		self.bits & 0b0000_0110 > 0
	}

	#[inline]
	fn has_spacing_values(&self) -> bool {
		self.bits & 0b0001_1000 > 0
	}

	#[inline]
	fn has_fraction_values(&self) -> bool {
		self.bits & 0b0110_0000 > 0
	}
}

impl<'a> Parse<'a> for FontVariantNumeric {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut value = Self::Normal;
		loop {
			match parser.peek() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					atom!("normal") => {
						parser.advance();
						return Ok(Self::Normal);
					}
					atom!("lining-nums") if !value.has_figure_values() => value |= Self::LiningNums,
					atom!("oldstyle-nums") if !value.has_figure_values() => value |= Self::OldstyleNums,
					atom!("proportional-nums") if !value.has_spacing_values() => value |= Self::ProportionalNums,
					atom!("tabular-nums") if !value.has_spacing_values() => value |= Self::TabularNums,
					atom!("diagonal-fractions") if !value.has_fraction_values() => value |= Self::DiagonalFractions,
					atom!("stacked-fractions") if !value.has_fraction_values() => value |= Self::StackedFractions,
					atom!("ordinal") if !value.contains(Self::Ordinal) => value |= Self::Ordinal,
					atom!("slashed-zero") if !value.contains(Self::SlashedZero) => value |= Self::SlashedZero,
					_ => unexpected_ident!(parser, atom),
				},
				_ => break,
			}
			parser.advance();
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for FontVariantNumeric {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match *self {
			Self::Normal => atom!("normal").write_css(sink),
			val => {
				if val.contains(Self::LiningNums) {
					atom!("lining-nums").write_css(sink)?;
					if val.has_spacing_values()
						|| val.has_fraction_values()
						|| val.contains(Self::Ordinal)
						|| val.contains(Self::SlashedZero)
					{
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::OldstyleNums) {
					atom!("oldstyle-nums").write_css(sink)?;
					if val.has_spacing_values()
						|| val.has_fraction_values()
						|| val.contains(Self::Ordinal)
						|| val.contains(Self::SlashedZero)
					{
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::ProportionalNums) {
					atom!("proportional-nums").write_css(sink)?;
					if val.has_fraction_values() || val.contains(Self::Ordinal) || val.contains(Self::SlashedZero) {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::TabularNums) {
					atom!("tabular-nums").write_css(sink)?;
					if val.has_fraction_values() || val.contains(Self::Ordinal) || val.contains(Self::SlashedZero) {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::DiagonalFractions) {
					atom!("diagonal-fractions").write_css(sink)?;
					if val.contains(Self::Ordinal) || val.contains(Self::SlashedZero) {
						sink.write_char(' ')?;
					}
				} else if val.contains(Self::StackedFractions) {
					atom!("stacked-fractions").write_css(sink)?;
					if val.contains(Self::Ordinal) || val.contains(Self::SlashedZero) {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::Ordinal) {
					atom!("ordinal").write_css(sink)?;
					if val != Self::Ordinal {
						sink.write_char(' ')?;
					}
				}
				if val.contains(Self::SlashedZero) {
					atom!("slashed-zero").write_css(sink)?;
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
		assert_size!(FontVariantNumeric, 2);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantNumeric, "normal");
		assert_parse!(FontVariantNumeric, "tabular-nums");
		assert_parse!(FontVariantNumeric, "oldstyle-nums stacked-fractions");
		assert_parse!(FontVariantNumeric, "lining-nums tabular-nums");
		assert_parse!(FontVariantNumeric, "lining-nums proportional-nums diagonal-fractions ordinal slashed-zero");
	}
}
