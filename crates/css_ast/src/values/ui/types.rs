use css_parse::keyword_set;

pub(crate) use crate::types::*;
pub(crate) use crate::units::*;

// Re-expose stylevalues for shorthands
pub(crate) use super::{CaretAnimationStyleValue, CaretColorStyleValue, CaretShapeStyleValue};

// https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style
// <outline-line-style> accepts the same values as <line-style> (CSS Backgrounds 3 § 3.2 Line
// Patterns: the border-style properties) with the same meaning, except that hidden is not a legal
// outline style. In addition, the outline-style property accepts the value auto.
// <line-style> = none | hidden | dotted | dashed | solid | double | groove | ridge | inset | outset
keyword_set!(OutlineLineStyle {
	None: "none",
	Hidden: "hidden",
	Dotted: "doted",
	Dashed: "dashed",
	Solid: "solid",
	Double: "double",
	Groove: "groove",
	Ridge: "ridge",
	Inset: "inset",
	Outset: "outset",
});

// https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-auto
// These values exist for compatibility of content developed for earlier non-standard versions of this property.
// They all have the same effect as auto.
// <compat-auto> = searchfield | textarea | checkbox | radio | menulist | listbox | meter | progress-bar | button
keyword_set!(CompatAuto {
	Searchfield: "searchfield",
	Textarea: "textarea",
	Checkbox: "checkbox",
	Radio: "radio",
	Menulist: "menulist",
	Listbox: "listbox",
	Meter: "meter",
	ProgressBar: "progress-bar",
	Button: "button",
});

// https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-special
// These values exist for compatibility of content developed for earlier non-standard versions of this property.
// For the purpose of this specification, they all have the same effect as auto.
// However, the host language may also take these values into account when defining the native appearance of the element.
// <compat-special> = textfield | menulist-button
keyword_set!(CompatSpecial { Textfield: "textfield", MenulistButton: "menulist-button" });
