use hdx_derive::{Atomizable, Parsable, Value, Writable};

use crate::css::values::{Appearance, BoxSizing, ColumnCount, ColumnGap, TabSize, Todo, Transition, UserSelect};

pub type MozAppearance = Appearance;
pub type MozBoxSizing = BoxSizing;
pub type MozColumnCount = ColumnCount;
pub type MozColumnGap = ColumnGap;
pub type MozFloatEdge = Todo;
pub type MozForceBrokenImageIcon = Todo;
pub type MozImageRegion = Todo;

// https://developer.mozilla.org/en-US/docs/Web/CSS/font-smooth
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MozOsxFontSmoothing {
	#[default]
	Auto, // atom!("auto")
	Grayscale, // atom!("grayscale")
}

pub type MozRangeThumb = Todo;
pub type MozTabSize = TabSize;
pub type MozTransition = Transition;
pub type MozUserFocus = Todo;
pub type MozUserInput = Todo;
pub type MozUserSelect = UserSelect;
