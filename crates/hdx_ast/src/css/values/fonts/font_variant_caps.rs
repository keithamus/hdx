use crate::{Atomizable, Parsable, Writable};

// https://drafts.csswg.org/css-fonts/#font-variant-caps-prop
#[derive(Parsable, Writable, Atomizable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum FontVariantCaps {
	#[default]
	Normal, // atom!("normal")
	SmallCaps,     // atom!("small-caps")
	AllSmallCaps,  // atom!("all-small-caps")
	PetiteCaps,    // atom!("petite-caps")
	AllPetiteCaps, // atom!("all-petite-caps")
	Unicase,       // atom!("unicase")
	TitlingCaps,   // atom!("titling-caps")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontVariantCaps, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontVariantCaps, "normal");
		assert_parse!(FontVariantCaps, "small-caps");
		assert_parse!(FontVariantCaps, "all-small-caps");
		assert_parse!(FontVariantCaps, "titling-caps");
	}
}
