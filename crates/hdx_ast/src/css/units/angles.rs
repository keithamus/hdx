use hdx_atom::atom;
use hdx_derive::Writable;
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, T};

use super::{AbsoluteUnit, CSSFloat};

const DEG_GRAD: f32 = 0.9;
const DEG_RAD: f32 = 57.295_78;
const DEG_TURN: f32 = 360.0;

// https://drafts.csswg.org/css-values/#angles
#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Angle {
	#[writable(suffix = "grad")]
	Grad(CSSFloat),
	#[writable(suffix = "rad")]
	Rad(CSSFloat),
	#[writable(suffix = "turn")]
	Turn(CSSFloat),
	#[writable(suffix = "deg")]
	Deg(CSSFloat),
}

impl<'a> Peek<'a> for Angle {
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<T![Dimension]>()
	}
}

impl<'a> Parse<'a> for Angle {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let token = p.parse::<T![Dimension]>()?;
		match p.parse_atom_lower(*token) {
			atom!("grad") => Ok(Angle::Grad(p.parse_number(*token).into())),
			atom!("rad") => Ok(Angle::Rad(p.parse_number(*token).into())),
			atom!("turn") => Ok(Angle::Turn(p.parse_number(*token).into())),
			atom!("deg") => Ok(Angle::Deg(p.parse_number(*token).into())),
			atom => Err(diagnostics::UnexpectedDimension(atom, token.span()))?,
		}
	}
}

impl From<Angle> for CSSFloat {
	fn from(val: Angle) -> Self {
		match val {
			Angle::Grad(f) | Angle::Rad(f) | Angle::Turn(f) | Angle::Deg(f) => f,
		}
	}
}

impl AbsoluteUnit for Angle {
	fn to_base(&self) -> Self {
		Self::Deg(match self {
			Self::Grad(f) => *f * DEG_GRAD,
			Self::Rad(f) => *f * DEG_RAD,
			Self::Turn(f) => *f * DEG_TURN,
			Self::Deg(f) => *f,
		})
	}
}
