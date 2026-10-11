use crate::{Alpha, Number, Srgb, ToAlpha};
use core::fmt;

/// An colour represented as Hue, Saturation, and Lightness expressed in the sRGB colour space.
/// The components are:
/// - Hue - a number between 0.0 and 360.0
/// - Saturation - a number between 0.0 and 100.0
/// - Brightness - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsl {
	pub hue: f64,
	pub saturation: f64,
	pub lightness: f64,
	pub alpha: f32,
}

impl Hsl {
	pub fn new(hue: f64, saturation: f64, lightness: f64, alpha: f32) -> Self {
		Self { hue: hue.rem_euclid(360.0), saturation, lightness, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Hsl {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Hsl {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { hue, saturation, lightness, alpha } = *self;
		write!(f, "hsl({} {}% {}%{})", Number(hue), Number(saturation), Number(lightness), Alpha(alpha))
	}
}

impl From<Srgb> for Hsl {
	fn from(value: Srgb) -> Self {
		let Srgb { red, green, blue, alpha } = value;
		let max = red.max(green).max(blue);
		let min = red.min(green).min(blue);
		let lightness = (max + min) / 2.0;
		let mut hue = value.hue();
		let mut saturation = if max - min <= 1e-9 || lightness == 0.0 || lightness == 1.0 {
			0.0
		} else {
			(max - lightness) / lightness.min(1.0 - lightness)
		};
		if saturation < 0.0 {
			hue += 180.0;
			saturation = -saturation;
		}
		if saturation <= 1e-5 {
			hue = 0.0;
		}
		Hsl::new(hue, saturation * 100.0, lightness * 100.0, alpha)
	}
}

impl From<Hsl> for Srgb {
	fn from(value: Hsl) -> Self {
		let Hsl { hue, saturation, lightness, alpha } = value;
		let (saturation, lightness) = (saturation / 100.0, lightness / 100.0);
		let a = saturation * lightness.min(1.0 - lightness);
		let f = |n: f64| {
			let k = (n + hue / 30.0) % 12.0;
			lightness - a * (k - 3.0).min(9.0 - k).clamp(-1.0, 1.0)
		};
		Srgb::new(f(0.0), f(8.0), f(4.0), alpha)
	}
}
