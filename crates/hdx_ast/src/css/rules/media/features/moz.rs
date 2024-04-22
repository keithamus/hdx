use crate::{
	css::units::CSSFloat,
	macros::{discrete_media_feature, ranged_media_feature},
};

ranged_media_feature!(MozDevicePixelRatioMediaFeature[atom!("-moz-device-pixel-ratio")], CSSFloat);

discrete_media_feature!(MozDeviceOrientationMediaFeature[atom!("-moz-device-orientation")] {
	Portrait: atom!("portrait"),
	Landscape: atom!("landscape"),
});
