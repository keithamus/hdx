use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{
	diagnostics, AtRule, Build, CursorSink, DeclarationList, DeclarationRuleList, NoPreludeAllowed, Parse, Parser,
	PreludeCommaList, Result as ParserResult, ToCursors, T,
};
use hdx_proc_macro::visit;

use crate::{
	css::{properties::Property, Visit, Visitable},
	specificity::{Specificity, ToSpecificity},
};

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(Left, atom!("left"));
	custom_keyword!(Right, atom!("right"));
	custom_keyword!(First, atom!("first"));
	custom_keyword!(Blank, atom!("blank"));
}

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct PageRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub selectors: Option<PageSelectorList<'a>>,
	pub block: PageRuleBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for PageRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, selectors, block) = Self::parse_at_rule(p, Some(atom!("page")))?;
		Ok(Self { at_keyword, selectors, block })
	}
}

impl<'a> AtRule<'a> for PageRule<'a> {
	type Prelude = PageSelectorList<'a>;
	type Block = PageRuleBlock<'a>;
}

impl<'a> ToCursors for PageRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		if let Some(selectors) = &self.selectors {
			ToCursors::to_cursors(selectors, s);
		}
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for PageRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_page_rule(self);
		if let Some(selectors) = &self.selectors {
			Visitable::accept(selectors, v);
		}
		Visitable::accept(&self.block, v);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PageSelectorList<'a>(pub Vec<'a, (PageSelector<'a>, Option<T![,]>)>);

impl<'a> PreludeCommaList<'a> for PageSelectorList<'a> {
	type PreludeItem = PageSelector<'a>;
}

impl<'a> Parse<'a> for PageSelectorList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> ToCursors for PageSelectorList<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for (selector, comma) in &self.0 {
			ToCursors::to_cursors(selector, s);
			if let Some(comma) = comma {
				s.append(comma.into());
			}
		}
	}
}

impl<'a> Visitable<'a> for PageSelectorList<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (selector, _) in &self.0 {
			Visitable::accept(selector, v);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct PageSelector<'a> {
	pub page_type: Option<T![Ident]>,
	pub pseudos: Vec<'a, PagePseudoClass>,
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut pseudos = Vec::new_in(p.bump());
		let page_type = p.parse_if_peek::<T![Ident]>()?;
		loop {
			if p.peek::<T![:]>() {
				pseudos.push(p.parse::<PagePseudoClass>()?);
			} else {
				return Ok(Self { page_type, pseudos });
			}
		}
	}
}

impl<'a> ToCursors for PageSelector<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(page_type) = self.page_type {
			s.append(page_type.into())
		}
		for pseudo in &self.pseudos {
			ToCursors::to_cursors(pseudo, s);
		}
	}
}

impl<'a> ToSpecificity for PageSelector<'a> {
	fn specificity(&self) -> Specificity {
		let specificity = self.pseudos.iter().map(ToSpecificity::specificity).sum();
		if self.page_type.is_some() {
			specificity + Specificity(1, 0, 0)
		} else {
			specificity
		}
	}
}

impl<'a> Visitable<'a> for PageSelector<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_page_selector(self);
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PagePseudoClass {
	Left(T![:], kw::Left),
	Right(T![:], kw::Right),
	First(T![:], kw::First),
	Blank(T![:], kw::Blank),
}

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let skip = p.set_skip(KindSet::NONE);
		let c = p.parse::<T![Ident]>()?.into();
		p.set_skip(skip);
		match p.parse_atom_lower(c) {
			atom!("left") => Ok(Self::Left(colon, kw::Left::build(p, c))),
			atom!("right") => Ok(Self::Left(colon, kw::Left::build(p, c))),
			atom!("first") => Ok(Self::Left(colon, kw::Left::build(p, c))),
			atom!("blank") => Ok(Self::Left(colon, kw::Left::build(p, c))),
			atom => Err(diagnostics::UnexpectedPseudoClass(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors for PagePseudoClass {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Left(colon, kw) => {
				s.append(colon.into());
				s.append(kw.into());
			}
			Self::Right(colon, kw) => {
				s.append(colon.into());
				s.append(kw.into());
			}
			Self::First(colon, kw) => {
				s.append(colon.into());
				s.append(kw.into());
			}
			Self::Blank(colon, kw) => {
				s.append(colon.into());
				s.append(kw.into());
			}
		}
	}
}

impl ToSpecificity for PagePseudoClass {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Blank(_, _) => Specificity(0, 1, 0),
			Self::First(_, _) => Specificity(0, 1, 0),
			Self::Left(_, _) => Specificity(0, 0, 1),
			Self::Right(_, _) => Specificity(0, 0, 1),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct PageRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Property<'a>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, MarginRule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> PageRuleBlock<'a> {
	pub fn is_empty(&self) -> bool {
		self.properties.is_empty() && self.rules.is_empty()
	}
}

impl<'a> Parse<'a> for PageRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, rules, close) = Self::parse_declaration_rule_list(p)?;
		Ok(Self { open, properties, rules, close })
	}
}

impl<'a> DeclarationRuleList<'a> for PageRuleBlock<'a> {
	type Declaration = Property<'a>;
	type AtRule = MarginRule<'a>;
}

impl<'a> ToCursors for PageRuleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for property in &self.properties {
			ToCursors::to_cursors(property, s);
		}
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

impl<'a> Visitable<'a> for PageRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for property in &self.properties {
			Visitable::accept(property, v);
		}
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
}

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct MarginRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub block: MarginRuleBlock<'a>,
}

impl<'a> AtRule<'a> for MarginRule<'a> {
	type Prelude = NoPreludeAllowed;
	type Block = MarginRuleBlock<'a>;
}

impl<'a> Parse<'a> for MarginRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, _, block) = Self::parse_at_rule(p, None)?;
		let c: Cursor = at_keyword.into();
		let atom = p.parse_atom_lower(c);
		if !matches!(
			atom,
			atom!("top-left-corner")
				| atom!("top-left")
				| atom!("top-center")
				| atom!("top-right")
				| atom!("top-right-corner")
				| atom!("right-top")
				| atom!("right-middle")
				| atom!("right-bottom")
				| atom!("bottom-right-corner")
				| atom!("bottom-right")
				| atom!("bottom-center")
				| atom!("bottom-left")
				| atom!("bottom-left-corner")
				| atom!("left-bottom")
				| atom!("left-middle")
				| atom!("left-top")
		) {
			Err(diagnostics::UnexpectedAtRule(atom, c.into()))?
		}
		Ok(Self { at_keyword, block })
	}
}

impl<'a> ToCursors for MarginRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for MarginRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_margin_rule(self);
		Visitable::accept(&self.block, v);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MarginRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Property<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for MarginRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_declaration_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> DeclarationList<'a> for MarginRuleBlock<'a> {
	type Declaration = Property<'a>;
}

impl<'a> ToCursors for MarginRuleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for property in &self.properties {
			ToCursors::to_cursors(property, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

impl<'a> Visitable<'a> for MarginRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for property in &self.properties {
			Visitable::accept(property, v);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PageRule, 144);
		assert_size!(PageSelectorList, 32);
		assert_size!(PageSelector, 48);
		assert_size!(PagePseudoClass, 28);
		assert_size!(PageRuleBlock, 96);
		assert_size!(MarginRule, 80);
		assert_size!(MarginRuleBlock, 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PageRule, "@page{margin-top:4in;}");
		assert_parse!(PageRule, "@page wide{}");
		assert_parse!(PageRule, "@page wide:left{}");
		assert_parse!(MarginRule, "@top-right{}");
		assert_parse!(PageRule, "@page wide:left{@top-right{}}");
	}
}
