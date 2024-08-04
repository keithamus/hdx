use hdx_derive::{Atomizable, Parsable, Peekable, Writable};
use hdx_proc_macro::*;

use crate::css::types::Color;
use crate::css::units::LineWidth;

// mod accent_color;
// mod appearance;
// mod caret;
// mod caret_shape;
// mod cursor;
// mod input_security;
// mod nav_down;
// mod nav_left;
// mod nav_right;
// mod nav_up;
// mod outline;
// mod outline_color;
// mod outline_offset;
// mod outline_style;
// mod outline_width;
// mod pointer_events;
// mod resize;
// mod user_select;
// pub use accent_color::*;
// pub use appearance::*;
// pub use caret::*;
// pub use caret_color::*;
// pub use caret_shape::*;
// pub use cursor::*;
// pub use input_security::*;
// pub use nav_down::*;
// pub use nav_left::*;
// pub use nav_right::*;
// pub use nav_up::*;
// pub use outline::*;
// pub use outline_color::*;
// pub use outline_offset::*;
// pub use outline_style::*;
// pub use outline_width::*;
// pub use pointer_events::*;
// pub use resize::*;
// pub use user_select::*;

/**
 * https://drafts.csswg.org/css-ui-4/
 * css-ui 4
 */

/**
 * 3. Outline properties
 */

/*
 * https://drafts.csswg.org/css-ui/#outline
 * 3.1. Outlines Shorthand: the outline property
 */
// https://drafts.csswg.org/css-ui/#propdef-outline
// #[value(" [ <'outline-width'> || <'outline-style'> || <'outline-color'> ] ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct Outline;

/*
 * https://drafts.csswg.org/css-ui/#outline-width
 * 3.2. Outline Thickness: the outline-width property
 */
// https://drafts.csswg.org/css-ui/#propdef-outline-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct OutlineWidth;

/*
 * https://drafts.csswg.org/css-ui-4/#outline-style
 * 3.3. Outline Patterns: the outline-style property
 */
// https://drafts.csswg.org/css-ui/#propdef-outline-style
#[value(" auto | <outline-line-style> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum OutlineStyle {}

#[derive(Atomizable, Peekable, Parsable, Writable, Default, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum OutlineLineStyle {
	#[default]
	None,
	Dotted,
	Dashed,
	Solid,
	Double,
	Groove,
	Ridge,
	Inset,
	Outset,
}

/**
 * 5.2. Insertion caret
 */

/*
 * https://drafts.csswg.org/css-ui/#caret-color
 * 5.2.1. Coloring the Insertion Caret: the caret-color property
 */
// https://drafts.csswg.org/css-position/#propdef-caret-color
#[value(" auto | <color> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum CaretColor {}

/**
 * https://drafts.csswg.org/css-ui/#caret-animation
 * 5.2.2. Animation of the insertion caret: caret-animation
 */
// https://drafts.csswg.org/css-position/#propdef-caret-animation
#[value(" auto | manual ")]
#[initial("auto")]
#[applies_to("elements that accept input")]
#[inherited("yes")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum CaretAnimation {}

/**
 * https://drafts.csswg.org/css-ui/#caret-shape
 * 5.2.3. Shape of the insertion caret: caret-shape
 */
// https://drafts.csswg.org/css-position/#propdef-caret-shape
#[value(" auto | bar | block | underscore ")]
#[initial("auto")]
#[applies_to("elements that accept input")]
#[inherited("yes")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum CaretShape {}

/**
 * https://drafts.csswg.org/css-ui/#caret
 * 5.2.4. Insertion caret shorthand: caret
 */
// https://drafts.csswg.org/css-position/#propdef-caret
// #[value(" <'caret-color'> || <'caret-animation'> || <'caret-shape'>  ")]
// #[initial("auto")]
// #[applies_to("elements that accept input")]
// #[inherited("yes")]
// #[canonical_order("per grammar")]
// #[animation_type("by computed value")]
// pub struct Caret;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(OutlineWidth, 8);
		assert_size!(CaretColor, 36);
		assert_size!(CaretAnimation, 1);
		assert_size!(CaretShape, 1);
		// assert_size!(Caret, 1);
	}
}
