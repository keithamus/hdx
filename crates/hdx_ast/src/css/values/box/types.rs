pub(crate) use crate::css::units::*;
use hdx_parser::keyword_typedef;

// Re-expose stylevalues for shorthands
pub(crate) use super::{MarginTopStyleValue, PaddingTopStyleValue};

// https://drafts.csswg.org/css-box-4/#typedef-visual-box
// <visual-box> = content-box | padding-box | border-box
keyword_typedef!(VisualBox {
	ContentBox: atom!("content-box"),
	PaddingBox: atom!("padding-box"),
	BorderBox: atom!("border-box"),
});

// https://drafts.csswg.org/css-box-4/#typedef-layout-box
// <layout-box> = <visual-box> | margin-box
keyword_typedef!(LayoutBox {
	ContentBox: atom!("content-box"),
	LayoutBox: atom!("padding-box"),
	BorderBox: atom!("border-box"),
	MarginBox: atom!("margin-box"),
});

// https://drafts.csswg.org/css-box-4/#typedef-paint-box
// <paint-box> = <visual-box> | fill-box | stroke-box
keyword_typedef!(PaintBox {
	ContentBox: atom!("content-box"),
	PaddingBox: atom!("padding-box"),
	BorderBox: atom!("border-box"),
	FillBox: atom!("fill-box"),
	StrokeBox: atom!("stroke-box"),
});

// https://drafts.csswg.org/css-box-4/#typedef-coord-box
// <coord-box> = <paint-box> | view-box
keyword_typedef!(CoordBox {
	ContentBox: atom!("content-box"),
	PaddingBox: atom!("padding-box"),
	BorderBox: atom!("border-box"),
	FillBox: atom!("fill-box"),
	StrokeBox: atom!("stroke-box"),
	ViewBox: atom!("view-box"),
});
