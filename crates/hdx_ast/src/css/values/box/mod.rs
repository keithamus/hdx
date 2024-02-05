mod margin_trim;

pub use margin_trim::*;

use super::Todo;
use crate::macros::{length_percentage_property, positive_length_percentage_property};

// https://drafts.csswg.org/css-box-4

// https://drafts.csswg.org/css-box-4/#padding-physical
pub type Padding = Todo;
positive_length_percentage_property!(PaddingTop);
positive_length_percentage_property!(PaddingRight);
positive_length_percentage_property!(PaddingBottom);
positive_length_percentage_property!(PaddingLeft);

// https://drafts.csswg.org/css-box-4/#margin-physical
pub type Margin = Todo;
length_percentage_property!(MarginTop);
length_percentage_property!(MarginRight);
length_percentage_property!(MarginBottom);
length_percentage_property!(MarginLeft);
