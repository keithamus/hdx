use hdx_derive::Atomizable;

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PseudoElement {
	After,              // atom!("after")
	Backdrop,           // atom!("backdrop")
	Before,             // atom!("after")
	Cue,                // atom!("cue")
	CueRegion,          // atom!("cue-region")
	FirstLetter,        // atom!("first-letter")
	FirstLine,          // atom!("first-line")
	FileSelectorButton, // atom!("file-selector-button")
	GrammarError,       // atom!("grammar-error")
	Marker,             // atom!("marker")
	Placeholder,        // atom!("placeholder")
	Selection,          // atom!("selection")
	SpellingError,      // atom!("spelling-error")
	TargetText,         // atom!("target-text")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LegacyPseudoElement {
	After,       // atom!("after")
	Before,      // atom!("before")
	FirstLetter, // atom!("first-letter")
	FirstLine,   // atom!("first-line")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PseudoElement, 1);
		assert_size!(LegacyPseudoElement, 1);
	}
}
