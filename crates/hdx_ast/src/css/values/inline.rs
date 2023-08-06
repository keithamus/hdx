use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use super::{Expr, LengthPercentage, MathExpr, Shorthand};
use crate::{atom, Atom, Atomizable, Spanned};

// https://drafts.csswg.org/css-inline/#propdef-alignment-baseline
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum AlignmentBaselineValue {
	#[default]
	Baseline, // atom!("baseline")
	TextBottom,   // atom!("text-bottom")
	Alphabetic,   // atom!("alphabetic")
	Ideographic,  // atom!("ideograpic")
	Middle,       // atom!("middle")
	Central,      // atom!("central")
	Mathematical, // atom!("mathematical")
	TextTop,      // atom!("text-top")
}

// https://drafts.csswg.org/css-inline/#propdef-baseline-source
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BaselineSourceValue {
	#[default]
	Auto, // atom!("auto")
	First, // atom!("first")
	Last,  // atom!("last")
}

// https://drafts.csswg.org/css-inline-3/#propdef-baseline-shift
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum BaselineShiftValue {
	LengthPercentage(Spanned<LengthPercentage>),
	Sub,    // atom!("sub"),
	Super,  // atom!("super")
	Top,    // atom!("top")
	Center, // atom!("center")
	Bottom, // atom!("bottom")
}

impl Default for BaselineShiftValue {
	fn default() -> Self {
		Self::LengthPercentage(Spanned::dummy(LengthPercentage::default()))
	}
}

// https://drafts.csswg.org/css-inline/#propdef-dominant-baseline
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DominantBaselineValue {
	#[default]
	Auto, // atom!("auto")
	TextBottom,   // atom!("text-bottom")
	Alphabetic,   // atom!("alphabetic")
	Ideographic,  // atom!("ideographic")
	Middle,       // atom!("middle")
	Central,      // atom!("central")
	Mathematical, // atom!("mathematical")
	Hanging,      // atom!("hanging")
	TextTop,      // atom!("text-top")
}

// https://drafts.csswg.org/css-inline/#propdef-inline-sizing
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum InlineSizingValue {
	#[default]
	Normal, // atom!("normal")
	Stretch, // atom!("stetch")
}

// https://drafts.csswg.org/css-inline/#line-height-property
#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LineHeightValue {
	#[default]
	Normal, // atom!("normal")
	Number(f32),
	LengthPercentage(Spanned<LengthPercentage>),
}

impl Hash for LineHeightValue {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match self {
			Self::Normal => 0.hash(state),
			Self::Number(n) => {
				1.hash(state);
				n.to_bits().hash(state);
			}
			Self::LengthPercentage(lp) => {
				2.hash(state);
				lp.hash(state);
			}
		}
	}
}

// https://drafts.csswg.org/css-inline-3/#propdef-vertical-align
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct VerticalAlignShorthand<'a> {
	pub baseline_source: Shorthand<'a, Expr<'a, BaselineSourceValue>>,
	pub alignment_baseline: Shorthand<'a, Expr<'a, AlignmentBaselineValue>>,
	pub baseline_shift: Shorthand<'a, MathExpr<'a, BaselineShiftValue>>,
}
