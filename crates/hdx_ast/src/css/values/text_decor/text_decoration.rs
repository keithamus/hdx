use crate::{macros::*, Value};

use super::{TextDecorationColor, TextDecorationLine, TextDecorationStyle};

// https://drafts.csswg.org/css-text-decor/#text-decoration-property
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct TextDecoration(
	pub Option<TextDecorationLine>,
	pub Option<TextDecorationStyle>,
	pub Option<TextDecorationColor>,
);

parse_option_shorthand!(TextDecoration, TextDecorationLine, TextDecorationStyle, TextDecorationColor);
write_option_shorthand!(TextDecoration, 3);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextDecoration, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(TextDecoration, "none");
		assert_parse!(TextDecoration, "underline");
		assert_parse!(TextDecoration, "overline red");
		assert_parse!(TextDecoration, "underline dotted");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(TextDecoration, "");
	}
}
