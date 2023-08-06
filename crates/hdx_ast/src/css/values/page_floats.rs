#[cfg(feature = "serde")]
use serde::Serialize;

use super::Length;
use crate::{atom, Atom, Atomizable, Spanned};

// https://drafts.csswg.org/css-page-floats-3/#propdef-float
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FloatValue {
	BlockStart,
	BlockEnd,
	InlineStart,
	InlineEnd,
	SnapBlock,
	SnapBlockFunction(Spanned<Length>, SnapBlockFloat),
	SnapInline,
	SnapInlineFunction(Spanned<Length>, SnapInlineFloat),
	Left,
	Right,
	Top,
	Bottom,
	#[default]
	None,
}

impl FloatValue {
	pub fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("block-start") => Some(Self::BlockStart),
			atom!("block-end") => Some(Self::BlockEnd),
			atom!("inline-start") => Some(Self::InlineStart),
			atom!("inline-end") => Some(Self::InlineEnd),
			atom!("snap-block") => Some(Self::SnapBlock),
			atom!("snap-inline") => Some(Self::SnapInline),
			atom!("left") => Some(Self::Left),
			atom!("right") => Some(Self::Right),
			atom!("top") => Some(Self::Top),
			atom!("bottom") => Some(Self::Bottom),
			atom!("none") => Some(Self::None),
			_ => None,
		}
	}

	pub fn to_atom(&self) -> Option<Atom> {
		match self {
			Self::BlockStart => Some(atom!("block-start")),
			Self::BlockEnd => Some(atom!("block-end")),
			Self::InlineStart => Some(atom!("inline-start")),
			Self::InlineEnd => Some(atom!("inline-end")),
			Self::SnapBlock => Some(atom!("snap-block")),
			Self::SnapBlockFunction(_, _) => None,
			Self::SnapInline => Some(atom!("snap-inline")),
			Self::SnapInlineFunction(_, _) => None,
			Self::Left => Some(atom!("left")),
			Self::Right => Some(atom!("right")),
			Self::Top => Some(atom!("top")),
			Self::Bottom => Some(atom!("bottom")),
			Self::None => Some(atom!("none")),
		}
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum SnapBlockFloat {
	Start, // atom!("start")
	End,   // atom!("end")
	Near,  // atom!("near")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum SnapInlineFloat {
	Right, // atom!("right")
	Left,  // atom!("left")
	Near,  // atom!("near")
}

// https://drafts.csswg.org/css-page-floats-3/#propdef-clear
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum ClearValue {
	InlineStart, // atom!("inline-start")
	InlineEnd,   // atom!("inline-end")
	BlockStart,  // atom!("block-start")
	BlockEnd,    // atom!("block-end")
	Left,        // atom!("left")
	Right,       // atom!("right")
	Top,         // atom!("top")
	Bottom,      // atom!("bottom")
	BothInline,  // atom!("both-inline")
	BothBlock,   // atom!("both-block")
	Both,        // atom!("both")
	#[default]
	None, // atom!("none")
}

// https://drafts.csswg.org/css-page-floats-3/#propdef-float-defer
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum FloatDeferValue {
	#[default]
	Last,
	None,
	Integer(i32),
}

impl Atomizable for FloatDeferValue {
	fn to_atom(&self) -> Atom {
		match self {
			Self::Last => atom!("last"),
			Self::None => atom!("none"),
			Self::Integer(_) => atom!(""),
		}
	}

	fn from_atom(atom: Atom) -> Option<Self> {
		match atom {
			atom!("last") => Some(Self::Last),
			atom!("none") => Some(Self::None),
			_ => None,
		}
	}
}

impl From<f32> for FloatDeferValue {
	fn from(float: f32) -> Self {
		Self::Integer(float as i32)
	}
}

impl From<i32> for FloatDeferValue {
	fn from(int: i32) -> Self {
		Self::Integer(int)
	}
}

// https://drafts.csswg.org/css-page-floats-3/#propdef-float-reference
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FloatReferenceValue {
	#[default]
	Inline, // atom!("inline")
	Column, // atom!("column")
	Region, // atom!("region")
	Page,   // atom!("page")
}
