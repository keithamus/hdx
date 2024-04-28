use hdx_derive::{Atomizable, Parsable, Value, Writable};

use crate::css::types::Color;
use crate::css::values::{
	AlignContent, AlignItems, AlignSelf, Animation, AnimationDelay, AnimationDuration, AnimationFillMode,
	AnimationIterationCount, AnimationName, AnimationTimingFunction, Appearance, BackdropFilter, BackgroundClip,
	BoxAlign, BoxDecorationBreak, BoxDirection, BoxFlex, BoxOrdinalGroup, BoxOrient, BoxPack, BoxShadow, BoxSizing,
	FlexBasis, FlexDirection, FlexFlow, FlexGrow, FlexWrap, JustifyContent, TextDecoration, TextDecorationColor,
	TextDecorationSkipInk, TextSizeAdjust, Todo, TransitionDuration,
};

pub type WebkitAlignContent = AlignContent;
pub type WebkitAlignItems = AlignItems;
pub type WebkitAlignSelf = AlignSelf;
pub type WebkitAnimation = Animation;
pub type WebkitAnimationDelay = AnimationDelay;
pub type WebkitAnimationDuration = AnimationDuration;
pub type WebkitAnimationFillMode = AnimationFillMode;
pub type WebkitAnimationIterationCount = AnimationIterationCount;
pub type WebkitAnimationName = AnimationName;
pub type WebkitAnimationTimingFunction = AnimationTimingFunction;
pub type WebkitAppearance = Appearance;
pub type WebkitBackdropFilter = BackdropFilter;
pub type WebkitBackfaceVisibility = Todo;
pub type WebkitBackgroundClip = BackgroundClip;
pub type WebkitBorderBefore = Todo;
pub type WebkitBoxAlign = BoxAlign;
pub type WebkitBoxDecorationBreak = BoxDecorationBreak;
pub type WebkitBoxDirection = BoxDirection;
pub type WebkitBoxFlex = BoxFlex;
pub type WebkitBoxOrdinalGroup = BoxOrdinalGroup;
pub type WebkitBoxOrient = BoxOrient;
pub type WebkitBoxPack = BoxPack;
pub type WebkitBoxReflect = Todo;
pub type WebkitBoxShadow = BoxShadow;
pub type WebkitBoxSizing = BoxSizing;
pub type WebkitClipPath = Todo;
pub type WebkitColumnCount = Todo;
pub type WebkitColumnGap = Todo;
pub type WebkitFilter = Todo;
pub type WebkitFlex = BoxFlex;
pub type WebkitFlexBasis = FlexBasis;
pub type WebkitFlexDirection = FlexDirection;
pub type WebkitFlexFlow = FlexFlow;
pub type WebkitFlexGrow = FlexGrow;
pub type WebkitFlexWrap = FlexWrap;

// https://developer.mozilla.org/en-US/docs/Web/CSS/font-smooth
#[derive(Value, Parsable, Writable, Atomizable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum WebkitFontSmoothing {
	#[default]
	Auto, // atom!("auto")
	None,        // atom!("none")
	Antialiased, // atom!("antialiased")
	Grayscale,   // atom!("grayscale")
}

pub type WebkitJustifyContent = JustifyContent;
pub type WebkitLineClamp = Todo;
pub type WebkitMarginEnd = Todo;
pub type WebkitMaskAttachment = Todo;
pub type WebkitMaskBoxImage = Todo;
pub type WebkitMaskComposite = Todo;
pub type WebkitMaskImage = Todo;
pub type WebkitMaskPosition = Todo;
pub type WebkitMaskPositionX = Todo;
pub type WebkitMaskPositionY = Todo;
pub type WebkitMaskRepeatX = Todo;
pub type WebkitMaskRepeatY = Todo;
pub type WebkitMaskSize = Todo;
pub type WebkitOrder = Todo;
pub type WebkitOverflowScrolling = Todo;
pub type WebkitPrintColorAdjust = Todo;
pub type WebkitSlideThumb = Todo;

#[derive(Value, Parsable, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct WebkitTapHighlightColor(pub Color);

pub type WebkitTextDecoration = TextDecoration;
pub type WebkitTextDecorationColor = TextDecorationColor;
pub type WebkitTextDecorationSkipInk = TextDecorationSkipInk;
pub type WebkitTextFillColor = Todo;
pub type WebkitTextSecurity = Todo;
pub type WebkitTextSizeAdjust = TextSizeAdjust;
pub type WebkitTextStroke = Todo;
pub type WebkitTextStrokeColor = Todo;
pub type WebkitTextStrokeWidth = Todo;
pub type WebkitTouchCallout = Todo;
pub type WebkitTransform = Todo;
pub type WebkitTransformOrigin = Todo;
pub type WebkitTransition = Todo;
pub type WebkitTransitionDelay = Todo;
pub type WebkitTransitionDuration = TransitionDuration;
pub type WebkitTransitionProperty = Todo;
pub type WebkitTransitionTimingFunction = Todo;
pub type WebkitUserDrag = Todo;
pub type WebkitUserSelect = Todo;
