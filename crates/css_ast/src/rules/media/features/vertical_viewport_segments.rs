use super::prelude::*;
use crate::units::CSSInt;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::VerticalViewportSegments)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum VerticalViewportSegmentsMediaFeature<'a>{CssAtomSet::VerticalViewportSegments | CssAtomSet::MaxVerticalViewportSegments | CssAtomSet::MinVerticalViewportSegments, QueryValue<'a, CSSInt>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments:2)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments:8)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(min-vertical-viewport-segments:2)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(max-vertical-viewport-segments:2)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments<=3)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments>=5)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments>=8)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments=16)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(6=vertical-viewport-segments)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(2<=vertical-viewport-segments)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(2<vertical-viewport-segments<4)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(4>vertical-viewport-segments<8)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(4>=vertical-viewport-segments<=8)");
		assert_parse!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(4<=vertical-viewport-segments>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(vertical-viewport-segments:)");
		assert_parse_error!(
			CssAtomSet::ATOMS,
			VerticalViewportSegmentsMediaFeature,
			"(vertical-viewport-segments: > 10px)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			VerticalViewportSegmentsMediaFeature,
			"(max-vertical-viewport-segments > 10px)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			VerticalViewportSegmentsMediaFeature,
			"(min-vertical-viewport-segments > 10px)"
		);
		assert_parse_error!(
			CssAtomSet::ATOMS,
			VerticalViewportSegmentsMediaFeature,
			"(vertical-viewport-segments: 1px)"
		);
		assert_parse_error!(CssAtomSet::ATOMS, VerticalViewportSegmentsMediaFeature, "(pointer: 1)");
	}
}
