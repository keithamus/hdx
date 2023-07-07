use std::{fmt::Debug, hash::Hash};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
	atom,
	css::{component_values::ComponentValue, unknown::UnknownDeclaration, values::*},
	Atom, Atomizable, Box, Span, Spanned, Vec,
};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Custom<'a> {
	pub name: Atom,
	pub value: Box<'a, Vec<'a, Spanned<ComponentValue<'a>>>>,
	pub value_like: Spanned<ValueLike<'a>>,
	pub important: bool,
}

pub trait Declaration: Debug + PartialEq + Hash {
	type Value: Default + Debug + PartialEq + Hash;

	fn initial() -> Self::Value;
	fn name_as_atom() -> Atom;
	fn is_inherits() -> bool;
	fn is_standard() -> bool;
	fn is_shorthand() -> bool;
}

macro_rules! property_initial {
	($value: ty,) => {
		fn initial() -> $value {
			<$value>::default()
		}
	};
	($value: ty, $initial: expr) => {
		fn initial() -> $value {
			$initial
		}
	};
}

macro_rules! property_standard {
	($l:literal) => {
		fn is_standard() -> bool {
			$l
		}
	};
	() => {
		fn is_standard() -> bool {
			true
		}
	};
}

macro_rules! property_shorthand {
	($l:literal) => {
		fn is_shorthand() -> bool {
			$l
		}
	};
	() => {
		fn is_shorthand() -> bool {
			false
		}
	};
}

macro_rules! property_inherits {
	($l:literal) => {
		fn is_inherits() -> bool {
			$l
		}
	};
	() => {
		fn is_inherits() -> bool {
			false
		}
	};
}

macro_rules! property {
    (
        $atom: expr, $name: ident, $value: ty, $($standard:literal)?, $($shorthand:literal)?, $($inherits:literal)?, $($initial: expr)?
    ) => {
        #[derive(Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
        pub struct $name<'a> {
            pub important: bool,
            pub value: Box<'a, Spanned<$value>>,
        }

        impl<'a> Declaration for $name<'a> {
            type Value = $value;

            property_initial!($value, $($initial)?);
            property_inherits!($($inherits)?);
            property_standard!($($standard)?);
            property_shorthand!($($shorthand)?);

            fn name_as_atom() -> Atom {
                $atom
            }
        }
    };
}

macro_rules! properties {
    ( $(
        $atom: expr => $name: ident<$value: ty> $(standard=$standard:literal)? $(shorthand=$shorthand:literal)? $(inherits=$inherits:literal)? $(initial=$initial: expr)?,
    )+ ) => {
        #[derive(Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde(untagged))]
        pub enum Property<'a> {
            $(
                $name(Box<'a, Spanned<$name<'a>>>),
            )+
            Custom(Box<'a, Spanned<Custom<'a>>>),
            Unknown(Box<'a, Spanned<UnknownDeclaration<'a>>>),
        }
        #[derive(Debug, PartialEq, Hash)]
        pub enum PropertyId {
            $(
                $name,
            )+
        }
        impl Atomizable for PropertyId {
            fn from_atom(atom: Atom) -> Option<Self> {
                match atom {
                    $(
                        c if c == $atom => Some(Self::$name),
                    )+
                    _ => None
                }
            }
            fn to_atom(&self) -> Atom {
                match self {
                    $(
                        Self::$name => $atom,
                    )+
                }
            }
        }

        $(
            property!($atom, $name, $value, $($standard)?, $($inherits)?, $($shorthand)?, $($initial)?);
        )+

        #[cfg(test)]
        mod tests {

            use super::*;

            #[test]
            fn size_test() {
                use std::mem::size_of;
                $(
                    assert_eq!(size_of::<$name>(), 16);
                )+
            }
        }
    };
}

// TODO!
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Todo {
	#[default]
	Todo,
}

properties! {
	// https://drafts.csswg.org/css-align-3/#property-index
	atom!("align-content") => AlignContent<Expr<'a, Todo>>,
	atom!("align-items") => AlignItems<Expr<'a, Todo>>,
	atom!("align-self") => AlignSelf<Expr<'a, Todo>>,
	atom!("column-gap") => ColumnGap<MathExpr<'a, PositiveLengthPercentageOrNormal>>,
	atom!("gap") => Gap<DoubleShorthand<'a, MathExpr<'a, PositiveLengthPercentageOrNormal>>> shorthand=true,
	atom!("justify-content") => JustifyContent<Expr<'a, Todo>>,
	atom!("justify-items") => JustifyItems<Expr<'a, Todo>>,
	atom!("justify-self") => JustifySelf<Expr<'a, Todo>>,
	atom!("place-content") => PlaceContent<DoubleShorthand<'a, Expr<'a, Todo>>> shorthand=true,
	atom!("place-items") => PlaceItems<DoubleShorthand<'a, Expr<'a, Todo>>> shorthand=true,
	atom!("place-self") => PlaceSelf<DoubleShorthand<'a, Expr<'a, Todo>>> shorthand=true,
	atom!("row-gap") => RowGap<MathExpr<'a, PositiveLengthPercentageOrNormal>>,

	// https://drafts.csswg.org/css-anchor-position-1/#property-index
	atom!("anchor-default") => AnchorDefault<Expr<'a, Todo>>,
	atom!("anchor-position") => AnchorPosition<Expr<'a, Todo>>,
	atom!("position-fallback") => PositionFallback<Expr<'a, Todo>>,
	atom!("position-fallback-bounds") => PositionFallbackBounds<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-animations-1/#property-index
	atom!("animation") => Animation<Expr<'a, Todo>> shorthand=true,
	atom!("animation-delay") => AnimationDelay<Expr<'a, Todo>>,
	atom!("animation-direction") => AnimationDirection<Expr<'a, Todo>>,
	// ! animation-duration redefined in css-animations-2
	atom!("animation-fill-mode") => AnimationFillMode<Expr<'a, Todo>>,
	atom!("animation-iteration-count") => AnimationIterationCount<Expr<'a, Todo>>,
	atom!("animation-name") => AnimationName<Expr<'a, Todo>>,
	atom!("animation-play-state") => AnimationPlayState<Expr<'a, Todo>>,
	atom!("animation-timing-function") => AnimationTimingFunction<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-animations-2/#property-index
	atom!("animation-duration") => AnimationDuration<Expr<'a, TimeOrAuto>>,
	atom!("animation-composition") => AnimationComposition<Expr<'a, Todo>>,
	atom!("animation-timeline") => AnimationTimeline<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-backgrounds-3/#property-index
	atom!("background") => Background<ColorValue<'a>> shorthand=true,
	atom!("background-attachment") => BackgroundAttachment<Expr<'a, Todo>>,
	// ! background-clip redefined in css-backgrounds-4
	atom!("background-color") => BackgroundColor<Expr<'a, ColorValue<'a>>>,
	atom!("background-image") => BackgroundImage<Expr<'a, Todo>>,
	atom!("background-origin") => BackgroundOrigin<Expr<'a, Todo>>,
	// ! background-position redefined in css-backgrounds-4
	atom!("background-repeat") => BackgroundRepeat<Expr<'a, Todo>>,
	atom!("background-size") => BackgroundSize<Expr<'a, Todo>>,
	atom!("border") => Border<Expr<'a, Todo>> shorthand=true,
	atom!("border-bottom") => BorderBottom<Expr<'a, Todo>> shorthand=true,
	atom!("border-bottom-color") => BorderBottomColor<Expr<'a, ColorValue<'a>>> initial=Expr::Literal(Spanned::dummy(ColorValue::CurrentColor)),
	atom!("border-bottom-left-radius") => BorderBottomLeftRadius<Expr<'a, Todo>>,
	atom!("border-bottom-right-radius") => BorderBottomRightRadius<Expr<'a, Todo>>,
	atom!("border-bottom-style") => BorderBottomStyle<Expr<'a, LineStyle>>,
	atom!("border-bottom-width") => BorderBottomWidth<MathExpr<'a, LineWidth>>,
	atom!("border-color") => BorderColor<BoxShorthand<'a, Expr<'a, ColorValue<'a>>>> shorthand=true,
	atom!("border-image") => BorderImage<Expr<'a, Todo>> shorthand=true,
	atom!("border-image-outset") => BorderImageOutset<Expr<'a, Todo>>,
	atom!("border-image-repeat") => BorderImageRepeat<Expr<'a, Todo>>,
	atom!("border-image-slice") => BorderImageSlice<Expr<'a, Todo>>,
	atom!("border-image-source") => BorderImageSource<Expr<'a, Todo>>,
	atom!("border-image-width") => BorderImageWidth<Expr<'a, Todo>>,
	atom!("border-left") => BorderLeft<Expr<'a, Todo>> shorthand=true,
	atom!("border-left-color") => BorderLeftColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-left-style") => BorderLeftStyle<Expr<'a, LineStyle>>,
	atom!("border-left-width") => BorderLeftWidth<MathExpr<'a, LineWidth>>,
	atom!("border-radius") => BorderRadius<Expr<'a, Todo>> shorthand=true,
	atom!("border-right") => BorderRight<Expr<'a, Todo>> shorthand=true,
	atom!("border-right-color") => BorderRightColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-right-style") => BorderRightStyle<Expr<'a, LineStyle>>,
	atom!("border-right-width") => BorderRightWidth<MathExpr<'a, LineWidth>>,
	atom!("border-style") => BorderStyle<Expr<'a, Todo>> shorthand=true,
	atom!("border-top") => BorderTop<Expr<'a, Todo>> shorthand=true,
	atom!("border-top-color") => BorderTopColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-top-left-radius") => BorderTopLeftRadius<Expr<'a, Todo>>,
	atom!("border-top-right-radius") => BorderTopRightRadius<Expr<'a, Todo>>,
	atom!("border-top-style") => BorderTopStyle<Expr<'a, LineStyle>>,
	atom!("border-top-width") => BorderTopWidth<MathExpr<'a, LineWidth>>,
	atom!("border-width") => BorderWidth<Expr<'a, Todo>> shorthand=true,
	atom!("box-shadow") => BoxShadow<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-backgrounds-4/#property-index
	atom!("background-clip") => BackgroundClip<Expr<'a, Todo>>,
	atom!("background-position") => BackgroundPosition<Expr<'a, Todo>>,
	atom!("background-position-block") => BackgroundPositionBlock<Expr<'a, Todo>>,
	atom!("background-position-inline") => BackgroundPositionInline<Expr<'a, Todo>>,
	atom!("background-position-x") => BackgroundPositionX<Expr<'a, Todo>>,
	atom!("background-position-y") => BackgroundPositionY<Expr<'a, Todo>>,


	// https://drafts.csswg.org/css-box-3/#property-index
	// ! margin redefined in css-box-4
	// ! margin-bottom redefined in css-box-4
	// ! margin-left redefined in css-box-4
	// ! margin-right redefined in css-box-4
	// ! margin-top redefined in css-box-4
	// ! padding redefined in css-box-4
	// ! padding-bottom redefined in css-box-4
	// ! padding-left redefined in css-box-4
	// ! padding-right redefined in css-box-4
	// ! padding-top redefined in css-box-4

	// https://drafts.csswg.org/css-box-4/#property-index
	atom!("margin") => Margin<BoxShorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>> shorthand=true,
	atom!("margin-bottom") => MarginBottom<MathExpr<'a, LengthPercentageOrAuto>>,
	atom!("margin-left") => MarginLeft<MathExpr<'a, LengthPercentageOrAuto>>,
	atom!("margin-right") => MarginRight<MathExpr<'a, LengthPercentageOrAuto>>,
	atom!("margin-top") => MarginTop<MathExpr<'a, LengthPercentageOrAuto>>,
	atom!("margin-trim") => MarginTrim<MathExpr<'a, MarginTrimValue>>,
	atom!("padding") => Padding<BoxShorthand<'a, MathExpr<'a, PositiveLengthPercentage>>> shorthand=true,
	atom!("padding-bottom") => PaddingBottom<MathExpr<'a, PositiveLengthPercentage>>,
	atom!("padding-left") => PaddingLeft<MathExpr<'a, PositiveLengthPercentage>>,
	atom!("padding-right") => PaddingRight<MathExpr<'a, PositiveLengthPercentage>>,
	atom!("padding-top") => PaddingTop<MathExpr<'a, PositiveLengthPercentage>>,

	// https://drafts.csswg.org/css-break-3/#property-index
	// ! box-decoration-break redefined in css-break-4
	// ! break-after redefined in css-break-4
	// ! break-before redefined in css-break-4
	// ! break-inside redefined in css-break-4
	// ! orphans redefined in css-break-4
	// ! widows redefined in css-break-4

	// https://drafts.csswg.org/css-break-4/#property-index
	atom!("box-decoration-break") => BoxDecorationBreak<Expr<'a, Todo>>,
	atom!("break-after") => BreakAfter<Expr<'a, Todo>>,
	atom!("break-before") => BreakBefore<Expr<'a, Todo>>,
	atom!("break-inside") => BreakInside<Expr<'a, Todo>>,
	atom!("margin-break") => BreakMargin<Expr<'a, Todo>>,
	// For compatibility with CSS Level 2, UAs that conform to [CSS2] must alias the
	// page-break-before, page-break-after, and page-break-inside properties to break-before,
	// break-after, and break-inside by treating the page-break-* properties as legacy
	// shorthands for the break-* properties with the following value mappings:
	atom!("page-break-after") => PageBreakAfter<Expr<'a, Todo>>,
	atom!("page-break-before") => PageBreakBefore<Expr<'a, Todo>>,
	atom!("page-break-inside") => PageBreakInside<Expr<'a, Todo>>,
	atom!("orphans") => Orphans<Expr<'a, Todo>> inherits=true,
	atom!("widows") => Widows<Expr<'a, Todo>> inherits=true,

	// https://drafts.csswg.org/css-cascade-3/#property-index
	// ! all redefined in css-cascade-4

	// https://drafts.csswg.org/css-cascade-4/#property-index
	// ! all redefined in css-cascade-5

	// https://drafts.csswg.org/css-cascade-5/#property-index
	atom!("all") => All<NoNonGlobalValuesAllowed>,

	// https://drafts.csswg.org/css-cascade-6/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-3/#property-index
	// ! color redefined in css-color-4
	// ! opacity redefined in css-color-4

	// https://drafts.csswg.org/css-color-4/#property-index
	atom!("color") => Color<ColorValue<'a>> inherits=true,
	atom!("opacity") => Opacity<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-color-5/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-6/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-adjust-1/#property-index
	atom!("color-adjust") => ColorAdjust<Expr<'a, Todo>> shorthand=true,
	atom!("color-scheme") => ColorScheme<Expr<'a, Todo>> inherits=true,
	atom!("forced-color-adjust") => ForcedColorAdjust<Expr<'a, Todo>> inherits=true,
	atom!("print-color-adjust") => PrintColorAdjust<Expr<'a, Todo>> inherits=true,

	// https://drafts.csswg.org/css-color-hdr/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-conditional-3/#property-index
	// https://drafts.csswg.org/css-conditional-4/#property-index
	// https://drafts.csswg.org/css-conditional-5/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-conditional-values-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-contain-1/#property-index
	// ! contain redefined in css-contain-2
	// ! content-visibility redefined in css-contain-3

	// https://drafts.csswg.org/css-contain-2/#property-index
	atom!("contain") => Contain<Expr<'a, Todo>>,
	// ! content-visibility redefined in css-contain-3


	// https://drafts.csswg.org/css-contain-3/#property-index
	atom!("container") => Container<Expr<'a, Todo>> shorthand=true,
	atom!("container-name") => ContainerName<Expr<'a, Todo>>,
	atom!("container-type") => ContainerType<Expr<'a, Todo>>,
	atom!("content-visibility") => ContentVisibility<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-content-3/#property-index
	atom!("bookmark-label") => BookmarkLabel<Expr<'a, Todo>>,
	atom!("bookmark-level") => BookmarkLevel<Expr<'a, Todo>>,
	atom!("bookmark-state") => BookmarkState<Expr<'a, Todo>>,
	atom!("content") => Content<ContentsValue<'a>>,
	atom!("quotes") => Quotes<Expr<'a, Todo>> inherits=true,
	atom!("string-set") => StringSet<Expr<'a, Todo>> inherits=true,

	// https://drafts.csswg.org/css-counter-styles-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-display-3/#property-index
	// ! display redefined in css-display-4
	// ! order redefined in css-display-4
	// ! visibility redefined in css-display-4

	// https://drafts.csswg.org/css-display-4/#property-index
	atom!("display") => Display<DisplayValue>,
	atom!("layout-order") => LayoutOrder<Expr<'a, Todo>>,
	atom!("order") => Order<Expr<'a, Todo>>,
	atom!("reading-order") => ReadingOrder<Expr<'a, Todo>>,
	atom!("visibility") => Visibility<VisibilityValue> inherits=true,

	// https://drafts.csswg.org/css-easing-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-easing-2/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-egg-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-env-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-exclusions-1/#property-index
	atom!("wrap-flow") => WrapFlow<Expr<'a, Todo>>,
	atom!("wrap-through") => WrapThrough<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-extensions-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-flexbox-1/#property-index
	// ! align-content redefined in css-align-3
	// ! align-items redefined in css-align-3
	// ! align-self redefined in css-align-3
	atom!("flex") => Flex<Expr<'a, Todo>> shorthand=true,
	atom!("flex-basis") => FlexBasis<Expr<'a, Todo>>,
	atom!("flex-direction") => FlexDirection<Expr<'a, Todo>>,
	atom!("flex-flow") => FlexFlow<Expr<'a, Todo>> shorthand=true,
	atom!("flex-grow") => FlexGrow<Expr<'a, Todo>>,
	atom!("flex-shrink") => FlexShrink<Expr<'a, Todo>>,
	atom!("flex-wrap") => FlexWrap<Expr<'a, Todo>>,
	// ! justify-content redefined in css-align-3

	// https://drafts.csswg.org/css-font-loading-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-fonts-3/#property-index
	// ! font redefined in css-fonts-4
	// ! font-family redefined in css-fonts-4
	// ! font-feature-settings redefined in css-fonts-4
	// ! font-kerning redefined in css-fonts-4
	// ! font-size redefined in css-fonts-4
	// ! font-size-adjust redefined in css-fonts-4
	// ! font-stretch redefined in css-fonts-4
	// ! font-style redefined in css-fonts-4
	// ! font-synthesis redefined in css-fonts-4
	// ! font-variant redefined in css-fonts-4
	// ! font-variant-caps redefined in css-fonts-4
	// ! font-variant-east-asian redefined in css-fonts-4
	// ! font-variant-ligatures redefined in css-fonts-4
	// ! font-variant-numeric redefined in css-fonts-4
	// ! font-variant-position redefined in css-fonts-4
	// ! font-weight redefined in css-fonts-4

	// https://drafts.csswg.org/css-fonts-4/#property-index
	atom!("font") => Font<Expr<'a, Todo>> shorthand=true inherits=true,
	atom!("font-family") => FontFamily<ExprList<'a, FontFamilyValue>> inherits=true,
	atom!("font-feature-settings") => FontFeatureSettings<Expr<'a, Todo>> inherits=true,
	atom!("font-kerning") => FontKerning<Expr<'a, Todo>> inherits=true,
	atom!("font-language-override") => FontLanguageOverride<Expr<'a, Todo>> inherits=true,
	atom!("font-optical-sizing") => FontOpticalSizing<Expr<'a, Todo>> inherits=true,
	atom!("font-palette") => FontPalette<Expr<'a, Todo>> inherits=true,
	atom!("font-size") => FontSize<MathExpr<'a, FontSizeValue>> inherits=true,
	// ! font-size-adjust redefined in css-fonts-5
	atom!("font-stretch") => FontStretch<Expr<'a, Todo>> inherits=true,
	atom!("font-style") => FontStyle<Expr<'a, FontStyleValue<'a>>> inherits=true,
	atom!("font-synthesis") => FontSynthesis<Expr<'a, Todo>> inherits=true,
	atom!("font-synthesis-small-caps") => FontSynthesisSmallCaps<Expr<'a, Todo>> inherits=true,
	atom!("font-synthesis-style") => FontSynthesisStyle<Expr<'a, Todo>> inherits=true,
	atom!("font-synthesis-weight") => FontSynthesisWeight<Expr<'a, Todo>> inherits=true,
	atom!("font-variant") => FontVariant<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-alternates") => FontVariantAlternates<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-caps") => FontVariantCaps<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-east-asian") => FontVariantEastAsian<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-emoji") => FontVariantEmoji<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-ligatures") => FontVariantLigatures<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-numeric") => FontVariantNumeric<Expr<'a, Todo>> inherits=true,
	atom!("font-variant-position") => FontVariantPosition<Expr<'a, Todo>> inherits=true,
	atom!("font-variation-settings") => FontVariationSettings<Expr<'a, Todo>> inherits=true,
	atom!("font-weight") => FontWeight<MathExpr<'a, FontWeightValue>> inherits=true,

	// https://drafts.csswg.org/css-fonts-5/#property-index
	atom!("font-size-adjust") => FontSizeAdjust<Expr<'a, Todo>> inherits=true,

	// https://drafts.csswg.org/css-forms-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-gcpm-3/#property-index
	atom!("footnote-display") => FootnoteDisplay<Expr<'a, Todo>>,
	atom!("footnote-policy") => FootnotePolicy<Expr<'a, Todo>>,
	atom!("running") => Running<Expr<'a, Todo>>,
	// ! string-set redefined in css-content-3

	// https://drafts.csswg.org/css-gcpm-4/#property-index
	atom!("copy-into") => CopyInto<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-grid-1/#property-index
	// ! grid redefined in css-grid-2
	// ! grid-area redefined in css-grid-2
	// ! grid-auto-columns redefined in css-grid-2
	// ! grid-auto-flow redefined in css-grid-2
	// ! grid-auto-rows redefined in css-grid-2
	// ! grid-column redefined in css-grid-2
	// ! grid-column-end redefined in css-grid-2
	// ! grid-column-start redefined in css-grid-2
	// ! grid-row redefined in css-grid-2
	// ! grid-row-end redefined in css-grid-2
	// ! grid-row-start redefined in css-grid-2
	// ! grid-template redefined in css-grid-2
	// ! grid-template-areas redefined in css-grid-2
	// ! grid-template-columns redefined in css-grid-2
	// ! grid-template-rows redefined in css-grid-2

	// https://drafts.csswg.org/css-grid-2/#property-index
	atom!("grid") => Grid<Expr<'a, Todo>> shorthand=true,
	atom!("grid-area") => GridArea<Expr<'a, Todo>> shorthand=true,
	atom!("grid-auto-columns") => GridAutoColumns<Expr<'a, Todo>>,
	atom!("grid-auto-flow") => GridAutoFlow<Expr<'a, Todo>>,
	atom!("grid-auto-rows") => GridAutoRows<Expr<'a, Todo>>,
	atom!("grid-column") => GridColumn<Expr<'a, Todo>> shorthand=true,
	atom!("grid-column-end") => GridColumnEnd<Expr<'a, Todo>>,
	atom!("grid-column-start") => GridColumnStart<Expr<'a, Todo>>,
	atom!("grid-row") => GridRow<Expr<'a, Todo>> shorthand=true,
	atom!("grid-row-end") => GridRowEnd<Expr<'a, Todo>>,
	atom!("grid-row-start") => GridRowStart<Expr<'a, Todo>>,
	atom!("grid-template") => GridTemplate<Expr<'a, Todo>> shorthand=true,
	atom!("grid-template-areas") => GridTemplateAreas<Expr<'a, Todo>>,
	atom!("grid-template-columns") => GridTemplateColumns<Expr<'a, Todo>>,
	atom!("grid-template-rows") => GridTemplateRows<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-grid-3/#property-index
	atom!("align-tracks") => AlignTracks<Expr<'a, Todo>>,
	atom!("justify-tracks") => JustifyTracks<Expr<'a, Todo>>,
	atom!("masonry-auto-flow") => MasonryAutoFlow<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-images-3/#property-index
	atom!("image-orientation") => ImageOrientation<Expr<'a, Todo>>,
	atom!("image-rendering") => ImageRendering<Expr<'a, Todo>>,
	// ! object-fit redefined in css-images-4
	atom!("object-position") => ObjectPosition<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-images-4/#property-index
	atom!("image-resolution") => ImageResolution<Expr<'a, Todo>>,
	atom!("object-fit") => ObjectFit<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-images-5/#property-index
	atom!("object-view-box") => ObjectViewBox<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-inline-3/#property-index
	atom!("alignment-baseline") => AlignmentBaseline<Expr<'a, AlignmentBaselineValue>>,
	atom!("baseline-source") => BaselineSource<Expr<'a, BaselineSourceValue>>,
	atom!("baseline-shift") => BaselineShift<Expr<'a, BaselineShiftValue>>,
	atom!("dominant-baseline") => DominantBaseline<DominantBaselineValue> inherits=true,
	atom!("initial-letter") => InitialLetter<Expr<'a, Todo>>,
	atom!("initial-letter-align") => InitialLetterAlign<Expr<'a, Todo>>,
	atom!("initial-letter-wrap") => InitialLetterWrap<Expr<'a, Todo>> inherits=true,
	atom!("inline-sizing") => InlineSizing<InlineSizingValue> inherits=true,
	atom!("line-height") => LineHeight<MathExpr<'a, LineHeightValue>> inherits=true,
	atom!("text-box-edge") => TextBoxEdge<Expr<'a, Todo>> inherits=true,
	atom!("text-box-trim") => TextBoxTrim<Expr<'a, Todo>>,
	atom!("vertical-align") => VerticalAlign<VerticalAlignShorthand<'a>> shorthand=true,

	// https://drafts.csswg.org/css-line-grid-1/#property-index
	atom!("box-snap") => BoxSnap<Expr<'a, Todo>>,
	atom!("line-grid") => LineGrid<Expr<'a, Todo>>,
	atom!("line-snap") => LineSnap<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-link-params-1/#property-index
	atom!("link-parameters") => LinkParameters<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-lists-3/#property-index
	atom!("counter-increment") => CounterIncrement<Expr<'a, Todo>>,
	atom!("counter-reset") => CounterReset<Expr<'a, Todo>>,
	atom!("counter-set") => CounterSet<Expr<'a, Todo>>,
	atom!("list-style") => ListStyle<Expr<'a, Todo>> shorthand=true,
	atom!("list-style-image") => ListStyleImage<Expr<'a, Todo>> inherits=true,
	atom!("list-style-position") => ListStylePosition<Expr<'a, Todo>> inherits=true,
	atom!("list-style-type") => ListStyleType<Expr<'a, Todo>> inherits=true,
	atom!("marker-side") => MarkerSide<Expr<'a, Todo>> inherits=true,

	// https://drafts.csswg.org/css-logical-1/#property-index
	atom!("block-size") => BlockSize<Sizing>,
	atom!("border-block") => BorderBlock<Expr<'a, Todo>> shorthand=true,
	atom!("border-block-color") => BorderBlockColor<Expr<'a, Todo>> shorthand=true,
	atom!("border-block-end") => BorderBlockEnd<Expr<'a, Todo>> shorthand=true,
	atom!("border-block-end-color") => BorderBlockEndColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-block-end-style") => BorderBlockEndStyle<LineStyle>,
	atom!("border-block-end-width") => BorderBlockEndWidth<LineWidth>,
	atom!("border-block-start") => BorderBlockStart<Expr<'a, Todo>> shorthand=true,
	atom!("border-block-start-color") => BorderBlockStartColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-block-start-style") => BorderBlockStartStyle<LineStyle>,
	atom!("border-block-start-width") => BorderBlockStartWidth<LineWidth>,
	atom!("border-block-style") => BorderBlockStyle<Expr<'a, Todo>> shorthand=true,
	atom!("border-block-width") => BorderBlockWidth<Expr<'a, Todo>> shorthand=true,
	atom!("border-end-end-radius") => BorderEndEndRadius<LengthPercentage>,
	atom!("border-end-start-radius") => BorderEndStartRadius<LengthPercentage>,
	atom!("border-inline") => BorderInline<Expr<'a, Todo>> shorthand=true,
	atom!("border-inline-color") => BorderInlineColor<Expr<'a, Todo>> shorthand=true,
	atom!("border-inline-end") => BorderInlineEnd<Expr<'a, Todo>> shorthand=true,
	atom!("border-inline-end-color") => BorderInlineEndColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-inline-end-style") => BorderInlineEndStyle<LineStyle>,
	atom!("border-inline-end-width") => BorderInlineEndWidth<LineWidth>,
	atom!("border-inline-start") => BorderInlineStart<Expr<'a, Todo>> shorthand=true,
	atom!("border-inline-start-color") => BorderInlineStartColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("border-inline-start-style") => BorderInlineStartStyle<LineStyle>,
	atom!("border-inline-start-width") => BorderInlineStartWidth<LineWidth>,
	atom!("border-inline-style") => BorderInlineStyle<Expr<'a, Todo>> shorthand=true,
	atom!("border-inline-width") => BorderInlineWidth<Expr<'a, Todo>> shorthand=true,
	atom!("border-start-end-radius") => BorderStartEndRadius<LengthPercentage>,
	atom!("border-start-start-radius") => BorderStartStartRadius<LengthPercentage>,
	atom!("inline-size") => InlineSize<Sizing> shorthand=true,
	// ! inset redefined in css-position-3
	// ! inset-block redefined in css-position-3
	// ! inset-inline redefined in css-position-3
	// ! inset-block-end redefined in css-position-3
	// ! inset-inline redefined in css-position-3
	// ! inset-inline-end redefined in css-position-3
	// ! inset-inline-start redefined in css-position-3
	atom!("margin-block") => MarginBlock<Expr<'a, Todo>> shorthand=true,
	atom!("margin-block-end") => MarginBlockEnd<LengthPercentageOrAuto>,
	atom!("margin-block-start") => MarginBlockStart<LengthPercentageOrAuto>,
	atom!("margin-inline") => MarginInline<Expr<'a, Todo>> shorthand=true,
	atom!("margin-inline-end") => MarginInlineEnd<LengthPercentageOrAuto>,
	atom!("margin-inline-start") => MarginInlineStart<LengthPercentageOrAuto>,
	atom!("max-block-size") => MaxBlockSize<Sizing>,
	atom!("max-inline-size") => MaxInlineSize<Sizing>,
	atom!("min-block-size") => MinBlockSize<Sizing>,
	atom!("min-inline-size") => MinInlineSize<Sizing>,
	atom!("padding-block") => PaddingBlock<Expr<'a, Todo>> shorthand=true,
	atom!("padding-block-end") => PaddingBlockEnd<LengthPercentage>,
	atom!("padding-block-start") => PaddingBlockStart<LengthPercentage>,
	atom!("padding-inline") => PaddingInline<Expr<'a, Todo>> shorthand=true,
	atom!("padding-inline-end") => PaddingInlineEnd<LengthPercentage>,
	atom!("padding-inline-start") => PaddingInlineStart<LengthPercentage>,

	// https://drafts.csswg.org/css-mobile/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-multicol-1/#property-index
	atom!("column-count") => ColumnCount<Expr<'a, Todo>>,
	atom!("column-fill") => ColumnFill<Expr<'a, Todo>>,
	atom!("column-rule") => ColumnRule<Expr<'a, Todo>> shorthand=true,
	atom!("column-rule-color") => ColumnRuleColor<ColorValue<'a>> initial=ColorValue::CurrentColor,
	atom!("column-rule-style") => ColumnRuleStyle<LineStyle>,
	atom!("column-rule-width") => ColumnRuleWidth<LineWidth>,
	// ! column-span redined in css-multicol-2
	atom!("column-width") => ColumnWidth<Expr<'a, Todo>>,
	atom!("columns") => Columns<Expr<'a, Todo>> shorthand=true,

	// https://drafts.csswg.org/css-multicol-2/#property-index
	atom!("column-span") => ColumnSpan<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-namespaces-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-nav-1/#property-index
	atom!("spatial-navigation-action") => SpatialNavigationAction<Expr<'a, Todo>>,
	atom!("spatial-navigation-contain") => SpatialNavigationContain<Expr<'a, Todo>>,
	atom!("spatial-navigation-function") => SpatialNavigationFunction<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-nesting-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-overflow-3/#property-index
	atom!("overflow") => Overflow<XYShorthand<'a, Expr<'a, OverflowKeyword>>> shorthand=true,
	atom!("overflow-block") => OverflowBlock<Expr<'a, OverflowKeyword>>,
	// ! overflow-clip-margin redined in css-overflow-4
	atom!("overflow-inline") => OverflowInline<Expr<'a, OverflowKeyword>>,
	atom!("overflow-x") => OverflowX<Expr<'a, OverflowKeyword>>,
	atom!("overflow-y") => OverflowY<Expr<'a, OverflowKeyword>>,
	atom!("scroll-behavior") => ScrollBehavior<Expr<'a, Todo>>,
	atom!("scrollbar-gutter") => ScrollbarGutter<Expr<'a, Todo>>,
	// ! text-overflow redined in css-overflow-4

	// https://drafts.csswg.org/css-overflow-4/#property-index
	// (Yes this is really in the spec as -webkit-line-clamp)
	atom!("-webkit-line-clamp") => WebkitLineClamp<Expr<'a, Todo>> shorthand=true,
	atom!("block-ellipsis") => BlockEllipsis<Expr<'a, Todo>> inherits=true,
	atom!("continue") => Continue<Expr<'a, Todo>>,
	atom!("line-clamp") => LineClamp<Expr<'a, Todo>> shorthand=true,
	atom!("max-lines") => MaxLines<Expr<'a, Todo>>,
	atom!("overflow-clip-margin") => OverflowClipMargin<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-block") => OverflowClipMarginBlock<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-block-end") => OverflowClipMarginBlockEnd<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-block-start") => OverflowClipMarginBlockStart<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-bottom") => OverflowClipMarginBottom<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-inline") => OverflowClipMarginInline<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-inline-end") => OverflowClipMarginInlineEnd<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-inline-start") => OverflowClipMarginInlineStart<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-left") => OverflowClipMarginLeft<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-right") => OverflowClipMarginRight<Expr<'a, Todo>>,
	atom!("overflow-clip-margin-top") => OverflowClipMarginTop<Expr<'a, Todo>>,
	atom!("text-overflow") => TextOverflow<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-overscroll-1/#property-index
	atom!("overscroll-behavior") => OverscrollBehavior<Expr<'a, Todo>> shorthand=true,
	atom!("overscroll-behavior-block") => OverscrollBehaviorBlock<Expr<'a, Todo>>,
	atom!("overscroll-behavior-inline") => OverscrollBehaviorInline<Expr<'a, Todo>>,
	atom!("overscroll-behavior-x") => OverscrollBehaviorX<Expr<'a, Todo>>,
	atom!("overscroll-behavior-y") => OverscrollBehaviorY<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-page-3/#property-index
	atom!("page") => Page<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-page-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-page-floats-3/#property-index
	atom!("clear") => Clear<ClearValue>,
	atom!("float") => Float<FloatValue>,
	atom!("float-defer") => FloatDefer<FloatDeferValue>,
	atom!("float-offset") => FloatOffset<LengthPercentage>,
	atom!("float-reference") => FloatReference<FloatReferenceValue>,

	// https://drafts.csswg.org/css-page-template-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-position-3/#property-index
	atom!("bottom") => Bottom<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("inset") => Inset<BoxShorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>> shorthand=true,
	atom!("inset-block") => InsetBlock<DoubleShorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>> shorthand=true,
	atom!("inset-block-end") => InsetBlockEnd<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("inset-block-start") => InsetBlockStart<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("inset-inline") => InsetInline<DoubleShorthand<'a, MathExpr<'a, LengthPercentageOrAuto>>> shorthand=true,
	atom!("inset-inline-end") => InsetInlineEnd<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("inset-inline-start") => InsetInlineStart<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("left") => Left<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("position") => Position<Expr<'a, PositionValue>>,
	atom!("right") => Right<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),
	atom!("top") => Top<MathExpr<'a, LengthPercentageOrAuto>> initial=MathExpr::Literal(Spanned::dummy(LengthPercentageOrAuto::Auto)),

	// https://drafts.csswg.org/css-preslev-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-print/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-pseudo-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-regions-1/#property-index
	atom!("flow-from") => FlowFrom<Expr<'a, Todo>>,
	atom!("flow-into") => FlowInto<Expr<'a, Todo>>,
	atom!("region-fragment") => RegionFragment<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-rhythm-1/#property-index
	atom!("block-step") => BlockStep<Expr<'a, Todo>> shorthand=true,
	atom!("block-step-align") => BlockStepAlign<Expr<'a, Todo>>,
	atom!("block-step-insert") => BlockStepInsert<Expr<'a, Todo>>,
	atom!("block-step-round") => BlockStepRound<Expr<'a, Todo>>,
	atom!("block-step-size") => BlockStepSize<Expr<'a, Todo>>,
	atom!("line-height-step") => LineHeightStep<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-round-display-1/#property-index
	atom!("border-boundary") => BorderBoundary<Expr<'a, Todo>>,
	atom!("shape-inside") => ShapeInside<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-ruby-1/#property-index
	atom!("ruby-align") => RubyAlign<Expr<'a, Todo>>,
	atom!("ruby-merge") => RubyMerge<Expr<'a, Todo>>,
	atom!("ruby-overhang") => RubyOverhang<Expr<'a, Todo>>,
	atom!("ruby-position") => RubyPosition<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-scoping-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-scroll-anchoring-1/#property-index
	atom!("overflow-anchor") => OverflowAnchor<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-scroll-snap-1/#property-index
	atom!("scroll-margin") => ScrollMargin<Expr<'a, Todo>> shorthand=true,
	atom!("scroll-margin-block") => ScrollMarginBlock<Expr<'a, Todo>> shorthand=true,
	atom!("scroll-margin-block-end") => ScrollMarginBlockEnd<Length>,
	atom!("scroll-margin-block-start") => ScrollMarginBlockStart<Length>,
	atom!("scroll-margin-bottom") => ScrollMarginBottom<Length>,
	atom!("scroll-margin-inline") => ScrollMarginInline<Expr<'a, Todo>> shorthand=true,
	atom!("scroll-margin-inline-end") => ScrollMarginInlineEnd<Length>,
	atom!("scroll-margin-inline-start") => ScrollMarginInlineStart<Length>,
	atom!("scroll-margin-left") => ScrollMarginLeft<Length>,
	atom!("scroll-margin-right") => ScrollMarginRight<Length>,
	atom!("scroll-margin-top") => ScrollMarginTop<Length>,
	atom!("scroll-padding") => ScrollPadding<Expr<'a, Todo>> shorthand=true,
	atom!("scroll-padding-block") => ScrollPaddingBlock<Expr<'a, Todo>> shorthand=true,
	atom!("scroll-padding-block-end") => ScrollPaddingBlockEnd<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-block-start") => ScrollPaddingBlockStart<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-bottom") => ScrollPaddingBottom<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-inline") => ScrollPaddingInline<Expr<'a, Todo>> shorthand=true,
	atom!("scroll-padding-inline-end") => ScrollPaddingInlineEnd<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-inline-start") => ScrollPaddingInlineStart<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-left") => ScrollPaddingLeft<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-right") => ScrollPaddingRight<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-padding-top") => ScrollPaddingTop<LengthPercentageOrAuto> initial=LengthPercentageOrAuto::Auto,
	atom!("scroll-snap-align") => ScrollSnapAlign<Expr<'a, Todo>>,
	atom!("scroll-snap-stop") => ScrollSnapStop<Expr<'a, Todo>>,
	atom!("scroll-snap-type") => ScrollSnapType<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-scroll-snap-2/#property-index
	atom!("scroll-start") => ScrollStart<Expr<'a, Todo>>,
	atom!("scroll-start-block") => ScrollStartBlock<Expr<'a, Todo>>,
	atom!("scroll-start-inline") => ScrollStartInline<Expr<'a, Todo>>,
	atom!("scroll-start-target") => ScrollStartTarget<Expr<'a, Todo>>,
	atom!("scroll-start-target-block") => ScrollStartTargetBlock<Expr<'a, Todo>>,
	atom!("scroll-start-target-inline") => ScrollStartTargetInline<Expr<'a, Todo>>,
	atom!("scroll-start-target-x") => ScrollStartTargetX<Expr<'a, Todo>>,
	atom!("scroll-start-target-y") => ScrollStartTargetY<Expr<'a, Todo>>,
	atom!("scroll-start-x") => ScrollStartX<Expr<'a, Todo>>,
	atom!("scroll-start-y") => ScrollStartY<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-scrollbars-1/#property-index
	atom!("scrollbar-color") => ScrollbarColor<Expr<'a, Todo>>,
	atom!("scrollbar-width") => ScrollbarWidth<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-shadow-parts-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-shapes-1/#property-index
	atom!("shape-image-threshold") => ShapeImageThreshold<Expr<'a, Todo>>,
	atom!("shape-margin") => ShapeMargin<Expr<'a, Todo>>,
	atom!("shape-outside") => ShapeOutside<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-shapes-2/#property-index
	// ! shape-inside is redefined in css-round-display-1
	atom!("shape-padding") => ShapePadding<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-size-adjust-1/#property-index
	atom!("text-size-adust") => TextSizeAdjust<MathExpr<'a, TextSizeAdjustValue>>,

	// https://drafts.csswg.org/css-sizing-3/#property-index
	atom!("box-sizing") => BoxSizing<Expr<'a, BoxSizingValue>>,
	atom!("height") => Height<MathExpr<'a, Sizing>>,
	atom!("max-height") => MaxHeight<MathExpr<'a, MaxSizing>>,
	atom!("max-width") => MaxWidth<MathExpr<'a, MaxSizing>>,
	atom!("min-height") => MinHeight<MathExpr<'a, Sizing>>,
	atom!("min-width") => MinWidth<MathExpr<'a, Sizing>>,
	atom!("width") => Width<MathExpr<'a, Sizing>>,

	// https://drafts.csswg.org/css-sizing-4/#property-index
	atom!("aspect-ratio") => AspecRatio<RatioOrAuto>,
	atom!("contain-intrinsic-block-size") => ContainIntrinsicBlockSize<MathExpr<'a, Length>>,
	atom!("contain-intrinsic-height") => ContainIntrinsicHeight<MathExpr<'a, Length>>,
	atom!("contain-intrinsic-inline-size") => ContainIntrinsicInlineSize<MathExpr<'a, Length>>,
	atom!("contain-intrinsic-size") => ContainIntrinsicSize<MathExpr<'a, Length>>,
	atom!("contain-intrinsic-width") => ContainIntrinsicWidth<MathExpr<'a, Length>>,
	atom!("min-intrinsic-sizing") => MinIntrinsicSizing<Expr<'a, MinIntrinsicSizingValue>>,

	// https://drafts.csswg.org/css-speech-1/#property-index
	atom!("cue") => Cue<Expr<'a, Todo>>,
	atom!("cue-after") => CueAfter<Expr<'a, Todo>>,
	atom!("cue-before") => CueBefore<Expr<'a, Todo>>,
	atom!("pause") => Pause<Expr<'a, Todo>>,
	atom!("pause-after") => PauseAfter<Expr<'a, Todo>>,
	atom!("pause-before") => PauseBefore<Expr<'a, Todo>>,
	atom!("rest") => Rest<Expr<'a, Todo>>,
	atom!("rest-after") => RestAfter<Expr<'a, Todo>>,
	atom!("rest-before") => RestBefore<Expr<'a, Todo>>,
	atom!("speak") => Speak<Expr<'a, Todo>>,
	atom!("speak-as") => SpeakAs<Expr<'a, Todo>>,
	atom!("voice-balance") => VoiceBalance<Expr<'a, Todo>>,
	atom!("voice-duration") => VoiceDuration<Expr<'a, Todo>>,
	atom!("voice-family") => VoiceFamily<Expr<'a, Todo>>,
	atom!("voice-pitch") => VoicePitch<Expr<'a, Todo>>,
	atom!("voice-range") => VoiceRange<Expr<'a, Todo>>,
	atom!("voice-rate") => VoiceRate<Expr<'a, Todo>>,
	atom!("voice-stress") => VoiceStress<Expr<'a, Todo>>,
	atom!("voice-volume") => VoiceVolume<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-style-attr-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-syntax-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-tables-3/#property-index
	atom!("border-collapse") => BorderCollapse<Expr<'a, BorderCollapseValue>>,
	atom!("border-spacing") => BorderSpacing<DoubleShorthand<'a, MathExpr<'a, PositiveLength>>>,
	atom!("caption-side") => CaptionSide<Expr<'a, CaptionSideValue>>,
	atom!("empty-cells") => EmptyCells<Expr<'a, EmptyCellsValue>>,
	atom!("table-layout") => TableLayout<Expr<'a, TableLayoutValue>>,

	// https://drafts.csswg.org/css-template-1/#property-index
	// TODO: Is this even a thing?


	// https://drafts.csswg.org/css-text-3/#property-index
	// ! hanging-punctuation redefined in css-text-4
	// ! hypens redefined in css-text-4
	// ! letter-spacing redefined in css-text-4
	// ! line-break redefined in css-text-4
	// ! overflow-wrap redefined in css-text-4
	// ! tab-size redefined in css-text-4
	// ! text-align redefined in css-text-4
	// ! text-align-all redefined in css-text-4
	// ! text-align-last redefined in css-text-4
	// ! text-indent redefined in css-text-4
	// ! text-justify redefined in css-text-4
	// ! text-transform redefined in css-text-4
	// ! white-space redefined in css-text-4
	// ! word-break redefined in css-text-4
	// ! word-spacing redefined in css-text-4
	// ! word-wrap redefined in css-text-4

	// https://drafts.csswg.org/css-text-4/#property-index
	atom!("hanging-punctuation") => HangingPunctuation<Expr<'a, Todo>> inherits=true,
	atom!("hyphenate-character") => HyphenateCharacter<Expr<'a, Todo>> inherits=true,
	atom!("hyphenate-limit-chars") => HyphenateLimitChars<Expr<'a, Todo>> inherits=true,
	atom!("hyphenate-limit-last") => HyphenateLimitLast<Expr<'a, Todo>> inherits=true,
	atom!("hyphenate-limit-lines") => HyphenateLimitLines<Expr<'a, Todo>> inherits=true,
	atom!("hyphenate-limit-zone") => HyphenateLimitZone<Expr<'a, Todo>> inherits=true,
	atom!("hyphens") => Hyphens<Expr<'a, Todo>> inherits=true,
	atom!("letter-spacing") => LetterSpacing<Expr<'a, Todo>> inherits=true,
	atom!("line-break") => LineBreak<Expr<'a, Todo>> inherits=true,
	atom!("line-padding") => LinePadding<Expr<'a, Todo>> inherits=true,
	atom!("overflow-wrap") => OverflowWrap<Expr<'a, Todo>> inherits=true,
	atom!("tab-size") => TabSize<Expr<'a, Todo>> inherits=true,
	atom!("text-align") => TextAlign<Expr<'a, TextAlignValue>> inherits=true,
	atom!("text-align-all") => TextAlignAll<Expr<'a, TextAlignAllValue>> inherits=true,
	atom!("text-align-last") => TextAlignLast<Expr<'a, TextAlignLastValue>> inherits=true,
	atom!("text-autospace") => TextAutospace<Expr<'a, Todo>> inherits=true,
	atom!("text-group-align") => TextGroupAlign<Expr<'a, Todo>>,
	atom!("text-indent") => TextIndent<Expr<'a, Todo>> inherits=true,
	atom!("text-justify") => TextJustify<Expr<'a, Todo>> inherits=true,
	atom!("text-spacing") => TextSpacing<Expr<'a, Todo>> inherits=true,
	atom!("text-spacing-trim") => TextSpacingTrim<Expr<'a, Todo>> inherits=true,
	atom!("text-transform") => TextTransform<Expr<'a, Todo>> inherits=true,
	atom!("text-wrap") => TextWrap<Expr<'a, Todo>> inherits=true,
	atom!("white-space") => WhiteSpace<Expr<'a, Todo>> inherits=true,
	atom!("white-space-collapse") => WhiteSpaceCollapse<Expr<'a, Todo>> inherits=true,
	atom!("white-space-trim") => WhiteSpaceTrim<Expr<'a, Todo>>,
	atom!("word-boundary-detection") => WordBoundaryDetection<Expr<'a, Todo>> inherits=true,
	atom!("word-boundary-expansion") => WordBoundaryExpansion<Expr<'a, Todo>> inherits=true,
	atom!("word-break") => WordBreak<Expr<'a, Todo>> inherits=true,
	atom!("word-spacing") => WordSpacing<Expr<'a, Todo>> inherits=true,
	atom!("word-wrap") => WordWrap<Expr<'a, Todo>>,
	atom!("wrap-after") => WrapAfter<Expr<'a, Todo>>,
	atom!("wrap-before") => WrapBefore<Expr<'a, Todo>>,
	atom!("wrap-inside") => WrapInside<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-text-decor-3/#property-index
	// ! text-decoration redefined in css-text-decor-4
	// ! text-decoration-color redefined in css-text-decor-4
	// ! text-decoration-line redefined in css-text-decor-4
	// ! text-emphasis redefined in css-text-decor-4
	// ! text-emphasis-color redefined in css-text-decor-4
	// ! text-emphasis-position redefined in css-text-decor-4
	// ! text-emphasis-style redefined in css-text-decor-4
	// ! tex-tshadow redefined in css-text-decor-4
	// ! text-underline-position redefined in css-text-decor-4

	// https://drafts.csswg.org/css-text-decor-4/#property-index
	atom!("text-decoration") => TextDecoration<TextDecorationShorthand<'a>> shorthand=true,
	atom!("text-decoration-color") => TextDecorationColor<Expr<'a, ColorValue<'a>>> initial=Expr::Literal(Spanned::dummy(ColorValue::CurrentColor)),
	atom!("text-decoration-line") => TextDecorationLine<Expr<'a, TextDecorationLineValue>> inherits=true,
	atom!("text-decoration-skip") => TextDecorationSkip<Expr<'a, Todo>> inherits=true,
	atom!("text-decoration-skip-ink") => TextDecorationSkipInk<Expr<'a, TextDecorationSkipInkValue>> inherits=true,
	atom!("text-decoration-skip-self") => TextDecorationSkipSelf<Expr<'a, Todo>>,
	atom!("text-decoration-skip-spaces") => TextDecorationSkipSpaces<Expr<'a, Todo>> inherits=true,
	atom!("text-decoration-style") => TextDecorationStyle<Expr<'a, TextDecorationStyleValue>>,
	atom!("text-decoration-thickness") => TextDecorationThickness<Expr<'a, Todo>>,
	atom!("text-decoration-trim") => TextDecorationTrim<Expr<'a, Todo>>,
	atom!("text-emphasis") => TextEmphasis<Expr<'a, Todo>> shorthand=true,
	atom!("text-emphasis-color") => TextEmphasisColor<ColorValue<'a>> inherits=true initial=ColorValue::CurrentColor,
	atom!("text-emphasis-position") => TextEmphasisPosition<Expr<'a, Todo>> inherits=true,
	atom!("text-emphasis-skip") => TextEmphasisSkip<Expr<'a, Todo>> inherits=true,
	atom!("text-emphasis-style") => TextEmphasisStyle<Expr<'a, Todo>> inherits=true,
	atom!("text-shadow") => TextShadow<Expr<'a, Todo>> inherits=true,
	atom!("text-underline-offset") => TextUnderlineOffset<Expr<'a, Todo>> inherits=true,
	atom!("text-underline-position") => TextUnderlinePosition<Expr<'a, Todo>> inherits=true,

	// https://drafts.csswg.org/css-transitions-1/#property-index
	atom!("transition") => Transition<Expr<'a, Todo>> shorthand=true,
	atom!("transition-delay") => TransitionDelay<Expr<'a, Todo>>,
	atom!("transition-duration") => TransitionDuration<Expr<'a, Todo>>,
	atom!("transition-property") => TransitionProperty<Expr<'a, Todo>>,
	atom!("transition-timing-function") => TransitionTimingFunction<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-transitions-2/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-tv/#property-index
	// <Discontinued>

	// https://drafts.csswg.org/css-ui-3/#property-index
	// ! box-sizing refined in css-sizing-3
	// ! caret-color redefined in css-ui-4
	// ! outline redefined in css-ui-4
	// ! outline redefined in css-ui-4
	// ! outline-color redefined in css-ui-4
	// ! outline-offset redefined in css-ui-4
	// ! outline-style redefined in css-ui-4
	// ! outline-width redefined in css-ui-4
	// ! resize redefined in css-ui-4
	// ! text-overflow redefined in css-overflow-4

	// https://drafts.csswg.org/css-ui-4/#property-index
	atom!("accent-color") => AccentColor<Expr<'a, Todo>> inherits=true,
	atom!("appearance") => Appearance<Expr<'a, Todo>>,
	atom!("caret") => Caret<Expr<'a, Todo>> inherits=true,
	atom!("caret-color") => CaretColor<Expr<'a, Todo>> inherits=true,
	atom!("caret-shape") => CaretShape<Expr<'a, Todo>> inherits=true,
	atom!("cursor") => Cursor<Expr<'a, CursorValue<'a>>> inherits=true,
	atom!("input-security") => InputSecurity<AutoOrNone>,
	atom!("nav-down") => NavDown<Expr<'a, Todo>>,
	atom!("nav-left") => NavLeft<Expr<'a, Todo>>,
	atom!("nav-right") => NavRight<Expr<'a, Todo>>,
	atom!("nav-up") => NavUp<Expr<'a, Todo>>,
	atom!("outline") => Outline<Expr<'a, Todo>> shorthand=true,
	atom!("outline-color") => OutlineColor<Expr<'a, Todo>>,
	atom!("outline-offset") => OutlineOffset<Expr<'a, Todo>>,
	atom!("outline-style") => OutlineStyle<Expr<'a, Todo>>,
	atom!("outline-width") => OutlineWidth<LineWidth>,
	atom!("pointer-events") => PointerEvents<AutoOrNone> inherits=true,
	atom!("resize") => Resize<Expr<'a, Todo>>,
	atom!("user-select") => UserSelect<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-values-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-values-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-values-5/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-variables-1/#property-index
	// This spec defines the <dashed-ident> properties.

	// https://drafts.csswg.org/css-variables-2/#property-index
	// This spec defines the <dashed-ident> properties.

	// https://drafts.csswg.org/css-view-transitions-1/#property-index
	atom!("view-transition-name") => ViewTransitionName<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-viewport/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-will-change-1/#property-index
	atom!("will-change") => WillChange<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css-writing-modes-3/#property-index
	// ! direction is redefined in css-text-decor-4
	// ! glyph-orientation-horizontal is redefined in css-text-decor-4
	// ! text-combine-upright is redefined in css-text-decor-4
	// ! text-orientation is redefined in css-text-decor-4
	// ! unicode-bidi is redefined in css-text-decor-4
	// ! writing-mode is redefined in css-text-decor-4

	// https://drafts.csswg.org/css-writing-modes-4/#property-index
	atom!("direction") => Direction<Expr<'a, Todo>>,
	atom!("glyph-orientation-horizontal") => GlyphOrientationHorizontal<Expr<'a, Todo>>,
	atom!("text-combine-upright") => TextCombineUpright<Expr<'a, Todo>>,
	atom!("text-orientation") => TextOrientation<Expr<'a, Todo>>,
	atom!("unicode-bidi") => UnicodeBidi<Expr<'a, Todo>>,
	atom!("writing-mode") => WritingMode<Expr<'a, Todo>>,

	// https://drafts.csswg.org/css2/#property-index
	// ! background is redefined in css-backgrounds-3
	// ! background-attachment is redefined in css-backgrounds-3
	// ! background-color is redefined in css-backgrounds-3
	// ! background-image is redefined in css-backgrounds-3
	// ! background-position is redefined in css-backgrounds-3
	// ! background-repeat is redefined in css-backgrounds-3
	// ! border is redefined in css-backgrounds-3
	// ! border-bottom is redefined in css-backgrounds-3
	// ! border-bottom-color is redefined in css-backgrounds-3
	// ! border-bottom-style is redefined in css-backgrounds-3
	// ! border-bottom-width is redefined in css-backgrounds-3
	// ! border-collapse is redefined in css-tables-3
	// ! border-color is redefined in css-backgrounds-3
	// ! border-left is redefined in css-backgrounds-3
	// ! border-left-color is redefined in css-backgrounds-3
	// ! border-left-style is redefined in css-backgrounds-3
	// ! border-left-width is redefined in css-backgrounds-3
	// ! border-right is redefined in css-backgrounds-3
	// ! border-right-color is redefined in css-backgrounds-3
	// ! border-right-style is redefined in css-backgrounds-3
	// ! border-right-width is redefined in css-backgrounds-3
	// ! border-spacing is redefined in css-tables-3
	// ! border-style is redefined in css-backgrounds-3
	// ! border-top is redefined in css-backgrounds-3
	// ! border-top-color is redefined in css-backgrounds-3
	// ! border-top-style is redefined in css-backgrounds-3
	// ! border-top-width is redefined in css-backgrounds-3
	// ! border-width is redefined in css-backgrounds-3
	// ! bottom is redefined in css-position-3
	// ! caption-side is redefined in css-tables-3
	// ! clear is redefined in css-page-floats-3
	// ! clip is redefined in css-masking-1 (appendix)
	// ! color is redefined in css-color-3
	// ! content is redefined in css-content-3
	// ! counter-increment is redefined in css-lists-3
	// ! counter-reset is redefined in css-lists-3
	// ! cursor is redefined in css-ui-4
	// ! direction is redefined in css-writing-modes-4
	// ! display is redefined in css-display-3
	// ! empty-cells is redefined in css-tables-3
	// ! float is redefined in css-page-floats-3
	// ! font is redefined in css-fonts-3
	// ! font-family is redefined in css-fonts-3
	// ! font-size is redefined in css-fonts-3
	// ! font-style is redefined in css-fonts-3
	// ! font-variant is redefined in css-fonts-3
	// ! font-weight is redefined in css-fonts-3
	// ! height is redefined in css-sizing-3
	// ! left is redefined in css-position-3
	// ! letter-spacing is redefined in css-text-decor-4
	// ! line-height is redefined in css-line-grid-1
	// ! list-style is redefined in css-lists-3
	// ! list-style-image is redefined in css-lists-3
	// ! list-style-position is redefined in css-lists-3
	// ! list-style-type is redefined in css-lists-3
	// ! margin is redefined in css-box-3
	// ! margin-bottom is redefined in css-box-3
	// ! margin-left is redefined in css-box-3
	// ! margin-right is redefined in css-box-3
	// ! margin-top is redefined in css-box-3
	// ! max-height is redefined in css-sizing-3
	// ! max-width is redefined in css-sizing-3
	// ! min-height is redefined in css-sizing-3
	// ! min-width is redefined in css-sizing-3
	// ! orphans is redefined in css-page-3
	// ! outline is redefined in css-ui-4
	// ! outline-color is redefined in css-ui-4
	// ! outline-style is redefined in css-ui-4
	// ! outline-width is redefined in css-ui-4
	// ! overflow is redefined in css-overflow-3
	// ! padding is redefined in css-box-3
	// ! padding-bottom is redefined in css-box-3
	// ! padding-left is redefined in css-box-3
	// ! padding-right is redefined in css-box-3
	// ! padding-top is redefined in css-box-3
	// ! page-break-after is redefined in css-page-3
	// ! page-break-before is redefined in css-page-3
	// ! page-break-inside is redefined in css-page-3
	// ! position is redefined in css-position-3
	// ! quotes is redefined in css-content-3
	// ! right is redefined in css-position-3
	// ! table-layout is redefined in css-tables-3
	// ! text-align is redefined in css-text-3
	// ! text-decoration is redefined in css-text-decor-4
	// ! text-indent is redefined in css-text-3
	// ! text-transform is redefined in css-text-decor-4
	// ! top is redefined in css-position-3
	// ! unicode-bidi is redefined in css-writing-modes-4
	// ! vertical-align is redefined in css-inline-3
	// ! visibility is redefined in css-ui-4
	// ! white-space is redefined in css-text-3
	// ! widows is redefined in css-page-3
	// ! word-spacing is redefined in css-text-decor-4
	atom!("z-index") => ZIndex<Expr<'a, Todo>>,

	// https://drafts.csswg.org/cssom-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/cssom-view-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/mediaqueries-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/mediaqueries-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/mediaqueries-5/#property-index
	// <No properties>

	// https://drafts.csswg.org/resize-observer-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/scroll-animations-1/#property-index
	atom!("antimation-range") => AnimationRange<Expr<'a, Todo>>,
	atom!("antimation-range-end") => AnimationRangeEnd<Expr<'a, Todo>>,
	atom!("antimation-range-start") => AnimationRangeStart<Expr<'a, Todo>>,
	atom!("scroll-timeline") => ScrollTimeline<Expr<'a, Todo>>,
	atom!("scroll-timeline-axis") => ScrollTimelineAxis<Expr<'a, Todo>>,
	atom!("scroll-timeline-name") => ScrollTimelineName<Expr<'a, Todo>>,
	atom!("timeline-scope") => TimelineScope<Expr<'a, Todo>>,
	atom!("view-timeline") => ViewTimeline<Expr<'a, Todo>>,
	atom!("view-timeline-axis") => ViewTimelineAxis<Expr<'a, Todo>>,
	atom!("view-timeline-inset") => ViewTimelineInset<Expr<'a, Todo>>,
	atom!("view-timeline-name") => ViewTimelineName<Expr<'a, Todo>>,

	// https://drafts.csswg.org/selectors-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/selectors-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/selectors-nonelement-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/web-animations-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/web-animations-2/#property-index
	// <No properties>

	// Non Standard Properties
	atom!("zoom") => NonStandardZoom<ZoomValue> standard=false,
	// https://drafts.fxtf.org/css-masking/#clip-property
	atom!("clip") => NonStandardClip<Expr<'a, Todo>> standard=false,

	// Webkit NonStandards
	atom!("-webkit-text-size-adjust") => WebkitTextSizeAdjust<MathExpr<'a, TextSizeAdjustValue>> standard=false,
	atom!("-webkit-text-decoration") => WebkitTextDecoration<TextDecorationShorthand<'a>> standard=false,
	atom!("-webkit-tap-highlight-color") => WebkitTapHighlightColor<MathExpr<'a, ColorValue<'a>>> standard=false,
	atom!("-webkit-text-decoration-skip-ink") => WebkitTextDecorationSkipInk<Expr<'a, TextDecorationSkipInkValue>> standard=false,
}
