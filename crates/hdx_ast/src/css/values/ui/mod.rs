mod impls;
pub mod types;

use impls::*;

/*
 * https://drafts.csswg.org/css-ui-4/
 * CSS Basic User Interface Module Level 4
 */

// // https://drafts.csswg.org/css-ui-4/#outline
// #[value(" <'outline-width'> || <'outline-style'> || <'outline-color'> ")]
// #[initial("see individual properties")]
// #[applies_to("all elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("see individual properties")]
// pub struct OutlineStyleValue<'a>;

// https://drafts.csswg.org/css-ui-4/#outline-width
#[value(" <line-width> ")]
#[initial("medium")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct OutlineWidthStyleValue;

// https://drafts.csswg.org/css-ui-4/#outline-style
#[value(" auto | <outline-line-style> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum OutlineStyleStyleValue {}

// https://drafts.csswg.org/css-ui-4/#outline-color
#[value(" auto | <color> | <image-1D> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum OutlineColorStyleValue<'a> {}

// https://drafts.csswg.org/css-ui-4/#outline-offset
#[value(" <length> ")]
#[initial("0")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub struct OutlineOffsetStyleValue;

// https://drafts.csswg.org/css-ui-4/#resize
#[value(" none | both | horizontal | vertical | block | inline ")]
#[initial("none")]
#[applies_to(
	"elements that are scroll containers and optionally replaced elements such as images, videos, and iframes"
)]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum ResizeStyleValue {}

// // https://drafts.csswg.org/css-ui-4/#cursor
// #[value(" [ [ <url> | <url-set> ] [<x> <y>]? ]#?  [ auto | default | none | context-menu | help | pointer | progress | wait |  cell | crosshair | text | vertical-text |  alias | copy | move | no-drop | not-allowed | grab | grabbing |  e-resize | n-resize | ne-resize | nw-resize | s-resize | se-resize | sw-resize | w-resize | ew-resize | ns-resize | nesw-resize | nwse-resize | col-resize | row-resize | all-scroll | zoom-in | zoom-out  ] ")]
// #[initial("auto")]
// #[applies_to("all elements")]
// #[inherited("yes")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum CursorStyleValue<'a> {}

// https://drafts.csswg.org/css-ui-4/#caret-color
#[value(" auto | <color> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum CaretColorStyleValue {}

// https://drafts.csswg.org/css-ui-4/#caret-animation
#[value(" auto | manual ")]
#[initial("auto")]
#[applies_to("elements that accept input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum CaretAnimationStyleValue {}

// https://drafts.csswg.org/css-ui-4/#caret-shape
#[value(" auto | bar | block | underscore ")]
#[initial("auto")]
#[applies_to("elements that accept input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value")]
pub enum CaretShapeStyleValue {}

// https://drafts.csswg.org/css-ui-4/#caret
#[value(" <'caret-color'> || <'caret-animation'> || <'caret-shape'> ")]
#[initial("auto")]
#[applies_to("elements that accept input")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("see individual properties")]
pub struct CaretStyleValue;

// // https://drafts.csswg.org/css-ui-4/#nav-up
// #[value(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum NavUpStyleValue {}

// // https://drafts.csswg.org/css-ui-4/#nav-right
// #[value(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum NavRightStyleValue {}

// // https://drafts.csswg.org/css-ui-4/#nav-down
// #[value(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum NavDownStyleValue {}

// // https://drafts.csswg.org/css-ui-4/#nav-left
// #[value(" auto | <id> [ current | root | <target-name> ]? ")]
// #[initial("auto")]
// #[applies_to("all enabled elements")]
// #[inherited("no")]
// #[percentages("n/a")]
// #[canonical_order("per grammar")]
// #[animation_type("discrete")]
// pub enum NavLeftStyleValue {}

// https://drafts.csswg.org/css-ui-4/#user-select
#[value(" auto | text | none | contain | all ")]
#[initial("auto")]
#[applies_to("all elements, and optionally to the ::before and ::after pseudo-elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum UserSelectStyleValue {}

// https://drafts.csswg.org/css-ui-4/#pointer-events
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum PointerEventsStyleValue {}

// https://drafts.csswg.org/css-ui-4/#accent-color
#[value(" auto | <color> ")]
#[initial("auto")]
#[applies_to("all elements")]
#[inherited("yes")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum AccentColorStyleValue {}

// https://drafts.csswg.org/css-ui-4/#appearance
#[value(" none | auto | base | <compat-auto> | <compat-special> ")]
#[initial("none")]
#[applies_to("all elements")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum AppearanceStyleValue {}

// https://drafts.csswg.org/css-ui-4/#field-sizing
#[value(" fixed | content ")]
#[initial("fixed")]
#[applies_to("elements with default preferred size")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum FieldSizingStyleValue {}

// https://drafts.csswg.org/css-ui-4/#input-security
#[value(" auto | none ")]
#[initial("auto")]
#[applies_to("sensitive text inputs")]
#[inherited("no")]
#[percentages("n/a")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum InputSecurityStyleValue {}
