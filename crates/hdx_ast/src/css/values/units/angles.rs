const DEG_GRAD: f32 = 0.9;
const DEG_RAD: f32 = 57.29577951308232;
const DEG_TURN: f32 = 360.0;

use super::{AbsoluteUnit, CSSFloat};
use crate::{Parsable, Writable};

// https://drafts.csswg.org/css-values/#angles
#[derive(Parsable, Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Angle {
	#[writable(suffix = "grad")]
	#[parsable(Dimension)]
	Grad(CSSFloat),
	#[writable(suffix = "rad")]
	#[parsable(Dimension)]
	Rad(CSSFloat),
	#[writable(suffix = "turn")]
	#[parsable(Dimension)]
	Turn(CSSFloat),
	#[writable(suffix = "deg")]
	#[parsable(Dimension)]
	Deg(CSSFloat),
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
