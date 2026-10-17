use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Height)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum HeightMediaFeature<'a>{CssAtomSet::Height | CssAtomSet::MinHeight | CssAtomSet::MaxHeight, QueryValue<'a, Length>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height:360px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(min-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(max-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height<=800px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(1400px=height)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px<=height)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px<height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px>height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px>=height<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px<=height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height:)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(max-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(min-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(pointer: 1px)");
	}
}
