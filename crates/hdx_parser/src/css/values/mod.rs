pub mod angle;
pub mod backgrounds;
pub mod r#box;
pub mod color;
pub mod content;
pub mod counter_styles;
pub mod display;
pub mod expr;
pub mod fonts;
pub mod inline;
pub mod length;
pub mod lists;
pub mod non_standard;
pub mod page_floats;
pub mod shorthand;
pub mod size_adjust;
pub mod sizing;
pub mod text;
pub mod text_decor;
pub mod ui;

use hdx_ast::css::{properties::Todo, values::*};

use crate::{diagnostics, Atomizable, Parse, Parser, Result, Spanned, Token};

macro_rules! parse_for_enums {
	{$( $prop: ident, )+} => {
		$(
			impl<'a> Parse<'a> for $prop {
				fn parse(parser: &mut Parser<'a>) -> Result<Spanned<$prop>> {
					let span = parser.span();
					match parser.cur() {
						Token::Ident(ident) => {
							if let Some(val) = $prop::from_atom(*ident) {
								Ok(val.spanned(span))
							} else {
								Err(diagnostics::UnexpectedIdent(*ident, span))?
							}
						}
						token => Err(diagnostics::ExpectedIdent(*token, span))?,
					}
				}
			}
		)+
	}
}

parse_for_enums! {
	AlignmentBaselineValue,
	AutoOrNone,
	BaselineSourceValue,
	BorderCollapseValue,
	BoxDecorationBreakValue,
	BoxSizingValue,
	BreakValue,
	BreakInsideValue,
	CaptionSideValue,
	ClearValue,
	DominantBaselineValue,
	EmptyCellsValue,
	FloatReferenceValue,
	InlineSizingValue,
	LineStyle,
	ListStylePositionValue,
	MarginBreakValue,
	MinIntrinsicSizingValue,
	OverflowKeyword,
	PositionValue,
	TableLayoutValue,
	TextAlignAllValue,
	TextAlignLastValue,
	TextAlignValue,
	TextDecorationSkipInkValue,
	TextDecorationStyleValue,
	TextWrapValue,
	VisibilityValue,
	WhiteSpaceCollapseValue,
}

// TODO:
impl<'a> Parse<'a> for Image<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.span()))?
	}
}

// TODO:
impl<'a> Parse<'a> for RatioOrAuto {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.span()))?
	}
}

// TODO:
impl<'a> Parse<'a> for TimeOrAuto {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.span()))?
	}
}

impl<'a> Parse<'a> for NoNonGlobalValuesAllowed {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		unexpected!(parser);
	}
}

// TODO:
impl<'a> Parse<'a> for Todo {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.span()))?
	}
}
