use css_parse::{discrete_feature, keyword_set};

discrete_feature!(ScanMediaFeature, "scan", ScanMediaFeatureKeyword);

keyword_set!(ScanMediaFeatureKeyword { Interlace: "interlace", Progressive: "progressive" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScanMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScanMediaFeature, "(scan)");
		assert_parse!(ScanMediaFeature, "(scan:interlace)");
		assert_parse!(ScanMediaFeature, "(scan:progressive)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScanMediaFeature, "(scan:)");
		assert_parse_error!(ScanMediaFeature, "(scan: landscope)");
	}
}
