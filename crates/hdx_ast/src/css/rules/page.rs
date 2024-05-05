use hdx_lexer::{Include, Kind, Token};
use hdx_parser::{
	diagnostics, expect, expect_ignore_case, unexpected, AtRule, DeclarationRuleList, Parse, Parser,
	Result as ParserResult, Spanned, Vec,
};
use hdx_writer::{write_css, write_list, CssWriter, OutputOption, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use super::NoPreludeAllowed;
use crate::{css::properties::Property, Specificity, ToSpecificity};
use hdx_atom::{atom, Atom, Atomizable};
use hdx_derive::Atomizable;

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Page<'a> {
	pub selectors: Option<Spanned<PageSelectorList>>,
	pub style: Spanned<PageDeclaration<'a>>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for Page<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(parser.next(), Token::AtKeyword(atom!("page")));
		let span = parser.span();
		let (selectors, style) = Self::parse_at_rule(parser)?;
		if let Some(style) = style {
			Ok(Self { selectors, style })
		} else {
			Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?
		}
	}
}

impl<'a> AtRule<'a> for Page<'a> {
	type Block = PageDeclaration<'a>;
	type Prelude = PageSelectorList;
}

impl<'a> WriteCss<'a> for Page<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if !sink.can_output(OutputOption::RedundantRules) && self.style.node.is_empty() {
			return Ok(());
		}
		write_css!(sink, '@', atom!("page"));
		if self.selectors.is_some() {
			sink.write_whitespace()?;
			self.selectors.write_css(sink)?;
			sink.write_whitespace()?;
		} else {
			sink.write_whitespace()?;
		}
		self.style.write_css(sink)
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PageSelectorList(pub SmallVec<[Spanned<PageSelector>; 1]>);

impl<'a> Parse<'a> for PageSelectorList {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut selectors = smallvec![];
		loop {
			let selector = PageSelector::parse_spanned(parser)?;
			selectors.push(selector);
			if matches!(parser.cur(), Token::Comma) {
				parser.advance();
			} else {
				return Ok(Self(selectors));
			}
		}
	}
}

impl<'a> WriteCss<'a> for PageSelectorList {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_list!(sink, self.0,);
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct PageSelector {
	pub page_type: Option<Atom>,
	pub pseudos: SmallVec<[Spanned<PagePseudoClass>; 1]>,
}

impl<'a> Parse<'a> for PageSelector {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut page_type = None;
		let mut pseudos = smallvec![];
		if let Token::Ident(atom) = parser.peek() {
			page_type = Some(atom.clone());
			parser.advance();
		}
		while let Token::Colon = parser.peek() {
			pseudos.push(PagePseudoClass::parse_spanned(parser)?);
		}
		Ok(Self { page_type, pseudos })
	}
}

impl<'a> WriteCss<'a> for PageSelector {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if let Some(page_type) = &self.page_type {
			sink.write_str(page_type.as_ref())?;
		}
		for pseudo in self.pseudos.iter() {
			sink.write_char(':')?;
			sink.write_str(pseudo.to_atom().as_ref())?;
		}
		Ok(())
	}
}

impl PageSelector {
	pub fn selector(&self) -> &str {
		todo!();
		// format!("{}{}", self.page_type.unwrap_or("").to_owned(),
		// self.pseudos.into_iter().fold("", |p| p.as_str())join("")).as_str()
	}

	pub fn specificity(&self) -> Specificity {
		let mut spec = Specificity(self.page_type.is_some() as u8, 0, 0);
		for pseudo in &self.pseudos {
			spec += pseudo.specificity();
		}
		spec
	}
}

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PagePseudoClass {
	Left,
	Right,
	First,
	Blank,
}

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser.next(), Kind::Colon);
		match parser.next_with(Include::Whitespace) {
			Token::Ident(name) => match Self::from_atom(name) {
				Some(v) => Ok(v),
				_ => Err(diagnostics::UnexpectedPseudoClass(name.clone(), parser.span()).into()),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl ToSpecificity for PagePseudoClass {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Blank => Specificity(0, 1, 0),
			Self::First => Specificity(0, 1, 0),
			Self::Left => Specificity(0, 0, 1),
			Self::Right => Specificity(0, 0, 1),
		}
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct PageDeclaration<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Spanned<Property<'a>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Spanned<MarginRule<'a>>>,
}

impl<'a> PageDeclaration<'a> {
	pub fn is_empty(&self) -> bool {
		self.properties.is_empty() && self.rules.is_empty()
	}
}

impl<'a> Parse<'a> for PageDeclaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (properties, rules) = Self::parse_declaration_rule_list(parser)?;
		Ok(Self { properties, rules })
	}
}

impl<'a> DeclarationRuleList<'a> for PageDeclaration<'a> {
	type AtRule = MarginRule<'a>;
	type Declaration = Property<'a>;
}

impl<'a> WriteCss<'a> for PageDeclaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('{')?;
		sink.write_newline()?;
		sink.indent();
		let mut iter = self.properties.iter().peekable();
		while let Some(decl) = iter.next() {
			sink.write_indent()?;
			decl.write_css(sink)?;
			if iter.peek().is_none() && self.rules.is_empty() {
				sink.dedent();
				sink.write_trailing_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in self.rules.iter() {
			sink.write_newline()?;
			sink.write_indent()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.write_char('}')
	}
}

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MarginRule<'a> {
	pub name: PageMarginBox,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub style: Spanned<MarginDeclaration<'a>>,
}

impl<'a> Parse<'a> for MarginRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let span = parser.span();
		match parser.next() {
			Token::AtKeyword(atom) => {
				if let Some(name) = PageMarginBox::from_atom(atom) {
					let (_, style) = Self::parse_at_rule(parser)?;
					if let Some(style) = style {
						Ok(Self { name, style })
					} else {
						Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?
					}
				} else {
					Err(diagnostics::UnexpectedAtRule(atom.clone(), parser.span()))?
				}
			}
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> AtRule<'a> for MarginRule<'a> {
	type Block = MarginDeclaration<'a>;
	type Prelude = NoPreludeAllowed;

	fn parse_prelude(_parser: &mut Parser<'a>) -> ParserResult<Option<Spanned<Self::Prelude>>> {
		Ok(None)
	}
}

impl<'a> WriteCss<'a> for MarginRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		Ok(write_css!(sink, '@', self.name.to_atom(), (), self.style))
	}
}

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PageMarginBox {
	TopLeftCorner,     // atom!("top-left-corner")
	TopLeft,           // atom!("top-left")
	TopCenter,         // atom!("top-center")
	TopRight,          // atom!("top-right")
	TopRightCorner,    // atom!("top-right-corner")
	RightTop,          // atom!("right-top")
	RightMiddle,       // atom!("right-middle")
	RightBottom,       // atom!("right-bottom")
	BottomRightCorner, // atom!("bottom-right-corner")
	BottomRight,       // atom!("bottom-right")
	BottomCenter,      // atom!("bottom-center")
	BottomLeft,        // atom!("bottom-left")
	BottomLeftCorner,  // atom!("bottom-left-corner")
	LeftBottom,        // atom!("left-bottom")
	LeftMiddle,        // atom!("left-middle")
	LeftTop,           // atom!("left-top")
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MarginDeclaration<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Spanned<Property<'a>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Spanned<MarginRule<'a>>>,
}

impl<'a> Parse<'a> for MarginDeclaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let (properties, rules) = Self::parse_declaration_rule_list(parser)?;
		Ok(Self { properties, rules })
	}
}

impl<'a> DeclarationRuleList<'a> for MarginDeclaration<'a> {
	type AtRule = MarginRule<'a>;
	type Declaration = Property<'a>;
}

impl<'a> WriteCss<'a> for MarginDeclaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('{')?;
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		while let Some(decl) = iter.next() {
			sink.write_indent()?;
			decl.write_css(sink)?;
			if iter.peek().is_none() && self.rules.is_empty() {
				sink.dedent();
				sink.write_trailing_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in self.rules.iter() {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Page, 144);
		assert_size!(MarginRule, 80);
		assert_size!(PagePseudoClass, 1);
		assert_size!(PageMarginBox, 1);
		assert_size!(PagePseudoClass, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Page, "@page {\n\tmargin-top: 4in;\n}");
		assert_parse!(Page, "@page wide {\n}");
		assert_parse!(Page, "@page wide:left {\n\n\t@top-right {\n\t}\n}");
	}

	#[test]
	fn test_minify() {
		// empty rulesets get dropped
		assert_minify!(Page, "@page :left {}", "");
	}

	#[test]
	fn test_specificity() {
		assert_eq!(PagePseudoClass::Left.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::Right.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::First.specificity(), Specificity(0, 1, 0));
		assert_eq!(PagePseudoClass::Blank.specificity(), Specificity(0, 1, 0));
	}
}
