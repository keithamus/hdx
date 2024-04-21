use crate::{Parsable, Value, Writable};

#[derive(Value, Parsable, Writable, Default, PartialEq, Debug, Hash)]
#[value(Inherits)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum Visibility {
	#[default]
	Visible, // atom!("visible"),
	Hidden,   // atom!("hidden"),
	Collapse, // atom!("collapse"),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Visibility, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Visibility, "visible");
	}
}
