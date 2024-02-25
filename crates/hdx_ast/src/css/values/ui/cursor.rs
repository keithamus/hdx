#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-ui-4/#propdef-cursor
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Cursor {
	#[default]
	Auto, // atom!("auto")
	Default,      // atom!("default")
	None,         // atom!("none")
	ContextMenu,  // atom!("context-menu")
	Help,         // atom!("help")
	Pointer,      // atom!("pointer")
	Progress,     // atom!("progress")
	Wait,         // atom!("wait")
	Cell,         // atom!("cell")
	Crosshair,    // atom!("crosshair")
	Text,         // atom!("text")
	VerticalText, // atom!("vertical-text")
	Alias,        // atom!("alias")
	Copy,         // atom!("copy")
	Move,         // atom!("move")
	NoDrop,       // atom!("no-drop")
	NotAllowed,   // atom!("not-allowed")
	Grab,         // atom!("grab")
	Grabbing,     // atom!("grabbing")
	EResize,      // atom!("e-resize")
	NResize,      // atom!("n-resize")
	NeResize,     // atom!("ne-resize")
	NwResize,     // atom!("nw-resize")
	SResize,      // atom!("s-resize")
	SeResize,     // atom!("se-resize")
	SwResize,     // atom!("sw-resize")
	WResize,      // atom!("w-resize")
	EwResize,     // atom!("ew-resize")
	NsResize,     // atom!("ns-resize")
	NeswResize,   // atom!("nesw-resize")
	NwseResize,   // atom!("nwse-resize")
	ColResize,    // atom!("col-resize")
	RowResize,    // atom!("row-resize")
	AllResize,    // atom!("all-resize")
	ZoomIn,       // atom!("zoom-in")
	ZoomOut,      /* atom!("zoom-out")
	               * TODO: Custom? */
}

impl Value for Cursor {
	fn inherits() -> bool {
		true
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Cursor>(), 1);
	}
}
