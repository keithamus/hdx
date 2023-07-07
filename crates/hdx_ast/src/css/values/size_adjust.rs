use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-size-adjust-1/#propdef-text-size-adjust
#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum TextSizeAdjustValue {
	None,
	#[default]
	Auto,
	Percentage(f32),
}

impl Hash for TextSizeAdjustValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			TextSizeAdjustValue::None => state.write_u8(0),
			TextSizeAdjustValue::Auto => state.write_u8(1),
			TextSizeAdjustValue::Percentage(v) => {
				state.write_u8(2);
				v.to_bits().hash(state);
			}
		}
	}
}
