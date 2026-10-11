use crate::*;

/// Rounds `f` to `d` decimal places.
pub fn round_dp(f: f64, d: u32) -> f64 {
	let factor = 10u32.pow(d) as f64;
	(f * factor).round() / factor
}

pub(crate) struct Number<T>(pub T);

impl fmt::Display for Number<f64> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", round_css(self.0))
	}
}

impl fmt::Display for Number<f32> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", round_css(self.0 as f64) as f32)
	}
}

pub(crate) struct Alpha(pub f32);

impl fmt::Display for Alpha {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0 < 100.0 { write!(f, " / {}", Number(self.0 as f64 / 100.0)) } else { Ok(()) }
	}
}

fn round_css(value: f64) -> f64 {
	let scaled = value * 1e6;
	let floor = scaled.floor();
	let rounded = if scaled - floor < 0.5 { floor } else { floor + 1.0 };
	rounded / 1e6 + 0.0
}

/// Rounds a colour's channels to perceptually safe precision.
///
/// The number of decimal places per channel is determined by the channel's value range, so that each rounding step
/// represents roughly the same fraction of the perceptual range. Small-range channels (e.g. OKLab 0–1) keep more
/// decimal places, while large-range channels (e.g. hue 0–360, Lab L 0–100) need fewer.
///
/// The precisions used are chosen to be well below the Just Noticeable Difference (JND) while still surviving chained
/// colour operations like `color-mix()` and relative colour syntax without accumulating visible error.
///
/// See: <https://keithcirkel.co.uk/too-much-color/>
pub trait PerceptualRound: Sized {
	fn round(self) -> Self;
}

macro_rules! impl_perceptual_round {
	($ty:ident, $c1:ident: $dp1:expr, $c2:ident: $dp2:expr, $c3:ident: $dp3:expr) => {
		impl PerceptualRound for $ty {
			fn round(self) -> Self {
				$ty::new(
					round_dp(self.$c1 as f64, $dp1) as _,
					round_dp(self.$c2 as f64, $dp2) as _,
					round_dp(self.$c3 as f64, $dp3) as _,
					round_dp(self.alpha as f64, 2) as f32,
				)
			}
		}
	};
}

// oklch/oklab: 3dp for 0–1 range channels, 1dp for hue (0–360 range).
impl_perceptual_round!(Oklch, lightness: 3, chroma: 3, hue: 1);
impl_perceptual_round!(Oklab, lightness: 3, a: 3, b: 3);

// lab/lch: 1dp for all channels (0–100/±128/0–150/0–360 ranges).
impl_perceptual_round!(Lab, lightness: 1, a: 1, b: 1);
impl_perceptual_round!(Lch, lightness: 1, chroma: 1, hue: 1);

// RGB 0–1 types: 3dp (4dp for srgb-linear due to near-black divergence at 3dp).
impl_perceptual_round!(Srgb, red: 3, green: 3, blue: 3);
impl_perceptual_round!(LinearRgb, red: 4, green: 4, blue: 4);
impl_perceptual_round!(DisplayP3, red: 3, green: 3, blue: 3);
impl_perceptual_round!(A98Rgb, red: 3, green: 3, blue: 3);
impl_perceptual_round!(ProphotoRgb, red: 3, green: 3, blue: 3);
impl_perceptual_round!(Rec2020, red: 3, green: 3, blue: 3);

// XYZ 0–100 types: 2dp (4dp in CSS 0–1 scale, matching srgb-linear).
impl_perceptual_round!(XyzD50, x: 2, y: 2, z: 2);
impl_perceptual_round!(XyzD65, x: 2, y: 2, z: 2);

// HSL/HWB: 1dp for hue (0–360), 1dp for percentage channels (0–100).
impl_perceptual_round!(Hsl, hue: 1, saturation: 1, lightness: 1);
impl_perceptual_round!(Hwb, hue: 1, whiteness: 1, blackness: 1);

macro_rules! impl_perceptual_round_noop {
	($($ty:ident),+) => {
		$(impl PerceptualRound for $ty {
			fn round(self) -> Self { self }
		})+
	};
}

impl_perceptual_round_noop!(Hex, Named, Hsv);

impl PerceptualRound for Color {
	fn round(self) -> Self {
		match self {
			Color::A98Rgb(c) => Color::A98Rgb(c.round()),
			Color::DisplayP3(c) => Color::DisplayP3(c.round()),
			Color::Hex(c) => Color::Hex(c.round()),
			Color::Hsv(c) => Color::Hsv(c.round()),
			Color::Hsl(c) => Color::Hsl(c.round()),
			Color::Hwb(c) => Color::Hwb(c.round()),
			Color::Lab(c) => Color::Lab(c.round()),
			Color::Lch(c) => Color::Lch(c.round()),
			Color::LinearRgb(c) => Color::LinearRgb(c.round()),
			Color::Named(n) => Color::Named(n.round()),
			Color::Oklab(c) => Color::Oklab(c.round()),
			Color::Oklch(c) => Color::Oklch(c.round()),
			Color::ProphotoRgb(c) => Color::ProphotoRgb(c.round()),
			Color::Rec2020(c) => Color::Rec2020(c.round()),
			Color::Srgb(c) => Color::Srgb(c.round()),
			Color::XyzD50(c) => Color::XyzD50(c.round()),
			Color::XyzD65(c) => Color::XyzD65(c.round()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn oklch_round() {
		// 3dp L/C, 1dp hue
		let rounded = Oklch::new(0.6593827, 0.30412345, 203.27412, 100.0).round();
		assert_eq!(rounded, Oklch::new(0.659, 0.304, 203.3, 100.0));
	}

	#[test]
	fn oklab_round() {
		let rounded = Oklab::new(0.44027179, 0.08817676, -0.13386435, 100.0).round();
		assert_eq!(rounded, Oklab::new(0.44, 0.088, -0.134, 100.0));
	}

	#[test]
	fn lab_round() {
		// 1dp for all channels
		let rounded = Lab::new(32.39271642, 38.42945581, -47.68554267, 100.0).round();
		assert_eq!(rounded, Lab::new(32.4, 38.4, -47.7, 100.0));
	}

	#[test]
	fn lch_round() {
		let rounded = Lch::new(61.23323694, 50.27335275, 273.48455139, 100.0).round();
		assert_eq!(rounded, Lch::new(61.2, 50.3, 273.5, 100.0));
	}

	#[test]
	fn hsl_round() {
		let rounded = Hsl::new(218.54015, 79.19075, 66.07843, 100.0).round();
		assert_eq!(rounded, Hsl::new(218.5, 79.2, 66.1, 100.0));
	}

	#[test]
	fn display_p3_round() {
		let rounded = DisplayP3::new(0.39189772, 0.57889666, 0.92721090, 100.0).round();
		assert_eq!(rounded, DisplayP3::new(0.392, 0.579, 0.927, 100.0));
	}

	#[test]
	fn already_clean_values_unchanged() {
		let clean = Oklch::new(0.5, 0.2, 180.0, 100.0);
		assert_eq!(clean.round(), clean);
	}

	#[test]
	fn noop_types() {
		assert_eq!(Srgb::from_u8(102, 51, 153, 100.0).round(), Srgb::from_u8(102, 51, 153, 100.0));
		assert_eq!(Hex::new(0x663399FF).round(), Hex::new(0x663399FF));
		assert_eq!(Named::Rebeccapurple.round(), Named::Rebeccapurple);
	}

	#[test]
	fn alpha_is_rounded() {
		assert_eq!(Oklch::new(0.5, 0.2, 180.0, 75.555).round().alpha, 75.56);
	}

	#[test]
	fn color_enum_round() {
		let rounded = Color::Oklch(Oklch::new(0.6593827, 0.30412345, 203.27412, 100.0)).round();
		assert_eq!(rounded, Color::Oklch(Oklch::new(0.659, 0.304, 203.3, 100.0)));
	}

	#[test]
	fn round_stays_perceptually_close() {
		let colors = [
			Color::Oklch(Oklch::new(0.659, 0.304, 203.274, 100.0)),
			Color::Lab(Lab::new(50.0, 30.5, -20.123, 80.0)),
			Color::Hsl(Hsl::new(218.54015, 79.19075, 66.07843, 100.0)),
			Color::DisplayP3(DisplayP3::new(0.39189772, 0.57889666, 0.92721090, 100.0)),
			Color::Oklab(Oklab::new(0.44027179, 0.08817676, -0.13386435, 100.0)),
			Color::Lch(Lch::new(61.23323694, 50.27335275, 273.48455139, 100.0)),
		];
		for color in &colors {
			let rounded = color.round();
			assert!(color.close_to(rounded, 1.0), "ΔE = {} for {color:?}", color.delta_e(rounded));
		}
	}
}
