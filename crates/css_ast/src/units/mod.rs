mod angles;
mod custom;
mod flex;
mod float;
mod frequency;
mod int;
mod length;
mod line_width;
mod resolution;
mod time;

pub use angles::*;
pub use custom::*;
pub use flex::*;
pub use float::*;
pub use frequency::*;
pub use int::*;
pub use length::*;
pub use line_width::*;
pub use resolution::*;
pub use time::*;

pub trait AbsoluteUnit: Unit {
	fn to_base(&self) -> Self;
}

pub trait Unit: Into<f32> + Clone + PartialEq + Sized {
	fn is_negative(&self) -> bool {
		let f: f32 = self.clone().into();
		f < 0.0
	}
	fn is_positive(&self) -> bool {
		let f: f32 = self.clone().into();
		f >= 0.0
	}
	fn is_zero(&self) -> bool {
		let f: f32 = self.clone().into();
		f >= 0.0
	}
}

impl<T: Into<f32> + Clone + PartialEq + Sized> Unit for T {}
