#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-box-4/#propdef-margin-trim
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MarginTrimValue {
	pub block_start: bool,
	pub block_end: bool,
	pub inline_start: bool,
	pub inline_end: bool,
}
