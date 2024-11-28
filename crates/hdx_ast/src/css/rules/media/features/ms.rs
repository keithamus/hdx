use crate::css::units::{CSSFloat, CSSInt};
use hdx_parser::{discrete_media_feature, ranged_media_feature};

discrete_media_feature!(MsHighContrastMediaFeature[atom!("-ms-high-contrast")] {
	None: atom!("none"),
	Active: atom!("active"),
});

discrete_media_feature!(MsViewStateMediaFeature[atom!("-ms-view-state")] {
	Snapped: atom!("snapped"),
	FullscreenPortait: atom!("fullscreen-portrait"),
	FullscreenLandscape: atom!("fullscreen-landscape"),
});

discrete_media_feature!(MsImeAlignMediaFeature[atom!("-ms-ime-align")] {
	Auto: atom!("auto"),
});

ranged_media_feature!(MsDevicePixelRatioMediaFeature[atom!("-ms-device-pixel-ratio")], CSSFloat);

ranged_media_feature!(MsColumnCountMediaFeature[atom!("-ms-column-count")], CSSInt);
