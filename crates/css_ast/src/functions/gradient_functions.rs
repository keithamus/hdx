use css_parse::Box;

use super::prelude::*;
use crate::{AngleOrZero, AnglePercentage, AtPosition, CalcableValue, ColorInterpolationMethod, LengthPercentage};

/// <https://drafts.csswg.org/css-images-4/#typedef-gradient>
/// ```text,ignore
/// <gradient> = <linear-gradient()> | <repeating-linear-gradient()> | <radial-gradient()> | <repeating-radial-gradient()>
///            | <conic-gradient()> | <repeating-conic-gradient()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Gradient<'a> {
	#[atom(CssAtomSet::LinearGradient)]
	LinearGradientFunction(Box<'a, LinearGradientFunction<'a>>),
	#[atom(CssAtomSet::RepeatingLinearGradient)]
	RepeatingLinearGradientFunction(Box<'a, RepeatingLinearGradientFunction<'a>>),
	#[atom(CssAtomSet::RadialGradient)]
	RadialGradientFunction(Box<'a, RadialGradientFunction<'a>>),
	#[atom(CssAtomSet::RepeatingRadialGradient)]
	RepeatingRadialGradientFunction(Box<'a, RepeatingRadialGradientFunction<'a>>),
	#[atom(CssAtomSet::ConicGradient)]
	ConicGradientFunction(Box<'a, ConicGradientFunction<'a>>),
	#[atom(CssAtomSet::RepeatingConicGradient)]
	RepeatingConicGradientFunction(Box<'a, RepeatingConicGradientFunction<'a>>),
}

/// <https://drafts.csswg.org/css-images-4/#funcdef-linear-gradient>
/// ```text,ignore
/// <linear-gradient()> = linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ [ <angle> | <zero> | to <side-or-corner> ] || <color-interpolation-method> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Image)]
pub struct LinearGradientFunction<'a> {
	#[atom(CssAtomSet::LinearGradient)]
	pub name: T![Function],
	pub params: LinearGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LinearGradientFunctionParams<'a> {
	pub options: Option<LinearGradientOptions<'a>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub stops: CommaSeparated<'a, ColorStopOrHint<'a>>,
}

/// <https://drafts.csswg.org/css-images-4/#funcdef-repeating-linear-gradient>
/// ```text,ignore
/// <repeating-linear-gradient()> = repeating-linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ [ <angle> | <zero> | to <side-or-corner> ] || <color-interpolation-method> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Image)]
pub struct RepeatingLinearGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingLinearGradient)]
	pub name: T![Function],
	pub params: RepeatingLinearGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingLinearGradientFunctionParams<'a> {
	pub options: Option<LinearGradientOptions<'a>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub stops: CommaSeparated<'a, ColorStopOrHint<'a>>,
}

/// The direction and interpolation method of a linear gradient, in either order.
///
/// ```text,ignore
/// [ <angle> | <zero> | to <side-or-corner> ] || <color-interpolation-method>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
#[parse(one_must_occur)]
pub struct LinearGradientOptions<'a> {
	pub direction: Option<LinearDirection<'a>>,
	pub interpolation: Option<ColorInterpolationMethod>,
}

/// <https://drafts.csswg.org/css-images-4/#funcdef-radial-gradient>
/// ```text,ignore
/// <radial-gradient()> = radial-gradient( [ <radial-gradient-syntax> ] )
/// <radial-gradient-syntax> = [ [ [ <radial-shape> || <radial-size> ]? [ at <position> ]? ] || <color-interpolation-method> ]? , <color-stop-list>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Image)]
pub struct RadialGradientFunction<'a> {
	#[atom(CssAtomSet::RadialGradient)]
	pub name: T![Function],
	pub params: RadialGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RadialGradientFunctionParams<'a> {
	pub options: Option<RadialGradientOptions<'a>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub stops: CommaSeparated<'a, ColorStopOrHint<'a>>,
}

/// <https://drafts.csswg.org/css-images-4/#funcdef-repeating-radial-gradient>
/// ```text,ignore
/// <repeating-radial-gradient()> = repeating-radial-gradient( [ <radial-gradient-syntax> ] )
/// <radial-gradient-syntax> = [ [ [ <radial-shape> || <radial-size> ]? [ at <position> ]? ] || <color-interpolation-method> ]? , <color-stop-list>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Image)]
pub struct RepeatingRadialGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingRadialGradient)]
	pub name: T![Function],
	pub params: RepeatingRadialGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingRadialGradientFunctionParams<'a> {
	pub options: Option<RadialGradientOptions<'a>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub stops: CommaSeparated<'a, ColorStopOrHint<'a>>,
}

/// The geometry and interpolation method of a radial gradient, in either order.
///
/// ```text,ignore
/// [ [ <radial-shape> || <radial-size> ]? [ at <position> ]? ] || <color-interpolation-method>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
#[parse(one_must_occur)]
pub struct RadialGradientOptions<'a> {
	pub geometry: Option<RadialGeometry<'a>>,
	pub interpolation: Option<ColorInterpolationMethod>,
}

/// ```text,ignore
/// [ <radial-shape> || <radial-size> ]? [ at <position> ]?
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RadialGeometry<'a> {
	pub ending_shape: Option<RadialEndingShape<'a>>,
	pub at: Option<AtPosition<'a>>,
}

/// ```text,ignore
/// <radial-shape> || <radial-size>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
#[parse(one_must_occur)]
pub struct RadialEndingShape<'a> {
	pub shape: Option<RadialShape>,
	pub size: Option<RadialSize<'a>>,
}

/// <https://drafts.csswg.org/css-images-4/#funcdef-conic-gradient>
/// ```text,ignore
/// <conic-gradient()> = conic-gradient( [ <conic-gradient-syntax> ] )
/// <conic-gradient-syntax> = [ [ [ from [ <angle> | <zero> ] ]? [ at <position> ]? ] || <color-interpolation-method> ]? , <angular-color-stop-list>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Image)]
pub struct ConicGradientFunction<'a> {
	#[atom(CssAtomSet::ConicGradient)]
	pub name: T![Function],
	pub params: ConicGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ConicGradientFunctionParams<'a> {
	pub options: Option<ConicGradientOptions<'a>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub stops: CommaSeparated<'a, AngularColorStopOrHint<'a>>,
}

/// <https://drafts.csswg.org/css-images-4/#funcdef-repeating-conic-gradient>
/// ```text,ignore
/// <repeating-conic-gradient()> = repeating-conic-gradient( [ <conic-gradient-syntax> ] )
/// <conic-gradient-syntax> = [ [ [ from [ <angle> | <zero> ] ]? [ at <position> ]? ] || <color-interpolation-method> ]? , <angular-color-stop-list>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(value_kinds = Image)]
pub struct RepeatingConicGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingConicGradient)]
	pub name: T![Function],
	pub params: RepeatingConicGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingConicGradientFunctionParams<'a> {
	pub options: Option<ConicGradientOptions<'a>>,
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub stops: CommaSeparated<'a, AngularColorStopOrHint<'a>>,
}

/// The geometry and interpolation method of a conic gradient, in either order.
///
/// ```text,ignore
/// [ [ from [ <angle> | <zero> ] ]? [ at <position> ]? ] || <color-interpolation-method>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
#[parse(one_must_occur)]
pub struct ConicGradientOptions<'a> {
	pub geometry: Option<ConicGeometry<'a>>,
	pub interpolation: Option<ColorInterpolationMethod>,
}

/// ```text,ignore
/// [ from [ <angle> | <zero> ] ]? [ at <position> ]?
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ConicGeometry<'a> {
	pub from: Option<ConicFrom<'a>>,
	pub at: Option<AtPosition<'a>>,
}

/// ```text,ignore
/// from [ <angle> | <zero> ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ConicFrom<'a> {
	#[atom(CssAtomSet::From)]
	pub keyword: T![Ident],
	pub angle: CalcableValue<'a, AngleOrZero>,
}

#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NamedDirection {
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LinearDirection<'a> {
	Angle(CalcableValue<'a, Angle>),
	Named(#[atom(CssAtomSet::To)] T![Ident], NamedDirection, Option<NamedDirection>),
}

/// <https://drafts.csswg.org/css-images-4/#typedef-radial-size>
/// <https://drafts.csswg.org/css-images-4/#radial-size>
///
/// ```text,ignore
/// <radial-size> = <radial-extent>{1,2} | <length-percentage [0,∞]>{1,2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// ```
#[node]
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RadialSize<'a> {
	Extent(RadialExtent, Option<RadialExtent>),
	Circular(CalcableValue<'a, LengthPercentage>),
	Elliptical(CalcableValue<'a, LengthPercentage>, CalcableValue<'a, LengthPercentage>),
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-extent>
///
/// ```text,ignore
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// ```
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RadialExtent {
	#[atom(CssAtomSet::ClosestCorner)]
	ClosestCorner(T![Ident]),
	#[atom(CssAtomSet::ClosestSide)]
	ClosestSide(T![Ident]),
	#[atom(CssAtomSet::FarthestCorner)]
	FarthestCorner(T![Ident]),
	#[atom(CssAtomSet::FarthestSide)]
	FarthestSide(T![Ident]),
}

impl<'a> Parse<'a> for RadialSize<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if let Some(extent) = p.parse_if_peek::<RadialExtent>()? {
			let second = p.parse_if_peek::<RadialExtent>()?;
			return Ok(RadialSize::Extent(extent, second));
		}
		let first = p.parse::<CalcableValue<LengthPercentage>>()?;
		if let Some(second) = p.parse_if_peek::<CalcableValue<LengthPercentage>>()? {
			return Ok(Self::Elliptical(first, second));
		}
		Ok(Self::Circular(first))
	}
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-shape>
///
/// ```text,ignore
/// <radial-shape> = circle | ellipse
/// ```
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RadialShape {
	#[atom(CssAtomSet::Circle)]
	Circle(T![Ident]),
	#[atom(CssAtomSet::Ellipse)]
	Ellipse(T![Ident]),
}

/// <https://drafts.csswg.org/css-images-4/#typedef-color-stop-list>
///
/// ```text,ignore
/// <linear-color-stop> = <color> <length-percentage>{1,2}?
/// <linear-color-hint> = <length-percentage>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorStopOrHint<'a> {
	Hint(CalcableValue<'a, LengthPercentage>),
	Stop(Color<'a>, Option<CalcableValue<'a, LengthPercentage>>, Option<CalcableValue<'a, LengthPercentage>>),
}

/// <https://drafts.csswg.org/css-images-4/#typedef-angular-color-stop-list>
///
/// ```text,ignore
/// <angular-color-stop> = <color> <color-stop-angle>?
/// <color-stop-angle> = [ <angle-percentage> | <zero> ]{1,2}
/// <angular-color-hint> = <angle-percentage> | <zero>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum AngularColorStopOrHint<'a> {
	Hint(CalcableValue<'a, AnglePercentage>),
	Stop(Color<'a>, Option<CalcableValue<'a, AnglePercentage>>, Option<CalcableValue<'a, AnglePercentage>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(to bottom,yellow,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(yellow,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(to bottom,#fff,#fff 85%,#e6e6e6)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(45deg,#808080 25%,transparent 25%)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(to right,transparent,red 20%,red 80%,transparent)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(red 0% 40%,tan 0 70%,blue 0 100%)");
		assert_parse!(
			CssAtomSet::ATOMS,
			Gradient,
			"radial-gradient(closest-corner circle,rgba(1,65,255,0.4),rgba(1,65,255,0))"
		);
	}

	#[test]
	fn test_substitution() {
		// calc()/var() permitted in angle, stop-position, and radial-size slots.
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(calc(45deg),red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(red calc(20% + 5px),blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(calc(50px) circle,red,blue)");
	}

	#[test]
	fn test_interpolation() {
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(in oklab,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(45deg in oklch longer hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(in srgb to right,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(in oklab circle,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(ellipse 10px 20px at 10% 20% in srgb,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "conic-gradient(in hsl shorter hue,red,blue)");
	}

	#[test]
	fn test_radial_geometry() {
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(circle closest-side,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(at center,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(farthest-side at left top,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "repeating-radial-gradient(circle 10px at 0 0,red,blue 10%)");
	}

	#[test]
	fn test_double_position_stops() {
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(red 10% 20%,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "repeating-linear-gradient(red 0 10px,blue 10px 20px)");
	}

	#[test]
	fn test_conic() {
		assert_parse!(CssAtomSet::ATOMS, Gradient, "conic-gradient(red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "conic-gradient(from 45deg,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "conic-gradient(from 0,red,10%,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "conic-gradient(at 10% 20%,red 0deg,blue 50% 75%,green)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "conic-gradient(from 0.25turn at center in oklab,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "repeating-conic-gradient(red 0 10%,blue 10% 20%)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Gradient, "radial-gradient(at center circle,red,blue)");
		assert_parse_error!(CssAtomSet::ATOMS, Gradient, "conic-gradient(at center from 45deg,red,blue)");
		assert_parse_error!(CssAtomSet::ATOMS, Gradient, "linear-gradient(in oklab in srgb,red,blue)");
		assert_parse_error!(CssAtomSet::ATOMS, Gradient, "conic-gradient(red 10px,blue)");
	}
}
