use hdx_atom::{atom, Atom};
use hdx_parser::{diagnostics, token, Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use super::{CSSFloat, Flex};

// const PX_CM: f32 = PX_IN / 2.54;
// const PX_MM: f32 = PX_IN / 25.4;
// const PX_Q: f32 = PX_MM / 4.0;
// const PX_IN: f32 = 96.0;
// const PX_PC: f32 = PX_IN / 6.0;
// const PX_PT: f32 = PX_IN / 72.0;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Auto, atom!("auto"));
	custom_keyword!(Thin, atom!("thin"));
	custom_keyword!(Medium, atom!("medium"));
	custom_keyword!(Thick, atom!("thick"));
}

macro_rules! length {
    ( $(
        $name: ident: $atom: tt,
    )+ ) => {

		#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		pub enum Length {
			#[default]
			#[cfg_attr(feature = "serde", serde())]
			Zero,
			$($name(CSSFloat),)+
		}

		impl Length {
			pub fn new(val: CSSFloat, atom: Atom) -> Option<Self> {
				match atom {
					$(atom!($atom) => Some(Self::$name(val)),)+
					_ => None
				}
			}
		}

		impl Into<CSSFloat> for Length {
			fn into(self) -> CSSFloat {
				match self {
					$(Self::$name(f) => f,)+
					Self::Zero => 0.0.into(),
				}
			}
		}

		impl Into<f32> for Length {
			fn into(self) -> f32 {
				match self {
					$(Self::$name(f) => f.into(),)+
					Self::Zero => 0.0,
				}
			}
		}

		impl<'a> Peek<'a> for Length {
			fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
				parser.peek::<token::Number>().or_else(|| parser.peek::<token::Dimension>())
			}
		}

		impl<'a> Parse<'a> for Length {
			fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
				if let Some(token) = parser.peek::<T![Number]>() {
					parser.hop(token);
					if parser.parse_number(token) == 0.0 {
						return Ok(Self::Zero);
					} else {
						Err(diagnostics::Unexpected(token, token.span()))?
					}
				}
				let token = *parser.parse::<T![Dimension]>()?;
				if let Some(d) = Self::new(parser.parse_number(token).into(), parser.parse_atom_lower(token)) {
					Ok(d)
				} else {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
			}
		}

		impl<'a> WriteCss<'a> for Length {
			fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
				match self {
					Self::Zero => write_css!(sink, '0'),
					$(Self::$name(f) => write_css!(sink, f, $atom)),+
				}
				Ok(())
			}
		}

		#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		pub enum LengthPercentage {
			#[default]
			Zero,
			$($name(CSSFloat),)+
			#[cfg_attr(feature = "serde", serde(rename = "%"))]
			Percent(CSSFloat),
		}

		impl LengthPercentage {
			pub fn new(val: CSSFloat, atom: Atom) -> Option<Self> {
				match atom {
					$(atom!($atom) => Some(Self::$name(val)),)+
					atom!("%") => Some(Self::Percent(val)),
					_ => None
				}
			}
		}

		impl Into<CSSFloat> for LengthPercentage {
			fn into(self) -> CSSFloat {
				match self {
					$(Self::$name(f) => f,)+
					Self::Percent(f) => f,
					Self::Zero => 0.0.into(),
				}
			}
		}

		impl Into<f32> for LengthPercentage {
			fn into(self) -> f32 {
				match self {
					$(Self::$name(f) => f.into(),)+
					Self::Percent(f) => f.into(),
					Self::Zero => 0.0,
				}
			}
		}

		impl<'a> Peek<'a> for LengthPercentage {
			fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
				parser.peek::<T![Number]>().or_else(|| parser.peek::<T![Dimension]>())
			}
		}

		impl<'a> Parse<'a> for LengthPercentage {
			fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
				if let Some(token) = parser.peek::<T![Number]>() {
					parser.hop(token);
					if parser.parse_number(token) == 0.0 {
						return Ok(Self::Zero);
					} else {
						Err(diagnostics::Unexpected(token, token.span()))?
					}
				}
				let token = *parser.parse::<T![Dimension]>()?;
				if let Some(d) = Self::new(parser.parse_number(token).into(), parser.parse_atom_lower(token)) {
					Ok(d)
				} else {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
			}
		}

		impl<'a> WriteCss<'a> for LengthPercentage {
			fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
				match self {
					Self::Zero => write_css!(sink, '0'),
					Self::Percent(f) => write_css!(sink, f, '%'),
					$(Self::$name(f) => write_css!(sink, f, $atom)),+
				}
				Ok(())
			}
		}

	}
}

length! {
	// https://drafts.csswg.org/css-values/#font-relative-lengths
	Em: "em", // atom!("em")
	Rem: "rem", // atom!("rem")
	Ex: "ex", // atom!("ex")
	Rex: "rex", // atom!("rex")
	Cap: "cap", // atom!("cap")
	Rcap: "rcap", // atom!("rcap")
	Ch: "ch", // atom!("ch")
	Rch: "rch", // atom!("rch")
	Ic: "ic", // atom!("ic")
	Ric: "ric", // atom!("ric")
	Lh: "lh", // atom!("lh")
	Rlh: "rlh", // atom!("rlh")

	// https://drafts.csswg.org/css-values/#viewport-relative-units
	Vw: "vw", // atom!("vw")
	Svw: "svw", // atom!("svw")
	Lvw: "lvw", // atom!("lvw")
	Dvw: "dvw", // atom!("dvw")
	Vh: "vh", // atom!("vh")
	Svh: "svh", // atom!("svh")
	Lvh: "lvh", // atom!("lvh")
	Dvh: "dvh", // atom!("dvh")
	Vi: "vi", // atom!("vi")
	Svi: "svi", // atom!("svi")
	Lvi: "lvi", // atom!("lvi")
	Dvi: "dvi", // atom!("dvi")
	Vb: "vb", // atom!("vb")
	Svb: "svb", // atom!("svb")
	Lvb: "lvb", // atom!("lvb")
	Dvb: "dvb", // atom!("dvb")
	Vmin: "vmin", // atom!("vmin")
	Svmin: "svmin", // atom!("svmin")
	Lvmin: "lvmin", // atom!("lvmin")
	Dvmin: "dvmin", // atom!("dvmin")
	Vmax: "vmax", // atom!("vmax")
	Svmax: "svmax", // atom!("svmax")
	Lvmax: "lvmax", // atom!("lvmax")
	Dvmax: "dvmax", // atom!("dvmax")

	// https://drafts.csswg.org/css-values/#absolute-lengths
	Cm: "cm", // atom!("cm")
	Mm: "mm", // atom!("mm")
	Q: "q", // atom!("q")
	In: "in", // atom!("in")
	Pc: "pc", // atom!("pc")
	Pt: "pt", // atom!("pt")
	Px: "px", // atom!("px")

	// https://www.w3.org/TR/css-contain-3/#container-lengths
	Cqw: "cqw", // atom!("cqw")
	Cqh: "cqh", // atom!("cqh")
	Cqi: "cqi", // atom!("cqi")
	Cqb: "cqb", // atom!("cqb")
	Cqmin: "cqmin", // atom!("cqmin")
	Cqmax: "cqmax", // atom!("cqmax")
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LengthPercentageOrAuto {
	#[default]
	Auto,
	LengthPercentage(LengthPercentage),
}

impl<'a> Peek<'a> for LengthPercentageOrAuto {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<kw::Auto>().or_else(|| parser.peek::<LengthPercentage>())
	}
}

impl<'a> Parse<'a> for LengthPercentageOrAuto {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::Auto>() {
			parser.hop(token);
			Ok(Self::Auto)
		} else {
			Ok(Self::LengthPercentage(parser.parse::<LengthPercentage>()?))
		}
	}
}

impl<'a> WriteCss<'a> for LengthPercentageOrAuto {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Auto => kw::Auto::atom().write_css(sink),
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LengthPercentageOrFlex {
	Flex(Flex),
	LengthPercentage(LengthPercentage),
}

impl<'a> Peek<'a> for LengthPercentageOrFlex {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<Flex>().or_else(|| parser.peek::<LengthPercentage>())
	}
}

impl<'a> Parse<'a> for LengthPercentageOrFlex {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(flex) = parser.parse_if_peek::<Flex>()? {
			Ok(Self::Flex(flex))
		} else {
			Ok(Self::LengthPercentage(parser.parse::<LengthPercentage>()?))
		}
	}
}

impl<'a> WriteCss<'a> for LengthPercentageOrFlex {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Flex(f) => f.write_css(sink),
			Self::LengthPercentage(l) => l.write_css(sink),
		}
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LineWidth {
	Thin,
	#[default]
	Medium,
	Thick,
	Length(Length),
}

impl<'a> Peek<'a> for LineWidth {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser
			.peek::<kw::Thin>()
			.or_else(|| parser.peek::<kw::Medium>())
			.or_else(|| parser.peek::<kw::Thick>())
			.or_else(|| parser.peek::<Length>())
	}
}

impl<'a> Parse<'a> for LineWidth {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::Thin>() {
			parser.hop(token);
			Ok(Self::Thin)
		} else if let Some(token) = parser.peek::<kw::Medium>() {
			parser.hop(token);
			Ok(Self::Medium)
		} else if let Some(token) = parser.peek::<kw::Thick>() {
			parser.hop(token);
			Ok(Self::Thick)
		} else {
			Ok(Self::Length(parser.parse::<Length>()?))
		}
	}
}

impl<'a> WriteCss<'a> for LineWidth {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Thin => kw::Thin::atom().write_css(sink),
			Self::Medium => kw::Medium::atom().write_css(sink),
			Self::Thick => kw::Thick::atom().write_css(sink),
			Self::Length(l) => l.write_css(sink),
		}
	}
}

impl From<LineWidth> for Length {
	fn from(value: LineWidth) -> Self {
		match value {
			LineWidth::Thin => Length::Px(1.0.into()),
			LineWidth::Medium => Length::Px(3.0.into()),
			LineWidth::Thick => Length::Px(3.0.into()),
			LineWidth::Length(length) => length,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Length, 8);
		assert_size!(LengthPercentage, 8);
		assert_size!(LengthPercentageOrAuto, 8);
		assert_size!(LineWidth, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Length, "10px");
		// Truncates to 7dp
		assert_parse!(Length, "1.2345679px");
		// Removes redundant dp
		assert_parse!(Length, "-1px");
		// Percent
		assert_parse!(LengthPercentage, "1%");
		assert_parse!(LengthPercentageOrAuto, "auto");
		// LineWidth
		assert_parse!(LineWidth, "1px");
		assert_parse!(LineWidth, "medium");
	}
}
