pub(crate) use crate::units::*;
use css_parse::keyword_set;

// Re-expose stylevalues for shorthands
pub(crate) use super::{MarginTopStyleValue, PaddingTopStyleValue};

// https://drafts.csswg.org/css-box-4/#typedef-visual-box
// <visual-box> = content-box | padding-box | border-box
keyword_set!(VisualBox { ContentBox: "content-box", PaddingBox: "padding-box", BorderBox: "border-box" });

// https://drafts.csswg.org/css-box-4/#typedef-layout-box
// <layout-box> = <visual-box> | margin-box
keyword_set!(LayoutBox {
	ContentBox: "content-box",
	LayoutBox: "padding-box",
	BorderBox: "border-box",
	MarginBox: "margin-box",
});

// https://drafts.csswg.org/css-box-4/#typedef-paint-box
// <paint-box> = <visual-box> | fill-box | stroke-box
keyword_set!(PaintBox {
	ContentBox: "content-box",
	PaddingBox: "padding-box",
	BorderBox: "border-box",
	FillBox: "fill-box",
	StrokeBox: "stroke-box",
});

// https://drafts.csswg.org/css-box-4/#typedef-coord-box
// <coord-box> = <paint-box> | view-box
keyword_set!(CoordBox {
	ContentBox: "content-box",
	PaddingBox: "padding-box",
	BorderBox: "border-box",
	FillBox: "fill-box",
	StrokeBox: "stroke-box",
	ViewBox: "view-box",
});
