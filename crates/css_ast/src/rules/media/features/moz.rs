use crate::units::CSSFloat;
use css_parse::{boolean_feature, discrete_feature, keyword_set, ranged_feature, RangedFeatureKeyword};

// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#media_features

keyword_set!(MozDevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-moz-device-pixel-ratio",
	MaxDevicePixelRatio: "-moz-max-device-pixel-ratio",
	MinDevicePixelRatio: "-moz-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for MozDevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(MozDevicePixelRatioMediaFeature, MozDevicePixelRatioMediaFeatureKeyword, CSSFloat);

keyword_set!(MozDeviceOrientationMediaFeatureKeyword { Portrait: "portrait", Landscape: "landscape" });

discrete_feature!(MozDeviceOrientationMediaFeature, "-moz-device-orientation", MozDeviceOrientationMediaFeatureKeyword);

boolean_feature!(MozMacGraphiteThemeMediaFeature, "-moz-mac-graphite-theme");

boolean_feature!(MozMaemoClassicMediaFeature, "-moz-maemo-classic-theme");

boolean_feature!(MozImagesInMenusMediaFeature, "-moz-maemo-classic-theme");

keyword_set!(MozOsVersionMediaFeatureKeyword {
	WindowsVista: "windows-vista",
	WindowsXp: "windows-xp",
	WindowsWin7: "windows-win7",
	WindowsWin8: "windows-win8",
	WindowsWin10: "windows-win10",
});

discrete_feature!(MozOsVersionMediaFeature, "-moz-os-version", MozOsVersionMediaFeatureKeyword);

boolean_feature!(MozTouchEnabledMediaFeature, "-moz-touch-enabled");
