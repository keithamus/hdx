use crate::css::{properties::Property, selector::SelectorList};
use hdx_derive::Visitable;
use hdx_parser::{Block, Parse, Parser, QualifiedRule, Result as ParserResult, Spanned, Vec};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Visitable, PartialEq, Debug, Hash)]
#[visitable(call)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylerule"))]
pub struct StyleRule<'a> {
	#[visitable(skip)]
	pub selectors: Spanned<SelectorList<'a>>,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub style: Spanned<StyleDeclaration<'a>>,
}

impl<'a> Parse<'a> for StyleRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (selectors, style) = Self::parse_qualified_rule(parser)?;
		Ok(Self { selectors, style })
	}
}

impl<'a> QualifiedRule<'a> for StyleRule<'a> {
	type Block = StyleDeclaration<'a>;
	type Prelude = SelectorList<'a>;
}

impl<'a> WriteCss<'a> for StyleRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if !sink.can_output(OutputOption::RedundantRules) && self.style.node.is_empty() {
			return Ok(());
		}
		sink.write_indent()?;
		self.selectors.write_css(sink)?;
		sink.write_whitespace()?;
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		self.style.write_css(sink)?;
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')?;
		Ok(())
	}
}

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Visitable, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "style-declaration"))]
pub struct StyleDeclaration<'a> {
	pub declarations: Vec<'a, Spanned<Property<'a>>>,
	pub rules: Vec<'a, Spanned<StyleRule<'a>>>,
}

impl<'a> Parse<'a> for StyleDeclaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (declarations, rules) = Self::parse_block(parser)?;
		Ok(Self { declarations, rules })
	}
}

impl<'a> Block<'a> for StyleDeclaration<'a> {
	type Declaration = Property<'a>;
	type Rule = StyleRule<'a>;
}

impl<'a> StyleDeclaration<'a> {
	fn is_empty(&self) -> bool {
		self.declarations.is_empty() && self.rules.is_empty()
	}
}

impl<'a> WriteCss<'a> for StyleDeclaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.declarations.iter().peekable();
		while let Some(decl) = iter.next() {
			sink.write_indent()?;
			decl.write_css(sink)?;
			if iter.peek().is_none() {
				sink.write_trailing_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in self.rules.iter() {
			sink.write_indent()?;
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
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
		assert_size!(StyleRule, 112);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleRule, "body {\n}");
		assert_parse!(StyleRule, "body, body {\n}");
		assert_parse!(StyleRule, "body {\n\twidth: 1px;\n}");
		assert_parse!(StyleRule, "body {\n\topacity: 0;\n}");
		assert_parse!(StyleRule, ".foo *{}", ".foo * {\n}");
		assert_parse!(StyleRule, ":nth-child(1) {\n\topacity: 0;\n}");
		assert_parse!(StyleRule, ".foo {\n\t--bar: (baz);\n}");
		assert_parse!(StyleRule, ".foo {\n\twidth: calc(1px + (var(--foo)) + 1px);\n}");
	}

	#[test]
	fn test_minify() {
		assert_minify!(StyleRule, "body { width:1px }", "body{width:1px}");
		assert_minify!(StyleRule, ".a {}", "");
	}
}
