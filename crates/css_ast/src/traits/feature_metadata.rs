use css_lexer::Span;
use css_parse::Comparison;

use crate::{CssAtomSet, traits::declaration_metadata::PropertyGroup};

/// Which conditional rule context a feature belongs to.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FeatureType {
	/// Feature is part of an `@media` query.
	Media,
	/// Feature is part of an `@container` query.
	Container,
}

/// What kind of value a conditional feature evaluates against.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FeatureEvaluator {
	/// Evaluates against a CSS `<length>` value.
	Length,
	/// Evaluates against a CSS `<integer>` value.
	Integer,
	/// Evaluates against a CSS `<number>` (float) value.
	Number,
	/// Evaluates against a CSS `<ratio>` value.
	Ratio,
	/// Evaluates against a CSS `<resolution>` value.
	Resolution,
	/// Evaluates against a discrete keyword value.
	Keyword,
	/// Evaluates as a boolean (feature present/absent or 0/non-0).
	Boolean,
}

/// The form of a ranged feature expression.
///
/// Covers all syntactic forms including bare presence check, legacy min/max prefix,
/// modern colon, and range comparison operators.
#[derive(Debug, Clone)]
pub enum RangedForm {
	/// `(feature)` — bare presence/boolean check of a ranged feature.
	Bare,
	/// `(feature: value)` — modern colon form.
	Exact { value: Span },
	/// `(min-feature: value)` — legacy min-prefixed colon form.
	LegacyMin { name: Span, colon: Span, value: Span },
	/// `(max-feature: value)` — legacy max-prefixed colon form.
	LegacyMax { name: Span, colon: Span, value: Span },
	/// `(feature op value)` — feature name on the left of the comparison.
	Left { comparison: Comparison, value: Span },
	/// `(value op feature)` — feature name on the right of the comparison.
	Right { value: Span, comparison: Comparison },
	/// `(v1 op feature op v2)` — range with feature name in the middle.
	Range { left: Span, left_cmp: Comparison, right_cmp: Comparison, right: Span },
}

/// Structured data extracted from a parsed conditional feature node.
///
/// Built by [`FeatureMetadata::feature_metadata()`]. `Plain` covers both discrete
/// and boolean features (the distinction is expressed via [`FeatureEvaluator`] in
/// the static metadata). `Ranged` covers all ranged forms including bare presence.
#[derive(Debug, Clone)]
pub enum ConditionalFeature {
	/// Discrete or boolean feature — bare `(feature)` or colon `(feature: value)`.
	Plain {
		/// Canonical atom for the feature name (e.g. `CssAtomSet::Width`).
		name: CssAtomSet,
		/// Value span, present when the colon form is used.
		value: Option<Span>,
	},
	/// Ranged feature — any form from bare through full range expression.
	Ranged {
		/// Canonical atom for the feature name (e.g. `CssAtomSet::Width`).
		/// For legacy min/max forms this is still the canonical atom, not the prefixed one.
		name: CssAtomSet,
		/// The specific comparison form and associated value span(s).
		form: RangedForm,
	},
}

impl ConditionalFeature {
	/// Returns the canonical feature-name atom regardless of variant.
	pub fn name(&self) -> CssAtomSet {
		match self {
			Self::Plain { name, .. } | Self::Ranged { name, .. } => *name,
		}
	}
}

pub trait FeatureMetadata: Sized {
	/// Which conditional rule context this feature belongs to.
	fn feature_type() -> FeatureType {
		FeatureType::Media
	}

	/// What kind of value this feature evaluates against.
	fn evaluator() -> FeatureEvaluator {
		FeatureEvaluator::Keyword
	}

	/// Which CSS specification module this feature belongs to.
	fn property_group() -> PropertyGroup {
		PropertyGroup::none()
	}

	/// Extracts structured instance data from this parsed feature node.
	fn feature_metadata(&self) -> ConditionalFeature;
}
