use hdx_atom::Atom;
use hdx_derive::Writable;

use super::CSSFloat;

#[derive(Writable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Custom(CSSFloat, Atom);

impl Into<CSSFloat> for Custom {
	fn into(self) -> CSSFloat {
		self.0
	}
}
