use crate::{
	AppliesTo, BoxPortion, BoxSide, CssAtomSet, CssMetadata, DeclarationKind, DeclarationMetadata, Inherits, Longhand,
	NodeKinds, PropertyGroup, PropertyKind, Shorthand, Unresolved, VendorPrefixes, values,
};
use css_lexer::Kind;
use css_parse::{
	AtomSet, Box, ComponentValues, Cursor, Declaration, DeclarationValue, Diagnostic, KindSet, NodeMetadata,
	NodeWithMetadata, Parser, Peek, Result as ParserResult, SemanticEq as SemanticEqTrait, State, T,
};
use csskit_derives::*;
use csskit_proc_macro::node;
use std::{fmt::Debug, hash::Hash};

// The build.rs generates a list of CSS properties from the value mods
include!(concat!(env!("OUT_DIR"), "/css_apply_properties.rs"));

#[node]
#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Custom<'a>(pub ComponentValues<'a>);

#[node]
#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Unknown<'a>(pub ComponentValues<'a>);

macro_rules! style_value {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		#[node]
		#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
		pub enum StyleValue<'a> {
			#[cfg_attr(feature = "visitable", visit(skip))]
			Initial(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Inherit(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Unset(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Revert(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			RevertLayer(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			RevertRule(T![Ident]),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Custom(Custom<'a>),
			/// A whole-value substitution (e.g. `background: var(--a) var(--b) calc(..)`) whose slot assignment can't be
			/// resolved at parse time.
			#[cfg_attr(feature = "visitable", visit(skip))]
		#[cfg_attr(feature = "serde", serde(untagged))]
		Unresolved(Unresolved<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Unknown(Unknown<'a>),
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(Box<'a, values::$ty$(<$a>)?>),
			)+
		}
	}
}

apply_properties!(style_value);

impl<'a> NodeWithMetadata<CssMetadata> for StyleValue<'a> {
	fn metadata(&self) -> CssMetadata {
		macro_rules! metadata {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match self {
					Self::Initial(_) |
					Self::Inherit(_)|
					Self::Unset(_)|
					Self::Revert(_)|
					Self::RevertLayer(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::CssWideKeywords,
							..Default::default()
						}
					}
					Self::RevertRule(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::CssWideKeywords,
							..Default::default()
						}
					}
					Self::Custom(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::Custom,
							..Default::default()
						}
					}
					Self::Unresolved(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::Computed,
							uses_substitution: true,
							..Default::default()
						}
					},
					Self::Unknown(_) => {
						CssMetadata {
							node_kinds: NodeKinds::Unknown,
							..Default::default()
						}
					},
					$(
					Self::$name(v) => {
						let mut declaration_kinds = DeclarationKind::none();
						if values::$ty::shorthand().is_some() {
							declaration_kinds |= DeclarationKind::Shorthands;
						} else {
							declaration_kinds |= DeclarationKind::Longhands;
						}
						let self_meta = CssMetadata {
							property_groups: values::$ty::property_group(),
							applies_to: values::$ty::applies_to(),
							box_sides: values::$ty::box_side(),
							box_portions: values::$ty::box_portion(),
							declaration_kinds,
							unitless_zero_resolves: values::$ty::unitless_zero_resolves(),
							..Default::default()
						};
						let inner_meta = v.metadata();
						css_parse::NodeMetadata::merge(self_meta, inner_meta)
					}
					)+
				}
			};
		}
		apply_properties!(metadata)
	}
}

impl<'a> StyleValue<'a> {
	/// Returns the initial value string for a given property name.
	/// This is useful when you have `StyleValue::Initial` and need to know what the initial value
	/// should be based on the property name.
	pub fn initial_by_name(property_name: CssAtomSet) -> Option<&'static str> {
		macro_rules! get_initial_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::initial()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_initial_by_name)
	}

	/// Returns the inherits value for a given property name.
	pub fn inherits_by_name(property_name: CssAtomSet) -> Option<Inherits> {
		macro_rules! get_inherits_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::inherits()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_inherits_by_name)
	}

	/// Returns the applies_to value for a given property name.
	pub fn applies_to_by_name(property_name: CssAtomSet) -> Option<AppliesTo> {
		macro_rules! get_applies_to_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::applies_to()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_applies_to_by_name)
	}

	/// Returns the property_group for a given property name.
	pub fn property_group_by_name(property_name: CssAtomSet) -> Option<PropertyGroup> {
		macro_rules! get_property_group_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::property_group()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_property_group_by_name)
	}

	/// Returns the box_side for a given property name.
	pub fn box_side_by_name(property_name: CssAtomSet) -> Option<BoxSide> {
		macro_rules! get_box_side_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::box_side()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_box_side_by_name)
	}

	/// Returns the box_portion for a given property name.
	pub fn box_portion_by_name(property_name: CssAtomSet) -> Option<BoxPortion> {
		macro_rules! get_box_portion_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::box_portion()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_box_portion_by_name)
	}

	/// Returns what a shorthand property does to the properties it covers, or [None] if the
	/// property is not a shorthand.
	pub fn shorthand_by_name(property_name: CssAtomSet) -> Option<&'static Shorthand> {
		macro_rules! get_shorthand_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => values::$ty::shorthand(),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_shorthand_by_name)
	}

	/// Returns what the shorthands which cover a given property name do to it, or [None] if no
	/// shorthand covers it.
	pub fn longhand_by_name(property_name: CssAtomSet) -> Option<&'static Longhand> {
		macro_rules! get_longhand_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => values::$ty::longhand(),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_longhand_by_name)
	}
}

impl<'a> DeclarationValue<'a, CssMetadata> for StyleValue<'a> {
	fn declaration_metadata(decl: &Declaration<'a, Self, CssMetadata>) -> CssMetadata {
		// Mark this node as a declaration
		let mut meta = decl.value.metadata().with_declaration();
		if decl.important.is_some() {
			meta.declaration_kinds |= DeclarationKind::Important;
		}
		// Check if this is a custom property (dashed ident)
		if decl.name.is_dashed_ident() {
			meta.node_kinds |= NodeKinds::Custom;
		}
		// Check if the value is unknown
		if decl.value.is_unknown() {
			meta.node_kinds |= NodeKinds::Unknown;
		}
		// Extract vendor prefix from property name cursor
		let cursor: Cursor = decl.name.into();
		meta.vendor_prefixes =
			CssAtomSet::from_bits(cursor.token().atom_bits()).try_into().unwrap_or(VendorPrefixes::none());
		// Declarations always have a name property
		meta.property_kinds |= PropertyKind::Name;
		meta
	}

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let atom = p.to_atom::<CssAtomSet>(c);
		c.token().is_dashed_ident()
			|| crate::property_atoms::CSS_PROPERTY_ATOMS.contains(&atom)
			|| CSS_VENDOR_PROPERTY_ATOMS.contains(&atom)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn is_custom(&self) -> bool {
		matches!(self, Self::Custom(_))
	}

	fn is_initial(&self) -> bool {
		matches!(self, Self::Initial(_))
	}

	fn is_inherit(&self) -> bool {
		matches!(self, Self::Inherit(_))
	}

	fn is_unset(&self) -> bool {
		matches!(self, Self::Unset(_))
	}

	fn is_revert(&self) -> bool {
		matches!(self, Self::Revert(_))
	}

	fn is_revert_layer(&self) -> bool {
		matches!(self, Self::RevertLayer(_))
	}

	fn is_revert_rule(&self) -> bool {
		matches!(self, Self::RevertRule(_))
	}

	fn needs_computing(&self) -> bool {
		self.metadata().has_computed()
	}

	fn parse_custom_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Custom>().map(Self::Custom)
	}

	fn is_computed_declaration_value<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if !<T![Function]>::peek(p, c) {
			return false;
		}
		let atom = p.to_atom::<CssAtomSet>(c);
		values::is_substitution_function(atom) || crate::is_math_function(atom)
	}

	fn parse_computed_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Unresolved>().map(Self::Unresolved)
	}

	fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if c == Kind::Ident {
			match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::Initial => return Ok(Self::Initial(p.parse::<T![Ident]>()?)),
				CssAtomSet::Inherit => return Ok(Self::Inherit(p.parse::<T![Ident]>()?)),
				CssAtomSet::Unset => return Ok(Self::Unset(p.parse::<T![Ident]>()?)),
				CssAtomSet::Revert => return Ok(Self::Revert(p.parse::<T![Ident]>()?)),
				CssAtomSet::RevertLayer => return Ok(Self::RevertLayer(p.parse::<T![Ident]>()?)),
				CssAtomSet::RevertRule => return Ok(Self::RevertRule(p.parse::<T![Ident]>()?)),
				_ => {}
			}
		}
		macro_rules! parse_declaration_value {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $atom: ident,)+ ) => {
				match p.to_atom::<CssAtomSet>(name) {
					$(CssAtomSet::$atom => p.parse::<Box<values::$ty>>().map(Self::$name),)+
					_ => Err(Diagnostic::new(name, Diagnostic::unexpected))?,
				}
			}
		}
		apply_properties!(parse_declaration_value)
	}

	fn parse_unknown_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Unknown>().map(Self::Unknown)
	}
}

impl<'a> SemanticEqTrait for crate::StyleValue<'a> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		macro_rules! semantic_eq {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match (self, other) {
					(Self::Initial(_), Self::Initial(_)) => true,
					(Self::Inherit(_), Self::Inherit(_)) => true,
					(Self::Unset(_), Self::Unset(_)) => true,
					(Self::Revert(_), Self::Revert(_)) => true,
					(Self::RevertLayer(_), Self::RevertLayer(_)) => true,
					(Self::RevertRule(_), Self::RevertRule(_)) => true,
					(Self::Custom(a), Self::Custom(b)) => a.semantic_eq(b, source_text),
					(Self::Unresolved(a), Self::Unresolved(b)) => a.semantic_eq(b, source_text),
					(Self::Unknown(a), Self::Unknown(b)) => a.semantic_eq(b, source_text),
					$((Self::$name(a), Self::$name(b)) => a.semantic_eq(b, source_text),)+
					(_, _) => false,
				}
			};
		}
		apply_properties!(semantic_eq)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, CssMetadata, ShorthandReset, Writes};
	use css_lexer::Lexer;
	use css_parse::{Arena, Declaration, Parser, assert_parse};

	type Property<'a> = Declaration<'a, StyleValue<'a>, CssMetadata>;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Property, "width:inherit", Property { value: StyleValue::Inherit(_), .. });
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width:inherit!important",
			Property { value: StyleValue::Inherit(_), important: Some(_), .. }
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width:revert;",
			Property { value: StyleValue::Revert(_), semicolon: Some(_), .. }
		);
		assert_parse!(CssAtomSet::ATOMS, Property, "width:var(--a)", Property { value: StyleValue::Width(_), .. });
		assert_parse!(CssAtomSet::ATOMS, Property, "width: var(--a)", Property { value: StyleValue::Width(_), .. });
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width: calc(100px + 50px)",
			Property { value: StyleValue::Width(_), .. }
		);

		assert_parse!(CssAtomSet::ATOMS, Property, "float:none!important");
		assert_parse!(CssAtomSet::ATOMS, Property, "width:1px");
		assert_parse!(CssAtomSet::ATOMS, Property, "width:min(1px, 2px)");
		assert_parse!(CssAtomSet::ATOMS, Property, "border:1px solid var(--red)");
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"background:var(--background) var(--select-arrow) calc(100% - 12px) 50%",
			Property { value: StyleValue::Unresolved(_), .. }
		);
		// Should still parse unknown properties
		assert_parse!(CssAtomSet::ATOMS, Property, "dunno:like whatever");
		assert_parse!(CssAtomSet::ATOMS, Property, "rotate:1.21gw");
		assert_parse!(CssAtomSet::ATOMS, Property, "_background:black");
		assert_parse!(CssAtomSet::ATOMS, Property, "--custom:{foo:{bar};baz:(bing);}");
	}

	#[test]
	fn test_property_validation() {
		let alloc = Arena::new();

		let input = "width:1px";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&alloc, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(!decl.value.is_unknown(), "width should be recognized as a known property");

		let input = "notarealproperty:value";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&alloc, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(decl.value.is_unknown(), "notarealproperty should be parsed as unknown");

		let input = "-webkit-filter:blur(4px)";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&alloc, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(!decl.value.is_unknown(), "-webkit-filter should be recognized as a known property");

		let input = "--custom:value";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&alloc, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(decl.value.is_custom(), "--custom should be parsed as custom property");
	}

	#[test]
	fn exposes_additive_shorthand_reset_metadata() {
		let border = StyleValue::shorthand_by_name(CssAtomSet::Border).unwrap();
		assert!(border.longhands.contains(&CssAtomSet::BorderWidth));
		assert!(border.longhands.contains(&CssAtomSet::BorderTopWidth));
		assert_eq!(border.resets, ShorthandReset::Properties(&[CssAtomSet::BorderImage]));
		assert_eq!(
			StyleValue::longhand_by_name(CssAtomSet::BorderLeftColor).unwrap().shorthands,
			[CssAtomSet::BorderLeft, CssAtomSet::BorderColor, CssAtomSet::Border]
		);
		assert_eq!(StyleValue::shorthand_by_name(CssAtomSet::Margin).unwrap().resets, ShorthandReset::Properties(&[]));
		assert_eq!(StyleValue::shorthand_by_name(CssAtomSet::All).unwrap().resets, ShorthandReset::All);
		assert_eq!(StyleValue::longhand_by_name(CssAtomSet::BorderImageSource).unwrap().reset_by, [CssAtomSet::Border]);
		assert!(StyleValue::shorthand_by_name(CssAtomSet::FontSize).is_none());
	}

	#[test]
	fn exposes_slots_of_shorthand() {
		use crate::Slot;
		assert_eq!(
			StyleValue::shorthand_by_name(CssAtomSet::Font).unwrap().writes,
			Some(Writes::Slots(&[
				Slot { property: CssAtomSet::FontStyle, before: "", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::FontVariant, before: "", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::FontWeight, before: "", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::FontWidth, before: "", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::FontSize, before: "", after: "", optional: false, copies: false },
				Slot { property: CssAtomSet::LineHeight, before: "/", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::FontFamily, before: "", after: "", optional: false, copies: false },
			]))
		);
		assert_eq!(
			StyleValue::shorthand_by_name(CssAtomSet::Flex).unwrap().writes,
			Some(Writes::Slots(&[
				Slot { property: CssAtomSet::FlexGrow, before: "", after: "", optional: false, copies: false },
				Slot { property: CssAtomSet::FlexShrink, before: "", after: "", optional: false, copies: false },
				Slot { property: CssAtomSet::FlexBasis, before: "", after: "", optional: false, copies: false },
			]))
		);

		assert_eq!(StyleValue::shorthand_by_name(CssAtomSet::Margin).unwrap().writes, Some(Writes::Repeat));
		assert_eq!(
			StyleValue::shorthand_by_name(CssAtomSet::Margin).unwrap().longhands,
			[CssAtomSet::MarginTop, CssAtomSet::MarginRight, CssAtomSet::MarginBottom, CssAtomSet::MarginLeft]
		);

		assert_eq!(
			StyleValue::shorthand_by_name(CssAtomSet::Border).unwrap().writes,
			Some(Writes::Slots(&[
				Slot { property: CssAtomSet::BorderWidth, before: "", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::BorderStyle, before: "", after: "", optional: true, copies: false },
				Slot { property: CssAtomSet::BorderColor, before: "", after: "", optional: true, copies: false },
			]))
		);

		assert_eq!(StyleValue::shorthand_by_name(CssAtomSet::BorderColor).unwrap().writes, Some(Writes::Repeat));
		assert_eq!(
			StyleValue::shorthand_by_name(CssAtomSet::BorderColor).unwrap().longhands,
			[
				CssAtomSet::BorderTopColor,
				CssAtomSet::BorderRightColor,
				CssAtomSet::BorderBottomColor,
				CssAtomSet::BorderLeftColor,
			]
		);
	}

	#[test]
	fn longhand_by_name_maps_to_shorthands() {
		for (longhand, shorthands) in [
			(CssAtomSet::MarginTop, &[CssAtomSet::Margin][..]),
			(CssAtomSet::JustifyItems, &[CssAtomSet::PlaceItems][..]),
			(CssAtomSet::ColumnGap, &[CssAtomSet::Gap][..]),
			(CssAtomSet::RowRuleColor, &[CssAtomSet::RuleColor][..]),
			(CssAtomSet::LineHeight, &[CssAtomSet::Font][..]),
			(CssAtomSet::ColumnHeight, &[CssAtomSet::Columns][..]),
			(CssAtomSet::TextDecorationThickness, &[CssAtomSet::TextDecoration][..]),
			(CssAtomSet::Top, &[CssAtomSet::Inset][..]),
		] {
			assert_eq!(StyleValue::longhand_by_name(longhand).unwrap().shorthands, shorthands, "{}", longhand.to_str());
		}
	}

	macro_rules! every_property {
		( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
			[$((CssAtomSet::$name, values::$ty::shorthand())),+]
		};
	}

	fn leaves(property: CssAtomSet, out: &mut Vec<CssAtomSet>) {
		match StyleValue::shorthand_by_name(property) {
			Some(shorthand) => {
				for longhand in shorthand.longhands {
					leaves(*longhand, out);
				}
			}
			None => out.push(property),
		}
	}

	#[test]
	fn every_shorthand_writes_every_longhand_it_sets() {
		let mut fails = Vec::new();
		for (property, shorthand) in apply_properties!(every_property) {
			let Some(shorthand) = shorthand else { continue };
			let name = property.to_str();
			let mut set = Vec::new();
			leaves(property, &mut set);
			let written = match shorthand.writes {
				Some(Writes::Slots(slots)) => Some(slots.iter().map(|slot| slot.property).collect::<Vec<_>>()),
				Some(Writes::Repeat) => Some(shorthand.longhands.to_vec()),
				Some(Writes::Same) | None => None,
			};
			if let Some(written) = written {
				let mut expanded = Vec::new();
				for property in &written {
					leaves(*property, &mut expanded);
				}
				for property in &set {
					if !expanded.contains(property) {
						fails.push(format!("{name} states no value for {}", property.to_str()));
					}
				}
				for property in &expanded {
					if !set.contains(property) {
						fails.push(format!("{name} writes {}, which it does not set", property.to_str()));
					}
				}
				if matches!(shorthand.writes, Some(Writes::Repeat)) && written.len() != expanded.len() {
					fails.push(format!("{name} repeats its values over a shorthand of its own"));
				}
			}
			if let ShorthandReset::Properties(resets) = shorthand.resets {
				for reset in resets {
					if shorthand.longhands.contains(reset) {
						fails.push(format!("{name} both sets and resets {}", reset.to_str()));
					}
				}
			}
		}
		assert_eq!(fails, Vec::<String>::new(), "fails should be empty");
	}

	#[test]
	fn every_shorthand_and_longhand_reflect_each_other() {
		let mut fails = Vec::new();
		for (property, shorthand) in apply_properties!(every_property) {
			let name = property.to_str();
			for longhand in shorthand.map(|shorthand| shorthand.longhands).unwrap_or_default() {
				let states_it =
					StyleValue::longhand_by_name(*longhand).is_some_and(|record| record.shorthands.contains(&property));
				if !states_it {
					fails.push(format!("{name} sets {}, which does not state it", longhand.to_str()));
				}
			}
			let Some(longhand) = StyleValue::longhand_by_name(property) else { continue };
			for shorthand in longhand.shorthands {
				match StyleValue::shorthand_by_name(*shorthand) {
					Some(record) if !record.longhands.contains(&property) => {
						fails.push(format!("{name} states the shorthand {}, which does not set it", shorthand.to_str()))
					}
					Some(_) => {}
					None => fails.push(format!("{name} states the shorthand {}, which is not one", shorthand.to_str())),
				}
			}
			for shorthand in longhand.reset_by {
				match StyleValue::shorthand_by_name(*shorthand) {
					Some(record) if record.longhands.contains(&property) => {
						fails.push(format!("{name} is reset by {}, which expresses it instead", shorthand.to_str()))
					}
					Some(_) => {}
					None => fails.push(format!("{name} is reset by {}, which is not a shorthand", shorthand.to_str())),
				}
			}
		}
		assert_eq!(fails, Vec::<String>::new());
	}

	#[test]
	fn lists_the_shorthands_which_state_no_writes() {
		let mut missing = apply_properties!(every_property)
			.into_iter()
			.filter(|(_, shorthand)| shorthand.is_some_and(|shorthand| shorthand.writes.is_none()))
			.map(|(property, _)| property.to_str())
			.collect::<Vec<_>>();
		missing.sort_unstable();
		assert_eq!(
			missing,
			[
				"-webkit-animation",
				"all",
				"animation-delay",
				"animation-range",
				"font-synthesis",
				"font-variant",
				"grid",
				"grid-area",
				"grid-column",
				"grid-row",
				"grid-template",
				"offset",
				"scroll-timeline",
				"view-timeline",
			]
		);
	}
}
