use crate::{Alpha, Number, ToAlpha, XyzD65};
use core::fmt;

/// A more adequate expression of LAB, in the CIE colour space.
/// The components are:
/// - L - a number between 0.0 and 1.0
/// - A - a number between -0.4 and +0.4
/// - B - a number between -0.4 and +0.4
/// - Alpha - a number between 0.0 and 100.0
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Oklab {
	pub lightness: f64,
	pub a: f64,
	pub b: f64,
	pub alpha: f32,
}

impl Oklab {
	pub fn new(lightness: f64, a: f64, b: f64, alpha: f32) -> Self {
		Self { lightness, a, b, alpha: alpha.clamp(0.0, 100.0) }
	}

	/// Converts a CSS `<percentage>` lightness to a lightness, where `100%` is `1.0`.
	///
	/// <https://drafts.csswg.org/css-color-4/#specifying-oklab-oklch>
	pub fn lightness_from_percent(percent: f64) -> f64 {
		percent / 100.0
	}

	/// Converts a CSS `<percentage>` `a` or `b` axis to an axis value, where `100%` is `0.4` and `-100%` is `-0.4`.
	///
	/// <https://drafts.csswg.org/css-color-4/#specifying-oklab-oklch>
	pub fn ab_from_percent(percent: f64) -> f64 {
		percent / 100.0 * 0.4
	}
}

impl ToAlpha for Oklab {
	fn to_alpha(&self) -> f32 {
		self.alpha
	}
}

impl fmt::Display for Oklab {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let Self { lightness, a, b, alpha } = *self;
		write!(f, "oklab({} {} {}{})", Number(lightness), Number(a), Number(b), Alpha(alpha))
	}
}

impl From<XyzD65> for Oklab {
	fn from(value: XyzD65) -> Self {
		let XyzD65 { x, y, z, alpha } = value;
		let x = x / 100.0;
		let y = y / 100.0;
		let z = z / 100.0;
		let l = x * 0.819_022_437_996_703 + y * 0.3619062600528904 + z * (-0.1288737815209879);
		let m = x * 0.0329836539323885 + y * 0.9292868615863434 + z * 0.0361446663506424;
		let s = x * 0.0481771893596242 + y * 0.2642395317527308 + z * 0.6335478284694309;
		let l = l.cbrt();
		let m = m.cbrt();
		let s = s.cbrt();
		let lightness = l * 0.210_454_268_309_314 + m * 0.7936177747023054 + s * (-0.0040720430116193);
		let a = l * 1.9779985324311684 + m * (-2.428_592_242_048_58) + s * 0.450_593_709_617_411;
		let b = l * 0.0259040424655478 + m * 0.7827717124575296 + s * (-0.8086757549230774);
		Oklab::new(lightness, a, b, alpha)
	}
}

impl From<Oklab> for XyzD65 {
	fn from(value: Oklab) -> Self {
		let Oklab { lightness, a, b, alpha } = value;
		let l = lightness * 1.0000000000000000 + a * 0.3963377773761749 + b * 0.2158037573099136;
		let m = lightness * 1.0000000000000000 + a * (-0.1055613458156586) + b * (-0.0638541728258133);
		let l = l.powi(3);
		let s = lightness * 1.0000000000000000 + a * (-0.0894841775298119) + b * (-1.2914855480194092);
		let m = m.powi(3);
		let s = s.powi(3);
		let x = l * 1.2268798758459243 + m * (-0.5578149944602171) + s * 0.2813910456659647;
		let y = l * (-0.0405757452148008) + m * 1.112_286_803_280_317 + s * (-0.0717110580655164);
		let z = l * (-0.0763729366746601) + m * (-0.4214933324022432) + s * 1.5869240198367816;
		XyzD65::new(x * 100.0, y * 100.0, z * 100.0, alpha)
	}
}
