use crate::{css::units::Length, macros::ranged_media_feature};

ranged_media_feature!(WidthMediaFeature[atom!("width")], Length);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WidthMediaFeature, 20);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthMediaFeature, "width: 360px");
		assert_parse!(WidthMediaFeature, "width: 35rem");
		assert_parse!(WidthMediaFeature, "min-width: 35rem");
		assert_parse!(WidthMediaFeature, "max-width: 35rem");
		assert_parse!(WidthMediaFeature, "width <= 800px");
		assert_parse!(WidthMediaFeature, "width >= 1400px");
		assert_parse!(WidthMediaFeature, "width >= 1400px");
		assert_parse!(WidthMediaFeature, "width = 1400px");
		assert_parse!(WidthMediaFeature, "1400px = width", "width = 1400px");
		assert_parse!(WidthMediaFeature, "100px <= width", "width <= 100px");
		assert_parse!(WidthMediaFeature, "100px < width < 1400px");
		assert_parse!(WidthMediaFeature, "100px > width < 1400px");
		assert_parse!(WidthMediaFeature, "100px >= width <= 1400px");
		assert_parse!(WidthMediaFeature, "100px <= width > 1400px");
	}

	#[test]
	fn test_minify() {
		assert_minify!(WidthMediaFeature, "width: 1px", "width:1px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WidthMediaFeature, "100px = width = 1400px");
		assert_parse_error!(WidthMediaFeature, "width:");
		assert_parse_error!(WidthMediaFeature, "width: > 10px");
		assert_parse_error!(WidthMediaFeature, "max-width > 10px");
		assert_parse_error!(WidthMediaFeature, "min-width > 10px");
		assert_parse_error!(WidthMediaFeature, "width: 1%");
		assert_parse_error!(WidthMediaFeature, "width: 1%");
		assert_parse_error!(WidthMediaFeature, "pointer: 1px");
	}
}
