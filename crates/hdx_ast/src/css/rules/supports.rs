use crate::{
	css::{properties::Property, selector::ComplexSelector, stylesheet::Rule},
	syntax::ComponentValues,
};
use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Span;
use hdx_parser::{
	diagnostics, AtRule, ConditionalAtRule, CursorStream, Parse, Parser, Result as ParserResult, RuleList, ToCursors, T,
};

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(And, atom!("and"));
	custom_keyword!(Or, atom!("or"));
	custom_keyword!(Not, atom!("not"));
}

// https://drafts.csswg.org/css-conditional-3/#at-supports
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct SupportsRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub condition: SupportsCondition<'a>,
	pub block: SupportsBlock<'a>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-supports
impl<'a> Parse<'a> for SupportsRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, condition, block) = Self::parse_at_rule(p, Some(atom!("supports")))?;
		if let Some(condition) = condition {
			Ok(Self { at_keyword, condition, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for SupportsRule<'a> {
	type Prelude = SupportsCondition<'a>;
	type Block = SupportsBlock<'a>;
}

impl<'a> ToCursors<'a> for SupportsRule<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.condition, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SupportsBlock<'a> {
	pub open: T!['{'],
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for SupportsBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for SupportsBlock<'a> {
	type Rule = Rule<'a>;
}

impl<'a> ToCursors<'a> for SupportsBlock<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.open.into());
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = &self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(Box<SupportsCondition<'a>>),
	And(Vec<'a, SupportsFeature<'a>>),
	Or(Vec<'a, SupportsFeature<'a>>),
}

impl<'a> ConditionalAtRule<'a> for SupportsCondition<'a> {
	type Feature = SupportsFeature<'a>;
	fn new_is(feature: SupportsFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn new_not(feature: SupportsCondition<'a>) -> Self {
		Self::Not(Box::new(feature))
	}
	fn new_and(feature: Vec<'a, SupportsFeature<'a>>) -> Self {
		Self::And(feature)
	}
	fn new_or(feature: Vec<'a, SupportsFeature<'a>>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for SupportsCondition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Function]>() || p.peek::<T!['(']>() {
			return Ok(Self::Is(p.parse::<SupportsFeature>()?));
		}
		Self::parse_condition(p)
	}
}

impl<'a> ToCursors<'a> for SupportsCondition<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::Is(feature) => ToCursors::to_cursors(feature, s),
			Self::Not(feature) => ToCursors::to_cursors(feature.as_ref(), s),
			Self::And(features) => {
				for feature in features {
					ToCursors::to_cursors(feature, s);
				}
			}
			Self::Or(features) => {
				for feature in features {
					ToCursors::to_cursors(feature, s);
				}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SupportsFeature<'a> {
	FontTech(Option<T!['(']>, T![Function], ComponentValues<'a>, T![')'], Option<T![')']>),
	FontFormat(Option<T!['(']>, T![Function], ComponentValues<'a>, T![')'], Option<T![')']>),
	Selector(Option<T!['(']>, T![Function], ComplexSelector<'a>, T![')'], Option<T![')']>),
	Property(T!['('], Property<'a>, Option<T![')']>),
}

impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse_if_peek::<T!['(']>()?;
		if p.peek::<T![Function]>() {
			let function = p.parse::<T![Function]>()?;
			let c = function.into();
			match p.parse_atom_lower(c) {
				atom!("selector") => {
					let selector = p.parse::<ComplexSelector>()?;
					// End function
					let close = p.parse::<T![')']>()?;
					let open_close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
					Ok(Self::Selector(open, function, selector, close, open_close))
				}
				atom!("font-tech") => {
					todo!();
				}
				atom!("font-format") => {
					todo!();
				}
				atom => Err(diagnostics::UnexpectedFunction(atom, c.into()))?,
			}
		} else if let Some(open) = open {
			let property = p.parse::<Property>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Property(open, property, close))
		} else {
			let c = p.peek_n(1);
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> ToCursors<'a> for SupportsFeature<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::FontTech(open, function, feature, close, open_close) => {
				if let Some(open) = open {
					s.append(open.into());
				}
				s.append(function.into());
				ToCursors::to_cursors(feature, s);
				s.append(close.into());
				if let Some(open_close) = open_close {
					s.append(open_close.into());
				}
			}
			Self::FontFormat(open, function, feature, close, open_close) => {
				if let Some(open) = open {
					s.append(open.into());
				}
				s.append(function.into());
				ToCursors::to_cursors(feature, s);
				s.append(close.into());
				if let Some(open_close) = open_close {
					s.append(open_close.into());
				}
			}
			Self::Selector(open, function, feature, close, open_close) => {
				if let Some(open) = open {
					s.append(open.into());
				}
				s.append(function.into());
				ToCursors::to_cursors(feature, s);
				s.append(close.into());
				if let Some(open_close) = open_close {
					s.append(open_close.into());
				}
			}
			Self::Property(open, feature, close) => {
				s.append(open.into());
				ToCursors::to_cursors(feature, s);
				if let Some(close) = close {
					s.append(close.into());
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(SupportsRule, 456);
		assert_size!(SupportsCondition, 384);
		assert_size!(SupportsBlock, 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SupportsRule, "@supports(color:black){}");
		assert_parse!(SupportsRule, "@supports(width:1px){body{width:1px}}");
		// assert_parse!(SupportsRule, "@supports not (width:1--foo){}");
		// assert_parse!(SupportsRule, "@supports(width: 1--foo) or (width: 1foo) {\n\n}");
		// assert_parse!(SupportsRule, "@supports(width: 1--foo) and (width: 1foo) {\n\n}");
		// assert_parse!(SupportsRule, "@supports(width: 100vw) {\n\tbody {\n\t\twidth: 100vw;\n\t}\n}");
		// assert_parse!(SupportsRule, "@supports not ((text-align-last: justify) or (-moz-text-align-last: justify)) {\n\n}");
		// assert_parse!(SupportsRule, "@supports((position:-webkit-sticky)or (position:sticky)) {}");
		// assert_parse!(SupportsRule, "@supports selector(h2 > p) {\n\n}");
		// assert_parse!(SupportsRule, "@supports(selector(h2 > p)) {}", "@supports selector(h2 > p) {\n\n}");
		// assert_parse!(SupportsRule, "@supports not selector(h2 > p) {\n\n}");
		// assert_parse!(SupportsRule, "@supports not (selector(h2 > p)) {}", "@supports not selector(h2 > p) {\n\n}");
	}
}
