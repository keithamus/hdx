use super::prelude::*;
use crate::units::CSSInt;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Color)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum ColorMediaFeature<'a>{CssAtomSet::Color | CssAtomSet::MinColor | CssAtomSet::MaxColor, QueryValue<'a, CSSInt>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(color:2)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(color:8)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(min-color:2)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(max-color:2)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(color<=3)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(color>=5)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(color>=8)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(color=16)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(6=color)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(2<=color)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(2<color<4)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(4>color<8)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(4>=color<=8)");
		assert_parse!(CssAtomSet::ATOMS, ColorMediaFeature, "(4<=color>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(color:)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(color: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(max-color > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(min-color > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(color: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(color: red)");
		assert_parse_error!(CssAtomSet::ATOMS, ColorMediaFeature, "(pointer: 1)");
	}
}
