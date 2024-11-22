use hdx_atom::atom;
use hdx_derive::Writable;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};

use super::{AbsoluteUnit, CSSFloat};

const DPPX_IN: f32 = 96.0;
const DPPX_CM: f32 = DPPX_IN / 2.54;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Resolution {
	#[writable(suffix = "dpi")]
	Dpi(CSSFloat),
	#[writable(suffix = "dpcm")]
	Dpcm(CSSFloat),
	#[writable(suffix = "dppx")]
	Dppx(CSSFloat),
}

impl<'a> Peek<'a> for Resolution {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		if let Some(token) = p.peek::<T![Dimension]>() {
			if matches!(p.parse_atom_lower(token), atom!("dpi") | atom!("dpcm") | atom!("dppx")) {
				return Some(token);
			}
		}
		None
	}
}

impl<'a> Parse<'a> for Resolution {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *p.parse::<T![Dimension]>()?;
		return match p.parse_atom_lower(token) {
			atom!("dpi") => Ok(Self::Dpi(p.parse_number(token).into())),
			atom!("dpcm") => Ok(Self::Dpcm(p.parse_number(token).into())),
			atom!("dppx") => Ok(Self::Dppx(p.parse_number(token).into())),
			atom => Err(diagnostics::UnexpectedDimension(atom, token.span()))?,
		};
	}
}

impl Into<CSSFloat> for Resolution {
	fn into(self) -> CSSFloat {
		match self {
			Self::Dpi(f) | Self::Dpcm(f) | Self::Dppx(f) => f,
		}
	}
}

impl AbsoluteUnit for Resolution {
	fn to_base(&self) -> Self {
		Self::Dppx(match self {
			Self::Dpi(f) => *f * DPPX_IN,
			Self::Dpcm(f) => *f * DPPX_CM,
			Self::Dppx(f) => *f,
		})
	}
}
