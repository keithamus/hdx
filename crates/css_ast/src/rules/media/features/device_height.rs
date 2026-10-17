use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::DeviceHeight)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum DeviceHeightMediaFeature<'a>{CssAtomSet::DeviceHeight | CssAtomSet::MinDeviceHeight | CssAtomSet::MaxDeviceHeight, QueryValue<'a, Length>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height:360px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(min-device-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(max-device-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height<=800px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(1400px=device-height)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px<=device-height)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px<device-height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px>device-height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px>=device-height<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(100px<=device-height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height:)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(max-device-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(min-device-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(device-height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceHeightMediaFeature, "(pointer: 1px)");
	}
}
