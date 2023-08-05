mod backgrounds;
mod r#box;
mod color;
mod content;
mod counter_styles;
mod display;
mod expr;
mod fonts;
mod images;
mod inline;
mod length;
mod lists;
mod non_standard;
mod page_floats;
mod shorthand;
mod size_adjust;
mod sizing;
mod text;
mod text_decor;
mod ui;

use hdx_ast::css::values::*;
use hdx_atom::Atomizable;

use crate::{CssWriter, Result, WriteCss};

macro_rules! write_atomizable_values {
    {$( $prop: ident, )+} => {
        $(
            impl<'a> WriteCss<'a> for $prop {
                fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
                    sink.write_str(self.to_atom().as_ref())
                }
            }
        )+
    }
}

write_atomizable_values! {
	AbsoluteSize,
	AlignmentBaselineValue,
	AutoOrNone,
	BaselineSourceValue,
	BorderCollapseValue,
	BoxSizingValue,
	CaptionSideValue,
	ClearValue,
	DominantBaselineValue,
	EmptyCellsValue,
	FloatReferenceValue,
	InlineSizingValue,
	LineStyle,
	ListStylePositionValue,
	MinIntrinsicSizingValue,
	OverflowKeyword,
	PositionValue,
	RelativeSize,
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

impl<'a> WriteCss<'a> for TimeOrAuto {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for NoNonGlobalValuesAllowed {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		Ok(())
	}
}

impl<'a> WriteCss<'a> for ContentReplacement<'a> {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for InsetShorthand<'a> {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for RatioOrAuto {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}

impl<'a> WriteCss<'a> for Angle {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Zero => sink.write_char('0'),
			Self::Deg(val) => sink.write_str(&format!("{}deg", val)),
			Self::Grad(val) => sink.write_str(&format!("{}grad", val)),
			Self::Rad(val) => sink.write_str(&format!("{}rad", val)),
			Self::Turn(val) => sink.write_str(&format!("{}turn", val)),
		}
	}
}
