use crate::units::Length;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(DeviceHeightMediaFeatureKeyword {
	DeviceHeight: "device-height",
	MaxDeviceHeight: "max-device-height",
	MinDeviceHeight: "min-device-height",
});

impl RangedFeatureKeyword for DeviceHeightMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDeviceHeight(_) | Self::MinDeviceHeight(_))
	}
}

ranged_feature!(DeviceHeightMediaFeature, DeviceHeightMediaFeatureKeyword, Length);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DeviceHeightMediaFeature>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DeviceHeightMediaFeature, "(device-height:360px)");
		assert_parse!(DeviceHeightMediaFeature, "(device-height:35rem)");
		assert_parse!(DeviceHeightMediaFeature, "(min-device-height:35rem)");
		assert_parse!(DeviceHeightMediaFeature, "(max-device-height:35rem)");
		assert_parse!(DeviceHeightMediaFeature, "(device-height<=800px)");
		assert_parse!(DeviceHeightMediaFeature, "(device-height>=1400px)");
		assert_parse!(DeviceHeightMediaFeature, "(device-height>=1400px)");
		assert_parse!(DeviceHeightMediaFeature, "(device-height=1400px)");
		assert_parse!(DeviceHeightMediaFeature, "(1400px=device-height)");
		assert_parse!(DeviceHeightMediaFeature, "(100px<=device-height)");
		assert_parse!(DeviceHeightMediaFeature, "(100px<device-height<1400px)");
		assert_parse!(DeviceHeightMediaFeature, "(100px>device-height<1400px)");
		assert_parse!(DeviceHeightMediaFeature, "(100px>=device-height<=1400px)");
		assert_parse!(DeviceHeightMediaFeature, "(100px<=device-height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DeviceHeightMediaFeature, "(device-height:)");
		assert_parse_error!(DeviceHeightMediaFeature, "(device-height: > 10px)");
		assert_parse_error!(DeviceHeightMediaFeature, "(max-device-height > 10px)");
		assert_parse_error!(DeviceHeightMediaFeature, "(min-device-height > 10px)");
		assert_parse_error!(DeviceHeightMediaFeature, "(device-height: 1%)");
		assert_parse_error!(DeviceHeightMediaFeature, "(device-height: 1%)");
		assert_parse_error!(DeviceHeightMediaFeature, "(pointer: 1px)");
	}
}
