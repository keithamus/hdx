use crate::css::units::CSSFloat;
use hdx_parser::{bool_media_feature, discrete_media_feature, ranged_media_feature};

// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#media_features

ranged_media_feature!(MozDevicePixelRatioMediaFeature[atom!("-moz-device-pixel-ratio")], CSSFloat);

discrete_media_feature!(MozDeviceOrientationMediaFeature[atom!("-moz-device-orientation")] {
	Portrait: atom!("portrait"),
	Landscape: atom!("landscape"),
});

bool_media_feature!(MozMacGraphiteThemeMediaFeature[atom!("-moz-mac-graphite-theme")]);

bool_media_feature!(MozMaemoClassicMediaFeature[atom!("-moz-maemo-classic-theme")]);

bool_media_feature!(MozImagesInMenusMediaFeature[atom!("-moz-maemo-classic-theme")]);

discrete_media_feature!(MozOsVersionMediaFeature[atom!("-moz-os-version")] {
	WindowsVista: atom!("windows-vista"),
	WindowsXp: atom!("windows-xp"),
	WindowsWin7: atom!("windows-win7"),
	WindowsWin8: atom!("windows-win8"),
	WindowsWin10: atom!("windows-win10"),
});

bool_media_feature!(MozTouchEnabledMediaFeature[atom!("-moz-touch-enabled")]);
