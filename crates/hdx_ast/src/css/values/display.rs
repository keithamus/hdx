#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum DisplayValue {
	None,              // atom!("none")
	Contents,          // atom!("contents")
	TableRowGroup,     // atom!("table-row-group")
	TableHeaderGroup,  // atom!("table-header-group")
	TableFooterGroup,  // atom!("table-footer-group")
	TableRow,          // atom!("table-row")
	TableCell,         // atom!("table-cell")
	TableColumnGroup,  // atom!("table-column-group")
	TableColumn,       // atom!("table-column")
	TableCaption,      // atom!("table-caption")
	RubyBase,          // atom!("ruby-base")
	RubyText,          // atom!("ruby-text")
	RubyBaseContainer, // atom!("ruby-base-container")
	RubyTextContainer, // atom!("ruby-text-container")
	// Legacy
	InlineBlock, // atom!("inline-block")
	InlineTable, // atom!("inline-flex")
	InlineFlex,  // atom!("inline-flex")
	InlineGrid,  // atom!("inline-flex")
	Pair(DisplayOutside, DisplayInside),
	PairAndMarker(DisplayOutside, DisplayInside, DisplayMarker),
}

impl Default for DisplayValue {
	fn default() -> Self {
		Self::Pair(DisplayOutside::default(), DisplayInside::default())
	}
}

// https://drafts.csswg.org/css-display-4/#propdef-display
impl DisplayValue {
	pub fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("none") => Some(Self::None),
			atom!("contents") => Some(Self::Contents),
			atom!("table-row-group") => Some(Self::TableRowGroup),
			atom!("table-header-group") => Some(Self::TableHeaderGroup),
			atom!("table-footer-group") => Some(Self::TableFooterGroup),
			atom!("table-row") => Some(Self::TableRow),
			atom!("table-cell") => Some(Self::TableCell),
			atom!("table-column-group") => Some(Self::TableColumnGroup),
			atom!("table-column") => Some(Self::TableColumn),
			atom!("table-caption") => Some(Self::TableCaption),
			atom!("ruby-base") => Some(Self::RubyBase),
			atom!("ruby-text") => Some(Self::RubyText),
			atom!("ruby-base-container") => Some(Self::RubyBaseContainer),
			atom!("ruby-text-container") => Some(Self::RubyTextContainer),
			atom!("flow-root") => Some(Self::Pair(DisplayOutside::Block, DisplayInside::FlowRoot)),
			atom!("list-item") => Some(Self::PairAndMarker(
				DisplayOutside::Implicit,
				DisplayInside::Implicit,
				DisplayMarker::ListItem,
			)),
			// Legacy
			atom!("inline-block") => Some(Self::InlineBlock),
			atom!("inline-table") => Some(Self::InlineTable),
			atom!("inline-flex") => Some(Self::InlineFlex),
			atom!("inline-grid") => Some(Self::InlineGrid),
			_ => {
				if let Some(display) = DisplayInside::from_atom(atom.clone()) {
					Some(Self::Pair(DisplayOutside::Implicit, display))
				} else if let Some(display) = DisplayOutside::from_atom(atom) {
					Some(Self::Pair(display, DisplayInside::Implicit))
				} else {
					None
				}
			}
		}
	}

	pub fn from_atom_pair(atom1: Atom, atom2: Atom) -> Option<Self> {
		let outside = DisplayOutside::from_atom(atom1.clone());
		let inside = DisplayInside::from_atom(atom2.clone());
		match (outside, inside) {
			(Some(o), Some(i)) => Some(DisplayValue::Pair(o, i)),
			_ => {
				let outside = DisplayOutside::from_atom(atom2);
				let inside = DisplayInside::from_atom(atom1);
				match (outside, inside) {
					(Some(o), Some(i)) => Some(DisplayValue::Pair(o, i)),
					_ => None,
				}
			}
		}
	}

	pub fn to_atom(&self) -> Option<Atom> {
		match self {
			Self::None => Some(atom!("none")),
			Self::Contents => Some(atom!("contents")),
			Self::TableRowGroup => Some(atom!("table-row-group")),
			Self::TableHeaderGroup => Some(atom!("table-header-group")),
			Self::TableFooterGroup => Some(atom!("table-footer-group")),
			Self::TableRow => Some(atom!("table-row")),
			Self::TableCell => Some(atom!("table-cell")),
			Self::TableColumnGroup => Some(atom!("table-column-group")),
			Self::TableColumn => Some(atom!("table-column")),
			Self::TableCaption => Some(atom!("table-caption")),
			Self::RubyBase => Some(atom!("ruby-base")),
			Self::RubyText => Some(atom!("ruby-text")),
			Self::RubyBaseContainer => Some(atom!("ruby-base-container")),
			Self::RubyTextContainer => Some(atom!("ruby-text-container")),
			Self::InlineBlock => Some(atom!("inline-block")),
			Self::InlineTable => Some(atom!("inline-table")),
			Self::InlineFlex => Some(atom!("inline-flex")),
			Self::InlineGrid => Some(atom!("inline-grid")),
			Self::Pair(DisplayOutside::Implicit, i) => i.to_atom(),
			Self::Pair(o, DisplayInside::Implicit) => o.to_atom(),
			Self::Pair(_, _) => None,
			Self::PairAndMarker(DisplayOutside::Implicit, DisplayInside::Implicit, m) => {
				m.to_atom()
			}
			Self::PairAndMarker(_o, _i, _m) => None,
		}
	}
}

// https://drafts.csswg.org/css-display-4/#typedef-display-outside
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum DisplayOutside {
	Implicit,
	Block, // atom!("block")
	#[default]
	Inline, // atom!("inline")
	RunIn, // atom!("run-in")
}

impl DisplayOutside {
	pub fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("block") => Some(Self::Block),
			atom!("inline") => Some(Self::Inline),
			atom!("run-in") => Some(Self::RunIn),
			_ => None,
		}
	}

	pub fn to_atom(&self) -> Option<Atom> {
		match self {
			Self::Implicit => None,
			Self::Block => Some(atom!("block")),
			Self::Inline => Some(atom!("inline")),
			Self::RunIn => Some(atom!("run-in")),
		}
	}
}

// https://drafts.csswg.org/css-display-4/#typedef-display-inside
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum DisplayInside {
	Implicit,
	#[default]
	Flow, // atom!("flow")
	FlowRoot, // atom!("flow-root")
	Table,    // atom!("table")
	Flex,     // atom!("flex")
	Grid,     // atom!("grid")
	Ruby,     // atom!("ruby")
}

impl DisplayInside {
	pub fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("flow") => Some(Self::Flow),
			atom!("flow-root") => Some(Self::FlowRoot),
			atom!("table") => Some(Self::Table),
			atom!("flex") => Some(Self::Flex),
			atom!("grid") => Some(Self::Grid),
			atom!("ruby") => Some(Self::Ruby),
			_ => None,
		}
	}

	pub fn to_atom(&self) -> Option<Atom> {
		match self {
			Self::Implicit => None,
			Self::Flow => Some(atom!("flow")),
			Self::FlowRoot => Some(atom!("flow-root")),
			Self::Table => Some(atom!("table")),
			Self::Flex => Some(atom!("flex")),
			Self::Grid => Some(atom!("grid")),
			Self::Ruby => Some(atom!("ruby")),
		}
	}
}

// https://drafts.csswg.org/css-display-4/#typedef-display-listitem
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum DisplayMarker {
	ListItem, // atom!("list-item")
}

impl DisplayMarker {
	pub fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("list-item") => Some(Self::ListItem),
			_ => None,
		}
	}

	pub fn to_atom(&self) -> Option<Atom> {
		match self {
			Self::ListItem => Some(atom!("list-item")),
		}
	}
}

// https://drafts.csswg.org/css-display-4/#propdef-visibility
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum VisibilityValue {
	#[default]
	Visible, // atom!("visible")
	Hidden,   // atom!("hidden")
	Collapse, // atom!("collapse")
}
