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
impl crate::ToChromashift for ColorMixFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::{
			A98Rgb, Channel, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, PolarLayout, ProphotoRgb, Rec2020,
			Srgb, XyzD50, XyzD65, mix_channels,
		};

		/// Two-color mix in space `C`, returning the result as `chromashift::Color`.
		fn mix_in<C>(
			a: &Color<'_>,
			b: &Color<'_>,
			percentage: f64,
			hue: chromashift::HueInterpolation,
		) -> Option<chromashift::Color>
		where
			C: From<chromashift::Color>
				+ Into<[Channel; 4]>
				+ From<[Channel; 4]>
				+ Into<chromashift::Color>
				+ PolarLayout,
		{
			let fa = a.to_mix_channels::<C>()?;
			let fb = b.to_mix_channels::<C>()?;
			Some(C::from(mix_channels(fa, fb, percentage, C::HUE_INDEX, hue)).into())
		}

		let color_space = self.interpolation.as_ref().map(|i| &i.color_space);

		let hue = match color_space {
			Some(InterpolationColorSpace::Polar(_, Some(him))) => him.direction.to_hue_interpolation(),
			_ => chromashift::HueInterpolation::Shorter,
		};

		// Collect (color, percentage) pairs, defaulting missing percentages to None.
		// A substituted/unresolved percentage cannot be statically resolved, so bail.
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

		// Normalise percentages: fill missing as equal shares summing to 100.
		let n = parts.len() as f64;
		let default_pct = 100.0 / n;
		let mut stack: std::vec::Vec<(chromashift::Color, f64)> = std::vec::Vec::with_capacity(parts.len());
		for (color, pct) in &parts {
			let p = pct.unwrap_or(default_pct);
			stack.push((color.to_chromashift()?, p));
		}

		// Per spec: if the sum of all percentages is 0, return transparent black.
		if stack.iter().map(|(_, p)| p).sum::<f64>() == 0.0 {
			return Some(chromashift::Color::Srgb(chromashift::Srgb::new(0.0, 0.0, 0.0, 0.0)));
		}

		// Pairwise left-to-right reduction per the spec stack algorithm.
		while stack.len() >= 2 {
			let (color_b, pct_b) = stack.remove(1);
			let (color_a, pct_a) = stack.remove(0);
			let combined = pct_a + pct_b;
			let progress = pct_b / combined;

			// Get the source Color AST nodes for none-channel preservation.
			let idx = parts.len() - stack.len() - 2;
			let ast_a = parts[idx].0;
			let ast_b = parts[idx + 1].0;

			// We need to_mix_channels for none-aware mixing, but we have a resolved
			// chromashift::Color for intermediate results. For intermediates (idx > 0),
			// none channels are already resolved so we use the chromashift color directly.
			let mixed = if idx == 0 {
				let dispatch = |space: &InterpolationColorSpace| match space {
					InterpolationColorSpace::Rectangular(s) => match s {
						RectangularColorSpace::Srgb(_) => mix_in::<Srgb>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::SrgbLinear(_) => {
							mix_in::<LinearRgb>(ast_a, ast_b, progress * 100.0, hue)
						}
						RectangularColorSpace::DisplayP3(_) => mix_in::<DisplayP3>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::A98Rgb(_) => mix_in::<A98Rgb>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::ProphotoRgb(_) => {
							mix_in::<ProphotoRgb>(ast_a, ast_b, progress * 100.0, hue)
						}
						RectangularColorSpace::Rec2020(_) => mix_in::<Rec2020>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::Lab(_) => mix_in::<Lab>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::Oklab(_) => mix_in::<Oklab>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::XyzD50(_) => mix_in::<XyzD50>(ast_a, ast_b, progress * 100.0, hue),
						RectangularColorSpace::Xyz(_) | RectangularColorSpace::XyzD65(_) => {
							mix_in::<XyzD65>(ast_a, ast_b, progress * 100.0, hue)
						}
					},
					InterpolationColorSpace::Polar(s, _) => match s {
						PolarColorSpace::Hsl(_) => mix_in::<Hsl>(ast_a, ast_b, progress * 100.0, hue),
						PolarColorSpace::Hwb(_) => mix_in::<Hwb>(ast_a, ast_b, progress * 100.0, hue),
						PolarColorSpace::Lch(_) => mix_in::<Lch>(ast_a, ast_b, progress * 100.0, hue),
						PolarColorSpace::Oklch(_) => mix_in::<Oklch>(ast_a, ast_b, progress * 100.0, hue),
					},
				};
				// Default to oklab when no interpolation method specified.
				if let Some(space) = color_space {
					dispatch(space)
				} else {
					mix_in::<Oklab>(ast_a, ast_b, progress * 100.0, hue)
				}
			} else {
				// Intermediate results have no none channels; mix directly in target space.
				let mix_direct = |space: &InterpolationColorSpace| {
					let fa: [Channel; 4] = match space {
						InterpolationColorSpace::Rectangular(s) => match s {
							RectangularColorSpace::Srgb(_) => Srgb::from(color_a).into(),
							RectangularColorSpace::SrgbLinear(_) => LinearRgb::from(color_a).into(),
							RectangularColorSpace::DisplayP3(_) => DisplayP3::from(color_a).into(),
							RectangularColorSpace::A98Rgb(_) => A98Rgb::from(color_a).into(),
							RectangularColorSpace::ProphotoRgb(_) => ProphotoRgb::from(color_a).into(),
							RectangularColorSpace::Rec2020(_) => Rec2020::from(color_a).into(),
							RectangularColorSpace::Lab(_) => Lab::from(color_a).into(),
							RectangularColorSpace::Oklab(_) => Oklab::from(color_a).into(),
							RectangularColorSpace::XyzD50(_) => XyzD50::from(color_a).into(),
							RectangularColorSpace::Xyz(_) | RectangularColorSpace::XyzD65(_) => {
								XyzD65::from(color_a).into()
							}
						},
						InterpolationColorSpace::Polar(s, _) => match s {
							PolarColorSpace::Hsl(_) => Hsl::from(color_a).into(),
							PolarColorSpace::Hwb(_) => Hwb::from(color_a).into(),
							PolarColorSpace::Lch(_) => Lch::from(color_a).into(),
							PolarColorSpace::Oklch(_) => Oklch::from(color_a).into(),
						},
					};
					let fb: [Channel; 4] = match space {
						InterpolationColorSpace::Rectangular(s) => match s {
							RectangularColorSpace::Srgb(_) => Srgb::from(color_b).into(),
							RectangularColorSpace::SrgbLinear(_) => LinearRgb::from(color_b).into(),
							RectangularColorSpace::DisplayP3(_) => DisplayP3::from(color_b).into(),
							RectangularColorSpace::A98Rgb(_) => A98Rgb::from(color_b).into(),
							RectangularColorSpace::ProphotoRgb(_) => ProphotoRgb::from(color_b).into(),
							RectangularColorSpace::Rec2020(_) => Rec2020::from(color_b).into(),
							RectangularColorSpace::Lab(_) => Lab::from(color_b).into(),
							RectangularColorSpace::Oklab(_) => Oklab::from(color_b).into(),
							RectangularColorSpace::XyzD50(_) => XyzD50::from(color_b).into(),
							RectangularColorSpace::Xyz(_) | RectangularColorSpace::XyzD65(_) => {
								XyzD65::from(color_b).into()
							}
						},
						InterpolationColorSpace::Polar(s, _) => match s {
							PolarColorSpace::Hsl(_) => Hsl::from(color_b).into(),
							PolarColorSpace::Hwb(_) => Hwb::from(color_b).into(),
							PolarColorSpace::Lch(_) => Lch::from(color_b).into(),
							PolarColorSpace::Oklch(_) => Oklch::from(color_b).into(),
						},
					};
					Some(match space {
						InterpolationColorSpace::Rectangular(s) => match s {
							RectangularColorSpace::Srgb(_) => {
								Srgb::from(mix_channels(fa, fb, progress * 100.0, Srgb::HUE_INDEX, hue)).into()
							}
							RectangularColorSpace::SrgbLinear(_) => {
								LinearRgb::from(mix_channels(fa, fb, progress * 100.0, LinearRgb::HUE_INDEX, hue))
									.into()
							}
							RectangularColorSpace::DisplayP3(_) => {
								DisplayP3::from(mix_channels(fa, fb, progress * 100.0, DisplayP3::HUE_INDEX, hue))
									.into()
							}
							RectangularColorSpace::A98Rgb(_) => {
								A98Rgb::from(mix_channels(fa, fb, progress * 100.0, A98Rgb::HUE_INDEX, hue)).into()
							}
							RectangularColorSpace::ProphotoRgb(_) => {
								ProphotoRgb::from(mix_channels(fa, fb, progress * 100.0, ProphotoRgb::HUE_INDEX, hue))
									.into()
							}
							RectangularColorSpace::Rec2020(_) => {
								Rec2020::from(mix_channels(fa, fb, progress * 100.0, Rec2020::HUE_INDEX, hue)).into()
							}
							RectangularColorSpace::Lab(_) => {
								Lab::from(mix_channels(fa, fb, progress * 100.0, Lab::HUE_INDEX, hue)).into()
							}
							RectangularColorSpace::Oklab(_) => {
								Oklab::from(mix_channels(fa, fb, progress * 100.0, Oklab::HUE_INDEX, hue)).into()
							}
							RectangularColorSpace::XyzD50(_) => {
								XyzD50::from(mix_channels(fa, fb, progress * 100.0, XyzD50::HUE_INDEX, hue)).into()
							}
							RectangularColorSpace::Xyz(_) | RectangularColorSpace::XyzD65(_) => {
								XyzD65::from(mix_channels(fa, fb, progress * 100.0, XyzD65::HUE_INDEX, hue)).into()
							}
						},
						InterpolationColorSpace::Polar(s, _) => match s {
							PolarColorSpace::Hsl(_) => {
								Hsl::from(mix_channels(fa, fb, progress * 100.0, Hsl::HUE_INDEX, hue)).into()
							}
							PolarColorSpace::Hwb(_) => {
								Hwb::from(mix_channels(fa, fb, progress * 100.0, Hwb::HUE_INDEX, hue)).into()
							}
							PolarColorSpace::Lch(_) => {
								Lch::from(mix_channels(fa, fb, progress * 100.0, Lch::HUE_INDEX, hue)).into()
							}
							PolarColorSpace::Oklch(_) => {
								Oklch::from(mix_channels(fa, fb, progress * 100.0, Oklch::HUE_INDEX, hue)).into()
							}
						},
					})
				};
				if let Some(space) = color_space {
					mix_direct(space)
				} else {
					let fa: [Channel; 4] = Oklab::from(color_a).into();
					let fb: [Channel; 4] = Oklab::from(color_b).into();
					Some(Oklab::from(mix_channels(fa, fb, progress * 100.0, Oklab::HUE_INDEX, hue)).into())
				}
			}?;

			stack.insert(0, (mixed, combined));
		}

		stack.into_iter().next().map(|(c, _)| c)
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
