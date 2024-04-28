use hdx_derive::Value;

use crate::{
	css::{keywords::LineStyle, types::Color, units::LineWidth},
	macros::*,
};

// https://drafts.csswg.org/css-backgrounds/#propdef-border
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Border(pub Option<LineWidth>, pub Option<LineStyle>, pub Option<Color>);

parse_option_shorthand!(Border, LineWidth, LineStyle, Color);
write_option_shorthand!(Border, 3);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Border, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Border, "none");
		assert_parse!(Border, "1px solid red");
		assert_parse!(Border, "0");
		assert_parse!(Border, "1px solid rgb(255, 0, 0)");
		assert_parse!(Border, "thin solid transparent");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(Border, "");
	}
}
