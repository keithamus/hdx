use crate::units::Length;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(DeviceWidthMediaFeatureKeyword {
	DeviceWidth: "device-width",
	MaxDeviceWidth: "max-device-width",
	MinDeviceWidth: "min-device-width",
});

impl RangedFeatureKeyword for DeviceWidthMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDeviceWidth(_) | Self::MinDeviceWidth(_))
	}
}

ranged_feature!(DeviceWidthMediaFeature, DeviceWidthMediaFeatureKeyword, Length);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DeviceWidthMediaFeature>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DeviceWidthMediaFeature, "(device-width:360px)");
		assert_parse!(DeviceWidthMediaFeature, "(device-width:35rem)");
		assert_parse!(DeviceWidthMediaFeature, "(min-device-width:35rem)");
		assert_parse!(DeviceWidthMediaFeature, "(max-device-width:35rem)");
		assert_parse!(DeviceWidthMediaFeature, "(device-width<=800px)");
		assert_parse!(DeviceWidthMediaFeature, "(device-width>=1400px)");
		assert_parse!(DeviceWidthMediaFeature, "(device-width>=1400px)");
		assert_parse!(DeviceWidthMediaFeature, "(device-width=1400px)");
		assert_parse!(DeviceWidthMediaFeature, "(1400px=device-width)");
		assert_parse!(DeviceWidthMediaFeature, "(100px<=device-width)");
		assert_parse!(DeviceWidthMediaFeature, "(100px<device-width<1400px)");
		assert_parse!(DeviceWidthMediaFeature, "(100px>device-width<1400px)");
		assert_parse!(DeviceWidthMediaFeature, "(100px>=device-width<=1400px)");
		assert_parse!(DeviceWidthMediaFeature, "(100px<=device-width>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DeviceWidthMediaFeature, "(device-width:)");
		assert_parse_error!(DeviceWidthMediaFeature, "(device-width: > 10px)");
		assert_parse_error!(DeviceWidthMediaFeature, "(max-device-width > 10px)");
		assert_parse_error!(DeviceWidthMediaFeature, "(min-device-width > 10px)");
		assert_parse_error!(DeviceWidthMediaFeature, "(device-width: 1%)");
		assert_parse_error!(DeviceWidthMediaFeature, "(device-width: 1%)");
		assert_parse_error!(DeviceWidthMediaFeature, "(pointer: 1px)");
	}
}
