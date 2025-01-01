use crate::units::CSSFloat;
use css_parse::{keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(ODevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-o-device-pixel-ratio",
	MaxDevicePixelRatio: "-o-max-device-pixel-ratio",
	MinDevicePixelRatio: "-o-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for ODevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(ODevicePixelRatioMediaFeature, ODevicePixelRatioMediaFeatureKeyword, CSSFloat);
