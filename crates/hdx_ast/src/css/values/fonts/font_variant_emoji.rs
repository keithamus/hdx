use hdx_derive::{Atomizable, Parsable, Value, Writable};

// https://drafts.csswg.org/css-fonts/#font-variant-caps-prop
#[derive(Value, Parsable, Writable, Atomizable, Debug, Default, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontVariantEmoji {
	#[default]
	Normal, // atom!("normal")
	Text,    // atom!("text")
	Emoji,   // atom!("emoji")
	Unicode, // atom!("unicode")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontVariantEmoji, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantEmoji, "normal");
		assert_parse!(FontVariantEmoji, "unicode");
		assert_parse!(FontVariantEmoji, "emoji");
	}
}
