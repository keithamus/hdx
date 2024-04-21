use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-fonts/#font-variant-position-prop
#[derive(Parsable, Writable, Atomizable, Debug, Default, PartialEq, Hash)]
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
