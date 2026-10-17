#[cfg(feature = "visitable")]
use crate::visit::NodeId;
use crate::{
	CssAtomSet,
	traits::{AppliesTo, BoxPortion, BoxSide, PropertyGroup},
};
use bitmask_enum::bitmask;
use css_lexer::{Span, ToSpan};
use css_parse::{NodeMetadata, SemanticEq, ToCursors};

/// How unitless zero (0 without a unit) resolves in a given context.
///
/// For most Style Values, a `0` can be a drop-in replacement for `0px`, but
/// certain style values will provide discrete syntax for `0px` and `0`, meaning
/// they resolve to different things. For properties that accept both `<number>`
/// and `<length>`, unitless zero may resolve to a _different value_. Using a
/// piece of metadata to describe this can be helpful for linting/minifying -
/// avoiding a reduction in semantic meaning.
///
/// Examples:
/// - `width: 0px` == `width: 0` (unitless zero resolves to length)
/// - `line-height: 0px` != `line-height: 0` (unitless zero resolves to number = 0x multiplier)
/// - `tab-size: 0px` != `tab-size: 0` (unitless zero resolves to number = 0 tab characters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnitlessZeroResolves {
	/// Unitless zero resolves to a length (0 = 0px).
	#[default]
	Length,
	/// Unitless zero resolves to a number or percentage. NOT safe to reduce.
	Number,
}

#[bitmask(u32)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtRuleId {
	Charset,
	ColorProfile,
	Container,
	CounterStyle,
	FontFace,
	FontFeatureValues,
	FontPaletteValues,
	Import,
	Keyframes,
	Layer,
	Media,
	Namespace,
	Page,
	Property,
	Scope,
	StartingStyle,
	Supports,
	Document,
	WebkitKeyframes,
	MozDocument,
}

#[cfg(feature = "visitable")]
impl NodeId {
	/// Converts a NodeId to an AtRuleId if the node is an at-rule type.
	/// Returns `None` for non-at-rule nodes like StyleRule, Declaration, etc.
	pub fn to_at_rule_id(self) -> Option<AtRuleId> {
		match self {
			Self::CharsetRule => Some(AtRuleId::Charset),
			Self::ColorProfileRule => Some(AtRuleId::ColorProfile),
			Self::ContainerRule => Some(AtRuleId::Container),
			Self::CounterStyleRule => Some(AtRuleId::CounterStyle),
			Self::DocumentRule => Some(AtRuleId::Document),
			Self::FontFaceRule => Some(AtRuleId::FontFace),
			Self::FontFeatureValuesRule => Some(AtRuleId::FontFeatureValues),
			Self::FontPaletteValuesRule => Some(AtRuleId::FontPaletteValues),
			Self::KeyframesRule => Some(AtRuleId::Keyframes),
			Self::LayerRule => Some(AtRuleId::Layer),
			Self::MediaRule => Some(AtRuleId::Media),
			Self::MozDocumentRule => Some(AtRuleId::MozDocument),
			Self::NamespaceRule => Some(AtRuleId::Namespace),
			Self::PageRule => Some(AtRuleId::Page),
			Self::PropertyRule => Some(AtRuleId::Property),
			Self::ScopeRule => Some(AtRuleId::Scope),
			Self::StartingStyleRule => Some(AtRuleId::StartingStyle),
			Self::SupportsRule => Some(AtRuleId::Supports),
			Self::WebkitKeyframesRule => Some(AtRuleId::WebkitKeyframes),
			_ => None,
		}
	}
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VendorPrefixes {
	Moz,
	WebKit,
	O,
	Ms,
}

impl TryFrom<CssAtomSet> for VendorPrefixes {
	type Error = ();
	fn try_from(atom: CssAtomSet) -> Result<Self, Self::Error> {
		const VENDOR_FLAG: u32 = 0b00000000_10000000_00000000_00000000;
		const VENDORS: [VendorPrefixes; 4] =
			[VendorPrefixes::WebKit, VendorPrefixes::Moz, VendorPrefixes::Ms, VendorPrefixes::O];

		let atom_bits = atom as u32;
		if atom_bits & VENDOR_FLAG == 0 {
			return Err(());
		}
		let index = (atom_bits >> 21) & 0b11;
		Ok(VENDORS[index as usize])
	}
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeclarationKind {
	/// If a declaration has !important
	Important,
	/// If a declaration used a css-wide keyword, e.g. `inherit` or `revert-layer`.
	CssWideKeywords,
	/// If a declaration is custom, e.g `--foo`
	Custom,
	/// If a declaration is computed-time, e.g. using `calc()` or `var()`
	Computed,
	/// If a declaration is shorthand
	Shorthands,
	/// If a declaration is longhand
	Longhands,
}

/// Categories of nodes present in metadata, used for selector filtering.
#[bitmask(u32)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKinds {
	/// Contains unknown nodes
	Unknown,
	/// Contains style rules
	StyleRule,
	/// Contains at-rules (media, keyframes, etc.)
	AtRule,
	/// Contains Declarations
	Declaration,
	/// Contains function nodes
	Function,
	/// Node has an empty prelude
	EmptyPrelude,
	/// Node has a block which contains no declarations and no rules
	EmptyBlock,
	/// Node is nested within another node
	Nested,
	/// Node is deprecated (non-conforming, obsolete)
	Deprecated,
	/// Node is experimental (not yet standardized)
	Experimental,
	/// Node is non-standard (vendor-specific, not in spec)
	NonStandard,
	/// Node is a dimension value (length, angle, time, flex, etc.)
	Dimension,
	/// Node is a custom element or custom property
	Custom,
	/// Node has an effect on rendering: a declaration, or a rule which is not inert. Rules without
	/// a block (`@import`, `@layer a;`) always have an effect.
	Effective,
	/// Node has no effect on rendering: a rule whose block holds no declarations, and no rules
	/// other than inert ones. An inert node can be removed without changing what the sheet does.
	Inert,
}

/// Queryable properties a node exposes for selector matching.
/// Used by attribute selectors like `[name]` or `[name=value]`.
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyKind {
	/// Node has a queryable `name` property (declarations, named at-rules, functions)
	Name,
}

/// All PropertyKind variants for iteration.
pub const PROPERTY_KIND_VARIANTS: &[PropertyKind] = &[PropertyKind::Name];

/// OR-composable bitflag recording the set of CSS value types a substitution position accepts.
///
/// Used by [`Unresolved`](crate::Unresolved) to carry grammar-type knowledge at positions where a
/// substitution function appears but the slot cannot be fully typed at parse time.
///
/// `ANY` (all bits set) is used for `Custom` declaration bodies and substitution-function
/// internals where no type constraint applies.
#[bitmask(u32)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CssTypes {
	Length,
	Percentage,
	Number,
	Integer,
	Angle,
	Time,
	Frequency,
	Flex,
	Color,
	Keyword,
	Image,
	Url,
	String,
}

impl CssTypes {
	/// All bits set - use for untyped contexts (custom declarations, substitution internals).
	pub const ANY: CssTypes = CssTypes { bits: !0 };
}

/// Aggregated metadata computed from declarations within a block.
/// This allows efficient checking of what types of properties a block contains
/// without iterating through all declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CssMetadata {
	/// Bitwise OR of all PropertyGroup values
	pub property_groups: PropertyGroup,
	/// Bitwise OR of all AppliesTo values
	pub applies_to: AppliesTo,
	/// Bitwise OR of all BoxSide values
	pub box_sides: BoxSide,
	/// Bitwise OR of all BoxPortion values
	pub box_portions: BoxPortion,
	/// Bitwise OR of all DeclarationKind values
	pub declaration_kinds: DeclarationKind,
	/// Bitwise OR of all AtRuleIds in a Node
	pub used_at_rules: AtRuleId,
	/// Bitwise OR of all VendorPrefixes in a Node
	pub vendor_prefixes: VendorPrefixes,
	/// Bitwise OR of node categories present
	pub node_kinds: NodeKinds,
	/// Bitwise OR of queryable properties present
	pub property_kinds: PropertyKind,
	/// Bitwise OR of literal value kinds present in this node and its subtree
	pub value_kinds: CssTypes,
	/// How unitless zero resolves in this context (Length or Number)
	pub unitless_zero_resolves: UnitlessZeroResolves,
	/// Size of vector-based nodes (e.g., number of declarations, selector list length)
	pub size: u16,
	/// True if any substitution function (var(), env(), attr(), etc.) or Unresolved node is present.
	/// Enables subtree-skip optimisations in visitors and the minifier.
	pub uses_substitution: bool,
	/// OR-union of all CssTypes bits for substitution positions in this node.
	/// Accumulates upward through CssMetadata::merge for type inference.
	pub expected_value_kinds: CssTypes,
}

impl Default for CssMetadata {
	fn default() -> Self {
		Self {
			property_groups: PropertyGroup::none(),
			applies_to: AppliesTo::none(),
			box_sides: BoxSide::none(),
			box_portions: BoxPortion::none(),
			declaration_kinds: DeclarationKind::none(),
			used_at_rules: AtRuleId::none(),
			vendor_prefixes: VendorPrefixes::none(),
			node_kinds: NodeKinds::none(),
			property_kinds: PropertyKind::none(),
			value_kinds: CssTypes::none(),
			unitless_zero_resolves: UnitlessZeroResolves::default(),
			size: 0,
			uses_substitution: false,
			expected_value_kinds: CssTypes::none(),
		}
	}
}

impl CssMetadata {
	/// Returns true if this metadata is empty (contains no properties or at-rules)
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.property_groups == PropertyGroup::none()
			&& self.applies_to == AppliesTo::none()
			&& self.box_sides == BoxSide::none()
			&& self.box_portions == BoxPortion::none()
			&& self.declaration_kinds == DeclarationKind::none()
			&& self.used_at_rules == AtRuleId::none()
			&& self.vendor_prefixes == VendorPrefixes::none()
			&& self.node_kinds == NodeKinds::none()
			&& self.property_kinds == PropertyKind::none()
			&& self.unitless_zero_resolves == UnitlessZeroResolves::Length
			&& self.size == 0
			&& !self.uses_substitution
			&& self.expected_value_kinds == CssTypes::none()
			&& self.value_kinds == CssTypes::none()
	}

	/// Returns true if this block modifies any positioning-related properties.
	#[inline]
	pub fn modifies_box(&self) -> bool {
		!self.box_portions.is_none()
	}

	/// Returns true if metadata contains important declarations.
	#[inline]
	pub fn has_important(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Important)
	}

	/// Returns true if metadata contains custom properties.
	#[inline]
	pub fn has_custom_properties(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Custom)
	}

	/// Returns true if metadata contains computed values.
	#[inline]
	pub fn has_computed(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Computed)
	}

	/// Returns true if metadata contains shorthand properties.
	#[inline]
	pub fn has_shorthands(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Shorthands)
	}

	/// Returns true if metadata contains longhand properties.
	#[inline]
	pub fn has_longhands(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Longhands)
	}

	/// Returns true if metadata contains unknown nodes.
	#[inline]
	pub fn has_unknown(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Unknown)
	}

	/// Returns true if metadata contains vendor-prefixed properties.
	#[inline]
	pub fn has_vendor_prefixes(&self) -> bool {
		!self.vendor_prefixes.is_none()
	}

	/// Returns the vendor prefix if exactly one is present, None otherwise.
	#[inline]
	pub fn single_vendor_prefix(&self) -> Option<VendorPrefixes> {
		if self.vendor_prefixes.is_none() || self.vendor_prefixes.bits().count_ones() != 1 {
			None
		} else {
			Some(self.vendor_prefixes)
		}
	}

	/// Returns true if metadata contains any rule nodes.
	#[inline]
	pub fn has_rules(&self) -> bool {
		self.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule)
	}

	/// Returns true if metadata contains style rules.
	#[inline]
	pub fn has_style_rules(&self) -> bool {
		self.node_kinds.contains(NodeKinds::StyleRule)
	}

	/// Returns true if metadata contains at-rules.
	#[inline]
	pub fn has_at_rules(&self) -> bool {
		self.node_kinds.contains(NodeKinds::AtRule)
	}

	/// Returns true if metadata contains function nodes.
	#[inline]
	pub fn has_functions(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Function)
	}

	/// Returns true if metadata contains deprecated nodes.
	#[inline]
	pub fn is_deprecated(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Deprecated)
	}

	/// Returns true if metadata contains experimental nodes.
	#[inline]
	pub fn is_experimental(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Experimental)
	}

	/// Returns true if metadata contains non-standard nodes.
	#[inline]
	pub fn is_non_standard(&self) -> bool {
		self.node_kinds.contains(NodeKinds::NonStandard)
	}

	/// Returns true if metadata contains dimension values.
	#[inline]
	pub fn is_dimension(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Dimension)
	}

	/// Returns true if metadata contains nodes with the given property kind.
	#[inline]
	pub fn has_property_kind(&self, kind: PropertyKind) -> bool {
		self.property_kinds.contains(kind)
	}

	/// Returns true if any substitution function or Unresolved node is present in this subtree.
	#[inline]
	pub fn has_substitution(&self) -> bool {
		self.uses_substitution
	}

	/// Returns true if this node or its subtree contains any of the given value kinds.
	#[inline]
	pub fn has_value_kinds(&self, kinds: CssTypes) -> bool {
		self.value_kinds.intersects(kinds)
	}

	/// Returns true if this is an empty container (no declarations, no nested rules).
	#[inline]
	pub fn is_empty_container(&self) -> bool {
		self.node_kinds.contains(NodeKinds::EmptyBlock)
	}

	/// Returns true if anything in this node or its subtree has an effect on rendering.
	#[inline]
	pub fn has_effect(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Effective)
	}

	/// Returns true if this node has no effect on rendering, so it can be removed.
	///
	/// For [self metadata](css_parse::NodeWithMetadata::self_metadata) this describes the node
	/// itself. Node kinds aggregate upwards, so for a subtree it only says that the subtree holds
	/// an inert node somewhere.
	#[inline]
	pub fn is_inert(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Inert)
	}

	/// Returns true if this node, or a node in its subtree, is nested inside another node.
	#[inline]
	pub fn is_nested(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Nested)
	}

	/// Returns true if this node has a prelude which covers no source text, such as the omitted
	/// selector of `@page {}` or the anonymous layer of `@layer {}`.
	#[inline]
	pub fn has_empty_prelude(&self) -> bool {
		self.node_kinds.contains(NodeKinds::EmptyPrelude)
	}

	/// Returns true if this node can be a container (has StyleRule or AtRule kind).
	#[inline]
	pub fn can_be_empty(&self) -> bool {
		self.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule)
	}
}

impl NodeMetadata for CssMetadata {
	#[inline]
	fn merge(mut self, other: Self) -> Self {
		self.property_groups |= other.property_groups;
		self.applies_to |= other.applies_to;
		self.box_sides |= other.box_sides;
		self.box_portions |= other.box_portions;
		self.declaration_kinds |= other.declaration_kinds;
		self.used_at_rules |= other.used_at_rules;
		self.vendor_prefixes |= other.vendor_prefixes;
		self.node_kinds |= other.node_kinds;
		self.property_kinds |= other.property_kinds;
		self.value_kinds |= other.value_kinds;
		// For unitless_zero_resolves, we keep Number if either side has it (conservative)
		if other.unitless_zero_resolves == UnitlessZeroResolves::Number {
			self.unitless_zero_resolves = UnitlessZeroResolves::Number;
		}
		self.size = self.size.max(other.size);
		self.uses_substitution |= other.uses_substitution;
		self.expected_value_kinds |= other.expected_value_kinds;
		self
	}

	#[inline]
	fn with_size(mut self, size: u16) -> Self {
		self.size = size;
		self
	}

	#[inline]
	fn with_declaration(mut self) -> Self {
		self.node_kinds |= NodeKinds::Declaration | NodeKinds::Effective;
		self
	}

	#[inline]
	fn with_nested(mut self) -> Self {
		self.node_kinds |= NodeKinds::Nested;
		self
	}
}

// Metadata is not serialized to tokens but providing these simplifies ToCursors/ToSpan impls
impl ToCursors for CssMetadata {
	fn to_cursors(&self, _: &mut impl css_parse::CursorSink) {}
}
impl ToSpan for CssMetadata {
	fn to_span(&self) -> Span {
		Span::DUMMY
	}
}

// CssTypes is not serialized to tokens; these no-op impls let it sit as a
// non-node field on Unresolved under derive(ToCursors)/derive(ToSpan).
impl ToCursors for CssTypes {
	fn to_cursors(&self, _: &mut impl css_parse::CursorSink) {}
}
impl ToSpan for CssTypes {
	fn to_span(&self) -> Span {
		Span::DUMMY
	}
}
impl SemanticEq for CssTypes {
	fn semantic_eq(&self, other: &Self, _source_text: &str) -> bool {
		self == other
	}
}

impl SemanticEq for CssMetadata {
	fn semantic_eq(&self, other: &Self, _source_text: &str) -> bool {
		self == other
	}
}

macro_rules! impl_token_metadata {
	($($token:tt),* $(,)?) => {
		$(
			impl css_parse::NodeWithMetadata<CssMetadata> for css_parse::T![$token] {
				fn metadata(&self) -> CssMetadata {
					CssMetadata::default()
				}
			}
		)*
	};
}

impl_token_metadata!(
	Ident,
	Number,
	Dimension,
	Hash,
	AtKeyword,
	String,
	Function,
	Url,
	Delim,
	Colon,
	Semicolon,
	Comma,
	LeftCurly,
	RightCurly,
	LeftSquare,
	RightSquare,
	LeftParen
);

macro_rules! impl_leaf_metadata {
	($($t:ty),* $(,)?) => {
		$(
			impl css_parse::NodeWithMetadata<CssMetadata> for $t {
				fn metadata(&self) -> CssMetadata {
					CssMetadata::default()
				}
			}
		)*
	};
}
impl_leaf_metadata!(
	css_parse::token_macros::delim::Slash,
	css_parse::token_macros::delim::Or,
	css_parse::token_macros::delim::Plus,
	css_parse::token_macros::delim::Tilde,
	css_parse::token_macros::delim::Star,
	css_parse::token_macros::delim::Question,
	css_parse::token_macros::delim::Underscore,
	css_parse::token_macros::delim::Eq,
	css_parse::token_macros::delim::Gt,
	css_parse::token_macros::delim::Lt,
	css_parse::token_macros::delim::Dot,
	css_parse::token_macros::delim::And,
	css_parse::token_macros::delim::At,
	css_parse::token_macros::delim::Caret,
	css_parse::token_macros::delim::Dash,
	css_parse::token_macros::delim::Dollar,
	css_parse::token_macros::delim::Bang,
	css_parse::token_macros::delim::Percent,
	css_parse::token_macros::delim::Hash,
	css_parse::token_macros::delim::Backtick,
	css_parse::token_macros::double::ColonColon,
	css_parse::token_macros::double::PipePipe,
	css_parse::token_macros::double::EqualEqual,
	css_parse::token_macros::double::BangEqual,
	css_parse::token_macros::double::TildeEqual,
	css_parse::token_macros::double::PipeEqual,
	css_parse::token_macros::double::CaretEqual,
	css_parse::token_macros::double::DollarEqual,
	css_parse::token_macros::double::StarEqual,
	css_parse::token_macros::Any,
	css_parse::token_macros::DashedIdent,
	css_parse::token_macros::Whitespace,
	css_parse::token_macros::RightParen,
	crate::HexColor,
	css_parse::Comparison,
);

impl<'a, T: css_parse::NodeWithMetadata<CssMetadata>> css_parse::NodeWithMetadata<CssMetadata>
	for css_parse::Vec<'a, T>
{
	fn metadata(&self) -> CssMetadata {
		self.iter().fold(CssMetadata::default(), |acc, item| NodeMetadata::merge(acc, item.metadata()))
	}
}

impl<'a, T: css_parse::NodeWithMetadata<CssMetadata>, const MIN: usize> css_parse::NodeWithMetadata<CssMetadata>
	for css_parse::CommaSeparated<'a, T, MIN>
{
	fn metadata(&self) -> CssMetadata {
		self.into_iter().fold(CssMetadata::default(), |acc, (item, _comma)| NodeMetadata::merge(acc, item.metadata()))
	}
}

macro_rules! impl_optionals_metadata {
	($name:ident, $($T:ident => $v:ident),+) => {
		impl<$($T: css_parse::NodeWithMetadata<CssMetadata>),+>
			css_parse::NodeWithMetadata<CssMetadata> for css_parse::$name<$($T),+>
		{
			fn metadata(&self) -> CssMetadata {
				let css_parse::$name($($v),+) = self;
				let mut meta = CssMetadata::default();
				$(
					if let Some(val) = $v {
						meta = NodeMetadata::merge(meta, val.metadata());
					}
				)+
				meta
			}
		}
	};
}

impl_optionals_metadata!(Optionals2, A => a, B => b);
impl_optionals_metadata!(Optionals3, A => a, B => b, C => c);
impl_optionals_metadata!(Optionals4, A => a, B => b, C => c, D => d);
impl_optionals_metadata!(Optionals5, A => a, B => b, C => c, D => d, E => e);

macro_rules! impl_tuple_metadata {
	($($T:ident),+) => {
		impl<$($T: css_parse::NodeWithMetadata<CssMetadata>),+>
			css_parse::NodeWithMetadata<CssMetadata> for ($($T,)+)
		{
			#[allow(non_snake_case)]
			fn metadata(&self) -> CssMetadata {
				let ($($T,)+) = self;
				let mut meta = CssMetadata::default();
				$(
					meta = NodeMetadata::merge(meta, $T.metadata());
				)+
				meta
			}
		}
	};
}

impl_tuple_metadata!(A, B);
impl_tuple_metadata!(A, B, C);
impl_tuple_metadata!(A, B, C, D);
impl_tuple_metadata!(A, B, C, D, E);
impl_tuple_metadata!(A, B, C, D, E, F);
impl_tuple_metadata!(A, B, C, D, E, F, G);
impl_tuple_metadata!(A, B, C, D, E, F, G, H);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, StyleSheet};
	use css_lexer::Lexer;
	use css_parse::{Arena, NodeMetadata, NodeWithMetadata, Parser};

	#[test]
	fn test_block_metadata_merge() {
		let meta1 = CssMetadata {
			property_groups: PropertyGroup::Color,
			declaration_kinds: DeclarationKind::Important,
			..Default::default()
		};

		let meta2 = CssMetadata {
			property_groups: PropertyGroup::Position,
			declaration_kinds: DeclarationKind::Custom,
			..Default::default()
		};

		let merged = meta1.merge(meta2);

		assert!(merged.property_groups.contains(PropertyGroup::Color));
		assert!(merged.property_groups.contains(PropertyGroup::Position));
		assert!(merged.declaration_kinds.contains(DeclarationKind::Important));
		assert!(merged.declaration_kinds.contains(DeclarationKind::Custom));
	}

	#[test]
	fn test_stylesheet_metadata_simple() {
		let css = "body { color: red; width: 100px; }";
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&alloc, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.property_groups.contains(PropertyGroup::Color));
		assert!(metadata.property_groups.contains(PropertyGroup::Sizing));
		assert!(metadata.modifies_box());
		assert!(metadata.has_longhands());
	}

	#[test]
	fn test_stylesheet_metadata_with_important() {
		let css = "body { color: red !important; }";
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&alloc, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.has_important());
		assert!(metadata.property_groups.contains(PropertyGroup::Color));
	}

	#[test]
	fn test_stylesheet_metadata_custom_properties() {
		let css = "body { --custom: value; }";
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&alloc, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.has_custom_properties());
	}

	#[test]
	fn test_stylesheet_metadata_nested_media() {
		let css = "@media screen { body { color: red; } }";
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&alloc, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.property_groups.contains(PropertyGroup::Color));
		assert!(metadata.used_at_rules.contains(AtRuleId::Media));
	}

	fn first_rule_metadata(css: &str) -> CssMetadata {
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&alloc, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();
		match stylesheet.rules.first().expect("stylesheet has no rules") {
			crate::Rule::Style(rule) => rule.self_metadata(),
			crate::Rule::Media(rule) => rule.self_metadata(),
			crate::Rule::FontFace(rule) => rule.self_metadata(),
			crate::Rule::Keyframes(rule) => rule.self_metadata(),
			crate::Rule::Layer(rule) => rule.self_metadata(),
			crate::Rule::Page(rule) => rule.self_metadata(),
			crate::Rule::Scope(rule) => rule.self_metadata(),
			rule => panic!("unexpected rule kind {rule:?}"),
		}
	}

	fn stylesheet_metadata(css: &str) -> CssMetadata {
		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&alloc, css, lexer);
		parser.parse::<StyleSheet>().unwrap().metadata()
	}

	#[test]
	fn nested_rules_are_marked_nested() {
		assert!(!stylesheet_metadata("a { color: red }").is_nested());
		assert!(!stylesheet_metadata("@media screen { color: red }").is_nested());
		assert!(stylesheet_metadata("a { b { color: red } }").is_nested());
		assert!(stylesheet_metadata("@media screen { a { color: red } }").is_nested());
	}

	#[test]
	fn omitted_preludes_are_marked_empty() {
		assert!(first_rule_metadata("@page {}").has_empty_prelude());
		assert!(first_rule_metadata("@layer { a { color: red } }").has_empty_prelude());
		assert!(first_rule_metadata("@scope { a { color: red } }").has_empty_prelude());
	}

	#[test]
	fn written_preludes_are_not_marked_empty() {
		assert!(!first_rule_metadata("@page :left {}").has_empty_prelude());
		assert!(!first_rule_metadata("@layer base { a { color: red } }").has_empty_prelude());
		assert!(!first_rule_metadata("@scope (.card) { a { color: red } }").has_empty_prelude());
	}

	#[test]
	fn rules_with_an_empty_block_are_marked_empty() {
		assert!(first_rule_metadata("a {}").is_empty_container());
		assert!(first_rule_metadata("@media screen {}").is_empty_container());
		assert!(first_rule_metadata("@page {}").is_empty_container());
		assert!(first_rule_metadata("@keyframes fade {}").is_empty_container());
	}

	#[test]
	fn rules_with_a_filled_block_are_not_marked_empty() {
		assert!(!first_rule_metadata("a { color: red }").is_empty_container());
		assert!(!first_rule_metadata("nav { a {} }").is_empty_container());
		assert!(!first_rule_metadata("@media screen { a { color: red } }").is_empty_container());
		assert!(!first_rule_metadata("@font-face { font-display: swap }").is_empty_container());
	}

	#[test]
	fn a_rule_is_either_inert_or_effective_never_both() {
		for (css, inert) in [("a {}", true), ("a { color: red }", false), ("nav { a {} }", true)] {
			let meta = first_rule_metadata(css);
			assert_eq!(meta.is_inert(), inert, "{css}");
			assert_eq!(meta.has_effect(), !inert, "{css}");
		}
	}

	#[test]
	fn a_subtree_can_hold_both_inert_and_effective_nodes() {
		// Node kinds merge with OR, so for anything but a node's own metadata the two bits answer
		// separate existential questions and are not complements: this sheet holds one of each.
		// Deciding a rule is inert needs "holds nothing effective", which the `Inert` bit alone
		// cannot answer.
		let meta = stylesheet_metadata("a {}\nb { color: red }");
		assert!(meta.is_inert());
		assert!(meta.has_effect());
	}

	#[test]
	fn rule_holding_only_empty_rules_is_inert_but_not_empty() {
		let meta = first_rule_metadata("nav { a {} }");
		assert!(meta.is_inert());
		// The block holds a rule, so it is not an empty container in the literal sense `:empty`
		// matches on.
		assert!(!meta.is_empty_container());
	}

	#[test]
	fn rule_with_unknown_value_has_effect() {
		let meta = first_rule_metadata("a { color: fnord }");
		assert!(meta.has_effect());
		assert!(!meta.is_inert());
	}

	#[test]
	fn at_rule_descriptors_have_effect() {
		let meta = first_rule_metadata("@font-face { font-display: swap }");
		assert!(meta.has_effect());
		assert!(!meta.is_inert());
	}

	#[test]
	fn keyframes_with_only_empty_keyframes_is_inert() {
		let meta = first_rule_metadata("@keyframes fade { 0% {} 100% {} }");
		assert!(meta.is_inert());
	}

	#[test]
	fn statement_at_rule_keeps_containing_rule_effective() {
		let meta = first_rule_metadata("@media screen { @layer a; }");
		assert!(meta.has_effect());
		assert!(!meta.is_inert());
	}

	#[test]
	fn unmarked_block_rules_are_never_inert() {
		// `@layer a {}` declares layer order, so it must survive even when empty.
		let meta = first_rule_metadata("@layer a {}");
		assert!(meta.has_effect());
		assert!(!meta.is_inert());
	}

	// Child leaf types carrying distinct node_kinds bits, used to verify delegation
	// propagates and merges children's metadata upward.
	#[derive(csskit_derives::NodeWithMetadata)]
	#[metadata(node_kinds = StyleRule)]
	struct ChildA;

	#[derive(csskit_derives::NodeWithMetadata)]
	#[metadata(node_kinds = AtRule)]
	struct ChildB;

	// Structs merge every field's metadata into self_metadata.
	#[derive(csskit_derives::NodeWithMetadata)]
	#[metadata(node_kinds = Function)]
	struct StructParent {
		a: ChildA,
		b: ChildB,
	}

	// Fields marked #[metadata(skip)] contribute nothing.
	#[derive(csskit_derives::NodeWithMetadata)]
	struct StructSkippedField {
		a: ChildA,
		#[metadata(skip)]
		#[allow(dead_code)]
		b: ChildB,
	}

	// Enums merge the active variant's fields into self_metadata.
	#[derive(csskit_derives::NodeWithMetadata)]
	enum EnumParent {
		Named { a: ChildA, b: ChildB },
		Tuple(ChildA),
		Empty,
	}

	#[test]
	fn test_struct_merges_all_fields() {
		let node = StructParent { a: ChildA, b: ChildB };
		let meta = node.metadata();
		// self_metadata bit plus both children.
		assert!(meta.node_kinds.contains(NodeKinds::Function));
		assert!(meta.node_kinds.contains(NodeKinds::StyleRule));
		assert!(meta.node_kinds.contains(NodeKinds::AtRule));
	}

	#[test]
	fn test_struct_skips_marked_field() {
		let meta = StructSkippedField { a: ChildA, b: ChildB }.metadata();
		assert!(meta.node_kinds.contains(NodeKinds::StyleRule));
		assert!(!meta.node_kinds.contains(NodeKinds::AtRule));
	}

	#[test]
	fn test_enum_named_variant() {
		let meta = EnumParent::Named { a: ChildA, b: ChildB }.metadata();
		assert!(meta.node_kinds.contains(NodeKinds::StyleRule));
		assert!(meta.node_kinds.contains(NodeKinds::AtRule));
		assert!(!meta.node_kinds.contains(NodeKinds::Function));
	}

	#[test]
	fn test_enum_tuple_variant() {
		let meta = EnumParent::Tuple(ChildA).metadata();
		assert!(meta.node_kinds.contains(NodeKinds::StyleRule));
		assert!(!meta.node_kinds.contains(NodeKinds::AtRule));
	}

	#[test]
	fn test_enum_empty_variant() {
		let meta = EnumParent::Empty.metadata();
		assert!(meta.is_empty());
	}

	#[test]
	fn test_vendor_prefixes_try_from() {
		// Vendor-prefixed atoms should convert successfully
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_WebkitTransform), Ok(VendorPrefixes::WebKit));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_WebkitAnimation), Ok(VendorPrefixes::WebKit));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_WebkitLineClamp), Ok(VendorPrefixes::WebKit));

		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MozAppearance), Ok(VendorPrefixes::Moz));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MozAny), Ok(VendorPrefixes::Moz));

		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MsFullscreen), Ok(VendorPrefixes::Ms));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MsBackdrop), Ok(VendorPrefixes::Ms));

		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_OPlaceholder), Ok(VendorPrefixes::O));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_OScrollbar), Ok(VendorPrefixes::O));

		// Non-vendor atoms should fail
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Px), Err(()));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Em), Err(()));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Auto), Err(()));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Transform), Err(()));
	}

	#[test]
	fn size_baseline_css_metadata() {
		// `property_groups` is a u128 bitmask, so CssMetadata is align 16 and rounds up to 48 bytes.
		// The payload is well under that, leaving spare bytes for new flag fields at no cost.
		assert_eq!(std::mem::size_of::<CssMetadata>(), 48);
	}

	#[test]
	fn test_substitution_fields_default() {
		let meta = CssMetadata::default();
		assert!(!meta.uses_substitution);
		assert_eq!(meta.expected_value_kinds, CssTypes::none());
		assert!(meta.is_empty());
	}

	#[test]
	fn test_substitution_fields_merge() {
		let meta1 = CssMetadata {
			uses_substitution: true,
			expected_value_kinds: CssTypes::Length | CssTypes::Percentage,
			..Default::default()
		};

		let meta2 = CssMetadata { expected_value_kinds: CssTypes::Color, ..Default::default() };

		let merged = NodeMetadata::merge(meta1, meta2);
		assert!(merged.uses_substitution);
		assert!(merged.expected_value_kinds.contains(CssTypes::Length));
		assert!(merged.expected_value_kinds.contains(CssTypes::Percentage));
		assert!(merged.expected_value_kinds.contains(CssTypes::Color));
	}

	#[test]
	fn test_has_substitution() {
		let mut meta = CssMetadata::default();
		assert!(!meta.has_substitution());
		meta.uses_substitution = true;
		assert!(meta.has_substitution());
	}

	#[test]
	fn test_is_empty_with_substitution() {
		let mut meta = CssMetadata::default();
		assert!(meta.is_empty());
		meta.uses_substitution = true;
		assert!(!meta.is_empty());
		let meta2 = CssMetadata { expected_value_kinds: CssTypes::Number, ..Default::default() };
		assert!(!meta2.is_empty());
	}
}
