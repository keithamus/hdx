use hdx_atom::Atom;
use hdx_parser::{
	CompoundSelector as CompoundSelectorTrait, CursorStream, Parse, Parser, Result as ParserResult,
	SelectorComponent as SelectorComponentTrait, SelectorList as SelectorListTrait, ToCursors, Vec, T,
};

mod attribute;
mod class;
mod combinator;
mod functional_pseudo_class;
mod functional_pseudo_element;
mod moz;
mod ms;
mod namespace;
mod nth;
mod o;
mod pseudo_class;
mod pseudo_element;
mod tag;
mod webkit;

use attribute::*;
use class::*;
use combinator::*;
use functional_pseudo_class::*;
use functional_pseudo_element::*;
use namespace::*;
use nth::*;
use pseudo_class::*;
use pseudo_element::*;
use tag::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SelectorList<'a>(pub Vec<'a, CompoundSelector<'a>>);

impl<'a> SelectorListTrait<'a> for SelectorList<'a> {
	type CompoundSelector = CompoundSelector<'a>;
}

impl<'a> Parse<'a> for SelectorList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_selector_list(p)?))
	}
}

impl<'a> ToCursors<'a> for SelectorList<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		for selector in &self.0 {
			ToCursors::to_cursors(selector, s);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CompoundSelector<'a> {
	pub components: Vec<'a, SelectorComponent<'a>>,
	pub comma: Option<T![,]>,
}

impl<'a> CompoundSelectorTrait<'a> for CompoundSelector<'a> {
	type SelectorComponent = SelectorComponent<'a>;
}

impl<'a> Parse<'a> for CompoundSelector<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (components, comma) = Self::parse_compound_selector(p)?;
		Ok(Self { components, comma })
	}
}

impl<'a> ToCursors<'a> for CompoundSelector<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		for component in &self.components {
			ToCursors::to_cursors(component, s);
		}
		if let Some(comma) = self.comma {
			s.append(comma.into())
		}
	}
}

pub type ComplexSelector<'a> = SelectorList<'a>;
pub type ForgivingSelector<'a> = SelectorList<'a>;
pub type RelativeSelector<'a> = SelectorList<'a>;

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more
// practical.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SelectorComponent<'a> {
	Id(T![Hash]),
	Class(Class),
	Tag(Tag),
	Wildcard(T![*]),
	Combinator(Combinator),
	Attribute(Attribute),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	FunctionalPseudoElement(FunctionalPseudoElement<'a>),
	LegacyPseudoElement(LegacyPseudoElement),
	FunctionalPseudoClass(FunctionalPseudoClass<'a>),
	Namespace(Namespace),
}

impl<'a> Parse<'a> for SelectorComponent<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_selector_component(p)
	}
}

impl<'a> ToCursors<'a> for SelectorComponent<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::Id(c) => s.append(c.into()),
			Self::Class(c) => ToCursors::to_cursors(c, s),
			Self::Tag(c) => s.append((*c).into()),
			Self::Wildcard(c) => s.append(c.into()),
			Self::Combinator(c) => ToCursors::to_cursors(c, s),
			Self::Attribute(c) => ToCursors::to_cursors(c, s),
			Self::PseudoClass(c) => ToCursors::to_cursors(c, s),
			Self::PseudoElement(c) => ToCursors::to_cursors(c, s),
			Self::FunctionalPseudoElement(c) => ToCursors::to_cursors(c, s),
			Self::LegacyPseudoElement(c) => ToCursors::to_cursors(c, s),
			Self::FunctionalPseudoClass(c) => ToCursors::to_cursors(c, s),
			Self::Namespace(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl<'a> SelectorComponentTrait<'a> for SelectorComponent<'a> {
	type Wildcard = T![*];
	type Id = T![Hash];
	type Type = Tag;
	type PseudoClass = PseudoClass;
	type PseudoElement = PseudoElement;
	type LegacyPseudoElement = LegacyPseudoElement;
	type Class = Class;
	type NsType = Namespace;
	type Combinator = Combinator;
	type Attribute = Attribute;
	type FunctionalPseudoClass = FunctionalPseudoClass<'a>;
	type FunctionalPseudoElement = FunctionalPseudoElement<'a>;

	fn is_legacy_pseudo_element(name: &Atom) -> bool {
		LegacyPseudoElement::matches_name(name)
	}

	fn build_wildcard(node: T![*]) -> Self {
		Self::Wildcard(node)
	}

	fn build_id(node: T![Hash]) -> Self {
		Self::Id(node)
	}

	fn build_class(node: Class) -> Self {
		Self::Class(node)
	}

	fn build_type(node: Tag) -> Self {
		Self::Tag(node)
	}

	fn build_pseudo_class(node: PseudoClass) -> Self {
		Self::PseudoClass(node)
	}

	fn build_pseudo_element(node: PseudoElement) -> Self {
		Self::PseudoElement(node)
	}

	fn build_legacy_pseudo_element(node: LegacyPseudoElement) -> Self {
		Self::LegacyPseudoElement(node)
	}

	fn build_ns_type(node: Namespace) -> Self {
		Self::Namespace(node)
	}

	fn build_combinator(node: Combinator) -> Self {
		Self::Combinator(node)
	}

	fn build_attribute(node: Attribute) -> Self {
		Self::Attribute(node)
	}

	fn build_functional_pseudo_class(node: FunctionalPseudoClass<'a>) -> Self {
		Self::FunctionalPseudoClass(node)
	}

	fn build_functional_pseudo_element(node: FunctionalPseudoElement<'a>) -> Self {
		Self::FunctionalPseudoElement(node)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(SelectorList, 32);
		assert_size!(ComplexSelector, 32);
		assert_size!(ForgivingSelector, 32);
		assert_size!(RelativeSelector, 32);
		assert_size!(SelectorComponent, 112);
		assert_size!(LegacyPseudoElement, 24);
		assert_size!(Combinator, 20);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SelectorList, ":root");
		assert_parse!(SelectorList, "body,body");
		assert_parse!(SelectorList, ".body .body");
		assert_parse!(SelectorList, "*");
		assert_parse!(SelectorList, "[attr|='foo']");
		assert_parse!(SelectorList, "*|x");
		assert_parse!(SelectorList, "* x");
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
		assert_parse!(SelectorList, ":lang(en-gb,en-us)");
		assert_parse!(SelectorList, "& .foo");
		assert_parse!(SelectorList, "&:hover");
		assert_parse!(SelectorList, ".foo &:hover");
		assert_parse!(SelectorList, ".foo & & &", ".foo & & &");
		assert_parse!(SelectorList, ".class&");
		assert_parse!(SelectorList, "&&");
		assert_parse!(SelectorList, "& + .foo,&.bar");
		assert_parse!(SelectorList, ":state(foo)&", ":state(foo)&");
		// Non Standard
		assert_parse!(SelectorList, "::-moz-focus-inner");
		assert_parse!(
			SelectorList,
			"::-moz-list-bullet::-webkit-scrollbar::-ms-clear:-ms-input-placeholder::-o-scrollbar:-o-prefocus"
		);
		assert_parse!(SelectorList, "button:-moz-focusring");
	}
}
