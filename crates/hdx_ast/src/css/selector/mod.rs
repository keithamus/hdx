#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable, Box, Spanned, Vec};

// This encapsulates both `simple-selector` and `compound-selector`.
// As `simple-selector` is a `compound-selector` but with only one `Component`.
// Having `Selector` be both ` simple-selector` and `compound-selector` makes parsing and visiting
// more practical.
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Selector<'a> {
	pub components: Box<'a, Vec<'a, Spanned<Component<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ForgivingSelector<'a> {
	pub components: Box<'a, Vec<'a, Spanned<Component<'a>>>>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct RelativeSelector<'a> {
	pub components: Box<'a, Vec<'a, Spanned<Component<'a>>>>,
}

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more practical.
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Component<'a> {
	Id(Atom),
	Class(Atom),
	Type(Atom),
	Wildcard,
	Combinator(Combinator),
	Attribute(Box<'a, Spanned<Attribute>>),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	LegacyPseudoElement(LegacyPseudoElement),
	PseudoFunction(PseudoFunction<'a>),
	NSPrefixedType(Box<'a, (NSPrefix, Atom)>),
	NSPrefixedWildcard(NSPrefix),
}

// https://drafts.csswg.org/css-pseudo/#index-defined-here
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum PseudoElement {
	After,              // atom!("after")
	Backdrop,           // atom!("backdrop")
	Before,             // atom!("after")
	Cue,                // atom!("cue")
	CueRegion,          // atom!("cue-region")
	FirstLetter,        // atom!("first-letter")
	FirstLine,          // atom!("first-line")
	FileSelectorButton, // atom!("file-selector-button")
	GrammarError,       // atom!("grammar-error")
	Marker,             // atom!("marker")
	Placeholder,        // atom!("placeholder")
	Selection,          // atom!("selection")
	SpellingError,      // atom!("spelling-error")
	TargetText,         // atom!("target-text")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum LegacyPseudoElement {
	After,       // atom!("after")
	Before,      // atom!("before")
	FirstLetter, // atom!("first-letter")
	FirstLine,   // atom!("first-line")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Attribute {
	pub ns_prefix: NSPrefix,
	pub name: Atom,
	pub value: Atom,
	pub matcher: AttributeMatch,
	pub modifier: AttributeModifier,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeMatch {
	Any,        // [attr]
	Exact,      // [attr=val]
	SpaceList,  // [attr~=val]
	LangPrefix, // [attr|=val]
	Prefix,     // [attr^=val]
	Suffix,     // [attr$=val]
	Contains,   // [attr*=val]
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum AttributeModifier {
	None,
	Sensitive,
	Insensitive,
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PseudoClass {
	Active,           // atom!("active")
	AnyLink,          // atom!("any-link")
	Autofill,         // atom!("autofill")
	Blank,            // atom!("blank")
	Checked,          // atom!("checked")
	Current,          // atom!("current")
	Default,          // atom!("default")
	Defined,          // atom!("defined")
	Disabled,         // atom!("disabled")
	Empty,            // atom!("empty")
	Enabled,          // atom!("enabled")
	First,            // atom!("first")
	FirstChild,       // atom!("first-child")
	FirstOfType,      // atom!("first-of-type")
	Fullscreen,       // atom!("fullscreen")
	Future,           // atom!("future")
	Focus,            // atom!("focus")
	FocusVisible,     // atom!("focus-visible")
	FocusWithin,      // atom!("focus-within")
	Host,             // atom!("host")
	Hover,            // atom!("hover")
	Indeterminate,    // atom!("indeterminate")
	InRange,          // atom!("in-range")
	Invalid,          // atom!("invalid")
	LastChild,        // atom!("last-child")
	LastOfType,       // atom!("last-of-type")
	Left,             // atom!("left")
	Link,             // atom!("link")
	LocalLink,        // atom!("local-link")
	Modal,            // atom!("modal")
	OnlyChild,        // atom!("only-child")
	OnlyOfType,       // atom!("only-of-type")
	Optional,         // atom!("optional")
	OutOfRange,       // atom!("out-of-range")
	Past,             // atom!("past")
	PictureInPicture, // atom!("picture-in-picture")
	PlaceholderShown, // atom!("placeholder-shown")
	PopoverOpen,      // atom!("popover-open")
	Paused,           // atom!("paused")
	Playing,          // atom!("playing")
	ReadOnly,         // atom!("read-only")
	ReadWrite,        // atom!("read-write")
	Required,         // atom!("required")
	Right,            // atom!("right")
	Root,             // atom!("root")
	Scope,            // atom!("scope")
	Target,           // atom!("target")
	TargetWithin,     // atom!("target-within")
	Valid,            // atom!("valid")
	Visited,          // atom!("visited")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PseudoFunction<'a> {
	Dir(DirValue),                // atom!("dir")
	Has(RelativeSelector<'a>),    // atom!("has")
	Host(Selector<'a>),           // atom!("host")
	HostContext(Selector<'a>),    // atom!("host-context")
	Is(ForgivingSelector<'a>),    // atom!("is")
	Lang(Box<'a, Vec<'a, Atom>>), // atom!("lang")
	Not(Selector<'a>),            // atom!("not")
	NthChild(ANBEvenOdd),         // atom!("nth-child")
	NthCol(ANB),                  // atom!("nth-col")
	NthLastChild(ANBEvenOdd),     // atom!("nth-last-child")
	NthLastCol(ANB),              // atom!("nth-last-col")
	NthLastOfType(ANBEvenOdd),    // atom!("nth-last-of-type")
	NthOfType(ANBEvenOdd),        // atom!("nth-of-type")
	Where(ForgivingSelector<'a>), // atom!("where")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum DirValue {
	Rtl, // atom!("rtl")
	Ltr, // atom!("ltr")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum NSPrefix {
	None,
	Wildcard,
	Named(Atom),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	ColumnCombintor,   // ||
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ANB {
	string: Atom,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ANBEvenOdd {
	string: Atom,
}

#[cfg(test)]
mod test {
	use oxc_allocator::Allocator;
	use serde_json::{from_str, json, to_string, Value};

	use super::*;
	use crate::{atom, Box, Span, Vec};

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Selector>(), 8);
		assert_eq!(::std::mem::size_of::<ForgivingSelector>(), 8);
		assert_eq!(::std::mem::size_of::<RelativeSelector>(), 8);
		assert_eq!(::std::mem::size_of::<Component>(), 24);
		assert_eq!(::std::mem::size_of::<PseudoElement>(), 1);
		assert_eq!(::std::mem::size_of::<LegacyPseudoElement>(), 1);
		assert_eq!(::std::mem::size_of::<Attribute>(), 40);
		assert_eq!(::std::mem::size_of::<AttributeMatch>(), 1);
		assert_eq!(::std::mem::size_of::<AttributeMatch>(), 1);
		assert_eq!(::std::mem::size_of::<PseudoClass>(), 1);
		assert_eq!(::std::mem::size_of::<PseudoFunction>(), 16);
		assert_eq!(::std::mem::size_of::<DirValue>(), 1);
		assert_eq!(::std::mem::size_of::<Combinator>(), 1);
		assert_eq!(::std::mem::size_of::<ANB>(), 8);
		assert_eq!(::std::mem::size_of::<ANBEvenOdd>(), 8);
	}
}
