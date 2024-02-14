use hdx_atom::Atom;
#[cfg(feature = "serde")]
use serde::Serialize;

use super::CSSFloat;
use crate::Writable;

#[derive(Writable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Custom(CSSFloat, Atom);

impl Into<CSSFloat> for Custom {
	fn into(self) -> CSSFloat {
		self.0
	}
}
