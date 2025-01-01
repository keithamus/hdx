use css_parse::keyword_set;

keyword_set!(LineStyle {
	None: "none",
	Hidden: "hidden",
	Dotted: "dotted",
	Dashed: "dashed",
	Solid: "solid",
	Double: "double",
	Groove: "groove",
	Ridge: "ridge",
	Inset: "inset",
	Outset: "outset",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LineStyle>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LineStyle, "none");
		assert_parse!(LineStyle, "hidden");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(LineStyle, "florp");
		// Empty!
		assert_parse_error!(LineStyle, "");
	}
}
