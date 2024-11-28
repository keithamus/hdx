use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet, Span};
use hdx_parser::{
	diagnostics, AtRule, Build, CursorStream, DeclarationRuleList, NoPreludeAllowed, Parse, Parser, PreludeList,
	Result as ParserResult, ToCursors, T,
};

use crate::{
	css::properties::Property,
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
pub struct Page<'a> {
	pub at_keyword: T![AtKeyword],
	pub selectors: Option<PageSelectorList<'a>>,
	pub block: PageDeclaration<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for Page<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, selectors, block) = Self::parse_at_rule(p, Some(atom!("page")))?;
		if let Some(block) = block {
			Ok(Self { at_keyword, selectors, block })
		} else {
			Err(diagnostics::MissingAtRuleBlock(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for Page<'a> {
	type Block = PageDeclaration<'a>;
	type Prelude = PageSelectorList<'a>;
}

impl<'a> ToCursors<'a> for Page<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		if let Some(selectors) = &self.selectors {
			ToCursors::to_cursors(selectors, s);
		}
		ToCursors::to_cursors(&self.block, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PageSelectorList<'a>(pub Vec<'a, PageSelector<'a>>);

impl<'a> PreludeList<'a> for PageSelectorList<'a> {
	type PreludeItem = PageSelector<'a>;
}

impl<'a> Parse<'a> for PageSelectorList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> ToCursors<'a> for PageSelectorList<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		for selector in &self.0 {
			ToCursors::to_cursors(selector, s);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct PageSelector<'a> {
	pub page_type: Option<T![Ident]>,
	pub pseudos: Vec<'a, PagePseudoClass>,
	pub comma: Option<T![,]>,
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut pseudos = Vec::new_in(p.bump());
		let page_type = p.parse_if_peek::<T![Ident]>()?;
		loop {
			if p.peek::<T![:]>() {
				pseudos.push(p.parse::<PagePseudoClass>()?);
			} else {
				let comma = p.parse_if_peek::<T![,]>()?;
				return Ok(Self { page_type, pseudos, comma });
			}
		}
	}
}

impl<'a> ToCursors<'a> for PageSelector<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		if let Some(page_type) = self.page_type {
			s.append(page_type.into())
		}
		for pseudo in &self.pseudos {
			ToCursors::to_cursors(pseudo, s);
		}
		if let Some(comma) = self.comma {
			s.append(comma.into())
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

impl<'a> ToCursors<'a> for PagePseudoClass {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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
pub struct PageDeclaration<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Property<'a>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, MarginRule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> PageDeclaration<'a> {
	pub fn is_empty(&self) -> bool {
		self.properties.is_empty() && self.rules.is_empty()
	}
}

impl<'a> Parse<'a> for PageDeclaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, rules, close) = Self::parse_declaration_rule_list(p)?;
		Ok(Self { open, properties, rules, close })
	}
}

impl<'a> DeclarationRuleList<'a> for PageDeclaration<'a> {
	type AtRule = MarginRule<'a>;
	type Declaration = Property<'a>;
}

impl<'a> ToCursors<'a> for PageDeclaration<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MarginRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub block: MarginDeclaration<'a>,
}

impl<'a> AtRule<'a> for MarginRule<'a> {
	type Prelude = NoPreludeAllowed;
	type Block = MarginDeclaration<'a>;
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
		if let Some(block) = block {
			Ok(Self { at_keyword, block })
		} else {
			Err(diagnostics::MissingAtRuleBlock(c.into()))?
		}
	}
}

impl<'a> ToCursors<'a> for MarginRule<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MarginDeclaration<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Property<'a>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, MarginRule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for MarginDeclaration<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, rules, close) = Self::parse_declaration_rule_list(p)?;
		Ok(Self { open, properties, rules, close })
	}
}

impl<'a> DeclarationRuleList<'a> for MarginDeclaration<'a> {
	type AtRule = MarginRule<'a>;
	type Declaration = Property<'a>;
}

impl<'a> ToCursors<'a> for MarginDeclaration<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Page, 136);
		assert_size!(PageSelectorList, 32);
		assert_size!(PageSelector, 64);
		assert_size!(PagePseudoClass, 24);
		assert_size!(PageDeclaration, 88);
		assert_size!(MarginRule, 104);
		assert_size!(MarginDeclaration, 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Page, "@page{margin-top:4in;}");
		assert_parse!(Page, "@page wide{}");
		assert_parse!(Page, "@page wide:left{}");
		assert_parse!(MarginRule, "@top-right{}");
		assert_parse!(Page, "@page wide:left{@top-right{}}");
	}
}
