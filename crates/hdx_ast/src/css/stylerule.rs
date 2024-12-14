use crate::{
	css::{properties::Property, selector::SelectorList},
	syntax::BadDeclaration,
};
use hdx_parser::{Block, CursorSink, Parse, Parser, QualifiedRule, Result as ParserResult, ToCursors, Vec, T};
use hdx_proc_macro::visit;

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylerule"))]
pub struct StyleRule<'a> {
	pub selectors: SelectorList<'a>,
	#[cfg_attr(feature = "serde", serde(flatten))]
	pub style: StyleDeclaration<'a>,
}

impl<'a> Parse<'a> for StyleRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (selectors, style) = Self::parse_qualified_rule(p)?;
		Ok(Self { selectors, style })
	}
}

impl<'a> QualifiedRule<'a> for StyleRule<'a> {
	type Block = StyleDeclaration<'a>;
	type Prelude = SelectorList<'a>;
	type BadDeclaration = BadDeclaration;
}

impl<'a> ToCursors for StyleRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.selectors, s);
		ToCursors::to_cursors(&self.style, s);
	}
}

// https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "style-declaration"))]
pub struct StyleDeclaration<'a> {
	pub open: T!['{'],
	pub declarations: Vec<'a, Property<'a>>,
	pub rules: Vec<'a, StyleRule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for StyleDeclaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, declarations, rules, close) = Self::parse_block(p)?;
		Ok(Self { open, declarations, rules, close })
	}
}

impl<'a> Block<'a> for StyleDeclaration<'a> {
	type Declaration = Property<'a>;
	type Rule = StyleRule<'a>;
}

impl<'a> ToCursors for StyleDeclaration<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open, s);
		for declaration in &self.declarations {
			ToCursors::to_cursors(declaration, s);
		}
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = &self.close {
			ToCursors::to_cursors(close, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(StyleRule, 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleRule, "body{}");
		assert_parse!(StyleRule, "body,body{}");
		assert_parse!(StyleRule, "body{width:1px;}");
		assert_parse!(StyleRule, "body{opacity:0;}");
		assert_parse!(StyleRule, ".foo *{}", ".foo *{}");
		assert_parse!(StyleRule, ":nth-child(1){opacity:0;}");
		assert_parse!(StyleRule, ".foo{--bar:(baz);}");
		assert_parse!(StyleRule, ".foo{width: calc(1px + (var(--foo)) + 1px);}");
		assert_parse!(StyleRule, ".foo{--bar:1}");
	}
}
