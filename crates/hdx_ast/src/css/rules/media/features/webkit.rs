use crate::css::units::CSSFloat;
use hdx_parser::{discrete_media_feature, ranged_media_feature};

discrete_media_feature!(WebkitAnimationMediaFeature[atom!("-webkit-animation")] {
	True: atom!("true"),
	False: atom!("false"),
});

discrete_media_feature!(WebkitTransform2dMediaFeature[atom!("-webkit-transform-2d")] {
	True: atom!("true"),
	False: atom!("false"),
});

discrete_media_feature!(WebkitTransform3dMediaFeature[atom!("-webkit-transform-3d")] {
	True: atom!("true"),
	False: atom!("false"),
});

discrete_media_feature!(WebkitTransitionMediaFeature[atom!("-webkit-transition")] {
	True: atom!("true"),
});

discrete_media_feature!(WebkitVideoPlayableInlineMediaFeature[atom!("-webkit-video-playable-inline")] {
	True: atom!("true"),
});

ranged_media_feature!(WebkitDevicePixelRatioMediaFeature[atom!("-webkit-device-pixel-ratio")], CSSFloat);
