use crate::{css::stylesheet::Rule, syntax::SimpleBlock};
use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, expect, expect_ignore_case, match_ignore_case, peek, todo, unexpected, unexpected_ident, AtRule,
	Parse, Parser, Result as ParserResult, RuleList, Spanned, Vec,
};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct SupportsRule<'a> {
	pub condition: Spanned<SupportsCondition<'a>>,
	pub rules: Spanned<SupportsRules<'a>>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-supports
impl<'a> Parse<'a> for SupportsRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(parser.next(), Token::AtKeyword(atom!("supports")));
		let span = parser.span();
		match Self::parse_at_rule(parser)? {
			(Some(condition), Some(rules)) => Ok(Self { condition, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
		}
	}
}

impl<'a> AtRule<'a> for SupportsRule<'a> {
	type Prelude = SupportsCondition<'a>;
	type Block = SupportsRules<'a>;
}

impl<'a> WriteCss<'a> for SupportsRule<'a> {
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
			sink.write_indent()?;
			rule.write_css(sink)?;
			if rules.peek().is_some() {
				sink.write_newline()?;
			}
		}
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum SupportsCondition<'a> {
	Is(SupportsFeature<'a>),
	Not(SupportsFeature<'a>),
	And(Vec<'a, SupportsFeature<'a>>),
	Or(Vec<'a, SupportsFeature<'a>>),
}

impl<'a> Parse<'a> for SupportsCondition<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.peek() {
			Token::LeftParen => {
				if peek!(parser, 2, Token::LeftParen) {
					todo!(parser)
				} else {
					let feature = SupportsFeature::parse(parser)?;
					match parser.peek() {
						Token::Ident(ident) => match ident.to_ascii_lowercase() {
							atom!("and") => {
								let mut features = parser.new_vec();
								features.push(feature);
								loop {
									expect_ignore_case!(parser.next(), Token::Ident(atom!("and")));
									features.push(SupportsFeature::parse(parser)?);
									if !match_ignore_case!(parser.peek(), Token::Ident(atom!("and"))) {
										return Ok(Self::And(features));
									}
								}
							}
							atom!("or") => {
								let mut features = parser.new_vec();
								features.push(feature);
								loop {
									expect_ignore_case!(parser.next(), Token::Ident(atom!("or")));
									features.push(SupportsFeature::parse(parser)?);
									if !match_ignore_case!(parser.peek(), Token::Ident(atom!("or"))) {
										return Ok(Self::Or(features));
									}
								}
							}
							_ => Ok(Self::Is(feature)),
						},
						_ => Ok(Self::Is(feature)),
					}
				}
			}
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("and") => {
					let mut features = parser.new_vec();
					loop {
						expect_ignore_case!(parser.next(), Token::Ident(atom!("and")));
						features.push(SupportsFeature::parse(parser)?);
						if !match_ignore_case!(parser.peek(), Token::Ident(atom!("and"))) {
							return Ok(Self::And(features));
						}
					}
				}
				atom!("or") => {
					let mut features = parser.new_vec();
					loop {
						expect_ignore_case!(parser.next(), Token::Ident(atom!("or")));
						features.push(SupportsFeature::parse(parser)?);
						if !match_ignore_case!(parser.peek(), Token::Ident(atom!("or"))) {
							return Ok(Self::And(features));
						}
					}
				}
				atom!("not") => {
					parser.advance();
					Ok(Self::Not(SupportsFeature::parse(parser)?))
				}
				_ => unexpected_ident!(parser, ident),
			},
			token => unexpected!(parser, token),
		}
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
				let mut iter = features.iter().peekable();
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
				let mut iter = features.iter().peekable();
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
		expect!(parser.peek(), Token::LeftParen);
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
		assert_size!(SupportsRule, 96);
		assert_size!(SupportsCondition, 48);
		assert_size!(SupportsRules, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SupportsRule, "@supports (color: black) {\n\n}");
		assert_parse!(SupportsRule, "@supports (width: 1px) {\n\tbody {\n\t\twidth: 1px;\n\t}\n}");
		assert_parse!(SupportsRule, "@supports not (width: 1--foo) {\n\n}");
		assert_parse!(SupportsRule, "@supports (width: 1--foo) or (width: 1foo) {\n\n}");
		assert_parse!(SupportsRule, "@supports (width: 1--foo) and (width: 1foo) {\n\n}");
		assert_parse!(SupportsRule, "@supports (width: 100vw) {\n\tbody {\n\t\twidth: 100vw;\n\t}\n}");
		assert_parse!(
			SupportsRule,
			"@supports not ((text-align-last: justify) or (-moz-text-align-last: justify)) {\n\n}"
		);
	}

	#[test]
	fn test_minify() {
		assert_minify!(
			SupportsRule,
			"@supports (width: 1px) { body { width:1px; } }",
			"@supports(width: 1px){body{width:1px}}"
		);
		assert_minify!(
			SupportsRule,
			"@supports not (width: 1--foo) { a { width:1px } }",
			"@supports not(width: 1--foo){a{width:1px}}"
		);
		assert_minify!(SupportsRule, "@supports (color: black) {}", "");
	}
}
