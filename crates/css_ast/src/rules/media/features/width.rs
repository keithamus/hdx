use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[derive(csskit_derives::FeatureMetadata)]
	#[feature_metadata(CssAtomSet::Width)]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
	#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WidthMediaFeature<'a>{CssAtomSet::Width | CssAtomSet::MinWidth | CssAtomSet::MaxWidth, QueryValue<'a, Length>}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, FeatureMetadata, RangedForm};
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_math_function_values() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(max-width:calc(48rem - 0.02px))", |node| {
			assert!(matches!(node, WidthMediaFeature::Max(_, _, _, QueryValue::Substituted(_), _)));
		});
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width<=min(50vw,800px))");
		assert_parse!(
			CssAtomSet::ATOMS,
			WidthMediaFeature,
			"(calc(1px + 2px)<width<=env(viewport-segment-width 0 0,800px))"
		);
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:var(--w))");
	}

	#[test]
	fn test_feature_metadata_exact() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:360px)", |node| {
			assert!(matches!(
				node.feature_metadata(),
				crate::ConditionalFeature::Ranged { name: CssAtomSet::Width, form: RangedForm::Exact { .. } }
			));
		});
	}

	#[test]
	fn test_feature_metadata_legacy_min() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(min-width:35rem)", |node| {
			assert!(matches!(
				node.feature_metadata(),
				crate::ConditionalFeature::Ranged { name: CssAtomSet::Width, form: RangedForm::LegacyMin { .. } }
			));
		});
	}

	#[test]
	fn test_feature_metadata_legacy_max() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(max-width:35rem)", |node| {
			assert!(matches!(
				node.feature_metadata(),
				crate::ConditionalFeature::Ranged { name: CssAtomSet::Width, form: RangedForm::LegacyMax { .. } }
			));
		});
	}

	#[test]
	fn test_feature_metadata_left_comparison() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width<=800px)", |node| {
			let crate::ConditionalFeature::Ranged { name, form: RangedForm::Left { comparison, .. } } =
				node.feature_metadata()
			else {
				panic!("expected Ranged::Left");
			};
			assert_eq!(name, CssAtomSet::Width);
			assert_eq!(comparison.to_string(), "<=");
		});
	}

	#[test]
	fn test_feature_metadata_right_comparison() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<=width)", |node| {
			let crate::ConditionalFeature::Ranged { name, form: RangedForm::Right { comparison, .. } } =
				node.feature_metadata()
			else {
				panic!("expected Ranged::Right");
			};
			assert_eq!(name, CssAtomSet::Width);
			assert_eq!(comparison.to_string(), "<=");
		});
	}

	#[test]
	fn test_feature_metadata_range() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<width<1400px)", |node| {
			let crate::ConditionalFeature::Ranged { name, form: RangedForm::Range { left_cmp, right_cmp, .. } } =
				node.feature_metadata()
			else {
				panic!("expected Ranged::Range");
			};
			assert_eq!(name, CssAtomSet::Width);
			assert_eq!(left_cmp.to_string(), "<");
			assert_eq!(right_cmp.to_string(), "<");
		});
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:360px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(min-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(max-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width<=800px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(1400px=width)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<=width)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px>width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px>=width<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<=width>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(max-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(min-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(pointer: 1px)");
	}
}
