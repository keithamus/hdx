use crate::{Alpha, Gamut, LinearRgb, Number, Srgb};
use core::fmt;

impl Srgb {
	/// Creates a colour from 8-bit channels, as written in `rgb(255, 0, 0)` or `#ff0000`.
	pub fn from_u8(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
		Self::new(red as f64 / 255.0, green as f64 / 255.0, blue as f64 / 255.0, alpha)
	}

	/// Returns the channels as 8-bit values, clamping out-of-gamut channels.
	pub fn to_u8(&self) -> [u8; 3] {
		[self.red, self.green, self.blue].map(|channel| (channel * 255.0).round().clamp(0.0, 255.0) as u8)
	}

	pub(crate) fn hue(&self) -> f64 {
		let Self { red, green, blue, .. } = *self;
		let max = red.max(green).max(blue);
		let delta = max - red.min(green).min(blue);
		if delta <= 1e-9 {
			return 0.0;
		}
		let sextant = if max == red {
			(green - blue) / delta + if green < blue { 6.0 } else { 0.0 }
		} else if max == green {
			(blue - red) / delta + 2.0
		} else {
			(red - green) / delta + 4.0
		};
		sextant * 60.0
	}

	fn legacy_alpha(&self) -> f64 {
		let byte = self.alpha as f64 * 2.55;
		let rounded = byte.round();
		if (byte - rounded).abs() >= 1e-3 {
			return self.alpha as f64 / 100.0;
		}
		let byte = rounded as u32;
		let percent = (byte * 100 + 127) / 255;
		if (percent * 255 + 50) / 100 == byte {
			percent as f64 / 100.0
		} else {
			((byte * 1000 + 127) / 255) as f64 / 1000.0
		}
	}
}

impl fmt::Display for Srgb {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if !self.in_gamut() {
			let Self { red, green, blue, alpha } = *self;
			return write!(f, "color(srgb {} {} {}{})", Number(red), Number(green), Number(blue), Alpha(alpha));
		}
		let [red, green, blue] = self.to_u8();
		if self.alpha < 100.0 {
			write!(f, "rgba({red}, {green}, {blue}, {})", Number(self.legacy_alpha()))
		} else {
			write!(f, "rgb({red}, {green}, {blue})")
		}
	}
}

impl From<Srgb> for LinearRgb {
	fn from(value: Srgb) -> Self {
		let Srgb { red, green, blue, alpha } = value;
		let [red, green, blue] = Srgb::SPACE.transfer.decode_all([red, green, blue]);
		LinearRgb::new(red, green, blue, alpha)
	}
}

impl From<LinearRgb> for Srgb {
	fn from(value: LinearRgb) -> Self {
		let LinearRgb { red, green, blue, alpha } = value;
		let [red, green, blue] = Srgb::SPACE.transfer.encode_all([red, green, blue]);
		Srgb::new(red, green, blue, alpha)
	}
}

#[cfg(feature = "anstyle")]
impl From<Srgb> for anstyle::RgbColor {
	fn from(value: Srgb) -> Self {
		let [red, green, blue] = value.to_u8();
		anstyle::RgbColor(red, green, blue)
	}
}

#[cfg(feature = "anstyle")]
impl From<anstyle::RgbColor> for Srgb {
	fn from(value: anstyle::RgbColor) -> Self {
		Srgb::from_u8(value.0, value.1, value.2, 100.0)
	}
}

#[cfg(feature = "owo-colors")]
impl From<Srgb> for owo_colors::Rgb {
	fn from(value: Srgb) -> Self {
		let [red, green, blue] = value.to_u8();
		owo_colors::Rgb(red, green, blue)
	}
}

#[cfg(feature = "owo-colors")]
impl From<owo_colors::Rgb> for Srgb {
	fn from(value: owo_colors::Rgb) -> Self {
		Srgb::from_u8(value.0, value.1, value.2, 100.0)
	}
}
