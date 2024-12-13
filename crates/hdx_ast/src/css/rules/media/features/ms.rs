use crate::css::units::{CSSFloat, CSSInt};
use hdx_parser::{discrete_feature, ranged_feature};

discrete_feature!(MsHighContrastMediaFeature[atom!("-ms-high-contrast")] {
	None: atom!("none"),
	Active: atom!("active"),
});

discrete_feature!(MsViewStateMediaFeature[atom!("-ms-view-state")] {
	Snapped: atom!("snapped"),
	FullscreenPortait: atom!("fullscreen-portrait"),
	FullscreenLandscape: atom!("fullscreen-landscape"),
});

discrete_feature!(MsImeAlignMediaFeature[atom!("-ms-ime-align")] {
	Auto: atom!("auto"),
});

ranged_feature!(MsDevicePixelRatioMediaFeature[atom!("-ms-device-pixel-ratio")], CSSFloat);

ranged_feature!(MsColumnCountMediaFeature[atom!("-ms-column-count")], CSSInt);
