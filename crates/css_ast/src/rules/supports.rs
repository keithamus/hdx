use crate::{properties::Property, selector::ComplexSelector, stylesheet::Rule, Visit, Visitable};
use bumpalo::collections::Vec;
use css_lexer::{Cursor, Span};
use css_parse::{
	diagnostics, function_set, syntax::ComponentValues, AtRule, Build, ConditionKeyword, CursorSink,
	FeatureConditionList, Parse, Parser, Result as ParserResult, RuleList, ToCursors, T,
};
use hdx_proc_macro::visit;

// https://drafts.csswg.org/css-conditional-3/#at-supports
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct SupportsRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub condition: SupportsCondition<'a>,
	pub block: SupportsRuleBlock<'a>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-supports
///
/// ```md
/// <general-enclosed>
///  │├─╮─ <function-token> ─╭─╮─ <any-value> ─╭─ ")" ─┤│
///     ╰─ "(" ──────────────╯ ╰───────────────╯
///
///
/// <supports-in-parens>
///  │├─╮─ "(" ─ <supports-condition> ─ ")" ─╭──┤│
///     ├─────── <supports-feature> ─────────┤
///     ╰─────── <general-enclosed> ─────────╯
///
/// <supports-feature>
///  │├─ <supports-decl> ──┤│
///
/// <supports-feature>
///  │├─ "(" ─ <declaration> ─ ")" ─┤│
///
///
/// <container-condition> = [ <container-name>? <container-query>? ]!
/// <container-name> = <custom-ident>
/// <container-query> = not <query-in-parens>
///                   | <query-in-parens> [ [ and <query-in-parens> ]* | [ or <query-in-parens> ]* ]
/// <query-in-parens> = ( <container-query> )
///                   | ( <size-feature> )
///                   | style( <style-query> )
///                   | scroll-state( <scroll-state-query> )
///                   | <general-enclosed>
impl<'a> Parse<'a> for SupportsRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, condition, block) = Self::parse_at_rule(p)?;
		if let Some(condition) = condition {
			Ok(Self { at_keyword, condition, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for SupportsRule<'a> {
	const NAME: Option<&'static str> = Some("supports");
	type Prelude = SupportsCondition<'a>;
	type Block = SupportsRuleBlock<'a>;
}

impl<'a> ToCursors for SupportsRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.condition, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for SupportsRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_supports_rule(self);
		Visitable::accept(&self.condition, v);
		Visitable::accept(&self.block, v);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SupportsRuleBlock<'a> {
	pub open: T!['{'],
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for SupportsRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for SupportsRuleBlock<'a> {
	type Rule = Rule<'a>;
}

impl<'a> ToCursors for SupportsRuleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = &self.close {
			s.append(close.into());
		}
	}
}

impl<'a> Visitable<'a> for SupportsRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(ConditionKeyword, SupportsFeature<'a>),
	And(Vec<'a, (SupportsFeature<'a>, Option<ConditionKeyword>)>),
	Or(Vec<'a, (SupportsFeature<'a>, Option<ConditionKeyword>)>),
}

impl<'a> FeatureConditionList<'a> for SupportsCondition<'a> {
	type FeatureCondition = SupportsFeature<'a>;
	fn build_is(feature: SupportsFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: SupportsFeature<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (SupportsFeature<'a>, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (SupportsFeature<'a>, Option<ConditionKeyword>)>) -> Self {
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

impl<'a> ToCursors for SupportsCondition<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Is(feature) => ToCursors::to_cursors(feature, s),
			Self::Not(keyword, feature) => {
				s.append(keyword.into());
				ToCursors::to_cursors(feature, s)
			}
			Self::And(features) => {
				for (feature, keyword) in features {
					ToCursors::to_cursors(feature, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
			Self::Or(features) => {
				for (feature, keyword) in features {
					ToCursors::to_cursors(feature, s);
					if let Some(keyword) = keyword {
						s.append(keyword.into());
					}
				}
			}
		}
	}
}

impl<'a> Visitable<'a> for SupportsCondition<'a> {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SupportsFeature<'a> {
	FontTech(Option<T!['(']>, T![Function], ComponentValues<'a>, T![')'], Option<T![')']>),
	FontFormat(Option<T!['(']>, T![Function], ComponentValues<'a>, T![')'], Option<T![')']>),
	Selector(Option<T!['(']>, T![Function], ComplexSelector<'a>, T![')'], Option<T![')']>),
	Property(T!['('], Property<'a>, Option<T![')']>),
}

function_set!(SupportsFeatureKeyword { FontTech: "font-tech", FontFormat: "font-format", Selector: "selector" });

impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse_if_peek::<T!['(']>()?;
		if p.peek::<T![Function]>() {
			let keyword = p.parse::<SupportsFeatureKeyword>()?;
			let c: Cursor = keyword.into();
			let function = <T![Function]>::build(p, c);
			match keyword {
				SupportsFeatureKeyword::Selector(_) => {
					let selector = p.parse::<ComplexSelector>()?;
					// End function
					let close = p.parse::<T![')']>()?;
					let open_close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
					Ok(Self::Selector(open, function, selector, close, open_close))
				}
				SupportsFeatureKeyword::FontTech(_) => {
					todo!();
				}
				SupportsFeatureKeyword::FontFormat(_) => {
					todo!();
				}
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

impl<'a> ToCursors for SupportsFeature<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
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

impl<'a> Visitable<'a> for SupportsFeature<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		match self {
			Self::FontTech(_, _, _, _, _) => todo!(),
			Self::FontFormat(_, _, _, _, _) => todo!(),
			Self::Selector(_, _, selector, _, _) => Visitable::accept(selector, v),
			Self::Property(_, property, _) => Visitable::accept(property, v),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SupportsRule>(), 512);
		assert_eq!(std::mem::size_of::<SupportsCondition>(), 432);
		assert_eq!(std::mem::size_of::<SupportsRuleBlock>(), 64);
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
