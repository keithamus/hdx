use crate::{Atomizable, Writable};

#[derive(Atomizable, Writable, Default, Debug, Clone, Copy, PartialEq, Hash)]
#[atomizable(FromToken)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LineStyle {
	#[default]
	None, // atom!("none")
	Hidden, // atom!("hidden")
	Dotted, // atom!("dotted")
	Dashed, // atom!("dashed")
	Solid,  // atom!("solid")
	Double, // atom!("double")
	Groove, // atom!("groove")
	Ridge,  // atom!("ridge")
	Inset,  // atom!("inset")
	Outset, // atom!("outset")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(LineStyle, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LineStyle, "none");
		assert_parse!(LineStyle, "hidden");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(LineStyle, "");
	}
}
