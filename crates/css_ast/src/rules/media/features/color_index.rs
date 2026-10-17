use super::prelude::*;
use crate::units::CSSInt;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::ColorIndex)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum ColorIndexMediaFeature<'a>{CssAtomSet::ColorIndex | CssAtomSet::MaxColorIndex | CssAtomSet::MinColorIndex, QueryValue<'a, CSSInt>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index:2)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index:8)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(min-color-index:2)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(max-color-index:2)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index<=3)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index>=5)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index>=8)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index=16)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(6=color-index)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(2<=color-index)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(2<color-index<4)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(4>color-index<8)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(4>=color-index<=8)");
		assert_parse!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(4<=color-index>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index:)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(max-color-index > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(min-color-index > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(color-index: red)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorIndexMediaFeature, "(pointer: 1)");
	}
}
