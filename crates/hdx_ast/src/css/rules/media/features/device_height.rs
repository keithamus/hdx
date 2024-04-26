use crate::{css::units::Length, macros::ranged_media_feature};

ranged_media_feature!(DeviceHeightMediaFeature[atom!("device-height")], Length);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DeviceHeightMediaFeature, 20);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DeviceHeightMediaFeature, "device-height: 360px");
		assert_parse!(DeviceHeightMediaFeature, "device-height: 35rem");
		assert_parse!(DeviceHeightMediaFeature, "min-device-height: 35rem");
		assert_parse!(DeviceHeightMediaFeature, "max-device-height: 35rem");
		assert_parse!(DeviceHeightMediaFeature, "device-height <= 800px");
		assert_parse!(DeviceHeightMediaFeature, "device-height >= 1400px");
		assert_parse!(DeviceHeightMediaFeature, "device-height >= 1400px");
		assert_parse!(DeviceHeightMediaFeature, "device-height = 1400px");
		assert_parse!(DeviceHeightMediaFeature, "1400px = device-height", "device-height = 1400px");
		assert_parse!(DeviceHeightMediaFeature, "100px <= device-height", "device-height <= 100px");
		assert_parse!(DeviceHeightMediaFeature, "100px < device-height < 1400px");
		assert_parse!(DeviceHeightMediaFeature, "100px > device-height < 1400px");
		assert_parse!(DeviceHeightMediaFeature, "100px >= device-height <= 1400px");
		assert_parse!(DeviceHeightMediaFeature, "100px <= device-height > 1400px");
	}

	#[test]
	fn test_minify() {
		assert_minify!(DeviceHeightMediaFeature, "device-height: 1px", "device-height:1px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DeviceHeightMediaFeature, "100px = device-height = 1400px");
		assert_parse_error!(DeviceHeightMediaFeature, "device-height:");
		assert_parse_error!(DeviceHeightMediaFeature, "device-height: > 10px");
		assert_parse_error!(DeviceHeightMediaFeature, "max-device-height > 10px");
		assert_parse_error!(DeviceHeightMediaFeature, "min-device-height > 10px");
		assert_parse_error!(DeviceHeightMediaFeature, "device-height: 1%");
		assert_parse_error!(DeviceHeightMediaFeature, "device-height: 1%");
		assert_parse_error!(DeviceHeightMediaFeature, "pointer: 1px");
	}
}
