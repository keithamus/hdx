use hdx_parser::keyword_typedef;

keyword_typedef!(LineStyle {
	None: atom!("none"),
	Hidden: atom!("hidden"),
	Dotted: atom!("dotted"),
	Dashed: atom!("dashed"),
	Solid: atom!("solid"),
	Double: atom!("double"),
	Groove: atom!("groove"),
	Ridge: atom!("ridge"),
	Inset: atom!("inset"),
	Outset: atom!("outset"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(LineStyle, 16);
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
