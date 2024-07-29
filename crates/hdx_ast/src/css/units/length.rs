use hdx_atom::{atom, Atom};
use hdx_derive::Writable;
use hdx_lexer::Kind;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};

use super::CSSFloat;

const PX_CM: f32 = PX_IN / 2.54;
const PX_MM: f32 = PX_IN / 25.4;
const PX_Q: f32 = PX_MM / 4.0;
const PX_IN: f32 = 96.0;
const PX_PC: f32 = PX_IN / 6.0;
const PX_PT: f32 = PX_IN / 72.0;

macro_rules! length {
    ( $(
        $name: ident: $atom: tt,
    )+ ) => {

		#[derive(Writable, Default, Debug, Clone, Copy, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		pub enum Length {
			#[writable(rename = "0")]
			#[default]
			#[cfg_attr(feature = "serde", serde())]
			Zero,
			$(
			#[writable(suffix = $atom)]
			$name(CSSFloat),
			)+
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

		impl<'a> Parse<'a> for Length {
			fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
				let token = parser.next();
				match token.kind() {
					Kind::Number if parser.parse_number(token) == 0.0 => Ok(Self::Zero),
					Kind::Dimension => {
						if let Some(d) = Self::new(parser.parse_number(token).into(), parser.parse_atom_lower(token)) {
							Ok(d)
						} else {
							unexpected!(parser, token)
						}
					}
					_ => unexpected!(parser, token),
				}
			}
		}

		#[derive(Writable, Default, Debug, Clone, Copy, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		pub enum LengthPercentage {
			#[default]
			#[writable(rename = "0")]
			Zero,
			$(
			#[writable(suffix = $atom)]
			$name(CSSFloat),
			)+
			#[writable(suffix = "%")]
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

		impl<'a> Parse<'a> for LengthPercentage {
			fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
				let token = parser.next();
				match token.kind() {
					Kind::Number if parser.parse_number(token) == 0.0 => Ok(Self::Zero),
					Kind::Dimension => {
						if let Some(d) = Self::new(parser.parse_number(token).into(), parser.parse_atom_lower(token)) {
							Ok(d)
						} else {
							unexpected!(parser, token)
						}
					}
					_ => unexpected!(parser, token),
				}
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

#[derive(Writable, Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LengthPercentageOrAuto {
	#[default]
	Auto,
	LengthPercentage(LengthPercentage),
}

impl<'a> Parse<'a> for LengthPercentageOrAuto {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.next();
		match token.kind() {
			Kind::Ident => match parser.parse_atom_lower(token) {
				atom!("auto") => Ok(Self::Auto),
				atom => unexpected_ident!(parser, atom),
			},
			Kind::Dimension => {
				if let Some(l) = LengthPercentage::new(parser.parse_number(token).into(), parser.parse_atom_lower(token)) {
					Ok(Self::LengthPercentage(l))
				} else {
					unexpected!(parser, token)
				}
			}
			Kind::Number if parser.parse_number(token) == 0.0 => Ok(Self::LengthPercentage(LengthPercentage::Zero)),
			_ => unexpected!(parser, token),
		}
	}
}

#[derive(Writable, Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LineWidth {
	Thin,
	#[default]
	Medium,
	Thick,
	Length(Length),
}

impl<'a> Parse<'a> for LineWidth {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = parser.next();
		match token.kind() {
			Kind::Ident => match parser.parse_atom_lower(token) {
				atom!("thin") => Ok(Self::Thin),
				atom!("medium") => Ok(Self::Medium),
				atom!("thick") => Ok(Self::Thick),
				_ => unexpected_ident!(parser, atom),
			},
			Kind::Dimension => {
				if let Some(l) = Length::new(parser.parse_number(token).into(), parser.parse_atom_lower(token)).map(Self::Length) {
					Ok(l)
				} else {
					unexpected!(parser, token)
				}
			}
			Kind::Number if parser.parse_number(token) == 0.0 => Ok(Self::Length(Length::Zero)),
			_ => unexpected!(parser, token),
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
