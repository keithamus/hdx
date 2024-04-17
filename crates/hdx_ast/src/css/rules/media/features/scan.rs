use crate::macros::discrete_media_feature;

discrete_media_feature!(ScanMediaFeature[atom!("scan")] {
	Interlace: atom!("interlace"),
	Progressive: atom!("progressive"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ScanMediaFeature, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScanMediaFeature, "scan");
		assert_parse!(ScanMediaFeature, "scan: interlace");
		assert_parse!(ScanMediaFeature, "scan: progressive");
	}

	#[test]
	fn test_minify() {
		assert_minify!(ScanMediaFeature, "scan: interlace", "scan:interlace");
		assert_minify!(ScanMediaFeature, "scan: progressive", "scan:progressive");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScanMediaFeature, "scan:");
		assert_parse_error!(ScanMediaFeature, "scan: landscope");
	}
}
