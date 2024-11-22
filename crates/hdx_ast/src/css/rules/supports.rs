use std::ops::Deref;

use crate::{
	css::{properties::Property, selector::ComplexSelector, stylesheet::Rule},
	syntax::ComponentValues,
};
use hdx_atom::atom;
use hdx_lexer::Span;
use hdx_parser::{
	diagnostics, AtRule, ConditionalAtRule, Parse, Parser, Result as ParserResult, RuleList, Spanned, Vec, T,
};
use hdx_writer::{write_css, CssWriter, OutputOption, Result as WriterResult, WriteCss};
use smallvec::SmallVec;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(And, atom!("and"));
	custom_keyword!(Or, atom!("or"));
	custom_keyword!(Not, atom!("not"));
}

// https://drafts.csswg.org/css-conditional-3/#at-supports
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Supports<'a> {
	pub condition: Spanned<SupportsCondition<'a>>,
	pub rules: Spanned<SupportsRules<'a>>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-supports
impl<'a> Parse<'a> for Supports<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		match Self::parse_at_rule(p, Some(atom!("supports")))? {
			(Some(condition), Some(rules)) => Ok(Self { condition, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(Span::new(start, p.offset())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?,
		}
	}
}

impl<'a> AtRule<'a> for Supports<'a> {
	type Prelude = SupportsCondition<'a>;
	type Block = SupportsRules<'a>;
}

impl<'a> WriteCss<'a> for Supports<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if !sink.can_output(OutputOption::RedundantRules) && self.rules.node.0.is_empty() {
			return Ok(());
		}
		sink.write_char('@')?;
		atom!("supports").write_css(sink)?;
		if matches!(self.condition.node, SupportsCondition::Not(_)) {
			sink.write_char(' ')?;
		} else {
			sink.write_whitespace()?;
		}
		self.condition.write_css(sink)?;
		sink.write_whitespace()?;
		sink.write_char('{')?;
		sink.write_newline()?;
		sink.indent();
		self.rules.write_css(sink)?;
		sink.write_newline()?;
		sink.dedent();
		sink.write_char('}')?;
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SupportsRules<'a>(pub Vec<'a, Spanned<Rule<'a>>>);

impl<'a> Parse<'a> for SupportsRules<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_rule_list(p)?))
	}
}

impl<'a> RuleList<'a> for SupportsRules<'a> {
	type Rule = Rule<'a>;
}

impl<'a> WriteCss<'a> for SupportsRules<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut rules = self.0.iter().peekable();
		while let Some(rule) = rules.next() {
			rule.write_css(sink)?;
			if rules.peek().is_some() {
				sink.write_newline()?;
			}
		}
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(Box<SupportsCondition<'a>>),
	And(SmallVec<[SupportsFeature<'a>; 2]>),
	Or(SmallVec<[SupportsFeature<'a>; 2]>),
}

impl<'a> ConditionalAtRule<'a> for SupportsCondition<'a> {
	type Feature = SupportsFeature<'a>;
	fn create_is(feature: SupportsFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn create_not(feature: SupportsCondition<'a>) -> Self {
		Self::Not(Box::new(feature))
	}
	fn create_and(feature: SmallVec<[SupportsFeature<'a>; 2]>) -> Self {
		Self::And(feature)
	}
	fn create_or(feature: SmallVec<[SupportsFeature<'a>; 2]>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for SupportsCondition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Function]>().is_some() {
			return Ok(Self::Is(p.parse::<SupportsFeature>()?));
		}
		Self::parse_condition(p)
	}
}

impl<'a> WriteCss<'a> for SupportsCondition<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Is(feature) => feature.write_css(sink),
			Self::Not(condition) => {
				match condition.deref() {
					SupportsCondition::Is(_) => {
						write_css!(sink, atom!("not"), (), condition)
					}
					_ => write_css!(sink, atom!("not"), (), '(', condition, ')'),
				}
				Ok(())
			}
			Self::And(features) => {
				let mut first = true;
				let mut iter = features.into_iter().peekable();
				while let Some(feature) = iter.next() {
					if first {
						first = false;
					} else {
						atom!("and").write_css(sink)?;
						sink.write_whitespace()?;
					}
					feature.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_whitespace()?;
					}
				}
				Ok(())
			}
			Self::Or(features) => {
				let mut first = true;
				let mut iter = features.into_iter().peekable();
				while let Some(feature) = iter.next() {
					if first {
						first = false;
					} else {
						atom!("or").write_css(sink)?;
						sink.write_whitespace()?;
					}
					feature.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_whitespace()?;
					}
				}
				Ok(())
			}
		}
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SupportsFeature<'a> {
	FontTech(ComponentValues<'a>),
	FontFormat(ComponentValues<'a>),
	Selector(ComplexSelector<'a>),
	Property(Property<'a>),
}

impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let parens = p.parse_if_peek::<T![LeftParen]>()?.is_some();
		if let Some(token) = p.peek::<T![Function]>() {
			match p.parse_atom_lower(token) {
				atom!("selector") => {
					p.hop(token);
					let selector = p.parse::<ComplexSelector>()?;
					// End function
					p.parse::<T![RightParen]>()?;
					if parens {
						p.parse::<T![RightParen]>()?;
					}
					Ok(Self::Selector(selector))
				}
				atom!("font-tech") => {
					todo!();
				}
				atom!("font-format") => {
					todo!();
				}
				atom => Err(diagnostics::UnexpectedFunction(atom, token.span()))?,
			}
		} else {
			if !parens {
				let token = p.peek::<T![Any]>().unwrap();
				Err(diagnostics::Unexpected(token, token.span()))?;
			}
			let property = p.parse::<Property>()?;
			p.parse::<T![RightParen]>()?;
			Ok(Self::Property(property))
		}
	}
}

impl<'a> WriteCss<'a> for SupportsFeature<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::FontTech(_) => todo!(),
			Self::FontFormat(_) => todo!(),
			Self::Selector(selector) => write_css!(sink, atom!("selector"), '(', selector, ')'),
			Self::Property(property) => write_css!(sink, '(', property, ')'),
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Supports, 344);
		assert_size!(SupportsCondition, 296);
		assert_size!(SupportsRules, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Supports, "@supports (color: black) {\n\n}");
		assert_parse!(Supports, "@supports (width: 1px) {\n\tbody {\n\t\twidth: 1px;\n\t}\n}");
		assert_parse!(Supports, "@supports not (width: 1--foo) {\n\n}");
		assert_parse!(Supports, "@supports (width: 1--foo) or (width: 1foo) {\n\n}");
		assert_parse!(Supports, "@supports (width: 1--foo) and (width: 1foo) {\n\n}");
		assert_parse!(Supports, "@supports (width: 100vw) {\n\tbody {\n\t\twidth: 100vw;\n\t}\n}");
		assert_parse!(Supports, "@supports not ((text-align-last: justify) or (-moz-text-align-last: justify)) {\n\n}");
		assert_parse!(
			Supports,
			"@supports ((position: -webkit-sticky) or (position: sticky)) {}",
			"@supports (position: -webkit-sticky) or (position: sticky) {\n\n}"
		);

		assert_parse!(Supports, "@supports selector(h2 > p) {\n\n}");
		assert_parse!(Supports, "@supports (selector(h2 > p)) {}", "@supports selector(h2 > p) {\n\n}");
		assert_parse!(Supports, "@supports not selector(h2 > p) {\n\n}");
		assert_parse!(Supports, "@supports not (selector(h2 > p)) {}", "@supports not selector(h2 > p) {\n\n}");
	}

	#[test]
	fn test_minify() {
		assert_minify!(
			Supports,
			"@supports (width: 1px) { body { width:1px; } }",
			"@supports(width:1px){body{width:1px}}"
		);
		assert_minify!(
			Supports,
			"@supports not (width: 1--foo) { a { width:1px } }",
			"@supports not(width:1--foo){a{width:1px}}"
		);
		assert_minify!(Supports, "@supports (color: black) {}", "");
	}
}
