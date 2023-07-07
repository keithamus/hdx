#[cfg(feature = "serde")]
use serde::Serialize;

use super::{BorderRadiusValue, LengthPercentage, PositionXY};
use crate::{Box, Vec};

// https://drafts.csswg.org/css-shapes/#basic-shape-functions
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum BasicShape<'a> {
    Inset(ShapeInset<'a>),
    Xywh(ShapeXywh<'a>),
    Rect(ShapeRect<'a>),
    Circle(ShapeCircle<'a>),
    Ellipse(ShapeEllipse<'a>),
    Polygon(ShapePolygon<'a>),
    Path(ShapePath<'a>),
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-inset
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapeInset<'a> {
    rect: Box<'a, ShapeRect<'a>>,
    radius: Box<'a, BorderRadiusValue<'a>>,
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-xywh
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapeXywh<'a> {
    rect: Box<'a, ShapeRect<'a>>,
    radius: Box<'a, BorderRadiusValue<'a>>,
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-rect
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapeRect<'a> {
    top: Box<'a, LengthPercentage>,
    right: Box<'a, LengthPercentage>,
    bottom: Box<'a, LengthPercentage>,
    left: Box<'a, LengthPercentage>,
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-circle
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapeCircle<'a> {
    radius: Box<'a, ShapeRadius>,
    position: Box<'a, PositionXY>,
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-ellipse
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapeEllipse<'a> {
    rx: Box<'a, ShapeRadius>,
    ry: Box<'a, ShapeRadius>,
    position: Box<'a, PositionXY>,
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-polygon
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapePolygon<'a> {
    fill_rule: FillRule,
    points: Vec<'a, Point>,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Point {
    x: LengthPercentage,
    y: LengthPercentage,
}

// https://drafts.csswg.org/css-shapes/#funcdef-basic-shape-path
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ShapePath<'a> {
    fill_rule: FillRule,
    path: Box<'a, &'a str>,
}

// https://svgwg.org/svg2-draft/painting.html#FillRuleProperty
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum FillRule {
    NonZero,
    EvenOdd,
}

// https://drafts.csswg.org/css-shapes/#typedef-shape-radius
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum ShapeRadius {
    LengthPercentage(LengthPercentage),
    ClosestSide,
    FarthestSide,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_test() {
        use std::mem::size_of;
        assert_eq!(size_of::<BasicShape>(), 40);
        assert_eq!(size_of::<ShapeInset>(), 16);
        assert_eq!(size_of::<ShapeXywh>(), 16);
        assert_eq!(size_of::<ShapeRect>(), 32);
        assert_eq!(size_of::<ShapeCircle>(), 16);
        assert_eq!(size_of::<ShapeEllipse>(), 24);
        assert_eq!(size_of::<ShapePolygon>(), 40);
        assert_eq!(size_of::<Point>(), 16);
        assert_eq!(size_of::<FillRule>(), 1);
        assert_eq!(size_of::<ShapeRadius>(), 8);
    }
}
