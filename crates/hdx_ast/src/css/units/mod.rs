mod angles;
// mod custom;
mod flex;
mod float;
// mod frequency;
mod int;
mod length;
mod percent;
mod resolution;
mod string;
mod time;

pub use angles::*;
// pub use custom::*;
pub use flex::*;
pub use float::*;
// pub use frequency::*;
pub use int::*;
pub use length::*;
pub use percent::*;
// pub use resolution::*;
pub use string::*;
pub use time::*;

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
