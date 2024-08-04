pub(crate) use crate::traits::StyleValue;
pub(crate) use hdx_proc_macro::*;

// `all` doesn't really have an initial value, but Self::Initial is a close approximation
impl Default for super::All {
	fn default() -> Self {
		Self::Initial
	}
}
