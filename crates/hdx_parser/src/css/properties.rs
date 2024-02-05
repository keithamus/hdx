/// Properties
use hdx_ast::css::{
	component_values::ComponentValue, properties::*, unknown::UnknownDeclaration, values::*,
};
use miette::Result;

use crate::{atom, diagnostics, Atomizable, Kind, Parse, Parser, Spanned, Token};

impl<'a> Parse<'a> for Custom<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut important = false;
		let name = parser.expect_ident_cased()?;
		parser.expect(Kind::Colon)?;
		let checkpoint = parser.checkpoint();
		let value_span = parser.span();
		let value_like = ValueLike::parse(parser)
			.unwrap_or(ValueLike::Unknown.spanned(value_span.end(parser.pos())));
		parser.rewind(checkpoint);
		let mut value = parser.parse_component_values(Kind::Semicolon, true)?;
		if parser.at(Kind::Semicolon) {
			parser.advance();
		}
		if let Some(Spanned { node: ComponentValue::Token(tok), .. }) = value.last() {
			if tok.kind == Kind::Ident && tok.as_atom_lower().unwrap() == atom!("important") {
				let len = value.len();
				if let Some(Spanned { node: ComponentValue::Token(tok), .. }) = value.get(len - 2) {
					if tok.kind == Kind::Delim && tok.value.as_char().unwrap() == '!' {
						value.truncate(len - 2);
						important = true
					}
				}
			}
		}
		Ok(Self { name, value_like, value: parser.boxup(value), important }
			.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for ValueLike<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let checkpoint = parser.checkpoint();
		let span = parser.span();
		let token = parser.cur().clone();
		let parsed = MathExpr::<Length>::parse(parser);
		if let Ok(value) = parsed {
			return Ok(Self::Length(parser.boxup(value)).spanned(span.end(parser.pos())));
		}
		parser.rewind(checkpoint);
		let checkpoint = parser.checkpoint();
		let parsed = MathExpr::<LengthPercentage>::parse(parser);
		if let Ok(value) = parsed {
			return Ok(Self::LengthPercentage(parser.boxup(value)).spanned(span.end(parser.pos())));
		}
		parser.rewind(checkpoint);
		let checkpoint = parser.checkpoint();
		let parsed = Expr::<ColorValue>::parse(parser);
		if let Ok(value) = parsed {
			return Ok(Self::Color(parser.boxup(value)).spanned(span.end(parser.pos())));
		}
		parser.rewind(checkpoint);
		let parsed = ExprList::<FontFamilyValue>::parse(parser);
		if let Ok(value) = parsed {
			return Ok(Self::FontFamily(parser.boxup(value)).spanned(span.end(parser.pos())));
		}
		Err(diagnostics::Unexpected(token.kind, token.span).into())
	}
}

macro_rules! parse_properties {
	{$( $prop: ident, )+} => {
		$(
			impl<'a> Parse<'a> for $prop<'a> {
				fn parse(parser: &mut Parser<'a>) -> Result<Spanned<$prop<'a>>> {
					let span = parser.span();
					parser.parse_declaration(
						Some($prop::name_as_atom()),
						|parser: &mut Parser<'a>, _name: &Token, value: Spanned<<$prop as Declaration>::Value>, important: bool| {
							Ok($prop { value: parser.boxup(value), important }.spanned(span.end(parser.pos())))
						},
					)
				}
			}
		)+

		impl<'a> Parse<'a> for Property<'a> {
			fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Property<'a>>> {
				let span = parser.span();
				if parser.cur().is_dashed_ident() {
					let custom = Custom::parse(parser)?;
					return Ok(Property::Custom(parser.boxup(custom)).spanned(span.end(parser.pos())));
				}
				let checkpoint = parser.checkpoint();
				let property = match PropertyId::from_atom(parser.cur().as_atom_lower().unwrap_or(atom!(""))) {
					$(
						Some(PropertyId::$prop) => {
							$prop::parse(parser).map(|p| Property::$prop(parser.boxup(p)))
						}
					)+
					_ => {
						Err(diagnostics::UnexpectedIdent(parser.cur().as_atom().unwrap(), parser.span()))?
					}
				}
				.or_else(|e| {
					parser.rewind(checkpoint);
					let parsed =
						UnknownDeclaration::parse(parser).map(|p| Property::Unknown(parser.boxup(p)));
					parser.warnings.push(e);
					parser.warnings.push(diagnostics::UnknownDeclaration(span.end(parser.pos())).into());
					parsed
				})?;
				Ok(property.spanned(span.end(parser.pos())))
			}
		}
	}
}

parse_properties! {
	// https://drafts.csswg.org/css-align-3/#property-index
	AlignContent,
	AlignItems,
	AlignSelf,
	ColumnGap,
	Gap,
	JustifyContent,
	JustifyItems,
	JustifySelf,
	PlaceContent,
	PlaceItems,
	PlaceSelf,
	RowGap,

	// https://drafts.csswg.org/css-anchor-position-1/#property-index
	AnchorDefault,
	AnchorPosition,
	PositionFallback,
	PositionFallbackBounds,

	// https://drafts.csswg.org/css-animations-1/#property-index
	Animation,
	AnimationDelay,
	AnimationDirection,
	// ! animation-duration redefined in css-animations-2
	AnimationFillMode,
	AnimationIterationCount,
	AnimationName,
	AnimationPlayState,
	AnimationTimingFunction,

	// https://drafts.csswg.org/css-animations-2/#property-index
	AnimationDuration,
	AnimationComposition,
	AnimationTimeline,

	// https://drafts.csswg.org/css-backgrounds-3/#property-index
	Background,
	BackgroundAttachment,
	// ! background-clip redefined in css-backgrounds-4
	BackgroundColor,
	BackgroundImage,
	BackgroundOrigin,
	// ! background-position redefined in css-backgrounds-4
	BackgroundRepeat,
	BackgroundSize,
	Border,
	BorderBottom,
	BorderBottomColor,
	BorderBottomLeftRadius,
	BorderBottomRightRadius,
	BorderBottomStyle,
	BorderBottomWidth,
	BorderColor,
	BorderImage,
	BorderImageOutset,
	BorderImageRepeat,
	BorderImageSlice,
	BorderImageSource,
	BorderImageWidth,
	BorderLeft,
	BorderLeftColor,
	BorderLeftStyle,
	BorderLeftWidth,
	BorderRadius,
	BorderRight,
	BorderRightColor,
	BorderRightStyle,
	BorderRightWidth,
	BorderStyle,
	BorderTop,
	BorderTopColor,
	BorderTopLeftRadius,
	BorderTopRightRadius,
	BorderTopStyle,
	BorderTopWidth,
	BorderWidth,
	BoxShadow,

	// https://drafts.csswg.org/css-backgrounds-4/#property-index
	BackgroundClip,
	BackgroundPosition,
	BackgroundPositionBlock,
	BackgroundPositionInline,
	BackgroundPositionX,
	BackgroundPositionY,


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
	Margin,
	MarginBottom,
	MarginLeft,
	MarginRight,
	MarginTop,
	MarginTrim,
	Padding,
	PaddingBottom,
	PaddingLeft,
	PaddingRight,
	PaddingTop,

	// https://drafts.csswg.org/css-break-3/#property-index
	// ! box-decoration-break redefined in css-break-4
	// ! break-after redefined in css-break-4
	// ! break-before redefined in css-break-4
	// ! break-inside redefined in css-break-4
	// ! orphans redefined in css-break-4
	// ! widows redefined in css-break-4

	// https://drafts.csswg.org/css-break-4/#property-index
	BoxDecorationBreak,
	BreakAfter,
	BreakBefore,
	BreakInside,
	BreakMargin,
	PageBreakAfter,
	PageBreakBefore,
	PageBreakInside,
	Orphans,
	Widows,

	// https://drafts.csswg.org/css-cascade-3/#property-index
	// ! all redefined in css-cascade-4

	// https://drafts.csswg.org/css-cascade-4/#property-index
	// ! all redefined in css-cascade-5

	// https://drafts.csswg.org/css-cascade-5/#property-index
	All,

	// https://drafts.csswg.org/css-cascade-6/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-3/#property-index
	// ! color redefined in css-color-4
	// ! opacity redefined in css-color-4

	// https://drafts.csswg.org/css-color-4/#property-index
	Color,
	Opacity,

	// https://drafts.csswg.org/css-color-5/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-6/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-adjust-1/#property-index
	ColorAdjust,
	ColorScheme,
	ForcedColorAdjust,
	PrintColorAdjust,

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
	Contain,
	// ! content-visibility redefined in css-contain-3


	// https://drafts.csswg.org/css-contain-3/#property-index
	Container,
	ContainerName,
	ContainerType,
	ContentVisibility,

	// https://drafts.csswg.org/css-content-3/#property-index
	BookmarkLabel,
	BookmarkLevel,
	BookmarkState,
	Content,
	Quotes,
	StringSet,

	// https://drafts.csswg.org/css-counter-styles-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-display-3/#property-index
	// ! display redefined in css-display-4
	// ! order redefined in css-display-4
	// ! visibility redefined in css-display-4

	// https://drafts.csswg.org/css-display-4/#property-index
	Display,
	LayoutOrder,
	Order,
	ReadingOrder,
	Visibility,

	// https://drafts.csswg.org/css-easing-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-easing-2/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-egg-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-env-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-exclusions-1/#property-index
	WrapFlow,
	WrapThrough,

	// https://drafts.csswg.org/css-extensions-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-flexbox-1/#property-index
	// ! align-content redefined in css-align-3
	// ! align-items redefined in css-align-3
	// ! align-self redefined in css-align-3
	Flex,
	FlexBasis,
	FlexDirection,
	FlexFlow,
	FlexGrow,
	FlexShrink,
	FlexWrap,
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
	Font,
	FontFamily,
	FontFeatureSettings,
	FontKerning,
	FontLanguageOverride,
	FontOpticalSizing,
	FontPalette,
	FontSize,
	// ! font-size-adjust redefined in css-fonts-5
	FontStretch,
	FontStyle,
	FontSynthesis,
	FontSynthesisSmallCaps,
	FontSynthesisStyle,
	FontSynthesisWeight,
	FontVariant,
	FontVariantAlternates,
	FontVariantCaps,
	FontVariantEastAsian,
	FontVariantEmoji,
	FontVariantLigatures,
	FontVariantNumeric,
	FontVariantPosition,
	FontVariationSettings,
	FontWeight,

	// https://drafts.csswg.org/css-fonts-5/#property-index
	FontSizeAdjust,

	// https://drafts.csswg.org/css-forms-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-gcpm-3/#property-index
	FootnoteDisplay,
	FootnotePolicy,
	Running,
	// ! string-set redefined in css-content-3

	// https://drafts.csswg.org/css-gcpm-4/#property-index
	CopyInto,

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
	Grid,
	GridArea,
	GridAutoColumns,
	GridAutoFlow,
	GridAutoRows,
	GridColumn,
	GridColumnEnd,
	GridColumnStart,
	GridRow,
	GridRowEnd,
	GridRowStart,
	GridTemplate,
	GridTemplateAreas,
	GridTemplateColumns,
	GridTemplateRows,

	// https://drafts.csswg.org/css-grid-3/#property-index
	AlignTracks,
	JustifyTracks,
	MasonryAutoFlow,

	// https://drafts.csswg.org/css-images-3/#property-index
	ImageOrientation,
	ImageRendering,
	// ! object-fit redefined in css-images-4
	ObjectPosition,

	// https://drafts.csswg.org/css-images-4/#property-index
	ImageResolution,
	ObjectFit,

	// https://drafts.csswg.org/css-images-5/#property-index
	ObjectViewBox,

	// https://drafts.csswg.org/css-inline-3/#property-index
	AlignmentBaseline,
	BaselineSource,
	BaselineShift,
	DominantBaseline,
	InitialLetter,
	InitialLetterAlign,
	InitialLetterWrap,
	InlineSizing,
	LineHeight,
	TextBoxEdge,
	TextBoxTrim,
	VerticalAlign,

	// https://drafts.csswg.org/css-line-grid-1/#property-index
	BoxSnap,
	LineGrid,
	LineSnap,

	// https://drafts.csswg.org/css-link-params-1/#property-index
	LinkParameters,

	// https://drafts.csswg.org/css-lists-3/#property-index
	CounterIncrement,
	CounterReset,
	CounterSet,
	ListStyle,
	ListStyleImage,
	ListStylePosition,
	ListStyleType,
	MarkerSide,

	// https://drafts.csswg.org/css-logical-1/#property-index
	BlockSize,
	BorderBlock,
	BorderBlockColor,
	BorderBlockEnd,
	BorderBlockEndColor,
	BorderBlockEndStyle,
	BorderBlockEndWidth,
	BorderBlockStart,
	BorderBlockStartColor,
	BorderBlockStartStyle,
	BorderBlockStartWidth,
	BorderBlockStyle,
	BorderBlockWidth,
	BorderEndEndRadius,
	BorderEndStartRadius,
	BorderInline,
	BorderInlineColor,
	BorderInlineEnd,
	BorderInlineEndColor,
	BorderInlineEndStyle,
	BorderInlineEndWidth,
	BorderInlineStart,
	BorderInlineStartColor,
	BorderInlineStartStyle,
	BorderInlineStartWidth,
	BorderInlineStyle,
	BorderInlineWidth,
	BorderStartEndRadius,
	BorderStartStartRadius,
	InlineSize,
	// ! inset redefined in css-position-3
	// ! inset-block redefined in css-position-3
	// ! inset-inline redefined in css-position-3
	// ! inset-block-end redefined in css-position-3
	// ! inset-inline redefined in css-position-3
	// ! inset-inline-end redefined in css-position-3
	// ! inset-inline-start redefined in css-position-3
	MarginBlock,
	MarginBlockEnd,
	MarginBlockStart,
	MarginInline,
	MarginInlineEnd,
	MarginInlineStart,
	MaxBlockSize,
	MaxInlineSize,
	MinBlockSize,
	MinInlineSize,
	PaddingBlock,
	PaddingBlockEnd,
	PaddingBlockStart,
	PaddingInline,
	PaddingInlineEnd,
	PaddingInlineStart,

	// https://drafts.csswg.org/css-mobile/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-multicol-1/#property-index
	ColumnCount,
	ColumnFill,
	ColumnRule,
	ColumnRuleColor,
	ColumnRuleStyle,
	ColumnRuleWidth,
	// ! column-span redined in css-multicol-2
	ColumnWidth,
	Columns,

	// https://drafts.csswg.org/css-multicol-2/#property-index
	ColumnSpan,

	// https://drafts.csswg.org/css-namespaces-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-nav-1/#property-index
	SpatialNavigationAction,
	SpatialNavigationContain,
	SpatialNavigationFunction,

	// https://drafts.csswg.org/css-nesting-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-overflow-3/#property-index
	Overflow,
	OverflowBlock,
	// ! overflow-clip-margin redined in css-overflow-4
	OverflowInline,
	OverflowX,
	OverflowY,
	ScrollBehavior,
	ScrollbarGutter,
	// ! text-overflow redined in css-overflow-4

	// https://drafts.csswg.org/css-overflow-4/#property-index
	// (Yes this is really in the spec as -webkit-line-clamp)
	WebkitLineClamp,
	BlockEllipsis,
	Continue,
	LineClamp,
	MaxLines,
	OverflowClipMargin,
	OverflowClipMarginBlock,
	OverflowClipMarginBlockEnd,
	OverflowClipMarginBlockStart,
	OverflowClipMarginBottom,
	OverflowClipMarginInline,
	OverflowClipMarginInlineEnd,
	OverflowClipMarginInlineStart,
	OverflowClipMarginLeft,
	OverflowClipMarginRight,
	OverflowClipMarginTop,
	TextOverflow,

	// https://drafts.csswg.org/css-overscroll-1/#property-index
	OverscrollBehavior,
	OverscrollBehaviorBlock,
	OverscrollBehaviorInline,
	OverscrollBehaviorX,
	OverscrollBehaviorY,

	// https://drafts.csswg.org/css-page-3/#property-index
	Page,

	// https://drafts.csswg.org/css-page-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-page-floats-3/#property-index
	Clear,
	Float,
	FloatDefer,
	FloatOffset,
	FloatReference,

	// https://drafts.csswg.org/css-page-template-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-position-3/#property-index
	Bottom,
	Inset,
	InsetBlock,
	InsetBlockEnd,
	InsetBlockStart,
	InsetInline,
	InsetInlineEnd,
	InsetInlineStart,
	Left,
	Position,
	Right,
	Top,

	// https://drafts.csswg.org/css-preslev-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-print/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-pseudo-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-regions-1/#property-index
	FlowFrom,
	FlowInto,
	RegionFragment,

	// https://drafts.csswg.org/css-rhythm-1/#property-index
	BlockStep,
	BlockStepAlign,
	BlockStepInsert,
	BlockStepRound,
	BlockStepSize,
	LineHeightStep,

	// https://drafts.csswg.org/css-round-display-1/#property-index
	BorderBoundary,
	ShapeInside,

	// https://drafts.csswg.org/css-ruby-1/#property-index
	RubyAlign,
	RubyMerge,
	RubyOverhang,
	RubyPosition,

	// https://drafts.csswg.org/css-scoping-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-scroll-anchoring-1/#property-index
	OverflowAnchor,

	// https://drafts.csswg.org/css-scroll-snap-1/#property-index
	ScrollMargin,
	ScrollMarginBlock,
	ScrollMarginBlockEnd,
	ScrollMarginBlockStart,
	ScrollMarginBottom,
	ScrollMarginInline,
	ScrollMarginInlineEnd,
	ScrollMarginInlineStart,
	ScrollMarginLeft,
	ScrollMarginRight,
	ScrollMarginTop,
	ScrollPadding,
	ScrollPaddingBlock,
	ScrollPaddingBlockEnd,
	ScrollPaddingBlockStart,
	ScrollPaddingBottom,
	ScrollPaddingInline,
	ScrollPaddingInlineEnd,
	ScrollPaddingInlineStart,
	ScrollPaddingLeft,
	ScrollPaddingRight,
	ScrollPaddingTop,
	ScrollSnapAlign,
	ScrollSnapStop,
	ScrollSnapType,

	// https://drafts.csswg.org/css-scroll-snap-2/#property-index
	ScrollStart,
	ScrollStartBlock,
	ScrollStartInline,
	ScrollStartTarget,
	ScrollStartTargetBlock,
	ScrollStartTargetInline,
	ScrollStartTargetX,
	ScrollStartTargetY,
	ScrollStartX,
	ScrollStartY,

	// https://drafts.csswg.org/css-scrollbars-1/#property-index
	ScrollbarColor,
	ScrollbarWidth,

	// https://drafts.csswg.org/css-shadow-parts-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-shapes-1/#property-index
	ShapeImageThreshold,
	ShapeMargin,
	ShapeOutside,

	// https://drafts.csswg.org/css-size-adjust-1/#property-index
	TextSizeAdjust,

	// https://drafts.csswg.org/css-shapes-2/#property-index
	// ! shape-inside is redefined in css-round-display-1
	ShapePadding,

	// https://drafts.csswg.org/css-sizing-3/#property-index
	BoxSizing,
	Height,
	MaxHeight,
	MaxWidth,
	MinHeight,
	MinWidth,
	Width,

	// https://drafts.csswg.org/css-sizing-4/#property-index
	AspecRatio,
	ContainIntrinsicBlockSize,
	ContainIntrinsicHeight,
	ContainIntrinsicInlineSize,
	ContainIntrinsicSize,
	ContainIntrinsicWidth,
	MinIntrinsicSizing,

	// https://drafts.csswg.org/css-speech-1/#property-index
	Cue,
	CueAfter,
	CueBefore,
	Pause,
	PauseAfter,
	PauseBefore,
	Rest,
	RestAfter,
	RestBefore,
	Speak,
	SpeakAs,
	VoiceBalance,
	VoiceDuration,
	VoiceFamily,
	VoicePitch,
	VoiceRange,
	VoiceRate,
	VoiceStress,
	VoiceVolume,

	// https://drafts.csswg.org/css-style-attr-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-syntax-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-tables-3/#property-index
	BorderCollapse,
	BorderSpacing,
	CaptionSide,
	EmptyCells,
	TableLayout,

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
	HangingPunctuation,
	HyphenateCharacter,
	HyphenateLimitChars,
	HyphenateLimitLast,
	HyphenateLimitLines,
	HyphenateLimitZone,
	Hyphens,
	LetterSpacing,
	LineBreak,
	LinePadding,
	OverflowWrap,
	TabSize,
	TextAlign,
	TextAlignAll,
	TextAlignLast,
	TextAutospace,
	TextGroupAlign,
	TextIndent,
	TextJustify,
	TextSpacing,
	TextSpacingTrim,
	TextTransform,
	TextWrap,
	WhiteSpace,
	WhiteSpaceCollapse,
	WhiteSpaceTrim,
	WordBoundaryDetection,
	WordBoundaryExpansion,
	WordBreak,
	WordSpacing,
	WordWrap,
	WrapAfter,
	WrapBefore,
	WrapInside,

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
	TextDecoration,
	TextDecorationColor,
	TextDecorationLine,
	TextDecorationSkip,
	TextDecorationSkipInk,
	TextDecorationSkipSelf,
	TextDecorationSkipSpaces,
	TextDecorationStyle,
	TextDecorationThickness,
	TextDecorationTrim,
	TextEmphasis,
	TextEmphasisColor,
	TextEmphasisPosition,
	TextEmphasisSkip,
	TextEmphasisStyle,
	TextShadow,
	TextUnderlineOffset,
	TextUnderlinePosition,

	// https://drafts.csswg.org/css-transitions-1/#property-index
	Transition,
	TransitionDelay,
	TransitionDuration,
	TransitionProperty,
	TransitionTimingFunction,

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
	AccentColor,
	Appearance,
	Caret,
	CaretColor,
	CaretShape,
	Cursor,
	InputSecurity,
	NavDown,
	NavLeft,
	NavRight,
	NavUp,
	Outline,
	OutlineColor,
	OutlineOffset,
	OutlineStyle,
	OutlineWidth,
	PointerEvents,
	Resize,
	UserSelect,

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
	ViewTransitionName,

	// https://drafts.csswg.org/css-viewport/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-will-change-1/#property-index
	WillChange,

	// https://drafts.csswg.org/css-writing-modes-3/#property-index
	// ! direction is redefined in css-text-decor-4
	// ! glyph-orientation-horizontal is redefined in css-text-decor-4
	// ! text-combine-upright is redefined in css-text-decor-4
	// ! text-orientation is redefined in css-text-decor-4
	// ! unicode-bidi is redefined in css-text-decor-4
	// ! writing-mode is redefined in css-text-decor-4

	// https://drafts.csswg.org/css-writing-modes-4/#property-index
	Direction,
	GlyphOrientationHorizontal,
	TextCombineUpright,
	TextOrientation,
	UnicodeBidi,
	WritingMode,

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
	// ! clip is redefined in css-masking-3
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
	ZIndex,

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
	AnimationRange,
	AnimationRangeEnd,
	AnimationRangeStart,
	ScrollTimeline,
	ScrollTimelineAxis,
	ScrollTimelineName,
	TimelineScope,
	ViewTimeline,
	ViewTimelineAxis,
	ViewTimelineInset,
	ViewTimelineName,

	// https://drafts.csswg.org/selectors-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/selectors-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/selectors-nonelement-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/web-animations-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/web-animations-2/#property-index

	// Non Standard
	NonStandardZoom,
	NonStandardClip,

	// Webkit Non Standard
	WebkitTextSizeAdjust,
	WebkitTextDecoration,
	WebkitTapHighlightColor,
}

#[cfg(test)]
mod test {

	use hdx_ast::css::{
		component_values::ComponentValue,
		values::{ColorValue, Expr, NamedColor},
	};
	use hdx_lexer::TokenValue;

	use super::*;
	use crate::{Allocator, Atom, ParserOptions, Span, Spanned};

	#[test]
	fn parses_display_block() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "display:block", ParserOptions::default());
		let boxed_property = parser.boxup(Spanned {
			span: Span::new(0, 13),
			node: Display {
				value: parser.boxup(Spanned {
					span: Span::new(8, 13),
					node: DisplayValue::Pair(DisplayOutside::Block, DisplayInside::Implicit),
				}),
				important: false,
			},
		});
		let parser_return = parser.parse_entirely_with::<Property>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned { span: Span::new(0, 13), node: Property::Display(boxed_property) }
		);
	}

	#[test]
	fn parses_overflow_hidden() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "overflow    :/**/ hidden;", ParserOptions::default());
		let boxed_property = parser.boxup(Spanned {
			span: Span::new(0, 25),
			node: Overflow {
				value: parser.boxup(Spanned {
					span: Span::new(18, 24),
					node: XYShorthand {
						x: Shorthand::Explicit(parser.boxup(Spanned {
							span: Span::new(18, 24),
							node: Expr::Literal(Spanned {
								span: Span::new(18, 24),
								node: OverflowKeyword::Hidden,
							}),
						})),
						y: Shorthand::Implicit,
					},
				}),
				important: false,
			},
		});
		let parser_return = parser.parse_entirely_with::<Property>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(
			ast,
			Spanned { span: Span::new(0, 25), node: Property::Overflow(boxed_property) }
		);
	}

	#[test]
	fn parses_custom_property() {
		let allocator = Allocator::default();
		let parser =
			Parser::new(&allocator, "--some-thing    :/**/ red;", ParserOptions::default());
		let mut component_values = parser.new_vec();
		component_values.push(Spanned {
			span: Span::new(22, 25),
			node: ComponentValue::Token(Token {
				kind: Kind::Ident,
				span: Span::new(22, 25),
				escaped: false,
				value: TokenValue::String(Atom::from("red")),
			}),
		});
		let boxed_property = parser.boxup(Spanned {
			span: Span::new(0, 26),
			node: Custom {
				name: Atom::from("--some-thing"),
				value: parser.boxup(component_values),
				value_like: Spanned {
					span: Span::new(22, 25),
					node: ValueLike::Color(parser.boxup(Spanned {
						span: Span::new(22, 25),
						node: Expr::Literal(Spanned {
							span: Span::new(22, 25),
							node: ColorValue::Named(NamedColor::Red),
						}),
					})),
				},
				important: false,
			},
		});
		let parser_return = parser.parse_entirely_with::<Property>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, Spanned { span: Span::new(0, 26), node: Property::Custom(boxed_property) });
	}
}
