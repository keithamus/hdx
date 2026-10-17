use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::DeviceWidth)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum DeviceWidthMediaFeature<'a>{CssAtomSet::DeviceWidth | CssAtomSet::MinDeviceWidth | CssAtomSet::MaxDeviceWidth, QueryValue<'a, Length>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width:360px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(min-device-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(max-device-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width<=800px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(1400px=device-width)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px<=device-width)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px<device-width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px>device-width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px>=device-width<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(100px<=device-width>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width:)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(max-device-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(min-device-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(device-width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, DeviceWidthMediaFeature, "(pointer: 1px)");
	}
}
