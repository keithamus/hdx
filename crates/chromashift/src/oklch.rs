use crate::{Alpha, Number, Oklab, ToAlpha};
use core::fmt;

/// A more adequate expression of LCH, in the CIE colour space.
/// The components are:
/// - Lightness / Luminance - a number between 0.0 and 100.0
/// - Chroma - a number between 0.0 and 150.0
/// - Hue - a number between 0.0 and 360.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklch {
	pub lightness: f64,
	pub chroma: f64,
	pub hue: f64,
	pub alpha: f32,
}

impl Oklch {
	pub fn new(lightness: f64, chroma: f64, hue: f64, alpha: f32) -> Self {
		Self { lightness, chroma, hue: hue.rem_euclid(360.0), alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Oklch {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Oklch {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { lightness, chroma, hue, alpha } = *self;
		write!(f, "oklch({} {} {}{})", Number(lightness), Number(chroma), Number(hue), Alpha(alpha))
	}
}

impl From<Oklab> for Oklch {
	fn from(value: Oklab) -> Self {
		let Oklab { lightness, a, b, alpha } = value;
		let chroma = (a * a + b * b).sqrt();
		let hue = b.atan2(a).to_degrees();
		let hue = if chroma <= 0.000004 {
			0.0
		} else if hue < 0.0 {
			hue + 360.0
		} else {
			hue
		};
		Oklch::new(lightness, chroma, hue, alpha)
	}
}

impl From<Oklch> for Oklab {
	fn from(value: Oklch) -> Self {
		let Oklch { lightness, chroma, hue, alpha } = value;
		let hue_rad = hue.to_radians();
		let a = chroma * hue_rad.cos();
		let b = chroma * hue_rad.sin();
		Oklab::new(lightness, a, b, alpha)
	}
}
