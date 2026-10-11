use crate::{Alpha, Number, ToAlpha};
use core::fmt;

/// An colour represented as Hue, Saturation, and Value expressed in the sRGB colour space.
/// The components are:
/// - Hue - a number between 0.0 and 360.0
/// - Saturation - a number between 0.0 and 100.0
/// - Value / Brightness - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
	pub hue: f32,
	pub saturation: f32,
	pub value: f32,
	pub alpha: f32,
}

impl Hsv {
	pub fn new(hue: f32, saturation: f32, value: f32, alpha: f32) -> Self {
		Self {
			hue: hue.rem_euclid(360.0),
			saturation: saturation.clamp(0.0, 100.0),
			value: value.clamp(0.0, 100.0),
			alpha: alpha.clamp(0.0, 100.0),
		}
	}
}

impl ToAlpha for Hsv {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Hsv {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { hue, saturation, value, alpha } = *self;
		write!(f, "hsv({} {} {}{})", Number(hue), Number(saturation), Number(value), Alpha(alpha))
	}
}
