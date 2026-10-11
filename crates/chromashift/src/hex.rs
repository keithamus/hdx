use crate::{Srgb, ToAlpha};
use core::fmt;

/// An Hex representation of the sRGB colour space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hex(u32);

const fn is_shorthand_byte(byte: u8) -> bool {
	(byte >> 4) == (byte & 0xF)
}

impl Hex {
	pub const fn new(hex: u32) -> Self {
		Self(hex)
	}

	pub fn can_use_3_digit(self) -> bool {
		let [red, green, blue, _] = self.0.to_be_bytes();
		!self.has_alpha() && is_shorthand_byte(red) && is_shorthand_byte(green) && is_shorthand_byte(blue)
	}

	pub fn can_use_4_digit(self) -> bool {
		let [red, green, blue, alpha] = self.0.to_be_bytes();
		is_shorthand_byte(red) && is_shorthand_byte(green) && is_shorthand_byte(blue) && is_shorthand_byte(alpha)
	}

	pub const fn has_alpha(&self) -> bool {
		self.0 & 0xFF != 0xFF
	}
}

impl ToAlpha for Hex {
	fn to_alpha(&self) -> f32 {
		((self.0 & 0xFF) as f32 / 255.0) * 100.0
	}
}

impl fmt::Display for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let [red, green, blue, alpha] = self.0.to_be_bytes();
		if self.can_use_3_digit() {
			write!(f, "#{:x}{:x}{:x}", red & 0xF, green & 0xF, blue & 0xF)
		} else if self.can_use_4_digit() {
			write!(f, "#{:x}{:x}{:x}{:x}", red & 0xF, green & 0xF, blue & 0xF, alpha & 0xF)
		} else if self.has_alpha() {
			write!(f, "#{red:02x}{green:02x}{blue:02x}{alpha:02x}")
		} else {
			write!(f, "#{red:02x}{green:02x}{blue:02x}")
		}
	}
}

impl From<Hex> for Srgb {
	fn from(value: Hex) -> Self {
		let [red, green, blue, alpha] = value.0.to_be_bytes();
		Srgb::from_u8(red, green, blue, alpha as f32 * 100.0 / 255.0)
	}
}

impl From<Srgb> for Hex {
	fn from(value: Srgb) -> Self {
		let [red, green, blue] = value.to_u8();
		let alpha = value.alpha;
		Hex::new(
			((red as u32) << 24)
				| ((green as u32) << 16)
				| ((blue as u32) << 8)
				| ((alpha * 255.0 / 100.0).round() as u32),
		)
	}
}
