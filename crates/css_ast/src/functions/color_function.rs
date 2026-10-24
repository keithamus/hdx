use super::prelude::*;
use crate::functions::color_mix_function::ColorMixFunction;
use crate::functions::light_dark_function::LightDarkFunction;
use crate::functions::relative_color::RelativeColorFunction;
use crate::{AngleOrNumber, NoneOr, NumberOrPercentage, NumericValue};
use css_parse::Box;

#[cfg(feature = "chromashift")]
enum Channel {
	Number(f64),
	Percentage(f64),
}

#[cfg(feature = "chromashift")]
impl Channel {
	fn unscaled(self) -> f64 {
		match self {
			Self::Number(value) | Self::Percentage(value) => value,
		}
	}

	fn scaled(self, percent_reference: f64) -> f64 {
		match self {
			Self::Number(number) => number,
			Self::Percentage(percent) => percent / 100.0 * percent_reference,
		}
	}

	fn resolve(channel: &NoneOr<NumericValue<'_, NumberOrPercentage>>) -> Option<Self> {
		match channel {
			NoneOr::None(_) => Some(Self::Number(0.0)),
			NoneOr::Some(NumericValue::Literal(NumberOrPercentage::Number(number))) => {
				Some(Self::Number(number.value() as f64))
			}
			NoneOr::Some(NumericValue::Literal(NumberOrPercentage::Percentage(percent))) => {
				Some(Self::Percentage(percent.value() as f64))
			}
			NoneOr::Some(_) => None,
		}
	}

	fn resolve_8(channel: &NoneOr<NumericValue<'_, NumberOrPercentage>>) -> Option<f64> {
		match channel {
			NoneOr::None(_) => Some(0.0),
			NoneOr::Some(NumericValue::Literal(NumberOrPercentage::Number(number))) => {
				Some(number.value() as f64 / 255.0)
			}
			NoneOr::Some(NumericValue::Literal(NumberOrPercentage::Percentage(percent))) => {
				Some(percent.value() as f64 / 100.0)
			}
			NoneOr::Some(_) => None,
		}
	}

	fn resolve_hue_degrees(channel: &NoneOr<NumericValue<'_, AngleOrNumber>>) -> Option<f64> {
		match channel {
			NoneOr::None(_) => Some(0.0),
			NoneOr::Some(NumericValue::Literal(AngleOrNumber::Number(number))) => Some(number.value() as f64),
			NoneOr::Some(NumericValue::Literal(AngleOrNumber::Angle(angle))) => Some(angle.as_degrees() as f64),
			NoneOr::Some(_) => None,
		}
	}

	fn resolve_alpha(alpha: &Option<NoneOr<NumericValue<'_, NumberOrPercentage>>>) -> Option<f32> {
		match alpha {
			None => Some(100.0),
			Some(alpha) => match Self::resolve(alpha)? {
				Self::Number(number) => Some((number * 100.0) as f32),
				Self::Percentage(percent) => Some(percent as f32),
			},
		}
	}
}

#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorSpace {
	#[atom(CssAtomSet::Srgb)]
	Srgb(T![Ident]),
	#[atom(CssAtomSet::SrgbLinear)]
	SrgbLinear(T![Ident]),
	#[atom(CssAtomSet::DisplayP3)]
	DisplayP3(T![Ident]),
	#[atom(CssAtomSet::A98Rgb)]
	A98Rgb(T![Ident]),
	#[atom(CssAtomSet::ProphotoRgb)]
	ProphotoRgb(T![Ident]),
	#[atom(CssAtomSet::Rec2020)]
	Rec2020(T![Ident]),
	#[atom(CssAtomSet::Xyz)]
	Xyz(T![Ident]),
	#[atom(CssAtomSet::XyzD50)]
	XyzD50(T![Ident]),
	#[atom(CssAtomSet::XyzD65)]
	XyzD65(T![Ident]),
}

#[node]
#[derive(IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CommaOrSlash(#[metadata(skip)] Cursor);

impl<'a> Peek<'a> for CommaOrSlash {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Delim]);

	#[inline(always)]
	fn peek<I>(_: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == ',' || c == '/'
	}
}

impl<'a> Parse<'a> for CommaOrSlash {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !p.peek::<Self>() {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
		Ok(Self(p.next()))
	}
}

/// <https://drafts.csswg.org/css-color/#typedef-color-function>
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorFunction<'a> {
	Relative(Box<'a, RelativeColorFunction<'a>>),
	Color(Box<'a, ColorFunctionColor<'a>>),
	ColorMix(Box<'a, ColorMixFunction<'a>>),
	LightDark(Box<'a, LightDarkFunction<'a>>),
	Rgb(RgbFunction<'a>),
	Rgba(RgbaFunction<'a>),
	Hsl(HslFunction<'a>),
	Hsla(HslaFunction<'a>),
	Hwb(HwbFunction<'a>),
	Lab(LabFunction<'a>),
	Lch(LchFunction<'a>),
	Oklab(OklabFunction<'a>),
	Oklch(OklchFunction<'a>),
}

impl<'a> Parse<'a> for ColorFunction<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Check for relative colour syntax first: any colour function with `from` as 2nd token.
		if p.peek::<Box<RelativeColorFunction>>() {
			return Ok(Self::Relative(p.parse()?));
		}
		if p.peek::<Box<ColorFunctionColor<'_>>>() {
			return Ok(Self::Color(p.parse()?));
		}
		if p.peek::<Box<ColorMixFunction>>() {
			return Ok(Self::ColorMix(p.parse()?));
		}
		if p.peek::<Box<LightDarkFunction>>() {
			return Ok(Self::LightDark(p.parse()?));
		}
		if p.peek::<RgbFunction>() {
			return Ok(Self::Rgb(p.parse()?));
		}
		if p.peek::<RgbaFunction>() {
			return Ok(Self::Rgba(p.parse()?));
		}
		if p.peek::<HslFunction>() {
			return Ok(Self::Hsl(p.parse()?));
		}
		if p.peek::<HslaFunction>() {
			return Ok(Self::Hsla(p.parse()?));
		}
		if p.peek::<HwbFunction>() {
			return Ok(Self::Hwb(p.parse()?));
		}
		if p.peek::<LabFunction>() {
			return Ok(Self::Lab(p.parse()?));
		}
		if p.peek::<LchFunction>() {
			return Ok(Self::Lch(p.parse()?));
		}
		if p.peek::<OklabFunction>() {
			return Ok(Self::Oklab(p.parse()?));
		}
		Ok(Self::Oklch(p.parse()?))
	}
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for ColorFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		match self {
			Self::Relative(r) => r.to_chromashift(),
			Self::Color(c) => c.to_chromashift(),
			Self::ColorMix(c) => c.to_chromashift(),
			Self::LightDark(_c) => None,
			Self::Rgb(c) => c.to_chromashift(),
			Self::Rgba(c) => c.to_chromashift(),
			Self::Hsl(c) => c.to_chromashift(),
			Self::Hsla(c) => c.to_chromashift(),
			Self::Hwb(c) => c.to_chromashift(),
			Self::Lab(c) => c.to_chromashift(),
			Self::Lch(c) => c.to_chromashift(),
			Self::Oklab(c) => c.to_chromashift(),
			Self::Oklch(c) => c.to_chromashift(),
		}
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-color>
///
/// ```text,ignore
/// color() = color( <colorspace-params> [ / [ <alpha-value> | none ] ]? )
/// <colorspace-params> = [ <predefined-rgb-params> | <xyz-params>]
/// <predefined-rgb-params> = <predefined-rgb> [ <number> | <percentage> | none ]{3}
/// <predefined-rgb> = srgb | srgb-linear | display-p3 | a98-rgb | prophoto-rgb | rec2020
/// <xyz-params> = <xyz-space> [ <number> | <percentage> | none ]{3}
/// <xyz-space> = xyz | xyz-d50 | xyz-d65
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorFunctionColor<'a> {
	#[atom(CssAtomSet::Color)]
	pub name: T![Function],
	pub params: ColorFunctionColorParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for ColorFunctionColor<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::{A98Rgb, DisplayP3, LinearRgb, ProphotoRgb, Rec2020, Srgb, XyzD50, XyzD65};

		let ColorFunctionColorParams(space, c1, c2, c3, _, alpha) = &self.params;

		let c1 = Channel::resolve(c1)?.scaled(1.0);
		let c2 = Channel::resolve(c2)?.scaled(1.0);
		let c3 = Channel::resolve(c3)?.scaled(1.0);
		let alpha = Channel::resolve_alpha(alpha)?;
		match space {
			ColorSpace::Srgb(_) => Some(chromashift::Color::Srgb(Srgb::new(c1, c2, c3, alpha))),
			ColorSpace::SrgbLinear(_) => Some(chromashift::Color::LinearRgb(LinearRgb::new(c1, c2, c3, alpha))),
			ColorSpace::DisplayP3(_) => Some(chromashift::Color::DisplayP3(DisplayP3::new(c1, c2, c3, alpha))),
			ColorSpace::A98Rgb(_) => Some(chromashift::Color::A98Rgb(A98Rgb::new(c1, c2, c3, alpha))),
			ColorSpace::ProphotoRgb(_) => Some(chromashift::Color::ProphotoRgb(ProphotoRgb::new(c1, c2, c3, alpha))),
			ColorSpace::Rec2020(_) => Some(chromashift::Color::Rec2020(Rec2020::new(c1, c2, c3, alpha))),
			ColorSpace::Xyz(_) | ColorSpace::XyzD65(_) => {
				Some(chromashift::Color::XyzD65(XyzD65::new(c1 * 100.0, c2 * 100.0, c3 * 100.0, alpha)))
			}
			ColorSpace::XyzD50(_) => {
				Some(chromashift::Color::XyzD50(XyzD50::new(c1 * 100.0, c2 * 100.0, c3 * 100.0, alpha)))
			}
		}
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorFunctionColorParams<'a>(
	pub ColorSpace,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)] pub Option<T![/]>,
	pub Option<NoneOr<NumericValue<'a, NumberOrPercentage>>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-rgb>
///
/// ```text,ignore
/// rgb() = [ <legacy-rgb-syntax> | <modern-rgb-syntax> ]
/// rgba() = [ <legacy-rgba-syntax> | <modern-rgba-syntax> ]
/// <legacy-rgb-syntax> =   rgb( <percentage>#{3} , <alpha-value>? ) |
///                   rgb( <number>#{3} , <alpha-value>? )
/// <legacy-rgba-syntax> = rgba( <percentage>#{3} , <alpha-value>? ) |
///                   rgba( <number>#{3} , <alpha-value>? )
/// <modern-rgb-syntax> = rgb(
///   [ <number> | <percentage> | none]{3}
///   [ / [<alpha-value> | none] ]?  )
/// <modern-rgba-syntax> = rgba(
///   [ <number> | <percentage> | none]{3}
///   [ / [<alpha-value> | none] ]?  )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RgbFunction<'a> {
	#[atom(CssAtomSet::Rgb)]
	pub name: T![Function],
	pub params: RgbFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for RgbFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.params.to_chromashift()
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RgbaFunction<'a> {
	#[atom(CssAtomSet::Rgba)]
	pub name: T![Function],
	pub params: RgbFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for RgbaFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.params.to_chromashift()
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RgbFunctionParams<'a>(
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub Option<CommaOrSlash>,
	pub Option<NoneOr<NumericValue<'a, NumberOrPercentage>>>,
);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for RgbFunctionParams<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Srgb;
		let Self(red, _, green, _, blue, _, alpha) = &self;
		let red = Channel::resolve_8(red)?;
		let green = Channel::resolve_8(green)?;
		let blue = Channel::resolve_8(blue)?;
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Srgb(Srgb::new(red, green, blue, alpha)))
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-hsl>
///
/// ```text,ignore
/// hsl() = [ <legacy-hsl-syntax> | <modern-hsl-syntax> ]
/// hsla() = [ <legacy-hsla-syntax> | <modern-hsla-syntax> ]
/// <modern-hsl-syntax> = hsl(
///     [<hue> | none]
///     [<percentage> | <number> | none]
///     [<percentage> | <number> | none]
///     [ / [<alpha-value> | none] ]? )
/// <modern-hsla-syntax> = hsla(
///     [<hue> | none]
///     [<percentage> | <number> | none]
///     [<percentage> | <number> | none]
///     [ / [<alpha-value> | none] ]? )
/// <legacy-hsl-syntax> = hsl( <hue>, <percentage>, <percentage>, <alpha-value>? )
/// <legacy-hsla-syntax> = hsla( <hue>, <percentage>, <percentage>, <alpha-value>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HslFunction<'a> {
	#[atom(CssAtomSet::Hsl)]
	pub name: T![Function],
	pub params: HslFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HslFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.params.to_chromashift()
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HslaFunction<'a> {
	#[atom(CssAtomSet::Hsla)]
	pub name: T![Function],
	pub params: HslFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HslaFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.params.to_chromashift()
	}
}

#[node]
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HslFunctionParams<'a>(
	pub NoneOr<NumericValue<'a, AngleOrNumber>>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub Option<CommaOrSlash>,
	pub Option<NoneOr<NumericValue<'a, NumberOrPercentage>>>,
);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HslFunctionParams<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Hsl;
		let Self(hue, _, saturation, _, lightness, _, alpha) = &self;
		let hue = Channel::resolve_hue_degrees(hue)?;
		let saturation = Channel::resolve(saturation)?.unscaled();
		let lightness = Channel::resolve(lightness)?.unscaled();
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Hsl(Hsl::new(hue, saturation, lightness, alpha)))
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-hwb>
///
/// ```text,ignore
/// hwb() = hwb(
///  [<hue> | none]
///  [<percentage> | <number> | none]
///  [<percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HwbFunction<'a> {
	#[atom(CssAtomSet::Hwb)]
	pub name: T![Function],
	pub params: HwbFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HwbFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Hwb;
		let HwbFunctionParams(hue, whiteness, blackness, _, alpha) = &self.params;
		let hue = Channel::resolve_hue_degrees(hue)?;
		let whiteness = Channel::resolve(whiteness)?.unscaled();
		let blackness = Channel::resolve(blackness)?.unscaled();
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Hwb(Hwb::new(hue, whiteness, blackness, alpha)))
	}
}

#[node]
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HwbFunctionParams<'a>(
	pub NoneOr<NumericValue<'a, AngleOrNumber>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)] pub Option<T![/]>,
	pub Option<NoneOr<NumericValue<'a, NumberOrPercentage>>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-lab>
///
/// ```text,ignore
/// lab() = lab( [<percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LabFunction<'a> {
	#[atom(CssAtomSet::Lab)]
	pub name: T![Function],
	pub params: LabFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for LabFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Lab;
		let LabFunctionParams(l, a, b, _, alpha) = &self.params;
		let l = Channel::resolve(l)?.unscaled();
		let a = Channel::resolve(a)?.scaled(125.0);
		let b = Channel::resolve(b)?.scaled(125.0);
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Lab(Lab::new(l, a, b, alpha)))
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LabFunctionParams<'a>(
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)] pub Option<T![/]>,
	pub Option<NoneOr<NumericValue<'a, NumberOrPercentage>>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-lch>
///
/// ```text,ignore
/// lch() = lch( [<percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <hue> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LchFunction<'a> {
	#[atom(CssAtomSet::Lch)]
	pub name: T![Function],
	pub params: LchFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for LchFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Lch;
		let LchFunctionParams(lightness, chroma, hue, _, alpha) = &self.params;
		let lightness = Channel::resolve(lightness)?.unscaled();
		let chroma = Channel::resolve(chroma)?.scaled(150.0);
		let hue = Channel::resolve_hue_degrees(hue)?;
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Lch(Lch::new(lightness, chroma, hue, alpha)))
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LchFunctionParams<'a>(
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, NumberOrPercentage>>,
	pub NoneOr<NumericValue<'a, AngleOrNumber>>,
	#[semantic_eq(skip)] pub Option<T![/]>,
	pub Option<NoneOr<NumericValue<'a, NumberOrPercentage>>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-oklab>
///
/// ```text,ignore
/// oklab() = oklab( [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
///  ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OklabFunction<'a> {
	#[atom(CssAtomSet::Oklab)]
	pub name: T![Function],
	pub params: LabFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for OklabFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Oklab;
		let LabFunctionParams(l, a, b, _, alpha) = &self.params;
		let l = Channel::resolve(l)?.scaled(1.0);
		let a = Channel::resolve(a)?.scaled(0.4);
		let b = Channel::resolve(b)?.scaled(0.4);
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Oklab(Oklab::new(l, a, b, alpha)))
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-oklch>
///
/// ```text,ignore
/// oklab() = oklab( [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
///  ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OklchFunction<'a> {
	#[atom(CssAtomSet::Oklch)]
	pub name: T![Function],
	pub params: LchFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for OklchFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Oklch;
		let LchFunctionParams(lightness, chroma, hue, _, alpha) = &self.params;
		let lightness = Channel::resolve(lightness)?.unscaled();
		let chroma = Channel::resolve(chroma)?.scaled(150.0);
		let hue = Channel::resolve_hue_degrees(hue)?;
		let alpha = Channel::resolve_alpha(alpha)?;
		Some(chromashift::Color::Oklch(Oklch::new(lightness, chroma, hue, alpha)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn substitution_in_channels() {
		use css_parse::assert_parse;
		// Math functions in channels.
		assert_parse!(CssAtomSet::ATOMS, RgbFunction, "rgb(calc(255/2) 0 0)");
		assert_parse!(CssAtomSet::ATOMS, HslFunction, "hsl(calc(120deg) 50% 50%)");
		// Substitution functions in channels.
		assert_parse!(CssAtomSet::ATOMS, RgbFunction, "rgb(var(--r) 0 0)");
		assert_parse!(CssAtomSet::ATOMS, HwbFunction, "hwb(90deg calc(10%) var(--b))");
		// Substitution in alpha.
		assert_parse!(CssAtomSet::ATOMS, RgbFunction, "rgb(0 0 0/var(--a))");
		// color() with math/substitution channels.
		assert_parse!(CssAtomSet::ATOMS, ColorFunctionColor, "color(srgb calc(0.5) var(--g) 0)");
	}
}
