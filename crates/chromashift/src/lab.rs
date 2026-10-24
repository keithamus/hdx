use crate::{Alpha, Number, ToAlpha, XyzD50};
use core::fmt;

const D50_WHITE: [f64; 3] = {
	let [x, y, z] = XyzD50::WHITE.xyz();
	[x * 100.0, y * 100.0, z * 100.0]
};

/// An CIE defined colour space representing L - perceptual lightness, and two axes A & B.
/// The components are:
/// - L - a number between 0.0 and 100.0
/// - A - a number between -125.0 and +125.0
/// - B - a number between -125.0 and +125.0
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
	pub lightness: f64,
	pub a: f64,
	pub b: f64,
	pub alpha: f32,
}

impl Lab {
	pub fn new(lightness: f64, a: f64, b: f64, alpha: f32) -> Self {
		Self { lightness, a, b, alpha: alpha.clamp(0.0, 100.0) }
	}

	/// Converts a CSS `<percentage>` lightness to a lightness, where `100%` is `100.0`.
	///
	/// <https://drafts.csswg.org/css-color-4/#specifying-lab-lch>
	pub fn lightness_from_percent(percent: f64) -> f64 {
		percent
	}

	/// Converts a CSS `<percentage>` `a` or `b` axis to an axis value, where `100%` is `125.0` and `-100%` is `-125.0`.
	///
	/// <https://drafts.csswg.org/css-color-4/#specifying-lab-lch>
	pub fn ab_from_percent(percent: f64) -> f64 {
		percent / 100.0 * 125.0
	}
}

impl ToAlpha for Lab {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Lab {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { lightness, a, b, alpha } = *self;
		write!(f, "lab({} {} {}{})", Number(lightness), Number(a), Number(b), Alpha(alpha))
	}
}

impl From<XyzD50> for Lab {
	fn from(value: XyzD50) -> Self {
		let XyzD50 { x, y, z, alpha } = value;
		let x = x / D50_WHITE[0];
		let y = y / D50_WHITE[1];
		let z = z / D50_WHITE[2];
		let epsilon = 216.0 / 24389.0; // 6^3/29^3
		let kappa = 24389.0 / 27.0; // 29^3/3^3
		let fx = if x > epsilon { x.cbrt() } else { (kappa * x + 16.0) / 116.0 };
		let fy = if y > epsilon { y.cbrt() } else { (kappa * y + 16.0) / 116.0 };
		let fz = if z > epsilon { z.cbrt() } else { (kappa * z + 16.0) / 116.0 };
		let lightness = 116.0 * fy - 16.0;
		let a = 500.0 * (fx - fy);
		let b = 200.0 * (fy - fz);
		Lab::new(lightness, a, b, alpha)
	}
}

impl From<Lab> for XyzD50 {
	fn from(value: Lab) -> Self {
		let Lab { lightness, a, b, alpha } = value;
		let epsilon = 216.0 / 24389.0; // 6^3/29^3
		let kappa = 24389.0 / 27.0; // 29^3/3^3
		let fy = (lightness + 16.0) / 116.0;
		let fx = a / 500.0 + fy;
		let fz = fy - b / 200.0;
		let x = if fx.powi(3) > epsilon { fx.powi(3) } else { (116.0 * fx - 16.0) / kappa };
		let y = if lightness > kappa * epsilon { ((lightness + 16.0) / 116.0).powi(3) } else { lightness / kappa };
		let z = if fz.powi(3) > epsilon { fz.powi(3) } else { (116.0 * fz - 16.0) / kappa };
		XyzD50::new(x * D50_WHITE[0], y * D50_WHITE[1], z * D50_WHITE[2], alpha)
	}
}
