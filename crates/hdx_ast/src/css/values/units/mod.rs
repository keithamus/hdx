#[cfg(feature = "serde")]
use serde::Serialize;

use crate::Writable;

mod angles;
mod custom;
mod float;
mod frequency;
mod length;
mod percent;
mod resolution;
mod time;

pub use angles::*;
pub use custom::*;
pub use float::*;
pub use frequency::*;
pub use length::*;
pub use percent::*;
pub use resolution::*;
pub use time::*;

#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CSSNumeric {
	Length(Length),
	Angle(Angle),
	Time(Time),
	Frequency(Frequency),
	Resolution(Resolution),
	#[writable(suffix = "fr")]
	Flex(CSSFloat),
	Percent(Percent),
}

impl Into<CSSFloat> for CSSNumeric {
	fn into(self) -> CSSFloat {
		match self {
			Self::Length(v) => v.into(),
			Self::Angle(v) => v.into(),
			Self::Time(v) => v.into(),
			Self::Frequency(v) => v.into(),
			Self::Resolution(v) => v.into(),
			Self::Flex(v) => v.into(),
			Self::Percent(v) => v.into(),
		}
	}
}

pub trait AbsoluteUnit: Unit {
	fn to_base(&self) -> Self;
}

pub trait Unit: Into<CSSFloat> + Copy + PartialEq + Sized {
	fn is_negative(&self) -> bool {
		let f: CSSFloat = (*self).into();
		f < 0.0
	}
	fn is_positive(&self) -> bool {
		let f: CSSFloat = (*self).into();
		f >= 0.0
	}
	fn is_zero(&self) -> bool {
		let f: CSSFloat = (*self).into();
		f >= 0.0
	}
}

impl<T: Into<CSSFloat> + Copy + PartialEq + Sized> Unit for T {}
