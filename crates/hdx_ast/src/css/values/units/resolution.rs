const DPPX_IN: f32 = 96.0;
const DPPX_CM: f32 = DPPX_IN / 2.54;

use super::{AbsoluteUnit, CSSFloat};
use crate::{Parsable, Writable};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Parsable, Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Resolution {
	#[writable(suffix = "dpi")]
	#[parsable(Dimension)]
	Dpi(CSSFloat),
	#[writable(suffix = "dpcm")]
	#[parsable(Dimension)]
	Dpcm(CSSFloat),
	#[writable(suffix = "dppx")]
	#[parsable(Dimension)]
	Dppx(CSSFloat),
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
