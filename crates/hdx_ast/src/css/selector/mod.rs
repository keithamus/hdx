use hdx_atom::Atom;
use hdx_lexer::Token;
use hdx_parser::{
	FromToken, Parse, Parser, Result as ParserResult, SelectorComponent as SelectorComponentTrait,
	SelectorList as SelectorListTrait, Spanned,
};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

use crate::{Atomizable, Vec};

mod attribute;
mod combinator;
mod functional_pseudo_class;
mod functional_pseudo_element;
mod moz;
mod nth;
mod pseudo_class;
mod pseudo_element;
mod tag;
mod webkit;

use attribute::*;
use combinator::*;
use functional_pseudo_class::*;
use functional_pseudo_element::*;
use moz::*;
use nth::*;
use pseudo_class::*;
use pseudo_element::*;
use tag::*;
use webkit::*;

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
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SelectorComponent<'a> {
	Id(Atom),
	Class(Atom),
	Tag(Tag),
	Wildcard,
	Combinator(Combinator),
	Attribute(Attribute),
	PseudoClass(PseudoClass),
	NonStandardMozPseudoClass(NonStandardMozPseudoClass),
	NonStandardWebkitPseudoClass(NonStandardWebkitPseudoClass),
	PseudoElement(PseudoElement),
	NonStandardMozPseudoElement(NonStandardMozPseudoElement),
	NonStandardWebkitPseudoElement(NonStandardWebkitPseudoElement),
	LegacyPseudoElement(LegacyPseudoElement),
	FunctionalPseudoClass(FunctionalPseudoClass<'a>),
	NonStandardMozFunctionalPseudoClass(NonStandardMozFunctionalPseudoClass),
	NonStandardWebkitFunctionalPseudoClass(NonStandardWebkitFunctionalPseudoClass),
	FunctionalPseudoElement(FunctionalPseudoElement<'a>),
	NonStandardMozFunctionalPseudoElement(NonStandardMozFunctionalPseudoElement),
	NonStandardWebkitFunctionalPseudoElement(NonStandardWebkitFunctionalPseudoElement),
	NSPrefixedTag((NSPrefix, Atom)),
	NSPrefixedWildcard(NSPrefix),
}

impl<'a> Parse<'a> for SelectorComponent<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_selector_component(parser)
	}
}

impl<'a> SelectorComponentTrait<'a> for SelectorComponent<'a> {
	fn wildcard() -> Self {
		Self::Wildcard
	}

	fn id_from_atom(atom: &Atom) -> Option<Self> {
		Some(Self::Id(atom.clone()))
	}

	fn class_from_atom(atom: &Atom) -> Option<Self> {
		Some(Self::Class(atom.clone()))
	}

	fn type_from_atom(atom: &Atom) -> Option<Self> {
		Tag::from_atom(atom).map(Self::Tag)
	}

	fn pseudo_class_from_atom(atom: &Atom) -> Option<Self> {
		PseudoClass::from_atom(atom)
			.map(Self::PseudoClass)
			.or_else(|| NonStandardMozPseudoClass::from_atom(atom).map(Self::NonStandardMozPseudoClass))
			.or_else(|| NonStandardWebkitPseudoClass::from_atom(atom).map(Self::NonStandardWebkitPseudoClass))
	}

	fn legacy_pseudo_element_from_token(atom: &Atom) -> Option<Self> {
		LegacyPseudoElement::from_atom(atom).map(Self::LegacyPseudoElement)
	}

	fn pseudo_element_from_atom(atom: &Atom) -> Option<Self> {
		PseudoElement::from_atom(atom)
			.map(Self::PseudoElement)
			.or_else(|| NonStandardMozPseudoElement::from_atom(atom).map(Self::NonStandardMozPseudoElement))
			.or_else(|| NonStandardWebkitPseudoElement::from_atom(atom).map(Self::NonStandardWebkitPseudoElement))
	}

	fn ns_type_from_token(ns_token: &Token, type_token: &Token) -> Option<Self> {
		if let Some(prefix) = NSPrefix::from_token(ns_token) {
			match type_token {
				Token::Ident(atom) => Some(Self::NSPrefixedTag((prefix, atom.clone()))),
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
		FunctionalPseudoClass::parse(parser)
			.map(Self::FunctionalPseudoClass)
			.or_else(|_| {
				NonStandardMozFunctionalPseudoClass::parse(parser).map(Self::NonStandardMozFunctionalPseudoClass)
			})
			.or_else(|_| {
				NonStandardWebkitFunctionalPseudoClass::parse(parser).map(Self::NonStandardWebkitFunctionalPseudoClass)
			})
	}

	fn parse_functional_pseudo_element(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self::FunctionalPseudoElement(FunctionalPseudoElement::parse(parser)?))
	}
}

impl<'a> WriteCss<'a> for SelectorComponent<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Tag(ty) => write_css!(sink, ty),
			Self::Id(id) => write_css!(sink, '#', id),
			Self::Class(class) => write_css!(sink, '.', class),
			Self::PseudoClass(pseudo) => write_css!(sink, ':', pseudo.to_atom()),
			Self::NonStandardMozPseudoClass(pseudo) => write_css!(sink, ':', pseudo.to_atom()),
			Self::NonStandardWebkitPseudoClass(pseudo) => write_css!(sink, ':', pseudo.to_atom()),
			Self::NonStandardMozFunctionalPseudoClass(pseudo) => write_css!(sink, ':', pseudo),
			Self::NonStandardWebkitFunctionalPseudoClass(pseudo) => write_css!(sink, ':', pseudo),
			Self::LegacyPseudoElement(pseudo) => write_css!(sink, ':', pseudo.to_atom()),
			Self::PseudoElement(pseudo) => write_css!(sink, ':', ':', pseudo.to_atom()),
			Self::NonStandardMozPseudoElement(pseudo) => write_css!(sink, ':', ':', pseudo.to_atom()),
			Self::NonStandardWebkitPseudoElement(pseudo) => write_css!(sink, ':', ':', pseudo.to_atom()),
			Self::NonStandardMozFunctionalPseudoElement(pseudo) => write_css!(sink, ':', ':', pseudo),
			Self::NonStandardWebkitFunctionalPseudoElement(pseudo) => write_css!(sink, ':', ':', pseudo),
			Self::Attribute(attr) => write_css!(sink, attr),
			Self::Combinator(combinator) => write_css!(sink, combinator),
			Self::Wildcard => write_css!(sink, '*'),
			Self::FunctionalPseudoClass(pseudo) => write_css!(sink, ':', pseudo),
			Self::FunctionalPseudoElement(pseudo) => write_css!(sink, ':', ':', pseudo),
			Self::NSPrefixedTag((prefix, ty)) => write_css!(sink, prefix, ty),
			Self::NSPrefixedWildcard(prefix) => write_css!(sink, prefix, '*'),
		}
		Ok(())
	}
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum NSPrefix {
	#[default]
	None,
	Wildcard,
	Named(Atom),
}

impl FromToken for NSPrefix {
	fn from_token(token: &Token) -> Option<Self> {
		match token {
			Token::Delim('*') => Some(Self::Wildcard),
			Token::Ident(atom) => Some(Self::Named(atom.clone())),
			Token::Delim('|') => Some(Self::None),
			_ => None,
		}
	}
}

impl<'a> WriteCss<'a> for NSPrefix {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::None => {}
			Self::Wildcard => write_css!(sink, '*', '|'),
			Self::Named(atom) => write_css!(sink, atom, '|'),
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
		assert_size!(SelectorList, 32);
		assert_size!(ForgivingSelector, 32);
		assert_size!(RelativeSelector, 32);
		assert_size!(SelectorComponent, 48);
		assert_size!(LegacyPseudoElement, 1);
		assert_size!(Combinator, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SelectorList, ":root");
		assert_parse!(SelectorList, "body, body");
		assert_parse!(SelectorList, ".body .body");
		assert_parse!(SelectorList, "*");
		assert_parse!(SelectorList, "[attr|='foo']");
		assert_parse!(SelectorList, "*|x");
		assert_parse!(SelectorList, "a b");
		assert_parse!(SelectorList, "  a b", "a b");
		assert_parse!(SelectorList, "body [attr|='foo']");
		assert_parse!(SelectorList, "*|x :focus-within");
		assert_parse!(SelectorList, ".foo[attr*=\"foo\"]");
		assert_parse!(SelectorList, "a > b");
		assert_parse!(SelectorList, ".foo[attr*=\"foo\"] > *");
		assert_parse!(SelectorList, ".foo[attr*=\"foo\"] > * + *");
		assert_parse!(SelectorList, ":after");
		assert_parse!(SelectorList, "::after");
		assert_parse!(SelectorList, ":before");
		assert_parse!(SelectorList, "::before");
		assert_parse!(SelectorList, "::before:focus:target:right:playing:popover-open:blank");
		assert_parse!(SelectorList, ":dir(ltr)");
		assert_parse!(SelectorList, "tr:nth-child(n-1):state(foo)");
		assert_parse!(SelectorList, " /**/ .foo", ".foo");
		// Non Standard
		assert_parse!(SelectorList, "::-moz-focus-inner");
		assert_parse!(SelectorList, "::-moz-list-bullet::-webkit-scrollbar");
		assert_parse!(SelectorList, "button:-moz-focusring");
	}

	#[test]
	fn test_minify() {
		assert_minify!(SelectorList, "[attr|='foo']", "[attr|=foo]");
		assert_minify!(SelectorList, "a   b", "a b");
		assert_minify!(SelectorList, ".foo[attr*='foo'] > * + *", ".foo[attr*=foo]>*+*");
	}
}
