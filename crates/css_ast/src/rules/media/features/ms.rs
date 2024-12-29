use crate::units::{CSSFloat, CSSInt};
use css_parse::{discrete_feature, keyword_set, ranged_feature, RangedFeatureKeyword};

keyword_set!(MsHighContrastMediaFeatureKeyword { None: "none", Active: "active" });

discrete_feature!(MsHighContrastMediaFeature, "-ms-high-contrast", MsHighContrastMediaFeatureKeyword);

keyword_set!(MsViewStateMediaFeatureKeyword {
	Snapped: "snapped",
	FullscreenPortait: "fullscreen-portrait",
	FullscreenLandscape: "fullscreen-landscape",
});

discrete_feature!(MsViewStateMediaFeature, "-ms-view-state", MsViewStateMediaFeatureKeyword);

keyword_set!(MsImeAlignMediaFeatureKeyword { Auto: "auto" });

discrete_feature!(MsImeAlignMediaFeature, "-ms-ime-align", MsImeAlignMediaFeatureKeyword);

keyword_set!(MsDevicePixelRatioMediaFeatureKeyword {
	DevicePixelRatio: "-ms-device-pixel-ratio",
	MaxDevicePixelRatio: "-ms-max-device-pixel-ratio",
	MinDevicePixelRatio: "-ms-min-device-pixel-ratio",
});

impl RangedFeatureKeyword for MsDevicePixelRatioMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxDevicePixelRatio(_) | Self::MinDevicePixelRatio(_))
	}
}

ranged_feature!(MsDevicePixelRatioMediaFeature, MsDevicePixelRatioMediaFeatureKeyword, CSSFloat);

keyword_set!(MsColumnCountMediaFeatureKeyword {
	ColumnCount: "-ms-column-count",
	MaxColumnCount: "-ms-max-column-count",
	MinColumnCount: "-ms-min-column-count",
});

impl RangedFeatureKeyword for MsColumnCountMediaFeatureKeyword {
	fn is_legacy(&self) -> bool {
		matches!(self, Self::MaxColumnCount(_) | Self::MinColumnCount(_))
	}
}

ranged_feature!(MsColumnCountMediaFeature, MsColumnCountMediaFeatureKeyword, CSSInt);
