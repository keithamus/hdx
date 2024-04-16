use crate::{
	css::{keywords::LineStyle, types::Color, units::LineWidth},
	macros::*,
	Value,
};

// https://drafts.csswg.org/css-backgrounds/#border-shorthands
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderRight(pub Option<LineWidth>, pub Option<LineStyle>, pub Option<Color>);

parse_option_shorthand!(BorderRight, LineWidth, LineStyle, Color);
write_option_shorthand!(BorderRight, 3);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderRight, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderRight, "none");
		assert_parse!(BorderRight, "1px solid red");
		assert_parse!(BorderRight, "0");
		assert_parse!(BorderRight, "1px solid rgb(255, 0, 0)");
		assert_parse!(BorderRight, "thin solid transparent");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(BorderRight, "");
	}
}
