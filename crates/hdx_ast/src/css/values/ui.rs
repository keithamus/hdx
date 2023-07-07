#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Expr, Length};
use crate::{atom, Atom, Atomizable, Span};

// https://drafts.csswg.org/css-ui-4/#propdef-cursor
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum CursorValue<'a> {
	Custom(Expr<'a, Length>), // todo
	#[default]
	Auto,           // atom!("auto")
	Default,                  // atom!("default")
	None,                     // atom!("none")
	ContextMenu,              // atom!("context-menu")
	Help,                     // atom!("help")
	Pointer,                  // atom!("pointer")
	Progress,                 // atom!("progress")
	Wait,                     // atom!("wait")
	Cell,                     // atom!("cell")
	Crosshair,                // atom!("crosshair")
	Text,                     // atom!("text")
	VerticalText,             // atom!("vertical-text")
	Alias,                    // atom!("alias")
	Copy,                     // atom!("copy")
	Move,                     // atom!("move")
	NoDrop,                   // atom!("no-drop")
	NotAllowed,               // atom!("not-allowed")
	Grab,                     // atom!("grab")
	Grabbing,                 // atom!("grabbing")
	EResize,                  // atom!("e-resize")
	NResize,                  // atom!("n-resize")
	NeResize,                 // atom!("ne-resize")
	NwResize,                 // atom!("nw-resize")
	SResize,                  // atom!("s-resize")
	SeResize,                 // atom!("se-resize")
	SwResize,                 // atom!("sw-resize")
	WResize,                  // atom!("w-resize")
	EwResize,                 // atom!("ew-resize")
	NsResize,                 // atom!("ns-resize")
	NeswResize,               // atom!("nesw-resize")
	NwseResize,               // atom!("nwse-resize")
	ColResize,                // atom!("col-resize")
	RowResize,                // atom!("row-resize")
	AllResize,                // atom!("all-resize")
	ZoomIn,                   // atom!("zoom-in")
	ZoomOut,                  // atom!("zoom-in")
}

impl<'a> Atomizable for CursorValue<'a> {
	fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("auto") => Some(Self::Auto),
			atom!("default") => Some(Self::Default),
			atom!("none") => Some(Self::None),
			atom!("context-menu") => Some(Self::ContextMenu),
			atom!("help") => Some(Self::Help),
			atom!("pointer") => Some(Self::Pointer),
			atom!("progress") => Some(Self::Progress),
			atom!("wait") => Some(Self::Wait),
			atom!("cell") => Some(Self::Cell),
			atom!("crosshair") => Some(Self::Crosshair),
			atom!("text") => Some(Self::Text),
			atom!("vertical-text") => Some(Self::VerticalText),
			atom!("alias") => Some(Self::Alias),
			atom!("copy") => Some(Self::Copy),
			atom!("move") => Some(Self::Move),
			atom!("no-drop") => Some(Self::NoDrop),
			atom!("not-allowed") => Some(Self::NotAllowed),
			atom!("grab") => Some(Self::Grab),
			atom!("grabbing") => Some(Self::Grabbing),
			atom!("e-resize") => Some(Self::EResize),
			atom!("n-resize") => Some(Self::NResize),
			atom!("ne-resize") => Some(Self::NeResize),
			atom!("nw-resize") => Some(Self::NwResize),
			atom!("s-resize") => Some(Self::SResize),
			atom!("se-resize") => Some(Self::SeResize),
			atom!("sw-resize") => Some(Self::SwResize),
			atom!("w-resize") => Some(Self::WResize),
			atom!("ew-resize") => Some(Self::EwResize),
			atom!("ns-resize") => Some(Self::NsResize),
			atom!("nesw-resize") => Some(Self::NeswResize),
			atom!("nwse-resize") => Some(Self::NwseResize),
			atom!("col-resize") => Some(Self::ColResize),
			atom!("row-resize") => Some(Self::RowResize),
			atom!("all-resize") => Some(Self::AllResize),
			atom!("zoom-in") => Some(Self::ZoomIn),
			atom!("zoom-out") => Some(Self::ZoomOut),
			_ => None,
		}
	}

	fn to_atom(&self) -> Atom {
		match &self {
			Self::Auto => atom!("auto"),
			Self::Default => atom!("default"),
			Self::None => atom!("none"),
			Self::ContextMenu => atom!("context-menu"),
			Self::Help => atom!("help"),
			Self::Pointer => atom!("pointer"),
			Self::Progress => atom!("progress"),
			Self::Wait => atom!("wait"),
			Self::Cell => atom!("cell"),
			Self::Crosshair => atom!("crosshair"),
			Self::Text => atom!("text"),
			Self::VerticalText => atom!("vertical-text"),
			Self::Alias => atom!("alias"),
			Self::Copy => atom!("copy"),
			Self::Move => atom!("move"),
			Self::NoDrop => atom!("no-drop"),
			Self::NotAllowed => atom!("not-allowed"),
			Self::Grab => atom!("grab"),
			Self::Grabbing => atom!("grabbing"),
			Self::EResize => atom!("e-resize"),
			Self::NResize => atom!("n-resize"),
			Self::NeResize => atom!("ne-resize"),
			Self::NwResize => atom!("nw-resize"),
			Self::SResize => atom!("s-resize"),
			Self::SeResize => atom!("se-resize"),
			Self::SwResize => atom!("sw-resize"),
			Self::WResize => atom!("w-resize"),
			Self::EwResize => atom!("ew-resize"),
			Self::NsResize => atom!("ns-resize"),
			Self::NeswResize => atom!("nesw-resize"),
			Self::NwseResize => atom!("nwse-resize"),
			Self::ColResize => atom!("col-resize"),
			Self::RowResize => atom!("row-resize"),
			Self::AllResize => atom!("all-resize"),
			Self::ZoomIn => atom!("zoom-in"),
			Self::ZoomOut => atom!("zoom-out"),
			Self::Custom(_) => atom!(""),
		}
	}
}
