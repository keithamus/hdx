use super::prelude::*;
use crate::units::Resolution;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Resolution)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum ResolutionMediaFeature<'a>{CssAtomSet::Resolution | CssAtomSet::MinResolution | CssAtomSet::MaxResolution, QueryValue<'a, ResolutionMediaFeatureValue>}
);

/// <https://drafts.csswg.org/mediaqueries-5/#resolution>
///
/// ```text,ignore
/// <resolution> | infinite
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ResolutionMediaFeatureValue {
	#[atom(CssAtomSet::Infinite)]
	Infinite(T![Ident]),
	Resolution(Resolution),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution:2dppx)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution:1x)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution:96dpi)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution:infinite)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(min-resolution:96dpi)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(max-resolution:300dpi)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution<=2dppx)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution>=96dpi)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution=1x)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(96dpi=resolution)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(1x<=resolution)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(1x<resolution<3x)");
		assert_parse!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(1x>=resolution<=3x)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution:)");
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution: > 1x)");
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(max-resolution > 1x)");
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(min-resolution > 1x)");
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(resolution: 10)");
		assert_parse_error!(CssAtomSet::ATOMS, ResolutionMediaFeature, "(pointer: 1x)");
	}
}
