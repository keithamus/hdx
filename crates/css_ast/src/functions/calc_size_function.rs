use super::prelude::*;
use crate::{CalcSum, LengthPercentage};

/// <https://drafts.csswg.org/css-values-5/#calc-size>
///
/// ```text,ignore
/// <calc-size()> = calc-size( <calc-size-basis>, <calc-sum> )
/// ```
///
/// The `<size-keyword>` production matches any sizing keywords allowed in the context.
/// For example, in width, it matches auto, min-content, stretch, etc.
///
/// The trailing `<calc-sum>` may use the `size` keyword, which refers to the basis.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CalcSizeFunction<'a> {
	#[atom(CssAtomSet::CalcSize)]
	pub name: T![Function],
	pub basis: Box<'a, CalcSizeBasis<'a>>,
	#[semantic_eq(skip)]
	pub comma: T![,],
	pub calculation: Box<'a, CalcSum<'a, CalcSizeOperand>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-values-5/#typedef-calc-size-basis>
///
/// ```text,ignore
/// <calc-size-basis> = [ <size-keyword> | <calc-size()> | any | <calc-sum> ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CalcSizeBasis<'a> {
	CalcSize(Box<'a, CalcSizeFunction<'a>>),
	SizeKeyword(SizeKeyword),
	#[atom(CssAtomSet::Any)]
	Any(Ident),
	Calculation(CalcSum<'a, LengthPercentage>),
}

/// The sizing keywords allowed as a `<calc-size-basis>`.
///
/// ```text,ignore
/// <size-keyword> = auto | min-content | max-content | fit-content | stretch | contain
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SizeKeyword {
	#[atom(CssAtomSet::Auto)]
	Auto(Ident),
	#[atom(CssAtomSet::MinContent)]
	MinContent(Ident),
	#[atom(CssAtomSet::MaxContent)]
	MaxContent(Ident),
	#[atom(CssAtomSet::FitContent)]
	FitContent(Ident),
	#[atom(CssAtomSet::Stretch)]
	Stretch(Ident),
	#[atom(CssAtomSet::Contain)]
	Contain(Ident),
	#[atom(CssAtomSet::_WebkitMinContent)]
	WebkitMinContent(Ident),
	#[atom(CssAtomSet::_WebkitMaxContent)]
	WebkitMaxContent(Ident),
	#[atom(CssAtomSet::_MozMinContent)]
	MozMinContent(Ident),
	#[atom(CssAtomSet::_MozMaxContent)]
	MozMaxContent(Ident),
}

/// A `<calc-value>` inside the `<calc-sum>` of a `calc-size()`, which additionally allows the
/// `size` keyword standing in for the function's basis.
///
/// <https://drafts.csswg.org/css-values-5/#calc-size>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CalcSizeOperand {
	#[atom(CssAtomSet::Size)]
	Size(Ident),
	LengthPercentage(LengthPercentage),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(any, size)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(auto, size)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(fit-content, size/2)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(max-content, size + 20px)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(100px, size*2)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(50%, size)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(calc-size(auto, size), size/2)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(-webkit-min-content, size)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(any, 100px)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(var(--basis), size)");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(any, var(--x))");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(auto, size*var(--n))");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(auto, calc(size/2))");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(auto, min(size, 100px))");
		assert_parse!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(env(--basis), size)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(auto)");
		assert_parse_error!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(nonsense, size)");
		assert_parse_error!(CssAtomSet::ATOMS, CalcSizeFunction, "calc-size(size, size)");
	}
}
