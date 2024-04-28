use bitmask_enum::bitmask;
use hdx_atom::{atom, Atom};

#[derive(Default, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Whitepoint {
	D50,
	#[default]
	D65,
}

#[derive(Default)]
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorFunctionSyntax {
	// The anatomy of the u8 for Display values is:
	//
	//    |------ "Special" flags
	//    |  |--- ColorSpace or Notation values
	//    v  v
	// 0b 00 000000
	//    ^  ^
	//    |  |--- Color Space: (color() function with space)
	//    |                    000000 = Srgb
	//    |                    000001 = SrgbLinear
	//    |                    000010 = DisplayP3
	//    |                    000011 = A98Rgb
	//    |                    000100 = ProphotoRgb
	//    |                    000101 = Rec2020
	//    |                    000110 = Xyz
	//    |                    000111 = XyzD50
	//    |                    001000 = XyzD65
	//    |
	//    |                    .. <Reserved>
	//    |
	//    |                    (named Color Functions)
	//    |                    010001 = RgbNamed (Srgb RGB notation)
	//    |                    010010 = HslNamed (Srgb HSL notation)
	//    |                    010011 = HwbNamed (Srgb HWB notation)
	//    |                    010100 = LabNamed (Srgb LAB notation)
	//    |                    010101 = LchNamed (Srgb LCH notation)
	//    |                    010110 = OklabNamed (Srgb OKLAB notation)
	//    |                    010111 = OklchNamed (Srgb OKLCH notation)
	//	  |
	//    |-- "Special" flags: 01 = Legacy
	//                         10 = Omit the Alpha channel entirely
	//
	// <predefined-rgb>
	// color(srgb, ...)
	#[default]
	Srgb = 0b0000_0000,
	// color(srgb-linear, ...)
	SrgbLinear = 0b0000_0001,
	// color(display-p3, ...)
	DisplayP3 = 0b0000_0010,
	// color(a98rgb, ...)
	A98Rgb = 0b0000_0011,
	// color(photo-rgb, ...)
	ProphotoRgb = 0b0000_0100,
	// color(rec2020, ...)
	Rec2020 = 0b0000_0101,
	// <xyz>
	// color(xyz, ...)
	Xyz = 0b0000_0110,
	// color(xyz-d50, ...)
	XyzD50 = 0b0000_0111,
	// color(xyz-d65, ...)
	XyzD65 = 0b0000_1000,

	// rgb(...)
	RgbNamed = 0b0001_0001,
	// hsl(...)
	HslNamed = 0b0001_0010,
	// hwb(...)
	HwbNamed = 0b0001_0011,
	// lab(...)
	LabNamed = 0b0001_0100,
	// lch(...)
	LchNamed = 0b0001_0101,
	// oklab(...)
	OklabNamed = 0b0001_0110,
	// oklch(...)
	OklchNamed = 0b0001_0111,

	Legacy = 0b0100_0000,
	OmitAlpha = 0b1000_0000,
}

impl ColorFunctionSyntax {
	#[inline]
	pub fn from_color_space(atom: Atom) -> Option<Self> {
		match atom.to_ascii_lowercase() {
			atom!("srgb") => Some(Self::Srgb),
			atom!("srgb-linear") => Some(Self::SrgbLinear),
			atom!("display-p3") => Some(Self::DisplayP3),
			atom!("a98-rgb") => Some(Self::A98Rgb),
			atom!("prophoto-rgb") => Some(Self::ProphotoRgb),
			atom!("rec2020") => Some(Self::Rec2020),
			atom!("xyz") => Some(Self::Xyz),
			atom!("xyz-d50") => Some(Self::XyzD50),
			atom!("xyz-d65") => Some(Self::XyzD65),
			_ => None,
		}
	}

	#[inline]
	pub fn from_named_function(atom: Atom) -> Option<Self> {
		match atom.to_ascii_lowercase() {
			atom!("rgb") | atom!("rgba") => Some(Self::RgbNamed),
			atom!("hsl") | atom!("hsla") => Some(Self::HslNamed),
			atom!("hwb") => Some(Self::HwbNamed),
			atom!("lab") => Some(Self::LabNamed),
			atom!("lch") => Some(Self::LchNamed),
			atom!("oklab") => Some(Self::OklabNamed),
			atom!("oklch") => Some(Self::OklchNamed),
			_ => None,
		}
	}

	#[inline]
	pub fn is_named(&self) -> bool {
		self.bits & 0b0001_0000 > 0
	}

	#[inline]
	pub fn is_legacy(&self) -> bool {
		self.contains(Self::Legacy)
	}

	#[inline]
	pub fn first_is_hue(&self) -> bool {
		self.contains(Self::HslNamed) || self.contains(Self::HwbNamed)
	}

	#[inline]
	pub fn third_is_hue(&self) -> bool {
		self.contains(Self::LchNamed) || self.contains(Self::OklchNamed)
	}

	#[inline]
	pub fn color_space(&self) -> Atom {
		if self.contains(Self::SrgbLinear) {
			atom!("srgb-linear")
		} else if self.contains(Self::DisplayP3) {
			atom!("display-p3")
		} else if self.contains(Self::A98Rgb) {
			atom!("a98-rgb")
		} else if self.contains(Self::ProphotoRgb) {
			atom!("photo-rgb")
		} else if self.contains(Self::Rec2020) {
			atom!("rec2020")
		} else if self.contains(Self::Xyz) {
			atom!("xyz")
		} else if self.contains(Self::XyzD50) {
			atom!("xyz-d50")
		} else if self.contains(Self::XyzD65) {
			atom!("xyz-d65")
		} else {
			atom!("srgb")
		}
	}

	#[inline]
	pub fn named_function(&self) -> Option<Atom> {
		if !self.is_named() {
			None
		} else if self.contains(Self::OklchNamed) {
			Some(atom!("oklch"))
		} else if self.contains(Self::OklabNamed) {
			Some(atom!("oklab"))
		} else if self.contains(Self::LchNamed) {
			Some(atom!("lch"))
		} else if self.contains(Self::LabNamed) {
			Some(atom!("lab"))
		} else if self.contains(Self::HwbNamed) {
			Some(atom!("hwb"))
		} else if self.contains(Self::HslNamed) && self.contains(Self::Legacy) && !self.contains(Self::OmitAlpha) {
			Some(atom!("hsla"))
		} else if self.contains(Self::HslNamed) {
			Some(atom!("hsl"))
		} else if self.contains(Self::RgbNamed) && self.contains(Self::Legacy) && !self.contains(Self::OmitAlpha) {
			Some(atom!("rgba"))
		} else if self.contains(Self::RgbNamed) {
			Some(atom!("rgb"))
		} else {
			None
		}
	}

	#[inline]
	pub fn whitepoint(&self) -> Whitepoint {
		if self.contains(Self::ProphotoRgb) || self.contains(Self::XyzD50) {
			Whitepoint::D50
		} else {
			Whitepoint::D65
		}
	}
}

// https://www.w3.org/TR/css-color-4/#color-interpolation-method
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorMixSyntax {
	// The anatomy of the u8 for Display values is:
	//
	//    |------- Hue Interpolation flags
	//    |   |--- ColorSpace or Notation values
	//    v   v
	// 0b 000 00000

	// color-mix(in srgb, ...)
	Srgb = 0b0000_0000,
	// color-mix(in srgb-linear, ...)
	SrgbLinear = 0b0000_0001,
	// color-mix(in display-p3, ...)
	DisplayP3 = 0b0000_0010,
	// color-mix(in a98-rgb, ...)
	A98Rgb = 0b0000_0011,
	// color-mix(in photo-rgb, ...)
	ProphotoRgb = 0b0000_0100,
	// color-mix(in rec-2020, ...)
	Rec2020 = 0b0000_0101,
	// color-mix(in xyz, ...)
	Xyz = 0b0000_0110,
	// color-mix(in xyz-d50, ...)
	XyzD50 = 0b0000_0111,
	// color-mix(in xyz-d65, ...)
	XyzD65 = 0b0000_1000,
	// color-mix(in lab, ...)
	Lab = 0b0000_1001,
	// color-mix(in oklab, ...)
	Oklab = 0b0000_1010,

	// <polar-color-space>
	Hsl = 0b0001_0001,
	Hwb = 0b0001_0010,
	Lch = 0b0001_0011,
	Oklch = 0b0001_0100,

	// <hue-interpolation-method>
	ShorterHue = 0b0010_0000,
	LongerHue = 0b0100_0000,
	IncreasingHue = 0b0110_0000,
	DecreasingHue = 0b1000_0000,
}

impl ColorMixSyntax {
	#[inline]
	pub fn from_color_space(atom: Atom) -> Option<Self> {
		match atom.to_ascii_lowercase() {
			atom!("hsl") => Some(Self::Hsl),
			atom!("hwb") => Some(Self::Hwb),
			atom!("lch") => Some(Self::Lch),
			atom!("oklch") => Some(Self::Oklch),
			atom!("srgb") => Some(Self::Srgb),
			atom!("srgb-linear") => Some(Self::SrgbLinear),
			atom!("display-p3") => Some(Self::DisplayP3),
			atom!("a98-rgb") => Some(Self::A98Rgb),
			atom!("prophoto-rgb") => Some(Self::ProphotoRgb),
			atom!("rec2020") => Some(Self::Rec2020),
			atom!("xyz") => Some(Self::Xyz),
			atom!("xyz-d50") => Some(Self::XyzD50),
			atom!("xyz-d65") => Some(Self::XyzD65),
			atom!("lab") => Some(Self::Lab),
			atom!("oklab") => Some(Self::Oklab),
			_ => None,
		}
	}

	#[inline]
	pub fn is_polar(&self) -> bool {
		self.bits & 0b0001_0000 > 0
	}

	#[inline]
	pub fn is_rectangular(&self) -> bool {
		!self.is_polar()
	}

	#[inline]
	pub fn is_interpolating_hue(&self) -> bool {
		self.bits & 0b1110_0000 > 0
	}

	#[inline]
	pub fn hue_interpolation_method(&self) -> Option<Atom> {
		if self.contains(Self::ShorterHue) {
			Some(atom!("shorter"))
		} else if self.contains(Self::LongerHue) {
			Some(atom!("longer"))
		} else if self.contains(Self::IncreasingHue) {
			Some(atom!("increasing"))
		} else if self.contains(Self::DecreasingHue) {
			Some(atom!("decreasing"))
		} else {
			None
		}
	}

	#[inline]
	pub fn color_space(&self) -> Option<Atom> {
		if self.contains(Self::Hsl) {
			Some(atom!("hsl"))
		} else if self.contains(Self::Hwb) {
			Some(atom!("hwb"))
		} else if self.contains(Self::Lch) {
			Some(atom!("lch"))
		} else if self.contains(Self::Oklch) {
			Some(atom!("oklch"))
		} else if self.contains(Self::Srgb) {
			Some(atom!("srgb"))
		} else if self.contains(Self::SrgbLinear) {
			Some(atom!("srgb-linear"))
		} else if self.contains(Self::DisplayP3) {
			Some(atom!("display-p3"))
		} else if self.contains(Self::A98Rgb) {
			Some(atom!("a98-rgb"))
		} else if self.contains(Self::ProphotoRgb) {
			Some(atom!("prophoto-rgb"))
		} else if self.contains(Self::Rec2020) {
			Some(atom!("rec2020"))
		} else if self.contains(Self::Xyz) {
			Some(atom!("xyz"))
		} else if self.contains(Self::XyzD50) {
			Some(atom!("xyz-d50"))
		} else if self.contains(Self::XyzD65) {
			Some(atom!("xyz-d65"))
		} else if self.contains(Self::Lab) {
			Some(atom!("lab"))
		} else if self.contains(Self::Oklab) {
			Some(atom!("oklab"))
		} else {
			None
		}
	}

	#[inline]
	pub fn whitepoint(&self) -> Whitepoint {
		if self.contains(Self::ProphotoRgb) || self.contains(Self::XyzD50) {
			Whitepoint::D50
		} else {
			Whitepoint::D65
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	pub fn size_test() {
		assert_size!(ColorFunctionSyntax, 1);
		assert_size!(ColorMixSyntax, 1);
	}
}
