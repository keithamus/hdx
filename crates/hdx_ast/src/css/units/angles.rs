const DEG_GRAD: f32 = 0.9;
const DEG_RAD: f32 = 57.295_78;
const DEG_TURN: f32 = 360.0;

use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::FromToken;

use super::{AbsoluteUnit, CSSFloat};
use crate::Writable;

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

impl FromToken for Angle {
	fn from_token(token: &Token) -> Option<Self> {
		if let Token::Dimension(n, unit, _) = token {
			match unit.to_ascii_lowercase() {
				atom!("grad") => Some(Angle::Grad(n.into())),
				atom!("rad") => Some(Angle::Rad(n.into())),
				atom!("turn") => Some(Angle::Turn(n.into())),
				atom!("deg") => Some(Angle::Deg(n.into())),
				_ => None,
			}
		} else {
			None
		}
	}
}

impl Into<CSSFloat> for Angle {
	fn into(self) -> CSSFloat {
		match self {
			Self::Grad(f) | Self::Rad(f) | Self::Turn(f) | Self::Deg(f) => f,
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
