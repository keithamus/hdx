#[cfg(feature = "serde")]
use serde::Serialize;

use super::{ColorValue, Expr, MathExpr, PositiveLength, Shorthand};
use crate::{atom, Atom, Atomizable, Span};

// https://drafts.csswg.org/css-tables-3/#propdef-border-collapse
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum BorderCollapseValue {
	#[default]
	Separate, // atom!("separate")
	Collapse, // atom!("collapse")
}

// https://drafts.csswg.org/css-tables-3/#propdef-caption-side
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum CaptionSideValue {
	#[default]
	Top, // atom!("top")
	Bottom, // atom!("bottom")
}

// https://drafts.csswg.org/css-tables-3/#propdef-caption-side
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum EmptyCellsValue {
	#[default]
	Show, // atom!("show")
	Hide, // atom!("hide")
}

// https://drafts.csswg.org/css-tables-3/#propdef-caption-side
#[derive(Atomizable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum TableLayoutValue {
	#[default]
	Auto, // atom!("auto")
	Fixed, // atom!("fixed")
}

// atom!("border-collapse") => BorderCollapse<Expr<'a, BorderCollapseValue>>,
// atom!("border-spacing") => BorderSpacing<Expr<'a, BorderSpacingValue>>,
// atom!("caption-side") => CaptionSide<Expr<'a, CaptionSideValue>>,
// atom!("empty-cells") => EmptyCells<Expr<'a, EmptyCellsValue>>,
// atom!("table-layout") => TableLayout<Expr<'a, TableLayoutValue>>,
