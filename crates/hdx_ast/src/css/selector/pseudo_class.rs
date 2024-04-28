use hdx_derive::Atomizable;

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PseudoClass {
	Active,           // atom!("active")
	AnyLink,          // atom!("any-link")
	Autofill,         // atom!("autofill")
	Blank,            // atom!("blank")
	Checked,          // atom!("checked")
	Current,          // atom!("current")
	Default,          // atom!("default")
	Defined,          // atom!("defined")
	Disabled,         // atom!("disabled")
	Empty,            // atom!("empty")
	Enabled,          // atom!("enabled")
	First,            // atom!("first")
	FirstChild,       // atom!("first-child")
	FirstOfType,      // atom!("first-of-type")
	Fullscreen,       // atom!("fullscreen")
	Future,           // atom!("future")
	Focus,            // atom!("focus")
	FocusVisible,     // atom!("focus-visible")
	FocusWithin,      // atom!("focus-within")
	Host,             // atom!("host")
	Hover,            // atom!("hover")
	Indeterminate,    // atom!("indeterminate")
	InRange,          // atom!("in-range")
	Invalid,          // atom!("invalid")
	LastChild,        // atom!("last-child")
	LastOfType,       // atom!("last-of-type")
	Left,             // atom!("left")
	Link,             // atom!("link")
	LocalLink,        // atom!("local-link")
	Modal,            // atom!("modal")
	OnlyChild,        // atom!("only-child")
	OnlyOfType,       // atom!("only-of-type")
	Optional,         // atom!("optional")
	OutOfRange,       // atom!("out-of-range")
	Past,             // atom!("past")
	PictureInPicture, // atom!("picture-in-picture")
	PlaceholderShown, // atom!("placeholder-shown")
	PopoverOpen,      // atom!("popover-open")
	Paused,           // atom!("paused")
	Playing,          // atom!("playing")
	ReadOnly,         // atom!("read-only")
	ReadWrite,        // atom!("read-write")
	Required,         // atom!("required")
	Right,            // atom!("right")
	Root,             // atom!("root")
	Scope,            // atom!("scope")
	Target,           // atom!("target")
	TargetWithin,     // atom!("target-within")
	Valid,            // atom!("valid")
	Visited,          // atom!("visited")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PseudoClass, 1);
	}
}
