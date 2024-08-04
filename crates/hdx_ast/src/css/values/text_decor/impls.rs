pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

impl Default for super::TextDecorationSkip {
	fn default() -> Self {
		Self::Auto
	}
}
