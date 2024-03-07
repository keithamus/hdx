use hdx_atom::Atom;
use hdx_lexer::Token;
use hdx_parser::{
	FromToken, Parse, Parser, Result as ParserResult, SelectorComponent as SelectorComponentTrait,
	SelectorList as SelectorListTrait, Spanned,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{Atomizable, Vec};

mod anb;
mod attribute;
mod combinator;
mod functional_pseudo_class;
mod functional_pseudo_element;
mod pseudo_class;
mod pseudo_element;

use anb::*;
use attribute::*;
use combinator::*;
use functional_pseudo_class::*;
use functional_pseudo_element::*;
use pseudo_class::*;
use pseudo_element::*;

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SelectorList<'a>(pub Vec<'a, Spanned<Vec<'a, SelectorComponent<'a>>>>);

impl<'a> Parse<'a> for SelectorList<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_selector_list(parser)?))
	}
}

impl<'a> SelectorListTrait<'a> for SelectorList<'a> {
	type SelectorComponent = SelectorComponent<'a>;
}

impl<'a> WriteCss<'a> for SelectorList<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut selectors = self.0.iter().peekable();
		while let Some(selector) = selectors.next() {
			selector.write_css(sink)?;
			if selectors.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}

pub type ForgivingSelector<'a> = SelectorList<'a>;
pub type RelativeSelector<'a> = SelectorList<'a>;

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more
// practical.
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum SelectorComponent<'a> {
	Id(Atom),
	Class(Atom),
	Type(Atom),
	Wildcard,
	Combinator(Combinator),
	Attribute(Attribute),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	LegacyPseudoElement(LegacyPseudoElement),
	FunctionalPseudoClass(FunctionalPseudoClass<'a>),
	FunctionalPseudoElement(FunctionalPseudoElement<'a>),
	NSPrefixedType((NSPrefix, Atom)),
	NSPrefixedWildcard(NSPrefix),
}

impl<'a> SelectorComponentTrait<'a> for SelectorComponent<'a> {
	fn wildcard() -> Self {
		Self::Wildcard
	}

	fn id_from_atom(atom: Atom) -> Option<Self> {
		Some(Self::Id(atom))
	}

	fn class_from_atom(atom: Atom) -> Option<Self> {
		Some(Self::Class(atom))
	}

	fn type_from_atom(atom: Atom) -> Option<Self> {
		Some(Self::Type(atom))
	}

	fn pseudo_class_from_atom(atom: Atom) -> Option<Self> {
		PseudoClass::from_atom(atom).map(Self::PseudoClass)
	}

	fn legacy_pseudo_element_from_token(atom: Atom) -> Option<Self> {
		LegacyPseudoElement::from_atom(atom).map(Self::LegacyPseudoElement)
	}

	fn pseudo_element_from_atom(atom: Atom) -> Option<Self> {
		PseudoElement::from_atom(atom).map(Self::PseudoElement)
	}

	fn ns_type_from_token(ns_token: &Token, type_token: &Token) -> Option<Self> {
		if let Some(prefix) = NSPrefix::from_token(ns_token.clone()) {
			match type_token {
				Token::Ident(atom) => Some(Self::NSPrefixedType((prefix, atom.clone()))),
				Token::Delim('*') => Some(Self::NSPrefixedWildcard(prefix)),
				_ => None,
			}
		} else {
			None
		}
	}

	fn parse_combinator(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self::Combinator(Combinator::parse(parser)?))
	}

	fn parse_attribute(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self::Attribute(Attribute::parse(parser)?))
	}

	fn parse_functional_pseudo_class(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self::FunctionalPseudoClass(FunctionalPseudoClass::parse(parser)?))
	}

	fn parse_functional_pseudo_element(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self::FunctionalPseudoElement(FunctionalPseudoElement::parse(parser)?))
	}
}

impl<'a> WriteCss<'a> for SelectorComponent<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Type(ty) => sink.write_str(ty),
			Self::Id(id) => {
				sink.write_char('#')?;
				sink.write_str(id)
			}
			Self::Class(class) => {
				sink.write_char('.')?;
				sink.write_str(class)
			}
			Self::PseudoClass(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())
			}
			Self::LegacyPseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())
			}
			Self::PseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())
			}
			Self::Attribute(attr) => {
				sink.write_char('[')?;
				attr.write_css(sink)?;
				sink.write_char(']')
			}
			Self::Combinator(combinator) => combinator.write_css(sink),
			Self::Wildcard => sink.write_char('*'),
			Self::FunctionalPseudoClass(pseudo) => {
				sink.write_char(':')?;
				pseudo.write_css(sink)
			}
			Self::FunctionalPseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_char(':')?;
				pseudo.write_css(sink)
			}
			Self::NSPrefixedType((prefix, ty)) => {
				prefix.write_css(sink)?;
				sink.write_char('|')?;
				ty.write_css(sink)
			}
			Self::NSPrefixedWildcard(prefix) => {
				prefix.write_css(sink)?;
				sink.write_char('|')?;
				sink.write_char('*')
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum NSPrefix {
	None,
	Wildcard,
	Named(Atom),
}

impl FromToken for NSPrefix {
	fn from_token(token: Token) -> Option<Self> {
		match token {
			Token::Delim('*') => Some(Self::Wildcard),
			Token::Ident(atom) => Some(Self::Named(atom)),
			Token::Delim('|') => Some(Self::None),
			_ => None,
		}
	}
}

impl<'a> WriteCss<'a> for NSPrefix {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::None => Ok(()),
			Self::Wildcard => sink.write_char('*'),
			Self::Named(atom) => atom.write_css(sink),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(SelectorList, 32);
		assert_size!(ForgivingSelector, 32);
		assert_size!(RelativeSelector, 32);
		assert_size!(SelectorComponent, 48);
		assert_size!(LegacyPseudoElement, 1);
		assert_size!(Combinator, 1);
		assert_size!(ANB, 8);
		assert_size!(ANBEvenOdd, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SelectorList, ":root");
		assert_parse!(SelectorList, "*");
		assert_parse!(SelectorList, "[attr|='foo']");
		assert_parse!(SelectorList, "*|x");
		assert_parse!(SelectorList, "a b");
		assert_parse!(SelectorList, "body [attr|='foo']");
		assert_parse!(SelectorList, "*|x :focus-within");
		assert_parse!(SelectorList, ".foo[attr*=foo]");
		assert_parse!(SelectorList, "a > b");
		assert_parse!(SelectorList, ".foo[attr*=foo] > *");
		assert_parse!(SelectorList, ".foo[attr*=foo] > * + *");
	}

	#[test]
	fn test_minify() {
		assert_minify!(SelectorList, "[attr|='foo']", "[attr|=foo]");
		assert_minify!(SelectorList, "a   b", "a b");
		assert_minify!(SelectorList, "a   b ", "a b");
		assert_minify!(SelectorList, ".foo[attr*='foo'] > * + *", ".foo[attr*=foo]>*+*");
	}
}
