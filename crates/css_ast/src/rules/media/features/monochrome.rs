use super::prelude::*;
use crate::units::CSSInt;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Monochrome)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum MonochromeMediaFeature<'a>{CssAtomSet::Monochrome | CssAtomSet::MinMonochrome | CssAtomSet::MaxMonochrome, QueryValue<'a, CSSInt>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome:2)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome:8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(min-monochrome:2)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(max-monochrome:2)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome<=3)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome>=5)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome>=8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome=16)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(6=monochrome)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(2<=monochrome)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(2<monochrome<4)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(4>monochrome<8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(4>=monochrome<=8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(4<=monochrome>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome:)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(max-monochrome > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(min-monochrome > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome: red)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(pointer: 1)");
	}
}
