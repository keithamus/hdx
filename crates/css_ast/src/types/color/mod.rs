mod hex;
mod named;
mod system;

use crate::{ColorFunction, CssAtomSet};
use css_parse::{Box, T};
use csskit_derives::*;
use csskit_proc_macro::node;

pub use hex::*;
pub use named::*;
pub use system::*;

#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(NodeWithMetadata)]
#[metadata(value_kinds = Color)]
pub enum Color<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Currentcolor)]
	Currentcolor(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Transparent)]
	Transparent(T![Ident]),
	System(SystemColor),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Hex(HexColor),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Named(NamedColor),
	Function(Box<'a, ColorFunction<'a>>),
}

impl Color<'_> {
	// Alias CanvasText for #[initial()]
	// #[allow(non_upper_case_globals)]
	// pub const Canvastext: Color = Color::System(SystemColor::CanvasText);
}

#[cfg(feature = "chromashift")]
pub trait ToChromashift {
	fn to_chromashift(&self) -> Option<chromashift::Color>;
}

#[cfg(feature = "chromashift")]
impl ToChromashift for HexColor {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		Some(chromashift::Color::Hex(chromashift::Hex::new(self.hex_value())))
	}
}

#[cfg(feature = "chromashift")]
impl Color<'_> {
	/// Convert this colour to `[Channel; 4]` in interpolation space `C`, preserving `none` channels.
	///
	/// Returns `None` if the colour has no fixed value (e.g. `currentcolor`).
	pub fn to_mix_channels<C>(&self) -> Option<[chromashift::Channel; 4]>
	where
		C: From<chromashift::Color> + Into<[chromashift::Channel; 4]>,
	{
		let none = self.missing_channels();
		let mut ch: [chromashift::Channel; 4] = C::from(self.to_chromashift()?).into();
		for i in 0..4 {
			if none[i] {
				ch[i] = None;
			}
		}
		Some(ch)
	}

	/// Which channels are `none`, with alpha last.
	pub fn missing_channels(&self) -> [bool; 4] {
		use crate::{ColorFunction, NoneOr};

		match self {
			Color::Function(func) => match &**func {
				ColorFunction::Rgb(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, NoneOr::None(_)),
						matches!(p.6, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Rgba(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, NoneOr::None(_)),
						matches!(p.6, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Hsl(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, NoneOr::None(_)),
						matches!(p.6, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Hsla(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, NoneOr::None(_)),
						matches!(p.6, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Hwb(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.1, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Lab(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.1, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Lch(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.1, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Oklab(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.1, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Oklch(f) => {
					let p = &f.params;
					[
						matches!(p.0, NoneOr::None(_)),
						matches!(p.1, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.4, Some(NoneOr::None(_))),
					]
				}
				ColorFunction::Color(f) => {
					let p = &f.params;
					[
						matches!(p.1, NoneOr::None(_)),
						matches!(p.2, NoneOr::None(_)),
						matches!(p.3, NoneOr::None(_)),
						matches!(p.5, Some(NoneOr::None(_))),
					]
				}
				_ => [false; 4],
			},
			_ => [false; 4],
		}
	}
}

#[cfg(feature = "chromashift")]
impl ToChromashift for Color<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Srgb;

		match self {
			Color::Named(named) => named.to_chromashift(),
			Color::Hex(hex) => hex.to_chromashift(),

			// Transparent is black with 0 alpha
			Color::Transparent(_) => Some(chromashift::Color::Srgb(Srgb::new(0.0, 0.0, 0.0, 0.0))),

			// CurrentColor and System colors don't have fixed values
			// They depend on context/system settings
			Color::Currentcolor(_) => None,
			Color::System(_) => None,

			// Color functions - convert based on function type
			Color::Function(func) => func.to_chromashift(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Color, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, Color, "#fff");
		assert_parse!(CssAtomSet::ATOMS, Color, "red");
		assert_parse!(CssAtomSet::ATOMS, Color, "#ababab");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 255 255)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255,255,255)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgba(255,255,255,0.5)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(29 164 192/95%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 255 255/0.5)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 20% 12.2/0.5)");
		assert_parse!(CssAtomSet::ATOMS, Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(CssAtomSet::ATOMS, Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(CssAtomSet::ATOMS, Color, "hwb(740deg 20% 30%/50%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "lch(20% 30% 740deg/50%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "color-mix(in srgb,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Color, "color-mix(in oklch longer hue,red 25%,blue)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(from red 200 g b / alpha)");
		assert_parse!(CssAtomSet::ATOMS, Color, "hwb(from rebeccapurple h w b / 100%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "oklch(from currentcolor l c h)");
		assert_parse!(CssAtomSet::ATOMS, Color, "oklch(from currentcolor l c h)");
		assert_parse!(CssAtomSet::ATOMS, Color, "oklch(from #123456 0.75 C H)");
		assert_parse!(CssAtomSet::ATOMS, Color, "oklch(from lab(1 3 40) l c h / alpha)");
		assert_parse!(
			CssAtomSet::ATOMS,
			Color,
			"color(from color(srgb 0.7 0.5 0.3 / 40%) srgb alpha alpha alpha / alpha)"
		);
	}

	#[test]
	fn substitution_in_channels() {
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(calc(255/2) 0 0)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(var(--r) 0 0)");
		assert_parse!(CssAtomSet::ATOMS, Color, "hsl(calc(120deg) 50% 50%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "color-mix(in srgb,red calc(50%),blue)");
	}

	#[test]
	fn test_recoverable_writes() {
		// Missing /
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 255 255 0.5)");
		// Mixed legacy values
		assert_parse!(CssAtomSet::ATOMS, Color, "rgba(255,20%,255,0.5)");
		// Trailing comma
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255,255,255,)");
		// Using legacy comma syntax but with /
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255,255,255/0.5)");
		// Missing a comma
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(29,164 192,95%)");
	}

	#[test]
	fn test_errors() {
		// Using degrees for RGB
		assert_parse_error!(CssAtomSet::ATOMS, Color, "rgba(250deg, 255, 255 / 0.5)");
		// Using % for first component in hsl
		assert_parse_error!(CssAtomSet::ATOMS, Color, "hsl(250%, 255, 255)");
		// Using % for first component in lch
		assert_parse_error!(CssAtomSet::ATOMS, Color, "lch(250%, 255, 255)");
		// Using degrees for wrong component in hsl
		assert_parse_error!(CssAtomSet::ATOMS, Color, "hsl(250, 255deg, 255)");
		// Using degrees for wrong component in lch
		assert_parse_error!(CssAtomSet::ATOMS, Color, "lch(250, 255deg, 255)");
		assert_peek_false!(CssAtomSet::ATOMS, Color, "#F0000");
		assert_peek_false!(CssAtomSet::ATOMS, Color, "#ff");
		assert_peek_false!(CssAtomSet::ATOMS, Color, "#fffffff");
		assert_peek_false!(CssAtomSet::ATOMS, Color, "#ggg");
		assert_peek_false!(CssAtomSet::ATOMS, Color, "#gggg");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("#fff", Color);
		assert_visits!("black", Color);
		assert_visits!("rgb(255 255 255)", Color, ColorFunction, RgbFunction);
		assert_visits!("rgba(255,255,255,0.5)", Color, ColorFunction, RgbaFunction);
		assert_visits!("lab(63.673% 51.577 5.811)", Color, ColorFunction, LabFunction);
		assert_visits!("hwb(740deg 20% 30%/50%)", Color, ColorFunction, HwbFunction);
	}

	#[test]
	#[cfg(feature = "chromashift")]
	fn test_chromashift() {
		use super::ToChromashift;
		use crate::CssAtomSet;
		use chromashift::{Hex, Named, Srgb};
		use css_parse::assert_parse;

		assert_parse!(CssAtomSet::ATOMS, Color, "red", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Named(Named::Red)));
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "#f00", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Hex(Hex::new(0xFF0000FF))));
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "#0000", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Hex(Hex::new(0))));
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "#00000000", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Hex(Hex::new(0))));
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255, 0, 0)", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Srgb(Srgb::from_u8(255, 0, 0, 100.0))));
		});
	}

	#[test]
	#[cfg(feature = "chromashift")]
	fn test_chromashift_preserves_extended_range() {
		use super::ToChromashift;
		use crate::CssAtomSet;
		use chromashift::{LinearRgb, Srgb};
		use css_parse::assert_parse;

		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(300 0 0)", |node| {
			assert!(LinearRgb::from(node.to_chromashift().unwrap()).red > 1.0);
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "color(srgb 1.25 -0.125 0.5)", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Srgb(Srgb::new(1.25, -0.125, 0.5, 100.0))));
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "hsl(120 150% 50%)", |node| {
			let Srgb { red, green, blue, .. } = Srgb::from(node.to_chromashift().unwrap());
			assert!(red < 0.0 && green > 1.0 && blue < 0.0, "{red} {green} {blue}");
		});
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(12.5% 0 0)", |node| {
			assert_eq!(node.to_chromashift(), Some(chromashift::Color::Srgb(Srgb::new(0.125, 0.0, 0.0, 100.0))));
		});
	}
}
