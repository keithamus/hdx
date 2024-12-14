use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	diagnostics, AtRule, CursorSink, Parse, Parser, PreludeCommaList, Result as ParserResult, RuleList, ToCursors, T,
};
use hdx_proc_macro::visit;

use crate::css::{stylesheet::Rule, Visit, Visitable};

// https://drafts.csswg.org/css-cascade-5/#layering
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LayerRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub names: Option<LayerNameList<'a>>,
	pub block: OptionalLayerBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for LayerRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, names, block) = Self::parse_at_rule(p, Some(atom!("layer")))?;
		if let Some(ref names) = names {
			if matches!(block, OptionalLayerBlock::Block(_)) && names.0.len() > 1 {
				let c: Cursor = names.0[0].0 .0.into();
				Err(diagnostics::DisallowedLayerBlockWithMultipleNames(c.into()))?
			}
		}
		Ok(Self { at_keyword, names, block })
	}
}

impl<'a> AtRule<'a> for LayerRule<'a> {
	type Prelude = LayerNameList<'a>;
	type Block = OptionalLayerBlock<'a>;
}

impl<'a> ToCursors for LayerRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		if let Some(names) = &self.names {
			ToCursors::to_cursors(names, s);
		}
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for LayerRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		todo!();
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerNameList<'a>(pub Vec<'a, (LayerName<'a>, Option<T![,]>)>);

impl<'a> PreludeCommaList<'a> for LayerNameList<'a> {
	type PreludeItem = LayerName<'a>;
}

impl<'a> Parse<'a> for LayerNameList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> ToCursors for LayerNameList<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for (selector, comma) in &self.0 {
			ToCursors::to_cursors(selector, s);
			if let Some(comma) = comma {
				s.append(comma.into());
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LayerName<'a>(T![Ident], Vec<'a, (T![.], T![Ident])>);

impl<'a> Parse<'a> for LayerName<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut parts = Vec::new_in(p.bump());
		let first = p.parse::<T![Ident]>()?;
		loop {
			if p.peek::<T![.]>() {
				let dot = p.parse::<T![.]>()?;
				let ident = p.parse::<T![Ident]>()?;
				parts.push((dot, ident));
			} else {
				return Ok(Self(first, parts));
			}
		}
	}
}

impl<'a> ToCursors for LayerName<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.0.into());
		for (dot, ident) in &self.1 {
			s.append(dot.into());
			s.append(ident.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum OptionalLayerBlock<'a> {
	None(T![;]),
	Block(LayerBlock<'a>),
}

impl<'a> Parse<'a> for OptionalLayerBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(semicolon) = p.parse_if_peek::<T![;]>()? {
			Ok(Self::None(semicolon))
		} else {
			Ok(Self::Block(p.parse::<LayerBlock>()?))
		}
	}
}

impl<'a> ToCursors for OptionalLayerBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			OptionalLayerBlock::None(semicolon) => s.append(semicolon.into()),
			OptionalLayerBlock::Block(block) => {
				ToCursors::to_cursors(block, s);
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct LayerBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for LayerBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for LayerBlock<'a> {
	type Rule = Rule<'a>;
}

impl<'a> ToCursors for LayerBlock<'a> {
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(LayerRule, 104);
		assert_size!(LayerNameList, 32);
		assert_size!(LayerName, 48);
		assert_size!(OptionalLayerBlock, 56);
		assert_size!(LayerBlock, 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LayerRule, "@layer foo{}");
		assert_parse!(LayerRule, "@layer foo;");
		assert_parse!(LayerRule, "@layer foo,bar;");
		assert_parse!(LayerRule, "@layer foo.bar,baz.bing.baz;");
		assert_parse!(LayerRule, "@layer foo.bar{body{color:black}}");
	}
}
