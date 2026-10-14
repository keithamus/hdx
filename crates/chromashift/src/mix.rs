use crate::{
	A98Rgb, Channel, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, PolarLayout, ProphotoRgb, Rec2020, Srgb,
	XyzD50, XyzD65,
};

/// A direction to interopolate hue values between, when mixing colours.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HueInterpolation {
	#[default]
	Shorter,
	Longer,
	Increasing,
	Decreasing,
}

/// Trait for calculating mixing two colors together.
///
/// This trait provides a static method which will receive two colours, and can output a Self which should be the result
/// of both colours mixed by the given percentage (the percentage pertains to how much the second colour should apply to
/// the first).
///
/// Per CSS Color (4 12.3 & 5 3.5), interpolation uses premultiplied alpha:
/// 1. Premultiply each component by its alpha
/// 2. Linearly interpolate premultiplied values and alpha independently
/// 3. Un-premultiply by dividing by the interpolated alpha
pub trait ColorMix<T, U>: Sized
where
	T: Into<Self>,
	U: Into<Self>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self;
}

/// Trait for calculating mixing two colors together, with a hue direction for Polar colour spaces.
///
/// This trait provides a static method which will receive two colours, and can output a Self which should be the result
/// of both colours mixed by the given percentage (the percentage pertains to how much the second colour should apply to
/// the first). The Hue direction should be respected. If the colour space is not Polar then consider [ColorMix]
/// instead.
///
/// Per CSS Color 4 12.3, premultiplied alpha is used for non-hue components. The hue component
/// is NOT premultiplied - it is interpolated directly using the specified hue interpolation method.
pub trait ColorMixPolar<T, U>: Sized
where
	T: Into<Self>,
	U: Into<Self>,
{
	fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self;
}

/// Interpolate a single component using premultiplied alpha.
///
/// CSS Color 4 12.3:
///   premultiplied1 = component1 * alpha1
///   premultiplied2 = component2 * alpha2
///   result_premultiplied = premultiplied1 * (1 - t) + premultiplied2 * t
///   result = result_premultiplied / result_alpha
fn premultiply_lerp(c1: f64, a1: f64, c2: f64, a2: f64, t: f64, result_alpha: f64) -> f64 {
	if result_alpha == 0.0 {
		return c1 * (1.0 - t) + c2 * t;
	}
	let pm1 = c1 * a1;
	let pm2 = c2 * a2;
	(pm1 * (1.0 - t) + pm2 * t) / result_alpha
}

/// Given two hues (`h1`, `h2`), a percentage transform (`t`), and an interpolation direction, return a new Hue rotation
/// transformed by that amount.
pub fn interpolate_hue(h1: f64, h2: f64, t: f64, interpolation: HueInterpolation) -> f64 {
	let (h1, h2) = (h1.rem_euclid(360.0), h2.rem_euclid(360.0));

	let diff = match interpolation {
		HueInterpolation::Shorter => {
			let d = h2 - h1;
			if d.abs() <= 180.0 {
				d
			} else if d > 180.0 {
				d - 360.0
			} else {
				d + 360.0
			}
		}
		HueInterpolation::Longer => {
			let d = h2 - h1;
			if d.abs() >= 180.0 {
				d
			} else if d > 0.0 {
				d - 360.0
			} else {
				d + 360.0
			}
		}
		HueInterpolation::Increasing => {
			let mut d = h2 - h1;
			if d < 0.0 {
				d += 360.0;
			}
			d
		}
		HueInterpolation::Decreasing => {
			let mut d = h2 - h1;
			if d > 0.0 {
				d -= 360.0;
			}
			d
		}
	};

	(h1 + diff * t).rem_euclid(360.0)
}

/// Mixes two colours channel-by-channel, honouring `none` channels by adopting the analogous channel from the other
/// colour. When both are missing the result resolves to 0.
///
/// Non-hue colour components and alpha use premultiplied alpha interpolation. Polar colours use `hue_index` to select
/// which (if any) component is a hue: that channel uses hue interpolation with the given direction.
pub fn mix_channels(
	first: [Channel; 4],
	second: [Channel; 4],
	percentage: f64,
	hue_index: Option<usize>,
	hue_interpolation: HueInterpolation,
) -> [Channel; 4] {
	let t = percentage / 100.0;
	let mut out = [Channel::default(); 4];

	// Alpha: resolve none, then lerp. Store in 0..100 range.
	let (alpha_a, alpha_b) = match (first[3], second[3]) {
		(None, None) => (0.0, 0.0),
		(None, Some(b)) => (b, b),
		(Some(a), None) => (a, a),
		(Some(a), Some(b)) => (a, b),
	};
	let a1 = alpha_a / 100.0;
	let a2 = alpha_b / 100.0;
	let alpha = a1 * (1.0 - t) + a2 * t;
	out[3] = if first[3].or(second[3]).is_none() { None } else { Some(alpha * 100.0) };

	for i in 0..3 {
		let (l, r) = (first[i], second[i]);
		let (a, b) = match (l, r) {
			(None, None) => (None, None),
			(None, Some(b)) => (Some(b), Some(b)),
			(Some(a), None) => (Some(a), Some(a)),
			(Some(a), Some(b)) => (Some(a), Some(b)),
		};
		out[i] = match (a, b) {
			(None, _) | (_, None) => None,
			(Some(av), Some(bv)) => Some(if hue_index == Some(i) {
				interpolate_hue(av, bv, t, hue_interpolation)
			} else {
				premultiply_lerp(av, a1, bv, a2, t, alpha)
			}),
		};
	}

	out
}

/// Mixes two values of the same colour type via [`mix_channels`]. Devolves types to [Channel; 4] before mixing.
fn mix_in<C, T, U>(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> C
where
	C: From<T> + From<U> + From<[Channel; 4]> + Into<[Channel; 4]> + PolarLayout,
{
	let first: [Channel; 4] = C::from(first).into();
	let second: [Channel; 4] = C::from(second).into();
	let out = mix_channels(first, second, percentage, C::HUE_INDEX, hue_interpolation);
	C::from(out)
}

mod sealed {
	pub trait PolarColor {}
}

impl sealed::PolarColor for Hsl {}
impl sealed::PolarColor for Hwb {}
impl sealed::PolarColor for Lch {}
impl sealed::PolarColor for Oklch {}

impl<T, U, V> ColorMix<T, U> for V
where
	V: ColorMixPolar<T, U> + sealed::PolarColor + Sized,
	T: Into<V>,
	U: Into<V>,
{
	fn mix(first: T, second: U, percentage: f64) -> V {
		ColorMixPolar::mix_polar(first, second, percentage, HueInterpolation::Shorter)
	}
}

/// Implements [`ColorMix`] for a rectangular space by delegating to [`mix_in`].
macro_rules! impl_color_mix {
	($ty:ty) => {
		impl<T, U> ColorMix<T, U> for $ty
		where
			Self: From<T> + From<U>,
		{
			fn mix(first: T, second: U, percentage: f64) -> Self {
				mix_in::<Self, T, U>(first, second, percentage, HueInterpolation::Shorter)
			}
		}
	};
}

/// Implements [`ColorMixPolar`] for a polar space by delegating to [`mix_in`].
macro_rules! impl_color_mix_polar {
	($ty:ty) => {
		impl<T, U> ColorMixPolar<T, U> for $ty
		where
			Self: From<T> + From<U>,
		{
			fn mix_polar(first: T, second: U, percentage: f64, hue_interpolation: HueInterpolation) -> Self {
				mix_in::<Self, T, U>(first, second, percentage, hue_interpolation)
			}
		}
	};
}

impl_color_mix!(Srgb);
impl_color_mix!(LinearRgb);
impl_color_mix!(A98Rgb);
impl_color_mix!(DisplayP3);
impl_color_mix!(ProphotoRgb);
impl_color_mix!(Rec2020);
impl_color_mix!(Lab);
impl_color_mix!(Oklab);
impl_color_mix!(XyzD50);
impl_color_mix!(XyzD65);
impl_color_mix_polar!(Hsl);
impl_color_mix_polar!(Hwb);
impl_color_mix_polar!(Lch);
impl_color_mix_polar!(Oklch);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::*;

	macro_rules! assert_close_to {
		($a: expr, $b: expr) => {
			assert!($a.close_to($b, COLOR_EPSILON), "Expected {:?} to be (closely) equal to {:?}", $a, $b);
		};
	}

	#[test]
	fn test_basic_mix() {
		let red = Srgb::from_u8(255, 0, 0, 100.0);
		let blue = Srgb::from_u8(0, 0, 255, 100.0);
		assert_close_to!(Srgb::mix(red, blue, 50.0), Srgb::new(0.5, 0.0, 0.5, 100.0));
	}

	#[test]
	fn longer_hue_keeps_a_half_turn_in_its_direction() {
		assert_eq!(interpolate_hue(0.0, 180.0, 0.5, HueInterpolation::Longer), 90.0);
		assert_eq!(interpolate_hue(180.0, 0.0, 0.5, HueInterpolation::Longer), 90.0);
		assert_eq!(interpolate_hue(0.0, 90.0, 0.5, HueInterpolation::Longer), 225.0);
		assert_eq!(interpolate_hue(90.0, 0.0, 0.5, HueInterpolation::Longer), 225.0);
	}

	#[test]
	fn test_mix_named_in_oklab() {
		assert_close_to!(
			Oklch::mix(Named::Rebeccapurple, Named::Hotpink, 50.0),
			Oklch::new(0.5842845967725198, 0.17868573405015944, 327.6838446374328, 100.0)
		);
	}

	#[test]
	fn test_mix_named_in_hsl_polar() {
		assert_close_to!(Hsl::mix(Named::Rebeccapurple, Named::Hotpink, 50.0), Hsl::new(300.0, 75.0, 55.294117, 100.0));
		assert_close_to!(
			Hsl::mix_polar(Named::Rebeccapurple, Named::Hotpink, 50.0, HueInterpolation::Longer),
			Hsl::new(120.0, 75.0, 55.294117, 100.0)
		);
		assert_close_to!(
			Hsl::mix_polar(Named::Rebeccapurple, Named::Hotpink, 50.0, HueInterpolation::Decreasing),
			Hsl::new(120.0, 75.0, 55.294117, 100.0)
		);
		assert_close_to!(
			Hsl::mix_polar(Named::Rebeccapurple, Named::Hotpink, 50.0, HueInterpolation::Increasing),
			Hsl::new(300.0, 75.0, 55.294117, 100.0)
		);
	}

	#[test]
	fn test_alpha_mixing() {
		let color1 = Srgb::from_u8(255, 0, 0, 80.0);
		let color2 = Srgb::from_u8(0, 0, 255, 40.0);

		let mixed = Srgb::mix(color1, color2, 50.0);
		assert_eq!(mixed.to_u8(), [170, 0, 85]);
		assert_eq!(mixed.alpha, 60.0);
	}

	#[test]
	fn test_hwb_mix() {
		// Hwb is polar - default mix uses Shorter hue interpolation
		// 0° to 240°: diff=240 > 180, so shorter wraps via 360°, midpoint is 300°
		let red = Hwb::new(0.0, 0.0, 0.0, 100.0);
		let blue = Hwb::new(240.0, 0.0, 0.0, 100.0);
		let mixed = Hwb::mix(red, blue, 50.0);
		assert_close_to!(mixed, Hwb::new(300.0, 0.0, 0.0, 100.0));
	}

	#[test]
	fn test_hwb_mix_polar() {
		// 0° to 240°: longer arc goes through 120°
		let red = Hwb::new(0.0, 0.0, 0.0, 100.0);
		let blue = Hwb::new(240.0, 0.0, 0.0, 100.0);
		assert_close_to!(Hwb::mix_polar(red, blue, 50.0, HueInterpolation::Longer), Hwb::new(120.0, 0.0, 0.0, 100.0));
	}

	#[test]
	fn test_a98_rgb_mix() {
		let c1 = A98Rgb::new(1.0, 0.0, 0.0, 100.0);
		let c2 = A98Rgb::new(0.0, 0.0, 1.0, 100.0);
		let mixed = A98Rgb::mix(c1, c2, 50.0);
		assert_close_to!(mixed, A98Rgb::new(0.5, 0.0, 0.5, 100.0));
	}

	/// WPT: color-mix(in lab, lab(10 20 30 / .4), lab(50 60 70 / .8)) → lab(36.666664 46.666664 56.666664 / 0.6)
	#[test]
	fn test_premultiplied_alpha_lab() {
		let c1 = Lab::new(10.0, 20.0, 30.0, 40.0);
		let c2 = Lab::new(50.0, 60.0, 70.0, 80.0);
		let mixed = Lab::mix(c1, c2, 50.0);
		assert_close_to!(mixed, Lab::new(36.666664, 46.666664, 56.666664, 60.0));
	}

	/// WPT: color-mix(in lab, lab(10 20 30 / .4) 25%, lab(50 60 70 / .8)) → lab(44.285713 54.285717 64.28571 / 0.7)
	#[test]
	fn test_premultiplied_alpha_lab_25_75() {
		let c1 = Lab::new(10.0, 20.0, 30.0, 40.0);
		let c2 = Lab::new(50.0, 60.0, 70.0, 80.0);
		// 25% first, 75% second: mix_percentage = 75
		let mixed = Lab::mix(c1, c2, 75.0);
		assert_close_to!(mixed, Lab::new(44.285713, 54.285717, 64.28571, 70.0));
	}

	/// WPT: color-mix(in oklch, oklch(0.1 0.2 30deg / .4), oklch(0.5 0.6 70deg / .8)) → oklch(0.36666664 0.46666664 50 / 0.6)
	#[test]
	fn test_premultiplied_alpha_oklch() {
		let c1 = Oklch::new(0.1, 0.2, 30.0, 40.0);
		let c2 = Oklch::new(0.5, 0.6, 70.0, 80.0);
		let mixed = Oklch::mix(c1, c2, 50.0);
		assert_close_to!(mixed, Oklch::new(0.36666664, 0.46666664, 50.0, 60.0));
	}

	/// When both alphas are 100%, premultiplied interpolation == simple interpolation.
	#[test]
	fn test_premultiplied_alpha_opaque_same_as_simple() {
		let c1 = Lab::new(10.0, 20.0, 30.0, 100.0);
		let c2 = Lab::new(50.0, 60.0, 70.0, 100.0);
		let mixed = Lab::mix(c1, c2, 50.0);
		assert_close_to!(mixed, Lab::new(30.0, 40.0, 50.0, 100.0));
	}

	/// `None` channel in one input adopts the analogous channel from the other.
	#[test]
	fn test_none_channel_substitution() {
		let first = [None, Some(0.0), Some(0.0), Some(100.0)];
		let second: [Channel; 4] = Srgb::from_u8(200, 0, 0, 100.0).into();
		let out = mix_channels(first, second, 50.0, Srgb::HUE_INDEX, HueInterpolation::Shorter);
		let mixed: Srgb = out.into();
		assert_eq!(mixed, Srgb::from_u8(200, 0, 0, 100.0));
	}
}
