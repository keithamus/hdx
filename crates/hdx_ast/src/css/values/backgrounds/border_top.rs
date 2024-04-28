use hdx_derive::Value;

use crate::{
	css::{keywords::LineStyle, types::Color, units::LineWidth},
	macros::*,
};

// https://drafts.csswg.org/css-backgrounds/#border-shorthands
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderTop(pub Option<LineWidth>, pub Option<LineStyle>, pub Option<Color>);

parse_option_shorthand!(BorderTop, LineWidth, LineStyle, Color);
write_option_shorthand!(BorderTop, 3);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderTop, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderTop, "none");
		assert_parse!(BorderTop, "1px solid red");
		assert_parse!(BorderTop, "0");
		assert_parse!(BorderTop, "1px solid rgb(255, 0, 0)");
		assert_parse!(BorderTop, "thin solid transparent");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(BorderTop, "");
	}
}
