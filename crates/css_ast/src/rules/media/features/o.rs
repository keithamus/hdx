use super::prelude::*;
use crate::units::CSSFloat;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::_ODevicePixelRatio)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum ODevicePixelRatioMediaFeature<'a>{CssAtomSet::_ODevicePixelRatio | CssAtomSet::_OMinDevicePixelRatio | CssAtomSet::_OMaxDevicePixelRatio, QueryValue<'a, CSSFloat>}
);
