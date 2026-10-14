use super::prelude::*;
use crate::{CalcableValue, Percentage};

/// <https://drafts.csswg.org/css-color-5/#color-mix>
///
/// ```text,ignore
/// color-mix() = color-mix( <color-interpolation-method>? , [ <color> && <percentage [0,100]>? ]# )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Color)]
pub struct ColorMixFunction<'a> {
	#[atom(CssAtomSet::ColorMix)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub interpolation: Option<ColorInterpolationMethod>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub interpolation_comma: Option<T![,]>,
	pub parts: CommaSeparated<'a, ColorMixPart<'a>, 1>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-color-4/#color-interpolation-method>
///
/// ```text,ignore
/// <color-interpolation-method> = in [ <rectangular-color-space> | <polar-color-space> <hue-interpolation-method>? ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorInterpolationMethod {
	#[atom(CssAtomSet::In)]
	pub in_keyword: T![Ident],
	pub color_space: InterpolationColorSpace,
}

/// The color space for color interpolation, which can be rectangular or polar.
///
/// ```text,ignore
/// <rectangular-color-space> | <polar-color-space> <hue-interpolation-method>?
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InterpolationColorSpace {
	Rectangular(RectangularColorSpace),
	Polar(PolarColorSpace, Option<HueInterpolationMethod>),
}

/// <https://drafts.csswg.org/css-color-4/#typedef-rectangular-color-space>
///
/// ```text,ignore
/// <rectangular-color-space> = srgb | srgb-linear | display-p3 | a98-rgb |
///     prophoto-rgb | rec2020 | lab | oklab | xyz | xyz-d50 | xyz-d65
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RectangularColorSpace {
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
	#[atom(CssAtomSet::Lab)]
	Lab(T![Ident]),
	#[atom(CssAtomSet::Oklab)]
	Oklab(T![Ident]),
	#[atom(CssAtomSet::Xyz)]
	Xyz(T![Ident]),
	#[atom(CssAtomSet::XyzD50)]
	XyzD50(T![Ident]),
	#[atom(CssAtomSet::XyzD65)]
	XyzD65(T![Ident]),
}

/// <https://drafts.csswg.org/css-color-4/#typedef-polar-color-space>
///
/// ```text,ignore
/// <polar-color-space> = hsl | hwb | lch | oklch
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PolarColorSpace {
	#[atom(CssAtomSet::Hsl)]
	Hsl(T![Ident]),
	#[atom(CssAtomSet::Hwb)]
	Hwb(T![Ident]),
	#[atom(CssAtomSet::Lch)]
	Lch(T![Ident]),
	#[atom(CssAtomSet::Oklch)]
	Oklch(T![Ident]),
}

/// <https://drafts.csswg.org/css-color-4/#typedef-hue-interpolation-method>
///
/// ```text,ignore
/// <hue-interpolation-method> = [ shorter | longer | increasing | decreasing ] hue
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HueInterpolationMethod {
	pub direction: HueInterpolationDirection,
	#[atom(CssAtomSet::Hue)]
	pub hue_keyword: T![Ident],
}

/// The direction keyword for hue interpolation.
///
/// ```text,ignore
/// shorter | longer | increasing | decreasing
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HueInterpolationDirection {
	#[atom(CssAtomSet::Shorter)]
	Shorter(T![Ident]),
	#[atom(CssAtomSet::Longer)]
	Longer(T![Ident]),
	#[atom(CssAtomSet::Increasing)]
	Increasing(T![Ident]),
	#[atom(CssAtomSet::Decreasing)]
	Decreasing(T![Ident]),
}

/// A color with an optional percentage in a color-mix() function.
///
/// ```text,ignore
/// [ <color> && <percentage [0,100]>? ]
/// ```
///
/// The color and percentage can appear in either order.
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorMixPart<'a> {
	pub color: Color<'a>,
	pub percentage: Option<CalcableValue<'a, Percentage>>,
}

impl<'a> Peek<'a> for ColorMixPart<'a> {
	const PEEK_KINDSET: KindSet = Color::PEEK_KINDSET.combine(CalcableValue::<Percentage>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Color::peek(p, c) || CalcableValue::<Percentage>::peek(p, c)
	}
}

impl<'a> Parse<'a> for ColorMixPart<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Either order: <color> <percentage>? or <percentage> <color>
		let mut color = p.parse_if_peek::<Color>()?;
		let percentage = p.parse_if_peek::<CalcableValue<Percentage>>()?;
		if color.is_none() {
			color = Some(p.parse::<Color>()?);
		}
		Ok(Self { color: color.unwrap(), percentage })
	}
}

#[cfg(feature = "chromashift")]
impl HueInterpolationDirection {
	/// Converts this AST node to the corresponding chromashift hue interpolation direction.
	pub fn to_hue_interpolation(&self) -> chromashift::HueInterpolation {
		match self {
			Self::Shorter(_) => chromashift::HueInterpolation::Shorter,
			Self::Longer(_) => chromashift::HueInterpolation::Longer,
			Self::Increasing(_) => chromashift::HueInterpolation::Increasing,
			Self::Decreasing(_) => chromashift::HueInterpolation::Decreasing,
		}
	}
}

#[cfg(feature = "chromashift")]
impl InterpolationColorSpace {
	/// Converts this AST node to the chromashift space it selects, with the hue interpolation it selects for polar spaces.
	pub fn to_chromashift(&self) -> chromashift::InterpolationSpace {
		use chromashift::InterpolationSpace;
		match self {
			Self::Rectangular(space) => match space {
				RectangularColorSpace::Srgb(_) => InterpolationSpace::Srgb,
				RectangularColorSpace::SrgbLinear(_) => InterpolationSpace::SrgbLinear,
				RectangularColorSpace::DisplayP3(_) => InterpolationSpace::DisplayP3,
				RectangularColorSpace::A98Rgb(_) => InterpolationSpace::A98Rgb,
				RectangularColorSpace::ProphotoRgb(_) => InterpolationSpace::ProphotoRgb,
				RectangularColorSpace::Rec2020(_) => InterpolationSpace::Rec2020,
				RectangularColorSpace::Lab(_) => InterpolationSpace::Lab,
				RectangularColorSpace::Oklab(_) => InterpolationSpace::Oklab,
				RectangularColorSpace::XyzD50(_) => InterpolationSpace::XyzD50,
				RectangularColorSpace::Xyz(_) | RectangularColorSpace::XyzD65(_) => InterpolationSpace::XyzD65,
			},
			Self::Polar(space, hue) => {
				let hue = hue.as_ref().map(|hue| hue.direction.to_hue_interpolation()).unwrap_or_default();
				match space {
					PolarColorSpace::Hsl(_) => InterpolationSpace::Hsl(hue),
					PolarColorSpace::Hwb(_) => InterpolationSpace::Hwb(hue),
					PolarColorSpace::Lch(_) => InterpolationSpace::Lch(hue),
					PolarColorSpace::Oklch(_) => InterpolationSpace::Oklch(hue),
				}
			}
		}
	}
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for ColorMixFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::{Channel, InterpolationSpace};

		let space = self.interpolation.as_ref().map_or(InterpolationSpace::Oklab, |i| i.color_space.to_chromashift());
		let channels = |color: &Color| {
			let mut channels = space.channels(color.to_chromashift()?);
			for (channel, missing) in channels.iter_mut().zip(color.missing_channels()) {
				if missing {
					*channel = None;
				}
			}
			Some(channels)
		};

		let parts: std::vec::Vec<(&Color<'_>, Option<f64>)> = (&self.parts)
			.into_iter()
			.map(|(p, _)| {
				let pct = match &p.percentage {
					None => None,
					Some(CalcableValue::Literal(pct)) => Some(pct.value() as f64),
					Some(_) => return None,
				};
				Some((&p.color, pct))
			})
			.collect::<Option<_>>()?;

		let default_pct = 100.0 / parts.len() as f64;
		let mut stack: std::vec::Vec<([Channel; 4], f64)> = std::vec::Vec::with_capacity(parts.len());
		for (color, pct) in &parts {
			stack.push((channels(color)?, pct.unwrap_or(default_pct)));
		}

		if stack.iter().map(|(_, p)| p).sum::<f64>() == 0.0 {
			return Some(chromashift::Color::Srgb(chromashift::Srgb::new(0.0, 0.0, 0.0, 0.0)));
		}

		while stack.len() >= 2 {
			let (b, pct_b) = stack.remove(1);
			let (a, pct_a) = stack.remove(0);
			let combined = pct_a + pct_b;
			stack.insert(0, (space.mix(a, b, pct_b / combined * 100.0), combined));
		}

		stack.into_iter().next().map(|(channels, _)| space.color(channels))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red 50%,blue 50%)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklch,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklch longer hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in hsl shorter hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in hsl increasing hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in hsl decreasing hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in lab,rgb(255 0 0),rgb(0 0 255))");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,50% red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red 50%,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklab,#fff 30%,#000 70%)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in xyz-d50,red,green)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in xyz-d65,red,green)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb-linear,red,green)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(red 50%,blue 50%)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklab,red,blue,green)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red 33%,blue 33%,green 34%)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(red,blue,green)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		// Named colors
		assert_visits!("color-mix(in srgb, red, blue)", ColorMixFunction, ColorInterpolationMethod, Color, Color,);
		// Function colors recurse into ColorFunction and its variant
		assert_visits!(
			"color-mix(in srgb, rgb(255, 0, 0), blue)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			ColorFunction,
			RgbFunction,
			Color,
		);
		// Percentages are visited
		assert_visits!(
			"color-mix(in srgb, red 50%, blue 50%)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			Percentage,
			Color,
			Percentage,
		);
		// Polar color space with hue interpolation
		assert_visits!(
			"color-mix(in oklch shorter hue, red, blue)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			Color,
		);
		assert_visits!("color-mix(red, blue)", ColorMixFunction, Color, Color,);
		assert_visits!(
			"color-mix(in oklab, red, blue, green)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			Color,
			Color,
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(srgb,red,blue)");
	}

	#[test]
	fn substitution_in_percentage() {
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red calc(50%),blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red var(--p),blue)");
	}
}
