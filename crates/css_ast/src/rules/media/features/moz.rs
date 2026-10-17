use super::prelude::*;
use crate::units::CSSFloat;

// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#media_features

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozDevicePixelRatio)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozDevicePixelRatioMediaFeature<'a>{CssAtomSet::_MozDevicePixelRatio | CssAtomSet::_MozMinDevicePixelRatio | CssAtomSet::_MozMaxDevicePixelRatio, QueryValue<'a, CSSFloat>}
);
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozDeviceOrientationMediaFeatureKeyword {
	#[atom(CssAtomSet::Portrait)]
	Portrait(T![Ident]),
	#[atom(CssAtomSet::Landscape)]
	Landscape(T![Ident]),
}

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozDeviceOrientation)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozDeviceOrientationMediaFeature{CssAtomSet::_MozDeviceOrientation, MozDeviceOrientationMediaFeatureKeyword}
);

boolean_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozMacGraphiteTheme)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozMacGraphiteThemeMediaFeature{CssAtomSet::_MozMacGraphiteTheme}
);

boolean_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozMaemoClassicTheme)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozMaemoClassicMediaFeature{CssAtomSet::_MozMaemoClassicTheme}
);

boolean_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozMaemoClassicTheme)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozImagesInMenusMediaFeature{CssAtomSet::_MozMaemoClassicTheme}
);
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MozOsVersionMediaFeatureKeyword {
	#[atom(CssAtomSet::WindowsVista)]
	WindowsVista(T![Ident]),
	#[atom(CssAtomSet::WindowsXp)]
	WindowsXp(T![Ident]),
	#[atom(CssAtomSet::WindowsWin7)]
	WindowsWin7(T![Ident]),
	#[atom(CssAtomSet::WindowsWin8)]
	WindowsWin8(T![Ident]),
	#[atom(CssAtomSet::WindowsWin10)]
	WindowsWin10(T![Ident]),
}

discrete_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozOsVersion)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozOsVersionMediaFeature{CssAtomSet::_MozOsVersion, MozOsVersionMediaFeatureKeyword}
);

boolean_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_MozTouchEnabled)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MozTouchEnabledMediaFeature{CssAtomSet::_MozTouchEnabled}
);
