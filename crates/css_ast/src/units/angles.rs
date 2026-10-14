use super::prelude::*;

/// <https://drafts.csswg.org/css-values/#angles>
///
/// ```text,ignore
/// <angle> = <dimension-token>
/// ```
#[node]
#[derive(
	IntoCursor, ToSpan, SemanticEq, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.types.angle"))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub enum Angle {
	#[atom(CssAtomSet::Grad)]
	Grad(T![Dimension]),
	#[atom(CssAtomSet::Rad)]
	Rad(T![Dimension]),
	#[atom(CssAtomSet::Turn)]
	Turn(T![Dimension]),
	#[atom(CssAtomSet::Deg)]
	Deg(T![Dimension]),
}

impl From<Angle> for f32 {
	fn from(val: Angle) -> Self {
		match val {
			Angle::Grad(f) => f.into(),
			Angle::Rad(f) => f.into(),
			Angle::Turn(f) => f.into(),
			Angle::Deg(f) => f.into(),
		}
	}
}

impl ToNumberValue for Angle {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl ToNormalisedValue for Angle {
	fn to_normalised_value(&self) -> Option<f32> {
		Some(self.as_degrees())
	}
}

impl Angle {
	const DEG_GRAD: f32 = 0.9;
	const DEG_RAD: f32 = 57.295_78;
	const DEG_TURN: f32 = 360.0;

	pub fn as_degrees(&self) -> f32 {
		match self {
			Self::Grad(d) => Into::<f32>::into(*d) * Self::DEG_GRAD,
			Self::Rad(d) => Into::<f32>::into(*d) * Self::DEG_RAD,
			Self::Turn(d) => Into::<f32>::into(*d) * Self::DEG_TURN,
			Self::Deg(d) => (*d).into(),
		}
	}
}

#[node]
#[derive(
	IntoCursor, ToSpan, SemanticEq, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum AngleOrZero {
	Angle(Angle),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Zero(Exact<T![Number], 0>),
}

#[node]
#[derive(
	IntoCursor, ToSpan, SemanticEq, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum AngleOrNumber {
	Angle(Angle),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Number(T![Number]),
}

/// <https://drafts.csswg.org/css-values/#typedef-angle-percentage>
///
/// ```text,ignore
/// <angle-percentage> = [ <angle> | <percentage> ]
/// ```
///
/// Also admits `<zero>`, as every use in the gradient grammars does.
#[node]
#[derive(
	IntoCursor, ToSpan, SemanticEq, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum AnglePercentage {
	#[cfg_attr(feature = "visitable", visit(skip))]
	Zero(Exact<T![Number], 0>),
	Angle(Angle),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Percent(crate::Percentage),
}

impl From<AnglePercentage> for f32 {
	fn from(val: AnglePercentage) -> Self {
		match val {
			AnglePercentage::Zero(_) => 0.0,
			AnglePercentage::Angle(f) => f.into(),
			AnglePercentage::Percent(f) => f.into(),
		}
	}
}

impl ToNumberValue for AnglePercentage {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl From<AngleOrZero> for f32 {
	fn from(val: AngleOrZero) -> Self {
		match val {
			AngleOrZero::Angle(f) => f.into(),
			AngleOrZero::Zero(f) => f.0.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Angle, "0grad");
		assert_parse!(CssAtomSet::ATOMS, Angle, "0deg");
		assert_parse!(CssAtomSet::ATOMS, AngleOrZero, "0", AngleOrZero::Zero(_));
	}
}
