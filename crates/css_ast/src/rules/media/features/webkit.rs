use super::prelude::*;
use crate::units::CSSFloat;

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BooleanKeyword {
	#[atom(CssAtomSet::True)]
	True(T![Ident]),
	#[atom(CssAtomSet::False)]
	False(T![Ident]),
}

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_WebkitAnimation)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitAnimationMediaFeature{CssAtomSet::_WebkitAnimation, BooleanKeyword}
);

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_WebkitTransform2d)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitTransform2dMediaFeature{CssAtomSet::_WebkitTransform2d, BooleanKeyword}
);

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_WebkitTransform3d)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitTransform3dMediaFeature{CssAtomSet::_WebkitTransform3d, BooleanKeyword}
);

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_WebkitTransition)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitTransitionMediaFeature{CssAtomSet::_WebkitTransition, BooleanKeyword}
);

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_WebkitVideoPlayableInline)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitVideoPlayableInlineMediaFeature{CssAtomSet::_WebkitVideoPlayableInline, BooleanKeyword});

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_WebkitDevicePixelRatio)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitDevicePixelRatioMediaFeature<'a>{CssAtomSet::_WebkitDevicePixelRatio | CssAtomSet::_WebkitMinDevicePixelRatio | CssAtomSet::_WebkitMaxDevicePixelRatio, QueryValue<'a, CSSFloat>}
);
