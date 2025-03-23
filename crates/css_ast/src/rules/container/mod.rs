use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, Span};
use css_parse::{
	diagnostics, keyword_set, AtRule, Build, ConditionKeyword, CursorSink, FeatureConditionList, Parse, Parser, Peek,
	PreludeList, Result as ParserResult, RuleList, ToCursors, T,
};
use csskit_proc_macro::visit;

use crate::{stylesheet::Rule, Visit, Visitable};

mod features;
pub use features::*;

// https://drafts.csswg.org/css-contain-3/#container-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct ContainerRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub query: ContainerConditionList<'a>,
	pub block: ContainerRules<'a>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for ContainerRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, query, block) = Self::parse_at_rule(p)?;
		if let Some(query) = query {
			Ok(Self { at_keyword, query, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for ContainerRule<'a> {
	const NAME: Option<&'static str> = Some("container");
	type Prelude = ContainerConditionList<'a>;
	type Block = ContainerRules<'a>;
}

impl<'a> ToCursors for ContainerRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.query, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for ContainerRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_container_rule(self);
		for condition in &self.query.0 {
			Visitable::accept(condition, v);
		}
		for rule in &self.block.rules {
			Visitable::accept(rule, v);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ContainerRules<'a> {
	pub open: T!['{'],
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for ContainerRules<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for ContainerRules<'a> {
	type Rule = Rule<'a>;
}

impl<'a> ToCursors for ContainerRules<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ContainerConditionList<'a>(pub Vec<'a, ContainerCondition<'a>>);

impl<'a> PreludeList<'a> for ContainerConditionList<'a> {
	type PreludeItem = ContainerCondition<'a>;
}

impl<'a> Parse<'a> for ContainerConditionList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> ToCursors for ContainerConditionList<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for query in &self.0 {
			ToCursors::to_cursors(query, s);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ContainerCondition<'a> {
	pub name: Option<T![Ident]>,
	pub condition: Option<ContainerQuery<'a>>,
}

impl<'a> Parse<'a> for ContainerCondition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut name = None;
		let c = p.peek_n(1);
		if c == Kind::Ident {
			match p.parse_str_lower(c) {
				"none" | "and" | "not" | "or" => {}
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

impl<'a> ToCursors for ContainerCondition<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(name) = &self.name {
			s.append(name.into());
		}
		if let Some(condition) = &self.condition {
			ToCursors::to_cursors(condition, s);
		}
	}
}

impl<'a> Visitable<'a> for ContainerCondition<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		if let Some(condition) = &self.condition {
			Visitable::accept(condition, v);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ContainerQuery<'a> {
	Is(ContainerFeature<'a>),
	Not(ConditionKeyword, ContainerFeature<'a>),
	And(Vec<'a, (ContainerFeature<'a>, Option<ConditionKeyword>)>),
	Or(Vec<'a, (ContainerFeature<'a>, Option<ConditionKeyword>)>),
}

impl<'a> Peek<'a> for ContainerQuery<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) || <T![Ident]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for ContainerQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

impl<'a> FeatureConditionList<'a> for ContainerQuery<'a> {
	type FeatureCondition = ContainerFeature<'a>;
	fn build_is(feature: ContainerFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: ContainerFeature<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ContainerFeature<'a>, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ContainerFeature<'a>, Option<ConditionKeyword>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> ToCursors for ContainerQuery<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Is(c) => ToCursors::to_cursors(c, s),
			Self::Not(keyword, c) => {
				s.append(keyword.into());
				ToCursors::to_cursors(c, s)
			}
			Self::And(cs) => {
				for (c, keyword) in cs {
					ToCursors::to_cursors(c, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
			Self::Or(cs) => {
				for (c, keyword) in cs {
					ToCursors::to_cursors(c, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
		}
	}
}

impl<'a> Visitable<'a> for ContainerQuery<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => Visitable::accept(feature, v),
			Self::Not(_, feature) => Visitable::accept(feature, v),
			Self::And(features) => {
				for (feature, _) in features {
					Visitable::accept(feature, v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					Visitable::accept(feature, v);
				}
			}
		}
	}
}

macro_rules! container_feature {
	( $($name: ident($typ: ident): $str: tt,)+ ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum ContainerFeature<'a> {
			$($name($typ),)+
			Style(StyleQuery<'a>),
			ScrollState(ScrollStateQuery<'a>),
		}
	}
}

apply_container_features!(container_feature);

macro_rules! container_feature_keyword {
	( $($name: ident($typ: ident): $str: tt,)+) => {
		keyword_set!(ContainerFeatureKeyword {
			$($name: $str,)+
		});
	}
}
apply_container_features!(container_feature_keyword);

impl<'a> Parse<'a> for ContainerFeature<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Function]>() {
			todo!();
		}
		let mut c = p.peek_n(2);
		macro_rules! match_feature {
			( $($name: ident($typ: ident): $str: tt,)+) => {
				// Only peek at the token as the underlying media feature parser needs to parse the leading keyword.
				{
					if ContainerFeatureKeyword::peek(p, c) {
						match ContainerFeatureKeyword::build(p, c) {
							$(ContainerFeatureKeyword::$name(_) => {
								let value = $typ::parse(p)?;
								Self::$name(value)
							},)+
						}
					} else {
						Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
					}
				}
			}
		}
		if c == Kind::Ident {
			Ok(apply_container_features!(match_feature))
		} else {
			// Styles like (1em < width < 1em) or (1em <= width <= 1em)
			c = p.peek_n(3);
			if c != Kind::Ident {
				c = p.peek_n(4)
			}
			Ok(apply_container_features!(match_feature))
		}
	}
}

impl ToCursors for ContainerFeature<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		macro_rules! match_feature {
			( $($name: ident($typ: ident): $str: tt,)+) => {
				match self {
					$(Self::$name(c) => ToCursors::to_cursors(c, s),)+
					Self::Style(c) => ToCursors::to_cursors(c, s),
					Self::ScrollState(c) => ToCursors::to_cursors(c, s),
				}
			};
		}
		apply_container_features!(match_feature)
	}
}

impl<'a> Visitable<'a> for ContainerFeature<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		macro_rules! match_feature {
			( $($name: ident($typ: ident): $str: tt,)+) => {
				match self {
					$(Self::$name(f) => Visitable::accept(f, v),)+
					Self::Style(f) => Visitable::accept(f, v),
					Self::ScrollState(f) => Visitable::accept(f, v),
				}
			};
		}
		apply_container_features!(match_feature)
	}
}

macro_rules! apply_container_features {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/css-conditional-5/#container-features
			Width(WidthContainerFeature): "width",
			Height(HeightContainerFeature): "height",
			InlineSize(InlineSizeContainerFeature): "inline-size",
			BlockSize(BlockSizeContainerFeature): "block-size",
			AspectRatio(AspectRatioContainerFeature): "aspect-ratio",
			Orientation(OrientationContainerFeature): "orientation",
		}
	};
}
use apply_container_features;

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContainerRule>(), 112);
		assert_eq!(std::mem::size_of::<ContainerConditionList>(), 32);
		assert_eq!(std::mem::size_of::<ContainerCondition>(), 440);
		assert_eq!(std::mem::size_of::<ContainerQuery>(), 424);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ContainerQuery, "(width:2px)");
		assert_parse!(ContainerCondition, "(width:2px)");
		assert_parse!(ContainerCondition, "(inline-size>30em)");
		assert_parse!(ContainerCondition, "(1em<width<1em)");
		assert_parse!(ContainerRule, "@container foo{}");
		assert_parse!(ContainerRule, "@container foo (width:2px){}");
		assert_parse!(ContainerRule, "@container foo (10em<width<10em){}");
		assert_parse!(ContainerRule, "@container foo (width:2px){body{color:black}}");
	}
}
