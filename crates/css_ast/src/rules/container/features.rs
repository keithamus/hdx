use super::super::prelude::*;
use crate::{types::Ratio, units::Length};
use css_parse::{Box, discrete_feature, ranged_feature};

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Width)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WidthContainerFeature{CssAtomSet::Width | CssAtomSet::MinWidth | CssAtomSet::MaxWidth, Length}
);

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Height)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum HeightContainerFeature{CssAtomSet::Height | CssAtomSet::MinHeight | CssAtomSet::MaxHeight, Length}
);

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::InlineSize)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum InlineSizeContainerFeature{CssAtomSet::InlineSize | CssAtomSet::MinInlineSize | CssAtomSet::MaxInlineSize, Length}
);

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::BlockSize)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum BlockSizeContainerFeature{CssAtomSet::BlockSize | CssAtomSet::MinBlockSize | CssAtomSet::MaxBlockSize, Length}
);

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::AspectRatio)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum AspectRatioContainerFeature<'a>{CssAtomSet::AspectRatio | CssAtomSet::MinAspectRatio | CssAtomSet::MaxAspectRatio, Ratio<'a>}
);

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum OrientationContainerFeatureKeyword {
	#[atom(CssAtomSet::Portrait)]
	Portrait(T![Ident]),
	#[atom(CssAtomSet::Landscape)]
	Landscape(T![Ident]),
}

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Orientation)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum OrientationContainerFeature{CssAtomSet::Orientation, OrientationContainerFeatureKeyword}
);

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum StyleQuery<'a> {
	Is(StyleFeature<'a>),
	Not(T![Ident], StyleFeature<'a>),
	And(Vec<'a, (StyleFeature<'a>, Option<T![Ident]>)>),
	Or(Vec<'a, (StyleFeature<'a>, Option<T![Ident]>)>),
}

impl<'a> Peek<'a> for StyleQuery<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen, Kind::Ident]);
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum StyleFeature<'a> {
	Declaration(Box<'a, Declaration<'a, StyleValue<'a>, CssMetadata>>),
	CustomProperty(T![Ident]),
}

impl<'a> Parse<'a> for StyleFeature<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if c == Kind::Ident && c.token().is_dashed_ident() && p.peek_n(2) != Kind::Colon {
			return Ok(Self::CustomProperty(p.parse::<T![Ident]>()?));
		}
		let decl = p.parse::<Declaration<'a, StyleValue<'a>, CssMetadata>>()?;
		Ok(Self::Declaration(Box::new_in(p.alloc(), decl)))
	}
}

impl<'a> FeatureConditionList<'a> for StyleQuery<'a> {
	type FeatureCondition = StyleFeature<'a>;
	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: Self::FeatureCondition) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: Self::FeatureCondition) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (Self::FeatureCondition, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (Self::FeatureCondition, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for StyleQuery<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ScrollStateQuery<'a> {
	Is(ScrollStateFeature),
	Not(T![Ident], ScrollStateFeature),
	And(Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>),
	Or(Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>),
}

impl<'a> Peek<'a> for ScrollStateQuery<'a> {
	const PEEK_KINDSET: KindSet = ScrollStateFeature::PEEK_KINDSET.add(Kind::Ident);
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<ScrollStateFeature>::peek(p, c) || (c == Kind::Ident && p.equals_atom(c, &CssAtomSet::Not))
	}
}

impl<'a> FeatureConditionList<'a> for ScrollStateQuery<'a> {
	type FeatureCondition = ScrollStateFeature;
	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: ScrollStateFeature) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: ScrollStateFeature) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for ScrollStateQuery<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ScrollStateFeature {
	Stuck(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Ident],
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![:]>,
		Option<StuckScrollStateFeatureKeyword>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![')']>,
	),
	Snapped(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Ident],
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![:]>,
		Option<SnappedScrollStateFeatureKeyword>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![')']>,
	),
	Scrollable(
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Ident],
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![:]>,
		Option<ScrollableScrollStateFeatureKeyword>,
		#[cfg_attr(feature = "visitable", visit(skip))]
		#[semantic_eq(skip)]
		Option<T![')']>,
	),
}

impl<'a> Peek<'a> for ScrollStateFeature {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::LeftParen]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Bare form: `stuck: top`
		if c == Kind::Ident
			&& matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::Stuck | CssAtomSet::Snapped | CssAtomSet::Scrollable)
		{
			return true;
		}
		// Paren-wrapped form: `(stuck: top)`
		if c == Kind::LeftParen {
			let c2 = p.peek_n(2);
			return c2 == Kind::Ident
				&& matches!(
					p.to_atom::<CssAtomSet>(c2),
					CssAtomSet::Stuck | CssAtomSet::Snapped | CssAtomSet::Scrollable
				);
		}
		false
	}
}

impl<'a> Parse<'a> for ScrollStateFeature {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let open = p.parse_if_peek::<T!['(']>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let colon = p.parse_if_peek::<T![:]>()?;
		match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::Stuck => {
				let value = if colon.is_some() { Some(p.parse::<StuckScrollStateFeatureKeyword>()?) } else { None };
				let close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
				Ok(Self::Stuck(open, ident, colon, value, close))
			}
			CssAtomSet::Snapped => {
				let value = if colon.is_some() { Some(p.parse::<SnappedScrollStateFeatureKeyword>()?) } else { None };
				let close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
				Ok(Self::Snapped(open, ident, colon, value, close))
			}
			CssAtomSet::Scrollable => {
				let value =
					if colon.is_some() { Some(p.parse::<ScrollableScrollStateFeatureKeyword>()?) } else { None };
				let close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
				Ok(Self::Scrollable(open, ident, colon, value, close))
			}
			_ => Err(Diagnostic::new(c, Diagnostic::unexpected_ident))?,
		}
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ScrollableScrollStateFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::X)]
	X(T![Ident]),
	#[atom(CssAtomSet::Y)]
	Y(T![Ident]),
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Inline)]
	Inline(T![Ident]),
	#[atom(CssAtomSet::Discrete)]
	Discrete(T![Ident]),
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SnappedScrollStateFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::X)]
	X(T![Ident]),
	#[atom(CssAtomSet::Y)]
	Y(T![Ident]),
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Inline)]
	Inline(T![Ident]),
	#[atom(CssAtomSet::Both)]
	Both(T![Ident]),
	#[atom(CssAtomSet::Discrete)]
	Discrete(T![Ident]),
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum StuckScrollStateFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::Discrete)]
	Discrete(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(width:360px)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(100px<=width)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(100px<=width>1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(height:360px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(100px<=height)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(100px<=height>1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(min-width:360px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(max-height:360px)");
		assert_parse!(CssAtomSet::ATOMS, InlineSizeContainerFeature, "(min-inline-size:30em)");
		assert_parse!(CssAtomSet::ATOMS, BlockSizeContainerFeature, "(max-block-size:30em)");
		assert_parse!(CssAtomSet::ATOMS, AspectRatioContainerFeature, "(min-aspect-ratio:16/9)");
		assert_parse!(CssAtomSet::ATOMS, StyleQuery, "--x");
		assert_parse!(CssAtomSet::ATOMS, StyleQuery, "--x:10px");
		assert_parse!(CssAtomSet::ATOMS, StyleQuery, "--x and --y:20px");
		assert_parse!(CssAtomSet::ATOMS, ScrollStateQuery, "stuck:top");
		assert_parse!(CssAtomSet::ATOMS, ScrollStateQuery, "scrollable:y and snapped:block");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WidthContainerFeature, "(min-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthContainerFeature, "(width: 1%)");
	}
}
