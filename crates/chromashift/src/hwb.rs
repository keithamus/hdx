use crate::{Alpha, Hsl, Hsv, Number, Srgb, ToAlpha};
use core::fmt;

/// An colour represented as Hue, Whiteness, and Blackness expressed in the sRGB colour space.
/// The components are:
/// - Hue - a number between 0.0 and 360.0
/// - Whiteness - a number between 0.0 and 100.0
/// - Blackness - a number between 0.0 and 100.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hwb {
	pub hue: f64,
	pub whiteness: f64,
	pub blackness: f64,
	pub alpha: f32,
}

impl Hwb {
	pub fn new(hue: f64, whiteness: f64, blackness: f64, alpha: f32) -> Self {
		Self { hue: hue.rem_euclid(360.0), whiteness, blackness, alpha: alpha.clamp(0.0, 100.0) }
	}
}

impl ToAlpha for Hwb {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Hwb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { hue, whiteness, blackness, alpha } = *self;
		write!(f, "hwb({} {}% {}%{})", Number(hue), Number(whiteness), Number(blackness), Alpha(alpha))
	}
}

impl From<Hsv> for Hwb {
	fn from(value: Hsv) -> Self {
		let Hsv { hue, saturation, value, alpha } = value;
		let s = saturation as f64 / 100.0;
		let v = value as f64 / 100.0;
		let whiteness = (1.0 - s) * v;
		let blackness = 1.0 - v;
		Hwb::new(hue as f64, whiteness * 100.0, blackness * 100.0, alpha)
	}
}

impl From<Hwb> for Hsv {
	fn from(value: Hwb) -> Self {
		let Hwb { hue, whiteness, blackness, alpha } = value;
		let w = whiteness / 100.0;
		let b = blackness / 100.0;
		let sum = w + b;
		let (s, v) = if sum >= 1.0 {
			(0.0, w / sum)
		} else {
			let v = 1.0 - b;
			let s = if v == 0.0 { 0.0 } else { 1.0 - w / v };
			(s, v)
		};
		Hsv::new(hue as f32, (s * 100.0) as f32, (v * 100.0) as f32, alpha)
	}
}

impl From<Srgb> for Hwb {
	fn from(value: Srgb) -> Self {
		let Srgb { red, green, blue, alpha } = value;
		let whiteness = red.min(green).min(blue);
		let blackness = 1.0 - red.max(green).max(blue);
		Hwb::new(value.hue(), whiteness * 100.0, blackness * 100.0, alpha)
	}
}

impl From<Hwb> for Srgb {
	fn from(value: Hwb) -> Self {
		let Hwb { hue, whiteness, blackness, alpha } = value;
		let (w, b) = (whiteness / 100.0, blackness / 100.0);
		if w + b >= 1.0 {
			let gray = w / (w + b);
			return Srgb::new(gray, gray, gray, alpha);
		}
		let Srgb { red, green, blue, .. } = Hsl::new(hue, 100.0, 50.0, alpha).into();
		let scale = 1.0 - w - b;
		Srgb::new(red * scale + w, green * scale + w, blue * scale + w, alpha)
	}
}
