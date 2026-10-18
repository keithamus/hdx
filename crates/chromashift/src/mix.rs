use crate::{
	A98Rgb, DisplayP3, Hsl, HueInterpolation, Hwb, InterpolationSpace, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb,
	Rec2020, Srgb, XyzD50, XyzD65,
};

/// Trait for calculating mixing two colors together.
///
/// This trait provides a static method which will receive two colours, and can output a Self which should be the result
/// of both colours mixed by the given percentage (the percentage pertains to how much the second colour should apply to
/// the first). Colours mix in the [`InterpolationSpace`] of `Self` as [`InterpolationSpace::mix`] mixes them, with hues
/// in polar spaces taking the shorter way round.
pub trait ColorMix<T, U>: Sized
where
	T: Into<Self>,
	U: Into<Self>,
{
	fn mix(first: T, second: U, percentage: f64) -> Self;
}

macro_rules! impl_color_mix {
	($($ty:ident => $space:expr),+ $(,)?) => {
		$(impl<T, U> ColorMix<T, U> for $ty
		where
			Self: From<T> + From<U>,
		{
			fn mix(first: T, second: U, percentage: f64) -> Self {
				let space = $space;
				let channels = |color: Self| space.channels(color.into());
				space.color(space.mix(channels(first.into()), channels(second.into()), percentage)).into()
			}
		})+
	};
}

impl_color_mix! {
	Srgb => InterpolationSpace::Srgb,
	LinearRgb => InterpolationSpace::SrgbLinear,
	A98Rgb => InterpolationSpace::A98Rgb,
	DisplayP3 => InterpolationSpace::DisplayP3,
	ProphotoRgb => InterpolationSpace::ProphotoRgb,
	Rec2020 => InterpolationSpace::Rec2020,
	Lab => InterpolationSpace::Lab,
	Oklab => InterpolationSpace::Oklab,
	XyzD50 => InterpolationSpace::XyzD50,
	XyzD65 => InterpolationSpace::XyzD65,
	Hsl => InterpolationSpace::Hsl(HueInterpolation::Shorter),
	Hwb => InterpolationSpace::Hwb(HueInterpolation::Shorter),
	Lch => InterpolationSpace::Lch(HueInterpolation::Shorter),
	Oklch => InterpolationSpace::Oklch(HueInterpolation::Shorter),
}

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
	fn test_mix_named_in_oklab() {
		assert_close_to!(
			Oklch::mix(Named::Rebeccapurple, Named::Hotpink, 50.0),
			Oklch::new(0.5842845967725198, 0.17868573405015944, 327.6838446374328, 100.0)
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
}
