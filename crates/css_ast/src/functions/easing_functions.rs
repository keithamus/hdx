use super::prelude::*;
use crate::{CalcableValue, NumericValue, Percentage};
use css_parse::Box;

/// <https://drafts.csswg.org/css-easing-2/#typedef-easing-function>
///
/// ```text,ignore
/// <easing-function> = <linear-easing-function>
///                      | <cubic-bezier-easing-function>
///                      | <step-easing-function>
///
/// <linear-easing-function> = linear | <linear()>
///
/// linear() = linear( [ <number> && <percentage>{0,2} ]# )
///
/// <cubic-bezier-easing-function> = ease | ease-in | ease-out | ease-in-out
///                                    | <cubic-bezier()>
///
/// cubic-bezier() = cubic-bezier( [ <number [0,1]>, <number> ]#{2} )
///
/// <step-easing-function> = step-start | step-end | <steps()>
///
/// steps() = steps( <integer>, <step-position>?)
///
/// <step-position> = jump-start | jump-end | jump-none | jump-both | start | end
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum EasingFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Linear)]
	Linear(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Ease)]
	Ease(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::EaseIn)]
	EaseIn(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::EaseOut)]
	EaseOut(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::EaseInOut)]
	EaseInOut(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::StepStart)]
	StepStart(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::StepEnd)]
	StepEnd(T![Ident]),
	#[atom(CssAtomSet::Linear)]
	LinearFunction(LinearFunction<'a>),
	#[atom(CssAtomSet::CubicBezier)]
	CubicBezierFunction(Box<'a, CubicBezierFunction<'a>>),
	#[atom(CssAtomSet::Steps)]
	StepsFunction(StepsFunction<'a>),
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LinearFunction<'a> {
	#[atom(CssAtomSet::Linear)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, LinearFunctionParams<'a>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LinearFunctionParams<'a>(
	pub NumericValue<'a, T![Number]>,
	pub Option<CalcableValue<'a, Percentage>>,
	pub Option<CalcableValue<'a, Percentage>>,
);

impl<'a> Parse<'a> for LinearFunctionParams<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut num = p.parse_if_peek::<NumericValue<T![Number]>>()?;
		let percent = p.parse_if_peek::<CalcableValue<Percentage>>()?;
		let percent2 = p.parse_if_peek::<CalcableValue<Percentage>>()?;
		if num.is_none() {
			num = Some(p.parse::<NumericValue<T![Number]>>()?);
		}
		Ok(Self(num.unwrap(), percent, percent2))
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CubicBezierFunction<'a> {
	#[atom(CssAtomSet::CubicBezier)]
	pub name: T![Function],
	pub params: CubicBezierFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CubicBezierFunctionParams<'a> {
	pub x1: NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)]
	pub c1: Option<T![,]>,
	pub y1: NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)]
	pub c2: Option<T![,]>,
	pub x2: NumericValue<'a, T![Number]>,
	#[semantic_eq(skip)]
	pub c3: Option<T![,]>,
	pub y2: NumericValue<'a, T![Number]>,
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StepsFunction<'a> {
	#[atom(CssAtomSet::Steps)]
	pub name: T![Function],
	pub params: StepsFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StepsFunctionParams<'a>(
	pub NumericValue<'a, CSSInt>,
	#[semantic_eq(skip)] pub Option<T![,]>,
	pub Option<StepPosition>,
);

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum StepPosition {
	#[atom(CssAtomSet::JumpStart)]
	JumpStart(T![Ident]),
	#[atom(CssAtomSet::JumpEnd)]
	JumpEnd(T![Ident]),
	#[atom(CssAtomSet::JumpNone)]
	JumpNone(T![Ident]),
	#[atom(CssAtomSet::JumpBoth)]
	JumpBoth(T![Ident]),
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "ease-in-out");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,0.25,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,0.5 25% 75%,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.25,0.1,0.25,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.1,-0.6,0.2,0)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0,0,1,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "steps(4,end)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "steps(10,jump-both)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,0.25,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.1 -0.6 0.2 0)");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(var(--a),1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(calc(0.1 + 0.1),0.1,0.25,var(--y))");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "steps(calc(2 + 2),end)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, EasingFunction, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, EasingFunction, "linear()");
		assert_parse_error!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.1, red, 1.0, green)");
	}
}
