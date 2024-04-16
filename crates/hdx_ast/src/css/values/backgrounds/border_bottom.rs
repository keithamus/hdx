use crate::{
	css::{keywords::LineStyle, types::Color, units::LineWidth},
	macros::*,
	Value,
};

// https://drafts.csswg.org/css-backgrounds/#border-shorthands
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BorderBottom(pub Option<LineWidth>, pub Option<LineStyle>, pub Option<Color>);

parse_option_shorthand!(BorderBottom, LineWidth, LineStyle, Color);
write_option_shorthand!(BorderBottom, 3);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BorderBottom, 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderBottom, "none");
		assert_parse!(BorderBottom, "1px solid red");
		assert_parse!(BorderBottom, "0");
		assert_parse!(BorderBottom, "1px solid rgb(255, 0, 0)");
		assert_parse!(BorderBottom, "thin solid transparent");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(BorderBottom, "");
	}
}
