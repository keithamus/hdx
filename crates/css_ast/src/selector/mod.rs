use crate::{
	CssMetadata,
	specificity::{Specificity, ToSpecificity},
};
use css_parse::{Box, Vec};
use css_parse::{
	CompoundSelector as CompoundSelectorTrait, Cursor, NodeMetadata, NodeWithMetadata, Parse, Parser,
	Result as ParserResult, SelectorComponent as SelectorComponentTrait, T, syntax::CommaSeparated,
};
use csskit_derives::*;
use csskit_proc_macro::node;

mod prelude {
	pub(crate) use crate::{CssAtomSet, diagnostics::CssDiagnostic};
	pub(crate) use css_parse::{
		Cursor, Diagnostic, Kind, KindSet, Parse, Parser, Peek, Result as ParserResult, T, pseudo_class, pseudo_element,
	};
	pub(crate) use csskit_derives::*;
	pub(crate) use csskit_proc_macro::node;
}

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

pub use attribute::*;
pub use class::*;
pub use combinator::*;
pub use functional_pseudo_class::*;
pub use functional_pseudo_element::*;
pub use moz::*;
pub use ms::*;
pub use namespace::*;
pub use nth::*;
pub use o::*;
pub use pseudo_class::*;
pub use pseudo_element::*;
pub use tag::*;
pub use webkit::*;

/// Represents a list of [CompoundSelectors][CompoundSelector], such as `body, dialog:modal`.
///
/// ```md
/// <selector-list>
///  │├─╭─ <compound-selector> ─╮─ "," ─╭─╮─┤│
///     │                       ╰───────╯ │
///     ╰─────────────────────────────────╯
/// ```
#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct SelectorList<'a>(pub CommaSeparated<'a, CompoundSelector<'a>>);

impl<'a> NodeWithMetadata<CssMetadata> for SelectorList<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata::default().with_size(self.0.len().min(u16::MAX as usize) as u16)
	}

	fn metadata(&self) -> CssMetadata {
		self.self_metadata()
	}
}

#[node]
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CompoundSelector<'a>(pub Vec<'a, SelectorComponent<'a>>);

impl<'a> CompoundSelectorTrait<'a> for CompoundSelector<'a> {
	type SelectorComponent = SelectorComponent<'a>;
}

impl<'a> Parse<'a> for CompoundSelector<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self(Self::parse_compound_selector(p)?))
	}
}

pub type ComplexSelector<'a> = SelectorList<'a>;
pub type ForgivingSelector<'a> = SelectorList<'a>;
pub type RelativeSelector<'a> = SelectorList<'a>;

#[node]
#[derive(
	Peek, Parse, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Id(T![Hash]);

#[node]
#[derive(
	Peek, Parse, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Wildcard(T![*]);

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more
// practical.
#[node]
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SelectorComponent<'a> {
	Id(Id),
	Class(Class),
	Tag(Tag),
	Wildcard(Wildcard),
	Combinator(Combinator),
	Attribute(Box<'a, Attribute>),
	PseudoClass(PseudoClass),
	PseudoElement(Box<'a, PseudoElement>),
	FunctionalPseudoElement(Box<'a, FunctionalPseudoElement<'a>>),
	LegacyPseudoElement(LegacyPseudoElement),
	FunctionalPseudoClass(Box<'a, FunctionalPseudoClass<'a>>),
	Namespace(Box<'a, Namespace>),
}

impl<'a> Parse<'a> for SelectorComponent<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_selector_component(p)
	}
}

impl<'a> ToSpecificity for SelectorComponent<'a> {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Id(_) => Specificity(1, 0, 0),
			Self::Class(_) | Self::Attribute(_) | Self::PseudoClass(_) => Specificity(0, 1, 0),
			Self::Tag(_) | Self::PseudoElement(_) | Self::LegacyPseudoElement(_) => Specificity(0, 0, 1),
			Self::FunctionalPseudoElement(_) => Specificity(0, 0, 1),
			Self::Combinator(_) | Self::Namespace(_) | Self::Wildcard(_) => Specificity(0, 0, 0),
			Self::FunctionalPseudoClass(f) => f.specificity(),
		}
	}
}

impl<'a> ToSpecificity for CompoundSelector<'a> {
	fn specificity(&self) -> Specificity {
		self.0.iter().map(ToSpecificity::specificity).sum()
	}
}

impl<'a> ToSpecificity for SelectorList<'a> {
	fn specificity(&self) -> Specificity {
		(&self.0).into_iter().map(|(s, _)| s.specificity()).max().unwrap_or_default()
	}
}

impl<'a> SelectorComponentTrait<'a> for SelectorComponent<'a> {
	type Wildcard = Wildcard;
	type Id = Id;
	type Type = Tag;
	type PseudoClass = PseudoClass;
	type PseudoElement = Box<'a, PseudoElement>;
	type LegacyPseudoElement = LegacyPseudoElement;
	type Class = Class;
	type NsType = Box<'a, Namespace>;
	type Combinator = Combinator;
	type Attribute = Box<'a, Attribute>;
	type FunctionalPseudoClass = Box<'a, FunctionalPseudoClass<'a>>;
	type FunctionalPseudoElement = Box<'a, FunctionalPseudoElement<'a>>;

	fn build_wildcard(node: Wildcard) -> Self {
		Self::Wildcard(node)
	}

	fn build_id(node: Id) -> Self {
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

	fn build_pseudo_element(node: Box<'a, PseudoElement>) -> Self {
		Self::PseudoElement(node)
	}

	fn build_legacy_pseudo_element(node: LegacyPseudoElement) -> Self {
		Self::LegacyPseudoElement(node)
	}

	fn build_ns_type(node: Box<'a, Namespace>) -> Self {
		Self::Namespace(node)
	}

	fn build_combinator(node: Combinator) -> Self {
		Self::Combinator(node)
	}

	fn build_attribute(node: Box<'a, Attribute>) -> Self {
		Self::Attribute(node)
	}

	fn build_functional_pseudo_class(node: Box<'a, FunctionalPseudoClass<'a>>) -> Self {
		Self::FunctionalPseudoClass(node)
	}

	fn build_functional_pseudo_element(node: Box<'a, FunctionalPseudoElement<'a>>) -> Self {
		Self::FunctionalPseudoElement(node)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, specificity::ToSpecificity};
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":root");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "body,body");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".body .body");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "*");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "[attr|='foo']");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "*|x");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "* x");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "a b");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "  a b");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "body [attr|='foo']");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "*|x :focus-within");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo[attr*=\"foo\"]");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "a > b");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo[attr*=\"foo\"] > *");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo[attr*=\"foo\"] > * + *");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":after");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::after");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":before");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::before");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::before:focus:target:right:playing:popover-open:blank");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":dir(ltr)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "tr:nth-child(n-1):state(foo)");
		// assert_parse!(CssAtomSet::ATOMS, SelectorList, " /**/ .foo");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":lang(en-gb,en-us)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "& .foo");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "&:hover");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo &:hover");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo & & &");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".class&");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "&&");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "& + .foo,&.bar");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":state(foo)&");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":heading(1)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":heading(1,2,3)");
		// Non Standard
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::-moz-focus-inner");
		assert_parse!(
			CssAtomSet::ATOMS,
			SelectorList,
			"::-moz-list-bullet::-webkit-scrollbar::-ms-clear:-ms-input-placeholder::-o-scrollbar:-o-prefocus"
		);
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "button:-moz-focusring");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::view-transition-group(*)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::view-transition-new(thing.foo.bar.baz)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":is(.a,#b)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!(".foo", CompoundSelector, Class);
		assert_visits!("#bar", CompoundSelector, Id);
		assert_visits!(".foo", SelectorList, CompoundSelector, Class);
		assert_visits!(".foo, #bar", SelectorList, CompoundSelector, Class, CompoundSelector, Id);
		assert_visits!(".foo#bar", CompoundSelector, Class, Id);
		assert_visits!(".foo.bar", CompoundSelector, Class, Class);
		assert_visits!(".foo", CompoundSelector, Class);
		assert_visits!(".foo#bar", CompoundSelector, Class, Id);
		assert_visits!(".foo", CompoundSelector, Class);
		assert_visits!("*.foo#bar", CompoundSelector, Wildcard, Class, Id);
		assert_visits!(".foo .bar", CompoundSelector, Class, Combinator, Class);
		assert_visits!(".foo ", CompoundSelector, Class);
		assert_visits!("a > b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!("a>b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!("a + b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!("a ~ b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!(".foo > .bar + .baz", CompoundSelector, Class, Combinator, Class, Combinator, Class);
	}

	#[test]
	#[should_panic]
	#[cfg(feature = "visitable")]
	fn test_assert_visits_fails() {
		use crate::assert_visits;
		assert_visits!(".foo", CompoundSelector, visit_id<Id>);
	}

	macro_rules! assert_specificity {
		($sel:literal, $a:literal, $b:literal, $c:literal) => {
			assert_parse!(CssAtomSet::ATOMS, SelectorList, $sel, |s| assert_eq!(
				s.specificity(),
				Specificity($a, $b, $c)
			));
		};
	}

	#[test]
	fn test_specificity_arithmetic() {
		assert_eq!(Specificity(0, 1, 0) + Specificity(0, 1, 0), Specificity(0, 2, 0));
		assert_eq!(Specificity(1, 0, 0) + Specificity(0, 1, 0), Specificity(1, 1, 0));
		assert_eq!(Specificity(255, 0, 0) + Specificity(1, 0, 0), Specificity(255, 0, 0));
	}

	#[test]
	fn test_specificity() {
		assert_specificity!("#foo", 1, 0, 0);
		assert_specificity!(".foo", 0, 1, 0);
		assert_specificity!(".a.b.c", 0, 3, 0);
		assert_specificity!("div", 0, 0, 1);
		assert_specificity!(":hover", 0, 1, 0);
		assert_specificity!("::before", 0, 0, 1);
		assert_specificity!(":before", 0, 0, 1);
		assert_specificity!("[href]", 0, 1, 0);
		assert_specificity!("*", 0, 0, 0);
		assert_specificity!("a.foo", 0, 1, 1);
		assert_specificity!("a.foo:hover", 0, 2, 1);
		assert_specificity!("#a.b", 1, 1, 0);
		assert_specificity!(":where(.a.b)", 0, 0, 0);
		assert_specificity!(":is(.a,#b)", 1, 0, 0);
		assert_specificity!(":not(.a,.b)", 0, 1, 0);
		assert_specificity!("a:has(.b)", 0, 1, 1);
		assert_specificity!(":nth-child(2)", 0, 1, 0);
		assert_specificity!(":nth-of-type(2n+1)", 0, 1, 0);
		assert_specificity!(".a,#b", 1, 0, 0);
	}

	#[test]
	fn test_specificity_complex() {
		assert_specificity!("nav ul li:nth-child(even) a:not([href^='#'])", 0, 2, 4);
		assert_specificity!("button:only-of-type:enabled:active:hover", 0, 4, 1);
		assert_specificity!("table tr:not(:first-child):hover td:nth-child(2n+1)", 0, 3, 3);
		assert_specificity!("input[type='checkbox'][checked]:indeterminate + label", 0, 3, 2);
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn visit_flow_baseline() {
		use crate::test_helpers::{ControlFlowTestVisitor, assert_visit_flow};
		assert_visit_flow!(
			".foo, #bar", SelectorList, ControlFlowTestVisitor::new(),
			entered: [SelectorList, CompoundSelector, Class, CompoundSelector, Id],
			exited: [Class, CompoundSelector, Id, CompoundSelector, SelectorList],
		);
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn visit_flow_stop() {
		use crate::test_helpers::{ControlFlowTestVisitor, assert_visit_flow};
		use visit_flow::{VisitBreak, VisitFlow, VisitFlowExt};
		assert_visit_flow!(
			".foo, #bar", SelectorList,
			ControlFlowTestVisitor::with_callback(|id| {
				if id == NodeId::CompoundSelector { VisitFlow::STOP } else { VisitFlow::DESCEND }
			}),
			entered: [SelectorList, CompoundSelector],
			exited: [],
			result: VisitFlow::Break(VisitBreak::Stop),
		);
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn visit_flow_skip_children() {
		use crate::test_helpers::{ControlFlowTestVisitor, assert_visit_flow};
		use visit_flow::{VisitFlow, VisitFlowExt};
		assert_visit_flow!(
			".foo, #bar", SelectorList,
			ControlFlowTestVisitor::with_callback(|id| {
				if id == NodeId::CompoundSelector { VisitFlow::SKIP_CHILDREN } else { VisitFlow::DESCEND }
			}),
			entered: [SelectorList, CompoundSelector, CompoundSelector],
			exited: [CompoundSelector, CompoundSelector, SelectorList],
		);
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn visit_flow_filter() {
		use crate::test_helpers::{ControlFlowTestVisitor, assert_visit_flow};
		use css_lexer::{SourceOffset, Span};
		assert_visit_flow!(
			".foo, #bar", SelectorList,
			ControlFlowTestVisitor::with_span_filter(Span::new(SourceOffset(0), SourceOffset(4))),
			entered: [SelectorList, CompoundSelector, Class],
			exited: [Class, CompoundSelector, SelectorList],
		);
	}
}
