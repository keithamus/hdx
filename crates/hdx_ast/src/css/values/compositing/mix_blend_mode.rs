use crate::{Atomizable, Parsable, Writable};

// https://drafts.fxtf.org/compositing/#propdef-mix-blend-mode
#[derive(Parsable, Writable, Atomizable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MixBlendMode {
	#[default]
	Normal, // atom!("normal")
	Multiply,    // atom!("multiply")
	Screen,      // atom!("screen")
	Overlay,     // atom!("overlay")
	Darken,      // atom!("darken")
	Lighten,     // atom!("lighten")
	ColorDodge,  // atom!("color-dodge")
	ColorBurn,   // atom!("color-burn")
	HardLight,   // atom!("hard-light")
	SoftLight,   // atom!("soft-light")
	Difference,  // atom!("difference")
	Exclusion,   // atom!("exclusion")
	Hue,         // atom!("hue")
	Saturation,  // atom!("saturation")
	Color,       // atom!("color")
	Luminosity,  // atom!("luminosity")
	PlusDarker,  // atom!("plus-darker")
	PlusLighter, // atom!("plus-lighter")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MixBlendMode, 1);
	}
}
