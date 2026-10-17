#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};

use super::prelude::*;

mod features;
pub use features::*;

/// <https://drafts.csswg.org/css-contain-3/#container-rule>
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.container"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Container)]
pub struct ContainerRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Container)]
	pub name: T![AtKeyword],
	pub prelude: ContainerConditionList<'a>,
	#[metadata(block)]
	pub block: ContainerRulesBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for ContainerRule<'a> {
	const NODE_ID: NodeId = NodeId::ContainerRule;
}

#[node]
#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ContainerRulesBlock<'a>(pub Block<'a, StyleValue<'a>, Rule<'a>, CssMetadata>);

#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ContainerConditionList<'a>(pub CommaSeparated<'a, ContainerCondition<'a>, 1>);

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ContainerCondition<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: Option<T![Ident]>,
	pub condition: Option<ContainerQuery<'a>>,
}

impl<'a> Peek<'a> for ContainerCondition<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::LeftParen, Kind::Function]);
}

impl<'a> Parse<'a> for ContainerCondition<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut name = None;
		let c = p.peek_n(1);
		if c == Kind::Ident {
			match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::None | CssAtomSet::And | CssAtomSet::Not | CssAtomSet::Or => {}
				_ => {
					name = Some(p.parse::<T![Ident]>()?);
				}
			}
		}
		let condition =
			if name.is_none() { Some(p.parse::<ContainerQuery>()?) } else { p.parse_if_peek::<ContainerQuery>()? };
		Ok(Self { name, condition })
	}
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContainerQuery<'a> {
	Is(ContainerFeature<'a>),
	Not(T![Ident], ContainerFeature<'a>),
	And(Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>),
	Or(Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>),
}

impl<'a> Peek<'a> for ContainerQuery<'a> {
	const PEEK_KINDSET: KindSet = ContainerFeature::PEEK_KINDSET.combine(<T![Ident]>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<ContainerFeature>::peek(p, c) || (<T![Ident]>::peek(p, c) && p.to_atom::<CssAtomSet>(c) == CssAtomSet::Not)
	}
}

impl<'a> Parse<'a> for ContainerQuery<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

impl<'a> FeatureConditionList<'a> for ContainerQuery<'a> {
	type FeatureCondition = ContainerFeature<'a>;
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
	fn build_is(feature: ContainerFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: ContainerFeature<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

macro_rules! container_feature {
	( $($name: ident($typ: ty): $pat: pat,)+ ) => {
		#[node]
		#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
		#[derive(csskit_derives::NodeWithMetadata)]
		pub enum ContainerFeature<'a> {
			$($name($typ),)+
			Style(
				#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
				StyleQuery<'a>,
				#[cfg_attr(feature = "visitable", visit(skip))] #[semantic_eq(skip)] T![')'],
			),
			ScrollState(
				#[cfg_attr(feature = "visitable", visit(skip))] T![Function],
				ScrollStateQuery<'a>,
				#[cfg_attr(feature = "visitable", visit(skip))] #[semantic_eq(skip)] T![')'],
			),
		}
	}
}

apply_container_features!(container_feature);

impl<'a> Peek<'a> for ContainerFeature<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen, Kind::Function]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c2 = p.peek_n(2);
		(c == Kind::LeftParen && (c2 == Kind::Ident || <crate::QueryValue<'a, crate::Length>>::peek(p, c2)))
			|| (c == Kind::Function
				&& matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::Style | CssAtomSet::ScrollState))
	}
}

impl<'a> Parse<'a> for ContainerFeature<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![Function]>() {
			let function = p.parse::<T![Function]>()?;
			return match p.to_atom::<CssAtomSet>(function.into()) {
				CssAtomSet::Style => {
					let query = p.parse::<StyleQuery>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::Style(function, query, close))
				}
				CssAtomSet::ScrollState => {
					let query = p.parse::<ScrollStateQuery>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::ScrollState(function, query, close))
				}
				_ => Err(Diagnostic::new(function.into(), Diagnostic::unexpected_function))?,
			};
		}
		let mut c = p.peek_n(2);
		macro_rules! match_feature {
			( $($name: ident($typ: ty): $pat: pat,)+ ) => {
				// Only peek at the token as the underlying media feature parser needs to parse the leading keyword.
				{
					match p.to_atom::<CssAtomSet>(c) {
						$($pat => {
							let value = <$typ>::parse(p)?;
							Self::$name(value)
						},)+
						_ => Err(Diagnostic::new(c, Diagnostic::unexpected))?
					}
				}
			}
		}
		if c == Kind::Ident {
			Ok(apply_container_features!(match_feature))
		} else {
			// Styles like (1em < width < 1em) or (1em <= width <= 1em)
			c = css_parse::peek_ranged_feature_name(p);
			Ok(apply_container_features!(match_feature))
		}
	}
}

macro_rules! apply_container_features {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/css-conditional-5/#container-features
			Width(WidthContainerFeature<'a>): CssAtomSet::Width | CssAtomSet::MinWidth | CssAtomSet::MaxWidth,
			Height(HeightContainerFeature<'a>): CssAtomSet::Height | CssAtomSet::MinHeight | CssAtomSet::MaxHeight,
			InlineSize(InlineSizeContainerFeature<'a>): CssAtomSet::InlineSize | CssAtomSet::MinInlineSize | CssAtomSet::MaxInlineSize,
			BlockSize(BlockSizeContainerFeature<'a>): CssAtomSet::BlockSize | CssAtomSet::MinBlockSize | CssAtomSet::MaxBlockSize,
			AspectRatio(AspectRatioContainerFeature<'a>): CssAtomSet::AspectRatio | CssAtomSet::MinAspectRatio | CssAtomSet::MaxAspectRatio,
			Orientation(OrientationContainerFeature): CssAtomSet::Orientation,
		}
	};
}
use apply_container_features;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContainerQuery, "(width:2px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(width:2px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(inline-size>30em)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(1em<width<1em)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(width > 400px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "--container");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "--container (width > 400px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerConditionList, "(width > 400px), --container (width > 400px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo{}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (width:2px){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (10em<width<10em){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (width:2px){body{color:black}}");
		// Style queries
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "style(--x:10px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "style(--x: 10px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "style(--x)");
		assert_parse!(CssAtomSet::ATOMS, ContainerQuery, "style(--x:10px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerQuery, "style(--x)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "style(--x:10px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "style(--x: 10px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "style(--x)");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(--x:10px){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(--x: 10px){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(--x){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(--x:10px){body{color:green}}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(--x: 10px){body{color:green}}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo style(--x:10px){}");
		// Scroll-state queries
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "scroll-state(stuck:top)");
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "scroll-state(scrollable:y and snapped:block)");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container sticky scroll-state(stuck: top){}");
	}

	#[test]
	fn test_math_function_values() {
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(width>calc(30em - 1px))");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(calc(10em + 1px)<inline-size<=max(40em,50vw))");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (min-width:calc(48rem - 0.02px)){}");
	}

	#[test]
	fn test_style_queries() {
		// Basic style query
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "style(color:red)");
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "style(--my-var:10px)");
		// Style query in container condition
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "style(color:red)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "style(--my-var:10px)");
		// Style query in full container rule
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(color:red){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(--my-var:10px){}");
		// Named container with style query
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container card style(color:red){}");
		// Style query with rules inside
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container style(color:red){body{color:black}}");
	}

	#[test]
	fn test_scroll_state_queries() {
		// Basic scroll-state query
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "scroll-state(scrollable:top)");
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "scroll-state(snapped:x)");
		assert_parse!(CssAtomSet::ATOMS, ContainerFeature, "scroll-state(stuck:top)");
		// Scroll-state query in container condition
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "scroll-state(stuck:top)");
		// Scroll-state query in full container rule
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container scroll-state(stuck:top){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container scroll-state(snapped:x){}");
		// Named container with scroll-state query
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container nav scroll-state(stuck:top){}");
		// Scroll-state query with rules inside
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container scroll-state(stuck:top){.item{opacity:0.5}}");
	}
}
