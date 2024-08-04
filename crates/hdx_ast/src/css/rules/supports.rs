use crate::{css::stylesheet::Rule, syntax::SimpleBlock};
use hdx_atom::atom;
use hdx_lexer::Span;
use hdx_parser::{diagnostics, AtRule, Parse, Parser, Result as ParserResult, RuleList, Spanned, Token, Vec};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};

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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let start = parser.offset();
		match Self::parse_at_rule(parser, Some(atom!("supports")))? {
			(Some(condition), Some(rules)) => Ok(Self { condition, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(Span::new(start, parser.offset())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, parser.offset())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, parser.offset())))?,
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
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_rule_list(parser)?))
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
	Not(SupportsFeature<'a>),
	And(Vec<'a, SupportsFeature<'a>>),
	Or(Vec<'a, SupportsFeature<'a>>),
}

impl<'a> Parse<'a> for SupportsCondition<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if parser.peek::<kw::And>().is_some() {
			let mut features = parser.new_vec();
			loop {
				parser.parse::<kw::And>()?;
				features.push(parser.parse::<SupportsFeature>()?);
				if parser.peek::<kw::And>().is_some() {
					continue;
				} else {
					return Ok(Self::And(features));
				}
			}
		} else if parser.peek::<kw::Or>().is_some() {
			let mut features = parser.new_vec();
			loop {
				parser.parse::<kw::Or>()?;
				features.push(parser.parse::<SupportsFeature>()?);
				if parser.peek::<kw::Or>().is_some() {
					continue;
				} else {
					return Ok(Self::And(features));
				}
			}
		} else if let Some(token) = parser.peek::<kw::Not>() {
			parser.hop(token);
			return parser.parse::<SupportsFeature>().map(Self::Not);
		}

		// handle double parens
		let mut wrapped = true;
		let checkpoint = parser.checkpoint();
		parser.parse::<Token![LeftParen]>()?;
		if parser.peek::<Token![LeftParen]>().is_none() {
			wrapped = false;
			parser.rewind(checkpoint);
		}
		let feature = parser.parse::<SupportsFeature>()?;
		if parser.peek::<kw::And>().is_some() {
			let mut features = parser.new_vec();
			features.push(feature);
			loop {
				parser.parse::<kw::And>()?;
				features.push(parser.parse::<SupportsFeature>()?);
				if parser.peek::<kw::And>().is_none() {
					if wrapped {
						parser.parse::<Token![RightParen]>()?;
					}
					return Ok(Self::And(features));
				}
			}
		} else if parser.peek::<kw::Or>().is_some() {
			let mut features = parser.new_vec();
			features.push(feature);
			loop {
				parser.parse::<kw::Or>()?;
				features.push(parser.parse::<SupportsFeature>()?);
				if parser.peek::<kw::Or>().is_none() {
					if wrapped {
						parser.parse::<Token![RightParen]>()?;
					}
					return Ok(Self::Or(features));
				}
			}
		}
		if wrapped {
			parser.parse::<Token![RightParen]>()?;
		}
		parser.parse::<Token![RightParen]>()?;
		Ok(Self::Is(feature))
	}
}

impl<'a> WriteCss<'a> for SupportsCondition<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Is(feature) => feature.write_css(sink),
			Self::Not(feature) => {
				atom!("not").write_css(sink)?;
				sink.write_whitespace()?;
				feature.write_css(sink)
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
pub struct SupportsFeature<'a>(pub SimpleBlock<'a>);

impl<'a> Parse<'a> for SupportsFeature<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if parser.peek::<Token![LeftParen]>().is_none() {
			let token = parser.peek::<Token![Any]>().unwrap();
			Err(diagnostics::ExpectedOpenCurly(token, token.span()))?
		}
		Ok(Self(SimpleBlock::parse(parser)?))
	}
}

impl<'a> WriteCss<'a> for SupportsFeature<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.0.write_css(sink)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Supports, 96);
		assert_size!(SupportsCondition, 48);
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
		assert_parse!(Supports, "@supports ((position: -webkit-sticky) or (position: sticky)) {}");
	}

	#[test]
	fn test_minify() {
		assert_minify!(
			Supports,
			"@supports (width: 1px) { body { width:1px; } }",
			"@supports(width: 1px){body{width:1px}}"
		);
		assert_minify!(
			Supports,
			"@supports not (width: 1--foo) { a { width:1px } }",
			"@supports not(width: 1--foo){a{width:1px}}"
		);
		assert_minify!(Supports, "@supports (color: black) {}", "");
	}
}
