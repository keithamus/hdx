use std::{
	hash::{Hash, Hasher},
	str::Chars,
};

#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Angle, MathExpr};
use crate::{atom, Atom, Atomizable, Box, Spanned};

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum ColorValue<'a> {
	CurrentColor,
	#[default]
	Transparent,
	Hex(u32),
	Named(NamedColor),
	RGB(Box<'a, Spanned<RGB<'a>>>),
	HSL(Box<'a, Spanned<HSL<'a>>>),
	HWB(Box<'a, Spanned<HWB<'a>>>),
	LAB(Box<'a, Spanned<LAB<'a>>>),
	LCH(Box<'a, Spanned<LCH<'a>>>),
	OKLAB(Box<'a, Spanned<LAB<'a>>>),
	OKLCH(Box<'a, Spanned<LCH<'a>>>),
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct RGB<'a> {
	pub r: Spanned<MathExpr<'a, NumberPercentageOrNone>>,
	pub g: Spanned<MathExpr<'a, NumberPercentageOrNone>>,
	pub b: Spanned<MathExpr<'a, NumberPercentageOrNone>>,
	pub alpha: Spanned<MathExpr<'a, NumberPercentageOrNone>>,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct HSL<'a> {
	pub h: MathExpr<'a, Hue>,
	pub s: MathExpr<'a, PercentageOrNone>,
	pub l: MathExpr<'a, PercentageOrNone>,
	pub alpha: MathExpr<'a, NumberPercentageOrNone>,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct HWB<'a> {
	pub h: MathExpr<'a, Hue>,
	pub w: MathExpr<'a, PercentageOrNone>,
	pub b: MathExpr<'a, PercentageOrNone>,
	pub alpha: MathExpr<'a, NumberPercentageOrNone>,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct LAB<'a> {
	pub l: MathExpr<'a, NumberPercentageOrNone>,
	pub a: MathExpr<'a, NumberPercentageOrNone>,
	pub b: MathExpr<'a, Hue>,
	pub alpha: MathExpr<'a, NumberPercentageOrNone>,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct LCH<'a> {
	pub l: MathExpr<'a, NumberPercentageOrNone>,
	pub c: MathExpr<'a, NumberPercentageOrNone>,
	pub h: MathExpr<'a, Hue>,
	pub alpha: MathExpr<'a, NumberPercentageOrNone>,
}

#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum NumberPercentageOrNone {
	#[default]
	None,
	Number(f32),
	Percentage(f32),
}

impl Hash for NumberPercentageOrNone {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::None => {
				0.hash(state);
			}
			Self::Number(n) => {
				1.hash(state);
				n.to_bits().hash(state);
			}
			Self::Percentage(p) => {
				2.hash(state);
				p.to_bits().hash(state);
			}
		}
	}
}

#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum PercentageOrNone {
	#[default]
	None,
	Percentage(f32),
}

impl Hash for PercentageOrNone {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::None => {
				0.hash(state);
			}
			Self::Percentage(p) => {
				1.hash(state);
				p.to_bits().hash(state);
			}
		}
	}
}

#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Hue {
	#[default]
	None,
	Number(f32),
	Angle(Angle),
}

impl Hash for Hue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::None => {
				0.hash(state);
			}
			Self::Number(n) => {
				1.hash(state);
				n.to_bits().hash(state);
			}
			Self::Angle(a) => {
				2.hash(state);
				a.hash(state);
			}
		}
	}
}

trait HexableChars {
	fn next_as_hex(&mut self) -> Option<u32>;
}

impl<'a> HexableChars for Chars<'a> {
	fn next_as_hex(&mut self) -> Option<u32> {
		match self.next() {
			Some(ch) => {
				let b = ch as u8;
				match b {
					b'A'..=b'F' => Some((b - b'A' + 10) as u32),
					b'a'..=b'f' => Some((b - b'a' + 10) as u32),
					b'0'..=b'9' => Some((b - b'0') as u32),
					_ => None,
				}
			}
			_ => None,
		}
	}
}

pub enum ToHexStyle {
	Compact,
	ExpandedElideAlpha,
	Expanded,
}

impl<'a> ColorValue<'a> {
	pub fn from_hex(str: &str) -> Option<ColorValue<'a>> {
		let mut chars = str.chars();
		if str.starts_with('#') {
			chars.next();
		}
		let (r, g, b, a) = match str.len() {
			// <r><g><b> implied alpha
			3 => (
				chars.next_as_hex().unwrap() * 17,
				chars.next_as_hex().unwrap() * 17,
				chars.next_as_hex().unwrap() * 17,
				255,
			),
			// <r><g><b><a>
			4 => (
				chars.next_as_hex().unwrap() * 17,
				chars.next_as_hex().unwrap() * 17,
				chars.next_as_hex().unwrap() * 17,
				chars.next_as_hex().unwrap() * 17,
			),
			// <rr><gg><bb> implied alpha
			6 => (
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
				255,
			),
			// <rr><gg><bb><aa>
			8 => (
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
				chars.next_as_hex().unwrap() << 4 | chars.next_as_hex().unwrap(),
			),
			_ => {
				return None;
			}
		};
		Some(ColorValue::Hex(r << 24 | g << 16 | b << 8 | a))
	}

	pub fn to_hex(&self, style: ToHexStyle) -> Option<String> {
		if let Self::Hex(d) = self {
			let compacted = ((d & 0x0FF00000) >> 12) | ((d & 0x00000FF0) >> 4);
			let expanded = ((compacted & 0xF000) << 16)
				| ((compacted & 0xFF00) << 12)
				| ((compacted & 0x0FF0) << 8)
				| ((compacted & 0x00FF) << 4)
				| (compacted & 0x000F);
			// Shorthand can be used
			if &expanded == d && d & 255 == 255 {
				Some(format!("#{:03x}", compacted >> 4))
			} else if &expanded == d {
				Some(format!("#{:04x}", compacted))
			} else if d & 255 == 255 {
				Some(format!("#{:06x}", d >> 8))
			} else {
				Some(format!("#{:08x}", d))
			}
		} else {
			None
		}
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum NamedColor {
	Aliceblue,            // atom!("aliceblue")
	Antiquewhite,         // atom!("antiquewhite")
	Aqua,                 // atom!("aqua")
	Aquamarine,           // atom!("aquamarine")
	Azure,                // atom!("azure")
	Beige,                // atom!("beige")
	Bisque,               // atom!("bisque")
	Black,                // atom!("black")
	Blanchedalmond,       // atom!("blanchedalmond")
	Blue,                 // atom!("blue")
	Blueviolet,           // atom!("blueviolet")
	Brown,                // atom!("brown")
	Burlywood,            // atom!("burlywood")
	Cadetblue,            // atom!("cadetblue")
	Chartreuse,           // atom!("chartreuse")
	Chocolate,            // atom!("chocolate")
	Coral,                // atom!("coral")
	Cornflowerblue,       // atom!("cornflowerblue")
	Cornsilk,             // atom!("cornsilk")
	Crimson,              // atom!("crimson")
	Cyan,                 // atom!("cyan")
	Darkblue,             // atom!("darkblue")
	Darkcyan,             // atom!("darkcyan")
	Darkgoldenrod,        // atom!("darkgoldenrod")
	Darkgray,             // atom!("darkgray")
	Darkgreen,            // atom!("darkgreen")
	Darkgrey,             // atom!("darkgrey")
	Darkkhaki,            // atom!("darkkhaki")
	Darkmagenta,          // atom!("darkmagenta")
	Darkolivegreen,       // atom!("darkolivegreen")
	Darkorange,           // atom!("darkorange")
	Darkorchid,           // atom!("darkorchid")
	Darkred,              // atom!("darkred")
	Darksalmon,           // atom!("darksalmon")
	Darkseagreen,         // atom!("darkseagreen")
	Darkslateblue,        // atom!("darkslateblue")
	Darkslategray,        // atom!("darkslategray")
	Darkslategrey,        // atom!("darkslategrey")
	Darkturquoise,        // atom!("darkturquoise")
	Darkviolet,           // atom!("darkviolet")
	Deeppink,             // atom!("deeppink")
	Deepskyblue,          // atom!("deepskyblue")
	Dimgray,              // atom!("dimgray")
	Dimgrey,              // atom!("dimgrey")
	Dodgerblue,           // atom!("dodgerblue")
	Firebrick,            // atom!("firebrick")
	Floralwhite,          // atom!("floralwhite")
	Forestgreen,          // atom!("forestgreen")
	Fuchsia,              // atom!("fuchsia")
	Gainsboro,            // atom!("gainsboro")
	Ghostwhite,           // atom!("ghostwhite")
	Gold,                 // atom!("gold")
	Goldenrod,            // atom!("goldenrod")
	Gray,                 // atom!("gray")
	Green,                // atom!("green")
	Greenyellow,          // atom!("greenyellow")
	Grey,                 // atom!("grey")
	Honeydew,             // atom!("honeydew")
	Hotpink,              // atom!("hotpink")
	Indianred,            // atom!("indianred")
	Indigo,               // atom!("indigo")
	Ivory,                // atom!("ivory")
	Khaki,                // atom!("khaki")
	Lavender,             // atom!("lavender")
	Lavenderblush,        // atom!("lavenderblush")
	Lawngreen,            // atom!("lawngreen")
	Lemonchiffon,         // atom!("lemonchiffon")
	Lightblue,            // atom!("lightblue")
	Lightcoral,           // atom!("lightcoral")
	Lightcyan,            // atom!("lightcyan")
	Lightgoldenrodyellow, // atom!("lightgoldenrodyellow")
	Lightgray,            // atom!("lightgray")
	Lightgreen,           // atom!("lightgreen")
	Lightgrey,            // atom!("lightgrey")
	Lightpink,            // atom!("lightpink")
	Lightsalmon,          // atom!("lightsalmon")
	Lightseagreen,        // atom!("lightseagreen")
	Lightskyblue,         // atom!("lightskyblue")
	Lightslategray,       // atom!("lightslategray")
	Lightslategrey,       // atom!("lightslategrey")
	Lightsteelblue,       // atom!("lightsteelblue")
	Lightyellow,          // atom!("lightyellow")
	Lime,                 // atom!("lime")
	Limegreen,            // atom!("limegreen")
	Linen,                // atom!("linen")
	Magenta,              // atom!("magenta")
	Maroon,               // atom!("maroon")
	Mediumaquamarine,     // atom!("mediumaquamarine")
	Mediumblue,           // atom!("mediumblue")
	Mediumorchid,         // atom!("mediumorchid")
	Mediumpurple,         // atom!("mediumpurple")
	Mediumseagreen,       // atom!("mediumseagreen")
	Mediumslateblue,      // atom!("mediumslateblue")
	Mediumspringgreen,    // atom!("mediumspringgreen")
	Mediumturquoise,      // atom!("mediumturquoise")
	Mediumvioletred,      // atom!("mediumvioletred")
	Midnightblue,         // atom!("midnightblue")
	Mintcream,            // atom!("mintcream")
	Mistyrose,            // atom!("mistyrose")
	Moccasin,             // atom!("moccasin")
	Navajowhite,          // atom!("navajowhite")
	Navy,                 // atom!("navy")
	Oldlace,              // atom!("oldlace")
	Olive,                // atom!("olive")
	Olivedrab,            // atom!("olivedrab")
	Orange,               // atom!("orange")
	Orangered,            // atom!("orangered")
	Orchid,               // atom!("orchid")
	Palegoldenrod,        // atom!("palegoldenrod")
	Palegreen,            // atom!("palegreen")
	Paleturquoise,        // atom!("paleturquoise")
	Palevioletred,        // atom!("palevioletred")
	Papayawhip,           // atom!("papayawhip")
	Peachpuff,            // atom!("peachpuff")
	Peru,                 // atom!("peru")
	Pink,                 // atom!("pink")
	Plum,                 // atom!("plum")
	Powderblue,           // atom!("powderblue")
	Purple,               // atom!("purple")
	Rebeccapurple,        // atom!("rebeccapurple")
	Red,                  // atom!("red")
	Rosybrown,            // atom!("rosybrown")
	Royalblue,            // atom!("royalblue")
	Saddlebrown,          // atom!("saddlebrown")
	Salmon,               // atom!("salmon")
	Sandybrown,           // atom!("sandybrown")
	Seagreen,             // atom!("seagreen")
	Seashell,             // atom!("seashell")
	Sienna,               // atom!("sienna")
	Silver,               // atom!("silver")
	Skyblue,              // atom!("skyblue")
	Slateblue,            // atom!("slateblue")
	Slategray,            // atom!("slategray")
	Slategrey,            // atom!("slategrey")
	Snow,                 // atom!("snow")
	Springgreen,          // atom!("springgreen")
	Steelblue,            // atom!("steelblue")
	Tan,                  // atom!("tan")
	Teal,                 // atom!("teal")
	Thistle,              // atom!("thistle")
	Tomato,               // atom!("tomato")
	Turquoise,            // atom!("turquoise")
	Violet,               // atom!("violet")
	Wheat,                // atom!("wheat")
	White,                // atom!("white")
	Whitesmoke,           // atom!("whitesmoke")
	Yellow,               // atom!("yellow")
	Yellowgreen,          // atom!("yellowgreen")
}

// https://drafts.csswg.org/css-color/#typedef-alpha-value
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AlphaValue {
	Number(f32),
	Percentage(f32),
}

impl Hash for AlphaValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		let (i, f) = match self {
			Self::Number(f) => (0, f),
			Self::Percentage(f) => (1, f),
		};
		i.hash(state);
		f.to_bits().hash(state);
	}
}

// https://drafts.csswg.org/css-color/#interpolation-space
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum ColorSpace {
	Rectangular(RectangularColorSpace),
	Polar(PolarColorSpace),
}

// https://drafts.csswg.org/css-color/#interpolation-space
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PolarColorSpace {
	Hsl,
	Hwb,
	Lch,
	OkLch,
}

// https://drafts.csswg.org/css-color/#interpolation-space
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum RectangularColorSpace {
	Srgb,
	SrgbLinear,
	DisplayP3,
	A98Rgb,
	ProphotoRgb,
	Rec2020,
	Lab,
	OkLab,
	Xyz,
	XyzD50,
	XyzD65,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<NamedColor>(), 1);
		assert_eq!(size_of::<ColorValue>(), 16);
		assert_eq!(size_of::<AlphaValue>(), 8);
		assert_eq!(size_of::<ColorSpace>(), 2);
		assert_eq!(size_of::<PolarColorSpace>(), 1);
		assert_eq!(size_of::<RectangularColorSpace>(), 1);
	}

	#[test]
	fn color_from_hex_3() {
		assert_eq!(ColorValue::from_hex("abc"), Some(ColorValue::Hex(2864434431)));
		assert_eq!(ColorValue::from_hex("fff"), Some(ColorValue::Hex(4294967295)));
		assert_eq!(ColorValue::from_hex("000"), Some(ColorValue::Hex(255)));
	}

	#[test]
	fn color_from_hex_4() {
		assert_eq!(ColorValue::from_hex("abcd"), Some(ColorValue::Hex(2864434397)));
		assert_eq!(ColorValue::from_hex("ffff"), Some(ColorValue::Hex(4294967295)));
		assert_eq!(ColorValue::from_hex("fff0"), Some(ColorValue::Hex(4294967040)));
	}

	#[test]
	fn color_from_hex_5() {
		assert_eq!(ColorValue::from_hex("abcde"), None);
		assert_eq!(ColorValue::from_hex("fffff"), None);
		assert_eq!(ColorValue::from_hex("fff00"), None);
	}

	#[test]
	fn color_from_hex_6() {
		assert_eq!(ColorValue::from_hex("bbccdd"), Some(ColorValue::Hex(3150765567)));
		assert_eq!(ColorValue::from_hex("ffffff"), Some(ColorValue::Hex(4294967295)));
	}

	#[test]
	fn color_from_hex_8() {
		assert_eq!(ColorValue::from_hex("bbccddee"), Some(ColorValue::Hex(3150765550)));
		assert_eq!(ColorValue::from_hex("ffffffff"), Some(ColorValue::Hex(4294967295)));
		assert_eq!(ColorValue::from_hex("ffffff00"), Some(ColorValue::Hex(4294967040)));
	}

	#[test]
	fn color_to_hex() {
		assert_eq!(ColorValue::Hex(255).to_hex(ToHexStyle::Compact), Some("#000".into()));
		assert_eq!(ColorValue::Hex(2864434431).to_hex(ToHexStyle::Compact), Some("#abc".into()));
		assert_eq!(ColorValue::Hex(4294967295).to_hex(ToHexStyle::Compact), Some("#fff".into()));
		assert_eq!(ColorValue::Hex(4294967040).to_hex(ToHexStyle::Compact), Some("#fff0".into()));
		assert_eq!(
			ColorValue::Hex(4210752000).to_hex(ToHexStyle::Compact),
			Some("#fafafa00".into())
		);
		// assert_eq!(
		// 	ColorValue::Hex(255).to_hex(ToHexStyle::ExpandedElideAlpha),
		// 	Some("#000000".into())
		// );
	}
}
