use crate::css::units::CSSFloat;
use hdx_parser::{discrete_feature, ranged_feature};

discrete_feature!(WebkitAnimationMediaFeature[atom!("-webkit-animation")] {
	True: atom!("true"),
	False: atom!("false"),
});

discrete_feature!(WebkitTransform2dMediaFeature[atom!("-webkit-transform-2d")] {
	True: atom!("true"),
	False: atom!("false"),
});

discrete_feature!(WebkitTransform3dMediaFeature[atom!("-webkit-transform-3d")] {
	True: atom!("true"),
	False: atom!("false"),
});

discrete_feature!(WebkitTransitionMediaFeature[atom!("-webkit-transition")] {
	True: atom!("true"),
});

discrete_feature!(WebkitVideoPlayableInlineMediaFeature[atom!("-webkit-video-playable-inline")] {
	True: atom!("true"),
});

ranged_feature!(WebkitDevicePixelRatioMediaFeature[atom!("-webkit-device-pixel-ratio")], CSSFloat);
