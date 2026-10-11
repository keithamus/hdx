use crate::{Alpha, Matrix3, Number, RgbSpace, ToAlpha, XyzD50, XyzD65};
use core::fmt;

macro_rules! rgb_color {
	($(#[$meta:meta])* $ty:ident, $css:literal, $space:ident, $xyz:ident) => {
		rgb_color!($(#[$meta])* $ty, $space, $xyz);

		impl fmt::Display for $ty {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				let Self { red, green, blue, alpha } = *self;
				write!(f, concat!("color(", $css, " {} {} {}{})"), Number(red), Number(green), Number(blue), Alpha(alpha))
			}
		}
	};
	($(#[$meta:meta])* $ty:ident, $space:ident, $xyz:ident) => {
		$(#[$meta])*
		/// The components are:
		/// - Red - a number between 0.0 and 1.0
		/// - Green - a number between 0.0 and 1.0
		/// - Blue - a number between 0.0 and 1.0
		/// - Alpha - a number between 0.0 and 100.0
		///
		/// Channels outside 0.0 to 1.0 are preserved, as CSS Color 4 requires; see
		/// [`Gamut`](crate::Gamut) to detect and resolve them.
		#[derive(Debug, Clone, Copy, PartialEq)]
		pub struct $ty {
			pub red: f64,
			pub green: f64,
			pub blue: f64,
			pub alpha: f32,
		}

		impl $ty {
			/// The colour space these channels are encoded in.
			pub const SPACE: RgbSpace = RgbSpace::$space;

			pub fn new(red: f64, green: f64, blue: f64, alpha: f32) -> Self {
				Self { red, green, blue, alpha: alpha.clamp(0.0, 100.0) }
			}
		}

		impl ToAlpha for $ty {
			fn to_alpha(&self) -> f32 {
				self.alpha
			}
		}

		impl From<$xyz> for $ty {
			fn from(value: $xyz) -> Self {
				let $xyz { x, y, z, alpha } = value;
				let [red, green, blue] = Self::SPACE.encoded_from_xyz([x / 100.0, y / 100.0, z / 100.0]);
				Self::new(red, green, blue, alpha)
			}
		}

		impl From<$ty> for $xyz {
			fn from(value: $ty) -> Self {
				let $ty { red, green, blue, alpha } = value;
				let [x, y, z] = $ty::SPACE.xyz_from_encoded([red, green, blue]);
				$xyz::new(x * 100.0, y * 100.0, z * 100.0, alpha)
			}
		}
	};
}

rgb_color!(
	/// A colour in the sRGB colour space.
	Srgb,
	SRGB,
	XyzD65
);

rgb_color!(
	/// A device independent expression of RGB: the sRGB primaries with no transfer function.
	LinearRgb,
	"srgb-linear",
	SRGB_LINEAR,
	XyzD65
);

rgb_color!(
	/// A colour in the Adobe RGB (1998) colour space.
	A98Rgb,
	"a98-rgb",
	A98_RGB,
	XyzD65
);

rgb_color!(
	/// A colour in the Display P3 colour space.
	DisplayP3,
	"display-p3",
	DISPLAY_P3,
	XyzD65
);

rgb_color!(
	/// A colour in the ProPhoto RGB colour space (ROMM RGB).
	ProphotoRgb,
	"prophoto-rgb",
	PROPHOTO_RGB,
	XyzD50
);

rgb_color!(
	/// A colour in the ITU-R BT.2020 colour space.
	Rec2020,
	"rec2020",
	REC2020,
	XyzD65
);

const P3_TO_SRGB: Matrix3 = Matrix3([
	[3685649.0 / 3008840.0, -676809.0 / 3008840.0, 0.0],
	[-5617931.0 / 133579120.0, 139197051.0 / 133579120.0, 0.0],
	[-1323971.0 / 67420360.0, -1514763.0 / 19262960.0, 148092003.0 / 134840720.0],
]);
const SRGB_TO_P3: Matrix3 = Matrix3([
	[2442703.0 / 2969989.0, 527286.0 / 2969989.0, 0.0],
	[621563.0 / 18725049.0, 18103486.0 / 18725049.0, 0.0],
	[281089.0 / 16454667.0, 10721482.0 / 148092003.0, 134840720.0 / 148092003.0],
]);

impl From<DisplayP3> for LinearRgb {
	fn from(value: DisplayP3) -> Self {
		let DisplayP3 { red, green, blue, alpha } = value;
		let [red, green, blue] = P3_TO_SRGB.transform(DisplayP3::SPACE.transfer.decode_all([red, green, blue]));
		LinearRgb::new(red, green, blue, alpha)
	}
}

impl From<LinearRgb> for DisplayP3 {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		let [red, green, blue] = DisplayP3::SPACE.transfer.encode_all(SRGB_TO_P3.transform([red, green, blue]));
		DisplayP3::new(red, green, blue, alpha)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn p3_srgb_matrices_are_products_of_the_css_color_4_matrices() {
		for (matrix, product) in [
			(P3_TO_SRGB, DisplayP3::SPACE.to_xyz.then(&LinearRgb::SPACE.from_xyz)),
			(SRGB_TO_P3, LinearRgb::SPACE.to_xyz.then(&DisplayP3::SPACE.from_xyz)),
		] {
			for (got, want) in matrix.0.iter().flatten().zip(product.0.iter().flatten()) {
				assert!((got - want).abs() < 1e-12, "{got} != {want}");
			}
		}
	}
}
