use hdx_parser::discrete_feature;

discrete_feature!(NavControlsMediaFeature[atom!("nav-controls")] {
	None: atom!("none"),
	Back: atom!("back"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(NavControlsMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NavControlsMediaFeature, "nav-controls");
		assert_parse!(NavControlsMediaFeature, "nav-controls:back");
		assert_parse!(NavControlsMediaFeature, "nav-controls:none");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(NavControlsMediaFeature, "nav-controls:");
		assert_parse_error!(NavControlsMediaFeature, "nav-controls: hoover");
	}
}
