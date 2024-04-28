use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-fonts/#font-variant-position-prop
#[derive(Value, Parsable, Writable, Atomizable, Debug, Default, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontVariantPosition {
	#[default]
	Normal, // atom!("normal")
	Sub,   // atom!("sub")
	Super, // atom!("super")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontVariantPosition, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantPosition, "normal");
		assert_parse!(FontVariantPosition, "sub");
		assert_parse!(FontVariantPosition, "super");
	}
}
