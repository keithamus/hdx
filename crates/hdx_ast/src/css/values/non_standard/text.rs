use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-overflow-3/#propdef-overflow-block
#[derive(Parsable, Writable, Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TextRendering {
	#[default]
	Auto, // atom!("auto")
	Optimizespeed,      // atom!("optimizespeed")
	Optimizelegibility, // atom!("optimizelegibility")
	Geometricprecision, // atom!("geometricprecision")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextRendering, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TextRendering, "auto");
		assert_parse!(TextRendering, "optimizelegibility");
	}
}
