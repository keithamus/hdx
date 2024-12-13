use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::{Cursor, Kind, Span};
use hdx_parser::{
	diagnostics, keyword_typedef, AtRule, Build, ConditionalAtRule, CursorStream, Is, Parse, Parser, Peek, PreludeList,
	Result as ParserResult, RuleList, ToCursors, T,
};

use crate::css::stylesheet::Rule;

mod features;
use features::*;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(And, atom!("and"));
	custom_keyword!(Not, atom!("not"));
	custom_keyword!(Only, atom!("only"));
}

// https://drafts.csswg.org/css-contain-3/#container-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct ContainerRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub query: ContainerConditionList<'a>,
	pub block: ContainerRules<'a>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for ContainerRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, query, block) = Self::parse_at_rule(p, Some(atom!("container")))?;
		if let Some(query) = query {
			Ok(Self { at_keyword, query, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for ContainerRule<'a> {
	type Prelude = ContainerConditionList<'a>;
	type Block = ContainerRules<'a>;
}

impl<'a> ToCursors<'a> for ContainerRule<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.query, s);
		ToCursors::to_cursors(&self.block, s);
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

impl<'a> ToCursors<'a> for ContainerRules<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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

impl<'a> ToCursors<'a> for ContainerConditionList<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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
			match p.parse_atom_lower(c) {
				atom!("none") | atom!("and") | atom!("not") | atom!("or") => {}
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

impl<'a> ToCursors<'a> for ContainerCondition<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		if let Some(name) = &self.name {
			ToCursors::to_cursors(name, s);
		}
		if let Some(condition) = &self.condition {
			ToCursors::to_cursors(condition, s);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ContainerQuery<'a> {
	Is(ContainerFeature<'a>),
	Not(Box<ContainerQuery<'a>>),
	And(Vec<'a, ContainerFeature<'a>>),
	Or(Vec<'a, ContainerFeature<'a>>),
}

impl<'a> Peek<'a> for ContainerQuery<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<T![Function]>() || p.peek::<T![Ident]>()
	}
}

impl<'a> Parse<'a> for ContainerQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

impl<'a> ConditionalAtRule<'a> for ContainerQuery<'a> {
	type Feature = ContainerFeature<'a>;
	fn new_is(feature: ContainerFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn new_not(condition: ContainerQuery<'a>) -> Self {
		Self::Not(Box::new(condition))
	}
	fn new_and(feature: Vec<'a, ContainerFeature<'a>>) -> Self {
		Self::And(feature)
	}
	fn new_or(feature: Vec<'a, ContainerFeature<'a>>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> ToCursors<'a> for ContainerQuery<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::Is(c) => ToCursors::to_cursors(c, s),
			Self::Not(c) => ToCursors::to_cursors(c.as_ref(), s),
			Self::And(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
			Self::Or(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
		}
	}
}

macro_rules! container_feature {
	( $($name: ident($typ: ident): atom!($atom: tt),)+) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum ContainerFeature<'a> {
			$($name(T!['('], $typ, T![')']),)+
			Style(StyleQuery<'a>),
			ScrollState(ScrollStateQuery<'a>),
		}
	}
}

apply_size_feature!(container_feature);

impl<'a> Parse<'a> for ContainerFeature<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Function]>() {
			todo!();
		}
		let open = p.parse::<T![LeftParen]>()?;
		let checkpoint = p.checkpoint();
		if p.peek::<T![Ident]>() {
			let c = p.peek_n(1);
			macro_rules! match_feature {
				( $($name: ident($typ: ident): atom!($atom: tt),)+) => {
					// Only peek at the token as the underlying media feature parser needs to parse the leading atom.
					{
						match p.parse_atom_lower(c) {
							$(atom!($atom) => {
								let value = $typ::parse(p)?;
								let close = p.parse::<T![')']>()?;
								Self::$name(open, value, close)
							},)+
							atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
						}
					}
				}
			}
			let value = apply_size_feature!(match_feature);
			Ok(value)
		} else {
			let c: Cursor = p.parse::<T![Any]>()?.into();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> ToCursors<'a> for ContainerFeature<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		macro_rules! match_feature {
			( $($name: ident($typ: ident): atom!($atom: tt),)+) => {
				match self {
					$(Self::$name(open, c, close) => {
						s.append(open.into());
						ToCursors::to_cursors(c, s);
						s.append(close.into());
					},)+
					Self::Style(c) => ToCursors::to_cursors(c, s),
					Self::ScrollState(c) => ToCursors::to_cursors(c, s),
				}
			};
		}
		apply_size_feature!(match_feature)
	}
}

macro_rules! apply_size_feature {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/css-conditional-5/#container-features
			Width(WidthContainerFeature): atom!("width"),
			Height(HeightContainerFeature): atom!("height"),
			InlineSize(InlineSizeContainerFeature): atom!("inline-size"),
			BlockSize(BlockSizeContainerFeature): atom!("block-size"),
			AspectRatio(AspectRatioContainerFeature): atom!("aspect-ratio"),
			Orientation(OrientationContainerFeature): atom!("orientation"),
		}
	};
}
use apply_size_feature;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ContainerRule, 104);
		assert_size!(ContainerConditionList, 32);
		assert_size!(ContainerCondition, 376);
		assert_size!(ContainerQuery, 360);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ContainerCondition, "(width:2px)");
		assert_parse!(ContainerCondition, "(inline-size>30em)");
		assert_parse!(ContainerRule, "@container foo{}");
		assert_parse!(ContainerRule, "@container foo (width:2px){}");
		assert_parse!(ContainerRule, "@container foo (width:2px){body{color:black}}");
	}
}
