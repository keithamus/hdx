use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

/// Values
mod align;
mod anchor_position;
mod animations;
mod backgrounds;
mod r#box;
mod r#break;
mod cascade;
mod color;
mod color_adjust;
mod compositing;
mod contain;
mod content;
mod css2;
mod display;
mod exclusions;
mod fill_stroke;
mod filter_effects;
mod flexbox;
mod fonts;
mod gcpm;
mod grid;
mod images;
mod inline;
mod line_grid;
mod link_params;
mod lists;
mod logical;
mod masking;
mod motion;
mod multicol;
mod nav;
mod non_standard;
mod overflow;
mod overscroll;
mod page;
mod page_floats;
mod position;
mod regions;
mod rhythm;
mod round_display;
mod ruby;
mod scroll_anchoring;
mod scroll_animations;
mod scroll_snap;
mod scrollbars;
mod shapes;
mod size_adjust;
mod sizing;
mod speech;
mod tables;
mod text;
mod text_decor;
mod transitions;
mod ui;
mod view_transitions;
mod webkit;
mod will_change;
mod writing_modes;

pub use align::*;
pub use anchor_position::*;
pub use animations::*;
pub use backgrounds::*;
pub use cascade::*;
pub use color::*;
pub use color_adjust::*;
pub use compositing::*;
pub use contain::*;
pub use content::*;
pub use css2::*;
pub use display::*;
pub use exclusions::*;
pub use fill_stroke::*;
pub use filter_effects::*;
pub use flexbox::*;
pub use fonts::*;
pub use gcpm::*;
pub use grid::*;
pub use images::*;
pub use inline::*;
pub use line_grid::*;
pub use link_params::*;
pub use lists::*;
pub use logical::*;
pub use masking::*;
pub use motion::*;
pub use multicol::*;
pub use nav::*;
pub use non_standard::*;
pub use overflow::*;
pub use overscroll::*;
pub use page::*;
pub use page_floats::*;
pub use position::*;
pub use r#box::*;
pub use r#break::*;
pub use regions::*;
pub use rhythm::*;
pub use round_display::*;
pub use ruby::*;
pub use scroll_anchoring::*;
pub use scroll_animations::*;
pub use scroll_snap::*;
pub use scrollbars::*;
pub use shapes::*;
pub use size_adjust::*;
pub use sizing::*;
pub use speech::*;
pub use tables::*;
pub use text::*;
pub use text_decor::*;
pub use transitions::*;
pub use ui::*;
pub use view_transitions::*;
pub use webkit::*;
pub use will_change::*;
pub use writing_modes::*;

mod units;

// TODO!
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Parse<'a> for Todo {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Err(diagnostics::Unimplemented(parser.span()))?
	}
}

impl<'a> WriteCss<'a> for Todo {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		todo!("Cannot write out Todo values")
	}
}

// #[derive(Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
// pub enum ValueLike<'a> {
// 	Color(Box<'a, Spanned<Expr<'a, ColorValue<'a>>>>),
// 	Length(Box<'a, Spanned<MathExpr<'a, Length>>>),
// 	LengthPercentage(Box<'a, Spanned<MathExpr<'a, LengthPercentage>>>),
// 	FontFamily(Box<'a, Spanned<ExprList<'a, FontFamilyValue>>>),
// 	Unknown,
// }
//
// // https://drafts.csswg.org/css-values-4/#typedef-position
// #[derive(Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// pub struct PositionXY {
// 	pub x: HorizontalPosition,
// 	pub y: VerticalPosition,
// }
//
// #[derive(Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// pub enum HorizontalPosition {
// 	Center,
// 	Length(LengthPercentage),
// 	Left(Option<LengthPercentage>),
// 	Right(Option<LengthPercentage>),
// }
//
// #[derive(Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// pub enum VerticalPosition {
// 	Center,
// 	Length(LengthPercentage),
// 	Top(Option<LengthPercentage>),
// 	Bottom(Option<LengthPercentage>),
// }
//
// #[derive(Default, Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde())]
// pub struct NoNonGlobalValuesAllowed;
//
// #[derive(Atomizable, Default, Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde())]
// pub enum AutoOrNone {
// 	#[default]
// 	Auto,
// 	None,
// }
//
// // https://drafts.csswg.org/css-values-4/#ratio-value
// #[derive(Default, Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde())]
// pub struct Ratio(u8, u8);
//
// #[derive(Default, Debug, PartialEq, Hash)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde())]
// pub enum TimeOrAuto {
// 	#[default]
// 	Auto,
// 	Time(Time),
// }
//
// // https://drafts.csswg.org/css-values/#typedef-length-percentage
// #[derive(Debug, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// pub enum FrequencyPercentage {
// 	Frequency(Frequency),
// 	Percentage(f32),
// 	// TODO: Calc(Box<'a, Calc<FrequencyPercentage>>)
// }
//
// impl Hash for FrequencyPercentage {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		match self {
// 			Self::Frequency(f) => f.hash(state),
// 			Self::Percentage(f) => f.to_bits().hash(state),
// 		}
// 	}
// }
//
// // https://drafts.csswg.org/css-values/#typedef-length-percentage
// #[derive(Debug, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// pub enum AnglePercentage {
// 	Angle(Angle),
// 	Percentage(f32),
// 	// TODO: Calc(Box<'a, Calc<FrequencyPercentage>>)
// }
//
// impl Hash for AnglePercentage {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		match self {
// 			Self::Angle(a) => a.hash(state),
// 			Self::Percentage(f) => f.to_bits().hash(state),
// 		}
// 	}
// }
//
// // https://drafts.csswg.org/css-values/#typedef-length-percentage
// #[derive(Debug, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// pub enum TimePercentage {
// 	Time(Time),
// 	Percentage(f32),
// 	// TODO: Calc(Box<'a, Calc<FrequencyPercentage>>)
// }
//
// impl Hash for TimePercentage {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		match self {
// 			Self::Time(t) => t.hash(state),
// 			Self::Percentage(f) => f.to_bits().hash(state),
// 		}
// 	}
// }
//
// #[cfg(test)]
// mod tests {
//
// 	use super::*;
//	use crate::test_helpers::*;
//
// 	#[test]
// 	fn size_test() {
// 		assert_size!(FrequencyPercentage, 8);
// 		assert_size!(AnglePercentage, 8);
// 		assert_size!(TimePercentage, 8);
// 		assert_size!(PositionXY, 24);
// 		assert_size!(HorizontalPosition, 12);
// 		assert_size!(VerticalPosition, 12);
// 	}
// }
