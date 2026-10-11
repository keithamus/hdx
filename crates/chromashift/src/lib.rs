use core::fmt;
mod channels;
mod chromaticity;
mod color_space;
mod conversion;
mod distance;
mod gamut;
mod hex;
mod hsb;
mod hsl;
mod hwb;
mod lab;
mod lch;
mod matrix;
mod mix;
mod named;
mod oklab;
mod oklch;
mod rgb;
mod rgb_space;
mod round;
mod srgb;
#[cfg(test)]
mod tests;
mod transfer;
mod wcag;
mod xyz;

pub use channels::{Channel, PolarLayout, ToAlpha};
pub use chromaticity::Chromaticity;
pub use color_space::ColorSpace;
pub use distance::ColorDistance;
pub use gamut::Gamut;
pub use hex::Hex;
pub use hsb::Hsv;
pub use hsl::Hsl;
pub use hwb::Hwb;
pub use lab::Lab;
pub use lch::Lch;
pub use matrix::Matrix3;
pub use mix::{ColorMix, ColorMixPolar, HueInterpolation, mix_channels};
pub use named::{Named, ToNamedError};
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use rgb::{A98Rgb, DisplayP3, LinearRgb, ProphotoRgb, Rec2020, Srgb};
pub use rgb_space::RgbSpace;
pub(crate) use round::{Alpha, Number};
pub use round::{PerceptualRound, round_dp};
pub use transfer::Transfer;
pub use wcag::{WcagColorContrast, WcagLevel};
pub use xyz::{XyzD50, XyzD65};

macro_rules! color {
	(settable: $($settable:ident),+ $(,)?; via_srgb: $($via_srgb:ident),+ $(,)?;) => {
		/// Any colour this crate can represent, in the colour space it was authored in.
		#[derive(Debug, Clone, Copy, PartialEq)]
		pub enum Color {
			$($settable($settable),)+
			$($via_srgb($via_srgb),)+
		}

		impl Color {
			/// Returns a copy of this colour with `alpha`, clamped to 0.0-100.0.
			///
			/// `Hex` and `Named` colours are returned as `Srgb`.
			pub fn with_alpha(self, alpha: f32) -> Self {
				let alpha = alpha.clamp(0.0, 100.0);
				match self {
					$(Self::$settable(mut c) => {
						c.alpha = alpha;
						Self::$settable(c)
					})+
					$(Self::$via_srgb(c) => {
						let mut srgb = Srgb::from(c);
						srgb.alpha = alpha;
						Self::Srgb(srgb)
					})+
				}
			}
		}

		impl fmt::Display for Color {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				match self {
					$(Self::$settable(c) => fmt::Display::fmt(c, f),)+
					$(Self::$via_srgb(c) => fmt::Display::fmt(c, f),)+
				}
			}
		}

		impl ToAlpha for Color {
			fn to_alpha(&self) -> f32 {
				match self {
					$(Self::$settable(c) => c.to_alpha(),)+
					$(Self::$via_srgb(c) => c.to_alpha(),)+
				}
			}
		}

		impl Color {
			fn convert<T>(self) -> T
			where
				$(T: From<$settable>,)+
				$(T: From<$via_srgb>,)+
			{
				match self {
					$(Color::$settable(c) => c.into(),)+
					$(Color::$via_srgb(c) => c.into(),)+
				}
			}
		}

		$(impl From<Color> for $settable {
			fn from(value: Color) -> Self {
				value.convert()
			}
		})+

		impl From<Color> for Hex {
			fn from(value: Color) -> Self {
				value.convert()
			}
		}

		$(impl From<$settable> for Color {
			fn from(c: $settable) -> Self {
				Color::$settable(c)
			}
		})+

		$(impl From<$via_srgb> for Color {
			fn from(c: $via_srgb) -> Self {
				Color::$via_srgb(c)
			}
		})+
	};
}

color! {
	settable: A98Rgb, DisplayP3, Hsv, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb, Rec2020, Srgb, XyzD50, XyzD65;
	via_srgb: Hex, Named;
}

/// The CIE Delta E distance below which two colours are considered perceptually identical.
pub const COLOR_EPSILON: f64 = 0.0072;
