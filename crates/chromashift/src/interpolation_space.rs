use crate::{
	A98Rgb, Channel, Color, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb, Rec2020, Srgb, XyzD50,
	XyzD65,
};

/// A direction hues travel in between two colours, when interpolating in a polar space.
///
/// <https://drafts.csswg.org/css-color-4/#hue-interpolation>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HueInterpolation {
	#[default]
	Shorter,
	Longer,
	Increasing,
	Decreasing,
}

impl HueInterpolation {
	/// The signed number of degrees from hue `from` to hue `to` when interpolating in this direction.
	pub fn delta(self, from: f64, to: f64) -> f64 {
		let d = to.rem_euclid(360.0) - from.rem_euclid(360.0);
		match self {
			Self::Shorter if d > 180.0 => d - 360.0,
			Self::Shorter if d < -180.0 => d + 360.0,
			Self::Longer if d > 0.0 && d < 180.0 => d - 360.0,
			Self::Longer if d > -180.0 && d <= 0.0 => d + 360.0,
			Self::Increasing if d < 0.0 => d + 360.0,
			Self::Decreasing if d > 0.0 => d - 360.0,
			_ => d,
		}
	}
}

/// A colour space colours can be interpolated in, with the direction hues travel in for polar spaces, as named by CSS
/// `<color-interpolation-method>`.
///
/// <https://drafts.csswg.org/css-color-4/#interpolation-space>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationSpace {
	Srgb,
	SrgbLinear,
	DisplayP3,
	A98Rgb,
	ProphotoRgb,
	Rec2020,
	Lab,
	Oklab,
	XyzD50,
	XyzD65,
	Hsl(HueInterpolation),
	Hwb(HueInterpolation),
	Lch(HueInterpolation),
	Oklch(HueInterpolation),
}

impl InterpolationSpace {
	/// The index of the hue among the first three channels, for polar spaces.
	pub const fn hue_index(self) -> Option<usize> {
		match self {
			Self::Hsl(_) | Self::Hwb(_) => Some(0),
			Self::Lch(_) | Self::Oklch(_) => Some(2),
			_ => None,
		}
	}

	/// The direction hues travel in, for polar spaces.
	pub const fn hue_interpolation(self) -> Option<HueInterpolation> {
		match self {
			Self::Hsl(hue) | Self::Hwb(hue) | Self::Lch(hue) | Self::Oklch(hue) => Some(hue),
			_ => None,
		}
	}

	/// The channels of `color` in this space, in the units of this space's colour type, with a powerless hue missing.
	///
	/// <https://drafts.csswg.org/css-color-4/#powerless>
	pub fn channels(self, color: Color) -> [Channel; 4] {
		let mut channels: [Channel; 4] = match self {
			Self::Srgb => Srgb::from(color).into(),
			Self::SrgbLinear => LinearRgb::from(color).into(),
			Self::DisplayP3 => DisplayP3::from(color).into(),
			Self::A98Rgb => A98Rgb::from(color).into(),
			Self::ProphotoRgb => ProphotoRgb::from(color).into(),
			Self::Rec2020 => Rec2020::from(color).into(),
			Self::Lab => Lab::from(color).into(),
			Self::Oklab => Oklab::from(color).into(),
			Self::XyzD50 => XyzD50::from(color).into(),
			Self::XyzD65 => XyzD65::from(color).into(),
			Self::Hsl(_) => Hsl::from(color).into(),
			Self::Hwb(_) => Hwb::from(color).into(),
			Self::Lch(_) => Lch::from(color).into(),
			Self::Oklch(_) => Oklch::from(color).into(),
		};
		if let Some(hue) = self.hue_index()
			&& self.hue_is_powerless(&channels)
		{
			channels[hue] = None;
		}
		channels
	}

	fn hue_is_powerless(self, channels: &[Channel; 4]) -> bool {
		match self {
			Self::Hsl(_) => channels[1].is_some_and(|s| s.abs() < 1e-4),
			Self::Hwb(_) => matches!((channels[1], channels[2]), (Some(w), Some(b)) if w + b >= 100.0 - 1e-4),
			Self::Lch(_) => channels[1].is_some_and(|c| c.abs() < 0.0015),
			Self::Oklch(_) => channels[1].is_some_and(|c| c.abs() < 0.000004),
			_ => false,
		}
	}

	/// The colour with `channels` in this space, in the units of this space's colour type. Missing channels are 0.
	pub fn color(self, channels: [Channel; 4]) -> Color {
		match self {
			Self::Srgb => Srgb::from(channels).into(),
			Self::SrgbLinear => LinearRgb::from(channels).into(),
			Self::DisplayP3 => DisplayP3::from(channels).into(),
			Self::A98Rgb => A98Rgb::from(channels).into(),
			Self::ProphotoRgb => ProphotoRgb::from(channels).into(),
			Self::Rec2020 => Rec2020::from(channels).into(),
			Self::Lab => Lab::from(channels).into(),
			Self::Oklab => Oklab::from(channels).into(),
			Self::XyzD50 => XyzD50::from(channels).into(),
			Self::XyzD65 => XyzD65::from(channels).into(),
			Self::Hsl(_) => Hsl::from(channels).into(),
			Self::Hwb(_) => Hwb::from(channels).into(),
			Self::Lch(_) => Lch::from(channels).into(),
			Self::Oklch(_) => Oklch::from(channels).into(),
		}
	}

	/// `first` and `second` ready to interpolate straight between, channel by channel: a channel missing from one takes
	/// the other's value, and stays missing when both miss it. In polar spaces, the first's hue is put within `0..360`
	/// and the second's turned by whole turns, so that going straight from one to the other travels this space's
	/// [`hue_interpolation`](Self::hue_interpolation).
	///
	/// <https://drafts.csswg.org/css-color-4/#interpolation-missing>
	pub fn endpoints(self, first: [Channel; 4], second: [Channel; 4]) -> [[Channel; 4]; 2] {
		let (mut first, mut second) = (first, second);
		for (a, b) in first.iter_mut().zip(&mut second) {
			(*a, *b) = (a.or(*b), b.or(*a));
		}
		if let Some((index, hue)) = self.hue_index().zip(self.hue_interpolation())
			&& let (Some(from), Some(to)) = (first[index], second[index])
		{
			let from = from.rem_euclid(360.0);
			first[index] = Some(from);
			second[index] = Some(from + hue.delta(from, to));
		}
		[first, second]
	}

	/// The channels `percentage` of the way from `first` to `second`, interpolated straight between their
	/// [`endpoints`](Self::endpoints), with every channel but the hue premultiplied by alpha. The hue ends within
	/// `0..360`.
	///
	/// <https://drafts.csswg.org/css-color-4/#interpolation-alpha>
	pub fn mix(self, first: [Channel; 4], second: [Channel; 4], percentage: f64) -> [Channel; 4] {
		let t = percentage / 100.0;
		let lerp = |a: f64, b: f64| a * (1.0 - t) + b * t;
		let [first, second] = self.endpoints(first, second);
		let [a1, a2] = [first[3], second[3]].map(|alpha| alpha.unwrap_or(0.0) / 100.0);
		let alpha = lerp(a1, a2);
		let mut mixed = [None, None, None, first[3].map(|_| alpha * 100.0)];
		for i in 0..3 {
			mixed[i] = first[i].zip(second[i]).map(|(a, b)| {
				if self.hue_index() == Some(i) {
					lerp(a, b).rem_euclid(360.0)
				} else if alpha == 0.0 {
					lerp(a, b)
				} else {
					lerp(a * a1, b * a2) / alpha
				}
			});
		}
		mixed
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{COLOR_EPSILON, ColorDistance, Named};

	const SHORTER: HueInterpolation = HueInterpolation::Shorter;

	fn halfway(space: InterpolationSpace, first: impl Into<Color>, second: impl Into<Color>) -> Color {
		space.color(space.mix(space.channels(first.into()), space.channels(second.into()), 50.0))
	}

	#[test]
	fn channels_round_trip_through_every_space() {
		let color = Color::Srgb(Srgb::new(0.25, 0.5, 0.75, 50.0));
		for space in [
			InterpolationSpace::Srgb,
			InterpolationSpace::SrgbLinear,
			InterpolationSpace::DisplayP3,
			InterpolationSpace::A98Rgb,
			InterpolationSpace::ProphotoRgb,
			InterpolationSpace::Rec2020,
			InterpolationSpace::Lab,
			InterpolationSpace::Oklab,
			InterpolationSpace::XyzD50,
			InterpolationSpace::XyzD65,
			InterpolationSpace::Hsl(SHORTER),
			InterpolationSpace::Hwb(SHORTER),
			InterpolationSpace::Lch(SHORTER),
			InterpolationSpace::Oklch(SHORTER),
		] {
			let srgb = Srgb::from(space.color(space.channels(color)));
			let [r, g, b, a] = [srgb.red, srgb.green, srgb.blue, srgb.alpha as f64];
			for (got, want) in [r, g, b, a].into_iter().zip([0.25, 0.5, 0.75, 50.0]) {
				assert!((got - want).abs() < 1e-3, "{space:?}: {srgb:?}");
			}
		}
	}

	#[test]
	fn achromatic_hues_are_powerless() {
		for space in [
			InterpolationSpace::Hsl(SHORTER),
			InterpolationSpace::Hwb(SHORTER),
			InterpolationSpace::Lch(SHORTER),
			InterpolationSpace::Oklch(SHORTER),
		] {
			let channels = space.channels(Color::Named(Named::White));
			assert_eq!(channels[space.hue_index().unwrap()], None, "{space:?}");
		}
		let red = InterpolationSpace::Oklch(SHORTER).channels(Color::Named(Named::Red))[2];
		assert!(red.is_some_and(|hue| (hue - 29.2339).abs() < 1e-3), "color.js: red is oklch(0.62796 0.25768 29.2339)");
	}

	#[test]
	fn longer_hue_keeps_a_half_turn_in_its_direction() {
		let hue = |from: f64, to: f64| {
			let color = |hue| [Some(hue), Some(50.0), Some(50.0), Some(100.0)];
			InterpolationSpace::Hsl(HueInterpolation::Longer).mix(color(from), color(to), 50.0)[0]
		};
		for (from, to, want) in [(0.0, 180.0, 90.0), (180.0, 0.0, 90.0), (0.0, 90.0, 225.0), (90.0, 0.0, 225.0)] {
			assert_eq!(hue(from, to), Some(want), "css-color-4 longer hue from {from} to {to}");
		}
	}

	#[test]
	fn hues_travel_in_the_direction_of_the_space() {
		let hsl = |hue| Hsl::from(halfway(InterpolationSpace::Hsl(hue), Named::Rebeccapurple, Named::Hotpink));
		for (hue, want) in [
			(HueInterpolation::Shorter, 300.0),
			(HueInterpolation::Longer, 120.0),
			(HueInterpolation::Decreasing, 120.0),
			(HueInterpolation::Increasing, 300.0),
		] {
			let mixed = hsl(hue);
			let source = "css-color-4: rebeccapurple is hsl(270 50% 40%), hotpink hsl(330 100% 70.588%)";
			let want = Hsl::new(want, 75.0, 55.294117, 100.0);
			assert!(mixed.close_to(want, COLOR_EPSILON), "{source}, so {hue:?} gives {want:?}, not {mixed:?}");
		}
		let [red, blue] = [0.0, 240.0].map(|hue| Hwb::new(hue, 0.0, 0.0, 100.0));
		let mixed = Hwb::from(halfway(InterpolationSpace::Hwb(HueInterpolation::Longer), red, blue));
		let want = Hwb::new(120.0, 0.0, 0.0, 100.0);
		assert!(
			mixed.close_to(want, COLOR_EPSILON),
			"css-color-4 longer hue from 0 to 240 gives {want:?}, not {mixed:?}"
		);
	}

	#[test]
	fn missing_channels_take_the_other_colours() {
		let first = [None, None, Some(0.0), Some(100.0)];
		let second = [Some(200.0 / 255.0), None, Some(0.0), Some(100.0)];
		let mixed = InterpolationSpace::Srgb.mix(first, second, 50.0);
		assert_eq!(mixed, [Some(200.0 / 255.0), None, Some(0.0), Some(100.0)], "css-color-4 missing components");
	}
}
