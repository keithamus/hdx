use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{diagnostics::UnexpectedDimension, unexpected, Parse, Parser, Result as ParserResult, Spanned};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::CSSFloat;
use crate::Writable;

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

		#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(Serialize), serde())]
		pub enum Length {
			#[writable(rename = "0")]
			Zero, // TODO: atom!("zero") <- shouldn't need to be an atom but something weird is happening
			$(
			#[writable(suffix = $atom)]
			$name(CSSFloat),
			)+
		}

		impl Length {
			pub fn new(val: CSSFloat, atom: Atom) -> Option<Length> {
				match atom {
					$(atom!($atom) => Some(Length::$name(val)),)+
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
			fn parse(parser: &mut Parser) -> ParserResult<Spanned<Self>> {
				let span = parser.span();
				match parser.cur() {
					Token::Number(n, _) if n == 0.0 => {
						parser.advance();
						Ok(Self::Zero.spanned(span))
					}
					Token::Dimension(n, unit, _) => {
						if let Some(length) = Self::new(n.into(), unit.clone()) {
							parser.advance();
							Ok(length.spanned(span))
						} else {
							Err(UnexpectedDimension(unit, span))?
						}
					}
					token => unexpected!(parser, token),
				}
			}
		}

		#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(Serialize), serde())]
		pub enum LengthPercentage {
			#[writable(rename = "0")]
			Zero,
			$(
			#[writable(suffix = $atom)]
			$name(CSSFloat),
			)+
			#[writable(suffix = "%")]
			Percent(CSSFloat),
		}

		impl LengthPercentage {
			pub fn new(val: CSSFloat, atom: Atom) -> Option<LengthPercentage> {
				match atom {
					$(atom!($atom) => Some(LengthPercentage::$name(val)),)+
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
			fn parse(parser: &mut Parser) -> ParserResult<Spanned<Self>> {
				let span = parser.span();
				match parser.cur() {
					Token::Number(n, _) if n == 0.0 => {
						parser.advance();
						Ok(Self::Zero.spanned(span))
					},
					Token::Dimension(n, unit, _) => {
						parser.advance();
						if let Some(length) = Self::new(n.into(), unit.clone()) {
							Ok(length.spanned(span))
						} else {
							Err(UnexpectedDimension(unit, span))?
						}
					}
					token => unexpected!(parser, token),
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

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Length>(), 8);
		assert_eq!(::std::mem::size_of::<LengthPercentage>(), 8);
	}

	#[test]
	fn test_variants() {
		let allocator = Allocator::default();
		test_write::<Length>(&allocator, "10px", "10px");
		// Truncates to 7dp
		test_write::<Length>(&allocator, "1.2345678901234px", "1.2345679px");
		// Removes redundant dp
		test_write::<Length>(&allocator, "-1.0px", "-1px");
	}
}
