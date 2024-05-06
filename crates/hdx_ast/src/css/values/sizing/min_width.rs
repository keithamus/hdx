use hdx_derive::{Parsable, Value, Writable};

use crate::css::units::LengthPercentage;

// https://drafts.csswg.org/css-sizing-4/#sizing-values
#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum MinWidth {
	#[default]
	Auto, // atom!("auto")
	MinContent, // atom!("min-content")
	MaxContent, // atom!("max-content")  TODO: `intrinsic` non standard
	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	Stretch,    // atom!("stretch")  TODO: -webkit-fill-available, -moz-available
	FitContent, // atom!("fit-content")
	Contain,    // atom!("contain")

	#[parsable(DimensionOrZero, Check::Range(0.0..), parse_inner)]
	LengthPercentage(LengthPercentage),
	#[parsable(Function, Check::Range(0.0..), atom = "fit-content", parse_inner)]
	#[writable(as_function = "fit-content")]
	FitContentFunction(LengthPercentage),
}

