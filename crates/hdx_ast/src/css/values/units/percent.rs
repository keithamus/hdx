use super::CSSFloat;
use crate::Writable;

#[derive(Writable, Debug, Clone, Copy, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[writable(suffix = "%")]
pub struct Percent(CSSFloat);

impl From<f32> for Percent {
	fn from(value: f32) -> Self {
		Self(value.into())
	}
}

impl Into<CSSFloat> for Percent {
	fn into(self) -> CSSFloat {
		self.0
	}
}
