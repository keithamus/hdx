use crate::css::units::Length;
use hdx_parser::ranged_feature;

ranged_feature!(DeviceWidthMediaFeature[atom!("device-width")], Length);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(DeviceWidthMediaFeature, 100);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DeviceWidthMediaFeature, "device-width:360px");
		assert_parse!(DeviceWidthMediaFeature, "device-width:35rem");
		assert_parse!(DeviceWidthMediaFeature, "min-device-width:35rem");
		assert_parse!(DeviceWidthMediaFeature, "max-device-width:35rem");
		assert_parse!(DeviceWidthMediaFeature, "device-width<=800px");
		assert_parse!(DeviceWidthMediaFeature, "device-width>=1400px");
		assert_parse!(DeviceWidthMediaFeature, "device-width>=1400px");
		assert_parse!(DeviceWidthMediaFeature, "device-width=1400px");
		assert_parse!(DeviceWidthMediaFeature, "1400px=device-width");
		assert_parse!(DeviceWidthMediaFeature, "100px<=device-width");
		assert_parse!(DeviceWidthMediaFeature, "100px<device-width<1400px");
		assert_parse!(DeviceWidthMediaFeature, "100px>device-width<1400px");
		assert_parse!(DeviceWidthMediaFeature, "100px>=device-width<=1400px");
		assert_parse!(DeviceWidthMediaFeature, "100px<=device-width>1400px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DeviceWidthMediaFeature, "device-width:");
		assert_parse_error!(DeviceWidthMediaFeature, "device-width: > 10px");
		assert_parse_error!(DeviceWidthMediaFeature, "max-device-width > 10px");
		assert_parse_error!(DeviceWidthMediaFeature, "min-device-width > 10px");
		assert_parse_error!(DeviceWidthMediaFeature, "device-width: 1%");
		assert_parse_error!(DeviceWidthMediaFeature, "device-width: 1%");
		assert_parse_error!(DeviceWidthMediaFeature, "pointer: 1px");
	}
}
