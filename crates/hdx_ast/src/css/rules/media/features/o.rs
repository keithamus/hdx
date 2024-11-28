use crate::css::units::CSSFloat;
use hdx_parser::ranged_media_feature;

ranged_media_feature!(ODevicePixelRatioMediaFeature[atom!("-o-device-pixel-ratio")], CSSFloat);
