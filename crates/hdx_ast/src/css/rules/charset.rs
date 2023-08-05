#[cfg(feature = "serde")]
use serde::Serialize;

use crate::Atom;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// TODO: maybe make this an enum? Can use:
// https://www.iana.org/assignments/character-sets/character-sets.xhtml
pub struct Charset {
	// Common charsets
	// atom!("UTF-8")
	// atom!("utf-8")
	pub encoding: Atom,
}
