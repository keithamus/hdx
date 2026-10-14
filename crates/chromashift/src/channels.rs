/// Trait for extracting the alpha channel of a color.
pub trait ToAlpha: Sized {
	/// Returns a number between 0.0 (fully transparent) to 100.0 (fully opaque).
	fn to_alpha(&self) -> f32;

	/// Returns true if the alpha of this colour is 100.0
	fn fully_opaque(&self) -> bool {
		self.to_alpha() == 100.0
	}

	/// Returns true if the alpha of this colour is 0.0
	fn fully_transparent(&self) -> bool {
		self.to_alpha() == 0.0
	}
}

/// A generic single color channel value, which may be None
///
/// `none` channels in `color-mix` adopt the analogous channel's value from the other color in the interpolation space
/// (see [`crate::InterpolationSpace::mix`]).
pub type Channel = Option<f64>;

use crate::{
	A98Rgb, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb, Rec2020, Srgb, XyzD50, XyzD65,
};

macro_rules! impl_channels {
	(@cast $v:expr, u8) => { $v.round() as u8 };
	(@cast $v:expr, f64) => { $v };
	(@cast $v:expr, f32) => { $v as f32 };
	($ty:ident { $f1:ident: $t1:tt, $f2:ident: $t2:tt, $f3:ident: $t3:tt $(,)? }) => {
		impl From<$ty> for [Channel; 4] {
			fn from(c: $ty) -> Self {
				[
					Some(c.$f1 as f64),
					Some(c.$f2 as f64),
					Some(c.$f3 as f64),
					Some(c.alpha as f64),
				]
			}
		}

		impl From<[Channel; 4]> for $ty {
			fn from(c: [Channel; 4]) -> Self {
				$ty::new(
					impl_channels!(@cast c[0].unwrap_or(0.0), $t1),
					impl_channels!(@cast c[1].unwrap_or(0.0), $t2),
					impl_channels!(@cast c[2].unwrap_or(0.0), $t3),
					c[3].unwrap_or(0.0) as f32,
				)
			}
		}
	};
}

impl_channels!(Srgb { red: f64, green: f64, blue: f64 });
impl_channels!(LinearRgb { red: f64, green: f64, blue: f64 });
impl_channels!(A98Rgb { red: f64, green: f64, blue: f64 });
impl_channels!(DisplayP3 { red: f64, green: f64, blue: f64 });
impl_channels!(ProphotoRgb { red: f64, green: f64, blue: f64 });
impl_channels!(Rec2020 { red: f64, green: f64, blue: f64 });
impl_channels!(Lab { lightness: f64, a: f64, b: f64 });
impl_channels!(Oklab { lightness: f64, a: f64, b: f64 });
impl_channels!(XyzD50 { x: f64, y: f64, z: f64 });
impl_channels!(XyzD65 { x: f64, y: f64, z: f64 });
impl_channels!(Hsl { hue: f64, saturation: f64, lightness: f64 });
impl_channels!(Hwb { hue: f64, whiteness: f64, blackness: f64 });
impl_channels!(Lch { lightness: f64, chroma: f64, hue: f64 });
impl_channels!(Oklch { lightness: f64, chroma: f64, hue: f64 });
