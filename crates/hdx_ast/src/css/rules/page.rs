use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, expect, unexpected, AtRule, DeclarationRuleList, Parse, Parser, Result as ParserResult, Spanned, Vec,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use super::NoPreludeAllowed;
use crate::{atom, css::properties::Property, Atom, Atomizable, Specificity, ToSpecificity};

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageRule<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub selectors: Option<Spanned<PageSelectorList<'a>>>,
	pub style: Spanned<PageDeclaration<'a>>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for PageRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser, Token::AtKeyword(atom!("page")));
		let span = parser.span();
		let (selectors, style) = Self::parse_at_rule(parser)?;
		if let Some(style) = style {
			Ok(Self { selectors, style })
		} else {
			Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?
		}
	}
}

impl<'a> AtRule<'a> for PageRule<'a> {
	type Block = PageDeclaration<'a>;
	type Prelude = PageSelectorList<'a>;
}

impl<'a> WriteCss<'a> for PageRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str("@page ")?;
		self.selectors.write_css(sink)?;
		self.style.write_css(sink)?;
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageSelectorList<'a> {
	pub children: Vec<'a, Spanned<PageSelector<'a>>>,
}

impl<'a> Parse<'a> for PageSelectorList<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self { children: parser.parse_comma_list_of::<PageSelector>()? })
	}
}

impl<'a> WriteCss<'a> for PageSelectorList<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.children.iter().peekable();
		while let Some(selector) = iter.next() {
			selector.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_trivia_char(' ')?;
			}
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageSelector<'a> {
	pub page_type: Option<Atom>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub pseudos: Vec<'a, Spanned<PagePseudoClass>>,
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut page_type = None;
		let mut pseudos = parser.new_vec();
		match parser.cur() {
			Token::Ident(atom) => {
				page_type = Some(atom);
				parser.advance();
			}
			_ => {}
		}
		loop {
			match parser.cur() {
				Token::Colon => {
					pseudos.push(PagePseudoClass::parse_spanned(parser)?);
				}
				_ => {
					break;
				}
			}
		}
		Ok(Self { page_type, pseudos })
	}
}

impl<'a> WriteCss<'a> for PageSelector<'a> {
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

impl<'a> PageSelector<'a> {
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
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "kebab-case"))]
pub enum PagePseudoClass {
	Left,
	Right,
	First,
	Blank,
}

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser, Token::Colon);
		parser.advance_including_whitespace();
		match parser.cur() {
			Token::Ident(name) => match Self::from_atom(name.clone()) {
				Some(v) => Ok(v),
				_ => Err(diagnostics::UnexpectedPseudo(name, parser.span()).into()),
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

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct PageDeclaration<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, Spanned<Property<'a>>>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Spanned<MarginRule<'a>>>,
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
		sink.indent();
		sink.write_newline()?;
		let mut iter = self.properties.iter().peekable();
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
			sink.write_newline()?;
			rule.write_css(sink)?;
			sink.write_newline()?;
		}
		sink.dedent();
		sink.write_indent()?;
		sink.write_char('}')
	}
}

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct MarginRule<'a> {
	pub name: PageMarginBox,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub style: Spanned<MarginDeclaration<'a>>,
}

impl<'a> Parse<'a> for MarginRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let span = parser.span();
		match parser.cur() {
			Token::AtKeyword(atom) => {
				if let Some(name) = PageMarginBox::from_atom(atom.clone()) {
					let (_, style) = Self::parse_at_rule(parser)?;
					if let Some(style) = style {
						Ok(Self { name, style })
					} else {
						Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?
					}
				} else {
					Err(diagnostics::UnexpectedAtRule(atom, parser.span()))?
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
		sink.write_char('@')?;
		sink.write_str(self.name.to_atom().as_ref())?;
		sink.write_trivia_char(' ')?;
		self.style.write_css(sink)
	}
}

#[derive(Atomizable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "kebab-case"))]
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

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
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
		let mut rule_iter = self.rules.iter().peekable();
		while let Some(decl) = iter.next() {
			decl.write_css(sink)?;
			if iter.peek().is_none() && rule_iter.peek().is_none() {
				sink.write_trivia_char(';')?;
			} else {
				sink.write_char(';')?;
			}
			sink.write_newline()?;
		}
		for rule in rule_iter {
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

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<PageRule>(), 112);
		assert_eq!(size_of::<MarginRule>(), 80);
		assert_eq!(size_of::<PagePseudoClass>(), 1);
		assert_eq!(size_of::<PageMarginBox>(), 1);
		assert_eq!(size_of::<PagePseudoClass>(), 1);
	}

	#[test]
	fn test_specificity() {
		assert_eq!(PagePseudoClass::Left.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::Right.specificity(), Specificity(0, 0, 1));
		assert_eq!(PagePseudoClass::First.specificity(), Specificity(0, 1, 0));
		assert_eq!(PagePseudoClass::Blank.specificity(), Specificity(0, 1, 0));
	}
}
