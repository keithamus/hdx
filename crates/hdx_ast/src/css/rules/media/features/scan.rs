use hdx_parser::discrete_feature;

discrete_feature!(ScanMediaFeature[atom!("scan")] {
	Interlace: atom!("interlace"),
	Progressive: atom!("progressive"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ScanMediaFeature, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScanMediaFeature, "scan");
		assert_parse!(ScanMediaFeature, "scan:interlace");
		assert_parse!(ScanMediaFeature, "scan:progressive");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScanMediaFeature, "scan:");
		assert_parse_error!(ScanMediaFeature, "scan: landscope");
	}
}
