#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{Atomizable, Parsable, Writable, Value};

// https://drafts.csswg.org/css-ui/#appearance-switching
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum Appearance {
	None, // atom!("none")
	#[default]
	Auto, // atom!("auto")
	// <compat-auto>
	Searchfield, // atom!("searchfield")
	Textarea, // atom!("textarea")
	Checkbox, // atom!("checkbox")
	Radio, // atom!("radio")
	Menulist, // atom!("menulist")
	Listbox, // atom!("listbox")
	Meter, // atom!("meter")
	ProgressBar, // atom!("progress-bar")
	Button, // atom!("button")
	// <compat-special>
	Textfield, //  atom!("textfield")
	MenulistButton, //  atom!("menulist-button")
}

impl Value for Appearance {}
