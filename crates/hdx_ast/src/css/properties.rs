use std::{fmt::Debug, hash::Hash};

use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{diagnostics, discard, expect, unexpected, Parse, Parser, Result as ParserResult, Spanned, State};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values, syntax::ComponentValues, Box};

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Custom<'a> {
	pub value: Box<'a, Spanned<ComponentValues<'a>>>,
	// pub value_like: Spanned<values::ValueLike<'a>>,
}

impl<'a> WriteCss<'a> for Custom<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.value.write_css(sink)
	}
}

impl<'a> Parse<'a> for Custom<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		parser.set(State::StopOnSemicolon);
		let value = ComponentValues::parse(parser)?;
		parser.unset(State::StopOnSemicolon);
		Ok(Self { value: parser.boxup(value) }.spanned(span.end(parser.pos())))
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Computed<'a> {
	pub value: Box<'a, Spanned<ComponentValues<'a>>>,
	// pub value_like: Spanned<values::ValueLike<'a>>,
}

impl<'a> WriteCss<'a> for Computed<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.value.write_css(sink)
	}
}

impl<'a> Parse<'a> for Computed<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		parser.set(State::StopOnSemicolon);
		let value = ComponentValues::parse(parser)?;
		parser.unset(State::StopOnSemicolon);
		Ok(Self { value: parser.boxup(value) }.spanned(span.end(parser.pos())))
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Unknown<'a> {
	pub value: Box<'a, Spanned<ComponentValues<'a>>>,
}

impl<'a> Parse<'a> for Unknown<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
		let span = parser.span();
		parser.set(State::StopOnSemicolon);
		let value = ComponentValues::parse(parser)?;
		parser.unset(State::StopOnSemicolon);
		Ok(Self { value: parser.boxup(value) }.spanned(span.end(parser.pos())))
	}
}

impl<'a> WriteCss<'a> for Unknown<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		self.value.write_css(sink)
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct StyleProperty<'a> {
	name: Atom,
	value: StyleValue<'a>,
	important: bool,
}

impl<'a> WriteCss<'a> for StyleProperty<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_str(self.name.as_ref())?;
		sink.write_char(':')?;
		sink.write_trivia_char(' ')?;
		self.value.write_css(sink)?;
		if self.important {
			sink.write_str("!important")?;
		}
		Ok(())
	}
}

#[inline]
fn is_computed_token(token: Token) -> bool {
	match token {
		Token::Function(atom) => match atom.to_ascii_lowercase() {
			atom!("var")
			| atom!("calc")
			| atom!("min")
			| atom!("max")
			| atom!("clamp")
			| atom!("round")
			| atom!("mod")
			| atom!("rem")
			| atom!("sin")
			| atom!("cos")
			| atom!("tan")
			| atom!("asin")
			| atom!("atan")
			| atom!("atan2")
			| atom!("pow")
			| atom!("sqrt")
			| atom!("hypot")
			| atom!("log")
			| atom!("exp")
			| atom!("abs")
			| atom!("sign") => true,
			_ => false,
		},
		_ => false,
	}
}

macro_rules! properties {
    ( $(
        $name: ident$(<$a: lifetime>)?: $atom: pat,
    )+ ) => {
		#[derive(Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(Serialize), serde())]
		pub enum StyleValue<'a> {
			Initial,
			Inherit,
			Unset,
			Revert,
			RevertLayer,
			Custom(Box<'a, Spanned<Custom<'a>>>),
			Computed(Box<'a, Spanned<Computed<'a>>>),
			Unknown(Box<'a, Spanned<Unknown<'a>>>),
			$(
				$name(Box<'a, Spanned<values::$name$(<$a>)?>>),
			)+
		}

		impl<'a> WriteCss<'a> for StyleValue<'a> {
			fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
				match self {
					Self::Initial => sink.write_str("initial")?,
					Self::Inherit => sink.write_str("inherit")?,
					Self::Unset => sink.write_str("unset")?,
					Self::Revert => sink.write_str("revert")?,
					Self::RevertLayer => sink.write_str("revert-layer")?,
					Self::Custom(v) => v.write_css(sink)?,
					Self::Unknown(v) => v.write_css(sink)?,
					Self::Computed(v) => v.write_css(sink)?,
					$(
						Self::$name(v) => v.write_css(sink)?,
					)+
				}
				Ok(())
			}
		}

		impl<'a> Parse<'a> for StyleProperty<'a> {
			fn parse(parser: &mut Parser<'a>) -> ParserResult<Spanned<Self>> {
				let span = parser.span();
				let (name, value) = match parser.cur() {
					Token::Ident(atom) => {
						parser.advance();
						expect!(parser, Token::Colon);
						parser.advance();
						let name = atom.to_ascii_lowercase();
						match name {
							$(
							$atom => {
								let value = match parser.cur() {
									Token::Ident(atom) => match atom.to_ascii_lowercase() {
										atom!("initial") => {
											parser.advance();
											StyleValue::Initial
										}
										atom!("inherit") => {
											parser.advance();
											StyleValue::Inherit
										}
										atom!("unset") => {
											parser.advance();
											StyleValue::Unset
										}
										atom!("revert") => {
											parser.advance();
											StyleValue::Revert
										}
										atom!("revert-layer") => {
											parser.advance();
											StyleValue::RevertLayer
										},
										_ => {
											let checkpoint = parser.checkpoint();
											if let Ok(value) = values::$name::parse(parser) {
												StyleValue::$name(parser.boxup(value))
											} else if is_computed_token(parser.cur()) {
												parser.rewind(checkpoint);
												let value = Computed::parse(parser)?;
												StyleValue::Computed(parser.boxup(value))
											} else {
												parser.rewind(checkpoint);
												unexpected!(parser)
											}
										}
									},
									_ => {
										if is_computed_token(parser.cur()) {
											let value = Computed::parse(parser)?;
											StyleValue::Computed(parser.boxup(value))
										} else {
											let checkpoint = parser.checkpoint();
											if let Ok(value) = values::$name::parse(parser) {
												StyleValue::$name(parser.boxup(value))
											} else if is_computed_token(parser.cur()) {
												parser.rewind(checkpoint);
												let value = Computed::parse(parser)?;
												StyleValue::Computed(parser.boxup(value))
											} else {
												parser.rewind(checkpoint);
												unexpected!(parser)
											}
										}
									}
								};
								(name, value)
							},
							)*
							_ => {
								let value = Unknown::parse(parser)?;
								parser.warn(diagnostics::UnknownDeclaration(value.span).into());
								(atom, StyleValue::Unknown(parser.boxup(value)))
							}
						}
					},
					token => unexpected!(parser, token),
				};
				let important = if matches!(parser.cur(), Token::Delim('!')) && matches!(parser.peek(), Token::Ident(atom!("important"))) {
					parser.advance();
					parser.advance_including_whitespace_and_comments();
					expect!(parser, Token::Ident(atom!("important")));
					true
				} else {
					false
				};
				discard!(parser, Token::Semicolon);
				Ok(Self { name, value, important }.spanned(span.end(parser.pos())))
			}
		}
    };
}

properties! {
	// https://drafts.csswg.org/css-align-3/#property-index
	AlignContent: atom!("align-content"),
	AlignItems: atom!("align-items"),
	AlignSelf: atom!("align-self"),
	ColumnGap: atom!("column-gap"),
	Gap: atom!("gap"),
	JustifyContent: atom!("justify-content"),
	JustifyItems: atom!("justify-items"),
	JustifySelf: atom!("justify-self"),
	PlaceContent: atom!("place-content"),
	PlaceItems: atom!("place-items"),
	PlaceSelf: atom!("place-self"),
	RowGap: atom!("row-gap"),

	// https://drafts.csswg.org/css-anchor-position-1/#property-index
	AnchorDefault: atom!("anchor-default"),
	AnchorPosition: atom!("anchor-position"),
	PositionFallback: atom!("position-fallback"),
	PositionFallbackBounds: atom!("position-fallback-bounds"),

	// https://drafts.csswg.org/css-animations-1/#property-index
	AnimationExpr: atom!("animation"),
	AnimationDelay: atom!("animation-delay"),
	AnimationDirection: atom!("animation-direction"),
	// ! animation-duration redefined in css-animations-2
	AnimationFillMode: atom!("animation-fill-mode"),
	AnimationIterationCount: atom!("animation-iteration-count"),
	AnimationName: atom!("animation-name"),
	AnimationPlayState: atom!("animation-play-state"),
	AnimationTimingFunction: atom!("animation-timing-function"),

	// https://drafts.csswg.org/css-animations-2/#property-index
	AnimationDuration: atom!("animation-duration"),
	AnimationComposition: atom!("animation-composition"),
	AnimationTimeline: atom!("animation-timeline"),

	// https://drafts.csswg.org/css-backgrounds-3/#property-index
	Background: atom!("background"),
	BackgroundAttachment: atom!("background-attachment"),
	// ! background-clip redefined in css-backgrounds-4
	BackgroundColor: atom!("background-color"),
	BackgroundImage: atom!("background-image"),
	BackgroundOrigin: atom!("background-origin"),
	// ! background-position redefined in css-backgrounds-4
	BackgroundRepeat: atom!("background-repeat"),
	BackgroundSize: atom!("background-size"),
	Border: atom!("border"),
	BorderBottom: atom!("border-bottom"),
	BorderBottomColor: atom!("border-bottom-color"),
	BorderBottomLeftRadius: atom!("border-bottom-left-radius"),
	BorderBottomRightRadius: atom!("border-bottom-right-radius"),
	BorderBottomStyle: atom!("border-bottom-style"),
	BorderBottomWidth: atom!("border-bottom-width"),
	BorderColor: atom!("border-color"),
	BorderImage: atom!("border-image"),
	BorderImageOutset: atom!("border-image-outset"),
	BorderImageRepeat: atom!("border-image-repeat"),
	BorderImageSlice: atom!("border-image-slice"),
	BorderImageSource: atom!("border-image-source"),
	BorderImageWidth: atom!("border-image-width"),
	BorderLeft: atom!("border-left"),
	BorderLeftColor: atom!("border-left-color"),
	BorderLeftStyle: atom!("border-left-style"),
	BorderLeftWidth: atom!("border-left-width"),
	BorderRadius: atom!("border-radius"),
	BorderRight: atom!("border-right"),
	BorderRightColor: atom!("border-right-color"),
	BorderRightStyle: atom!("border-right-style"),
	BorderRightWidth: atom!("border-right-width"),
	BorderStyle: atom!("border-style"),
	BorderTop: atom!("border-top"),
	BorderTopColor: atom!("border-top-color"),
	BorderTopLeftRadius: atom!("border-top-left-radius"),
	BorderTopRightRadius: atom!("border-top-right-radius"),
	BorderTopStyle: atom!("border-top-style"),
	BorderTopWidth: atom!("border-top-width"),
	BorderWidth: atom!("border-width"),
	BoxShadow: atom!("box-shadow"),

	// https://drafts.csswg.org/css-backgrounds-4/#property-index
	BackgroundClip: atom!("background-clip"),
	BackgroundPosition: atom!("background-position"),
	BackgroundPositionBlock: atom!("background-position-block"),
	BackgroundPositionInline: atom!("background-position-inline"),
	BackgroundPositionX: atom!("background-position-x"),
	BackgroundPositionY: atom!("background-position-y"),


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
	Margin: atom!("margin"),
	MarginBottom: atom!("margin-bottom"),
	MarginLeft: atom!("margin-left"),
	MarginRight: atom!("margin-right"),
	MarginTop: atom!("margin-top"),
	MarginTrim: atom!("margin-trim"),
	Padding: atom!("padding"),
	PaddingBottom: atom!("padding-bottom"),
	PaddingLeft: atom!("padding-left"),
	PaddingRight: atom!("padding-right"),
	PaddingTop: atom!("padding-top"),

	// https://drafts.csswg.org/css-break-3/#property-index
	// ! box-decoration-break redefined in css-break-4
	// ! break-after redefined in css-break-4
	// ! break-before redefined in css-break-4
	// ! break-inside redefined in css-break-4
	// ! orphans redefined in css-break-4
	// ! widows redefined in css-break-4

	// https://drafts.csswg.org/css-break-4/#property-index
	BoxDecorationBreak: atom!("box-decoration-break"),
	BreakAfter: atom!("break-after"),
	BreakBefore: atom!("break-before"),
	BreakInside: atom!("break-inside"),
	MarginBreak: atom!("margin-break"),
	// For compatibility with CSS Level 2, UAs that conform to [CSS2] must alias the
	// page-break-before, page-break-after, and page-break-inside properties to break-before,
	// break-after, and break-inside by treating the page-break-* properties as legacy
	// shorthands for the break-* properties with the following value mappings:
	PageBreakAfter: atom!("page-break-after"),
	PageBreakBefore: atom!("page-break-before"),
	PageBreakInside: atom!("page-break-inside"),
	Orphans: atom!("orphans"),
	Widows: atom!("widows"),

	// https://drafts.csswg.org/css-cascade-3/#property-index
	// ! all redefined in css-cascade-4

	// https://drafts.csswg.org/css-cascade-4/#property-index
	// ! all redefined in css-cascade-5

	// https://drafts.csswg.org/css-cascade-5/#property-index
	All: atom!("all"),

	// https://drafts.csswg.org/css-cascade-6/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-3/#property-index
	// ! color redefined in css-color-4
	// ! opacity redefined in css-color-4

	// https://drafts.csswg.org/css-color-4/#property-index
	Color: atom!("color"),
	Opacity: atom!("opacity"),

	// https://drafts.csswg.org/css-color-5/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-6/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-color-adjust-1/#property-index
	ColorAdjust: atom!("color-adjust"),
	ColorScheme: atom!("color-scheme"),
	ForcedColorAdjust: atom!("forced-color-adjust"),
	PrintColorAdjust: atom!("print-color-adjust"),

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
	Contain: atom!("contain"),
	// ! content-visibility redefined in css-contain-3


	// https://drafts.csswg.org/css-contain-3/#property-index
	Container: atom!("container"),
	ContainerName: atom!("container-name"),
	ContainerType: atom!("container-type"),
	ContentVisibility: atom!("content-visibility"),

	// https://drafts.csswg.org/css-content-3/#property-index
	BookmarkLabel: atom!("bookmark-label"),
	BookmarkLevel: atom!("bookmark-level"),
	BookmarkState: atom!("bookmark-state"),
	Content: atom!("content"),
	Quotes: atom!("quotes"),
	StringSet: atom!("string-set"),

	// https://drafts.csswg.org/css-counter-styles-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-display-3/#property-index
	// ! display redefined in css-display-4
	// ! order redefined in css-display-4
	// ! visibility redefined in css-display-4

	// https://drafts.csswg.org/css-display-4/#property-index
	Display: atom!("display"),
	LayoutOrder: atom!("layout-order"),
	Order: atom!("order"),
	ReadingOrder: atom!("reading-order"),
	Visibility: atom!("visibility"),

	// https://drafts.csswg.org/css-easing-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-easing-2/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-egg-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-env-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-exclusions-1/#property-index
	WrapFlow: atom!("wrap-flow"),
	WrapThrough: atom!("wrap-through"),

	// https://drafts.csswg.org/css-extensions-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-flexbox-1/#property-index
	// ! align-content redefined in css-align-3
	// ! align-items redefined in css-align-3
	// ! align-self redefined in css-align-3
	Flex: atom!("flex"),
	FlexBasis: atom!("flex-basis"),
	FlexDirection: atom!("flex-direction"),
	FlexFlow: atom!("flex-flow"),
	FlexGrow: atom!("flex-grow"),
	FlexShrink: atom!("flex-shrink"),
	FlexWrap: atom!("flex-wrap"),
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
	Font: atom!("font"),
	FontFamily: atom!("font-family"),
	FontFeatureSettings: atom!("font-feature-settings"),
	FontKerning: atom!("font-kerning"),
	FontLanguageOverride: atom!("font-language-override"),
	FontOpticalSizing: atom!("font-optical-sizing"),
	FontPalette: atom!("font-palette"),
	FontSize: atom!("font-size"),
	// ! font-size-adjust redefined in css-fonts-5
	FontStretch: atom!("font-stretch"),
	FontStyle: atom!("font-style"),
	FontSynthesis: atom!("font-synthesis"),
	FontSynthesisSmallCaps: atom!("font-synthesis-small-caps"),
	FontSynthesisStyle: atom!("font-synthesis-style"),
	FontSynthesisWeight: atom!("font-synthesis-weight"),
	FontVariant: atom!("font-variant"),
	FontVariantAlternates: atom!("font-variant-alternates"),
	FontVariantCaps: atom!("font-variant-caps"),
	FontVariantEastAsian: atom!("font-variant-east-asian"),
	FontVariantEmoji: atom!("font-variant-emoji"),
	FontVariantLigatures: atom!("font-variant-ligatures"),
	FontVariantNumeric: atom!("font-variant-numeric"),
	FontVariantPosition: atom!("font-variant-position"),
	FontVariationSettings: atom!("font-variation-settings"),
	FontWeight: atom!("font-weight"),

	// https://drafts.csswg.org/css-fonts-5/#property-index
	FontSizeAdjust: atom!("font-size-adjust"),

	// https://drafts.csswg.org/css-forms-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-gcpm-3/#property-index
	FootnoteDisplay: atom!("footnote-display"),
	FootnotePolicy: atom!("footnote-policy"),
	Running: atom!("running"),
	// ! string-set redefined in css-content-3

	// https://drafts.csswg.org/css-gcpm-4/#property-index
	CopyInto: atom!("copy-into"),

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
	Grid: atom!("grid"),
	GridArea: atom!("grid-area"),
	GridAutoColumns: atom!("grid-auto-columns"),
	GridAutoFlow: atom!("grid-auto-flow"),
	GridAutoRows: atom!("grid-auto-rows"),
	GridColumn: atom!("grid-column"),
	GridColumnEnd: atom!("grid-column-end"),
	GridColumnStart: atom!("grid-column-start"),
	GridRow: atom!("grid-row"),
	GridRowEnd: atom!("grid-row-end"),
	GridRowStart: atom!("grid-row-start"),
	GridTemplate: atom!("grid-template"),
	GridTemplateAreas: atom!("grid-template-areas"),
	GridTemplateColumns: atom!("grid-template-columns"),
	GridTemplateRows: atom!("grid-template-rows"),

	// https://drafts.csswg.org/css-grid-3/#property-index
	AlignTracks: atom!("align-tracks"),
	JustifyTracks: atom!("justify-tracks"),
	MasonryAutoFlow: atom!("masonry-auto-flow"),

	// https://drafts.csswg.org/css-images-3/#property-index
	ImageOrientation: atom!("image-orientation"),
	ImageRendering: atom!("image-rendering"),
	// ! object-fit redefined in css-images-4
	ObjectPosition: atom!("object-position"),

	// https://drafts.csswg.org/css-images-4/#property-index
	ImageResolution: atom!("image-resolution"),
	ObjectFit: atom!("object-fit"),

	// https://drafts.csswg.org/css-images-5/#property-index
	ObjectViewBox: atom!("object-view-box"),

	// https://drafts.csswg.org/css-inline-3/#property-index
	AlignmentBaseline: atom!("alignment-baseline"),
	BaselineSource: atom!("baseline-source"),
	BaselineShift: atom!("baseline-shift"),
	DominantBaseline: atom!("dominant-baseline"),
	InitialLetter: atom!("initial-letter"),
	InitialLetterAlign: atom!("initial-letter-align"),
	InitialLetterWrap: atom!("initial-letter-wrap"),
	InlineSizing: atom!("inline-sizing"),
	LineHeight: atom!("line-height"),
	TextBoxEdge: atom!("text-box-edge"),
	TextBoxTrim: atom!("text-box-trim"),
	VerticalAlign: atom!("vertical-align"),

	// https://drafts.csswg.org/css-line-grid-1/#property-index
	BoxSnap: atom!("box-snap"),
	LineGrid: atom!("line-grid"),
	LineSnap: atom!("line-snap"),

	// https://drafts.csswg.org/css-link-params-1/#property-index
	LinkParameters: atom!("link-parameters"),

	// https://drafts.csswg.org/css-lists-3/#property-index
	CounterIncrement: atom!("counter-increment"),
	CounterReset: atom!("counter-reset"),
	CounterSet: atom!("counter-set"),
	ListStyle: atom!("list-style"),
	ListStyleImage: atom!("list-style-image"),
	ListStylePosition: atom!("list-style-position"),
	ListStyleType: atom!("list-style-type"),
	MarkerSide: atom!("marker-side"),

	// https://drafts.csswg.org/css-logical-1/#property-index
	BlockSize: atom!("block-size"),
	BorderBlock: atom!("border-block"),
	BorderBlockColor: atom!("border-block-color"),
	BorderBlockEnd: atom!("border-block-end"),
	BorderBlockEndColor: atom!("border-block-end-color"),
	BorderBlockEndStyle: atom!("border-block-end-style"),
	BorderBlockEndWidth: atom!("border-block-end-width"),
	BorderBlockStart: atom!("border-block-start"),
	BorderBlockStartColor: atom!("border-block-start-color"),
	BorderBlockStartStyle: atom!("border-block-start-style"),
	BorderBlockStartWidth: atom!("border-block-start-width"),
	BorderBlockStyle: atom!("border-block-style"),
	BorderBlockWidth: atom!("border-block-width"),
	BorderEndEndRadius: atom!("border-end-end-radius"),
	BorderEndStartRadius: atom!("border-end-start-radius"),
	BorderInline: atom!("border-inline"),
	BorderInlineColor: atom!("border-inline-color"),
	BorderInlineEnd: atom!("border-inline-end"),
	BorderInlineEndColor: atom!("border-inline-end-color"),
	BorderInlineEndStyle: atom!("border-inline-end-style"),
	BorderInlineEndWidth: atom!("border-inline-end-width"),
	BorderInlineStart: atom!("border-inline-start"),
	BorderInlineStartColor: atom!("border-inline-start-color"),
	BorderInlineStartStyle: atom!("border-inline-start-style"),
	BorderInlineStartWidth: atom!("border-inline-start-width"),
	BorderInlineStyle: atom!("border-inline-style"),
	BorderInlineWidth: atom!("border-inline-width"),
	BorderStartEndRadius: atom!("border-start-end-radius"),
	BorderStartStartRadius: atom!("border-start-start-radius"),
	InlineSize: atom!("inline-size"),
	// ! inset redefined in css-position-3
	// ! inset-block redefined in css-position-3
	// ! inset-inline redefined in css-position-3
	// ! inset-block-end redefined in css-position-3
	// ! inset-inline redefined in css-position-3
	// ! inset-inline-end redefined in css-position-3
	// ! inset-inline-start redefined in css-position-3
	MarginBlock: atom!("margin-block"),
	MarginBlockEnd: atom!("margin-block-end"),
	MarginBlockStart: atom!("margin-block-start"),
	MarginInline: atom!("margin-inline"),
	MarginInlineEnd: atom!("margin-inline-end"),
	MarginInlineStart: atom!("margin-inline-start"),
	MaxBlockSize: atom!("max-block-size"),
	MaxInlineSize: atom!("max-inline-size"),
	MinBlockSize: atom!("min-block-size"),
	MinInlineSize: atom!("min-inline-size"),
	PaddingBlock: atom!("padding-block"),
	PaddingBlockEnd: atom!("padding-block-end"),
	PaddingBlockStart: atom!("padding-block-start"),
	PaddingInline: atom!("padding-inline"),
	PaddingInlineEnd: atom!("padding-inline-end"),
	PaddingInlineStart: atom!("padding-inline-start"),

	// https://drafts.csswg.org/css-mobile/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-multicol-1/#property-index
	ColumnCount: atom!("column-count"),
	ColumnFill: atom!("column-fill"),
	ColumnRule: atom!("column-rule"),
	ColumnRuleColor: atom!("column-rule-color"),
	ColumnRuleStyle: atom!("column-rule-style"),
	ColumnRuleWidth: atom!("column-rule-width"),
	// ! column-span redined in css-multicol-2
	ColumnWidth: atom!("column-width"),
	Columns: atom!("columns"),

	// https://drafts.csswg.org/css-multicol-2/#property-index
	ColumnSpan: atom!("column-span"),

	// https://drafts.csswg.org/css-namespaces-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-nav-1/#property-index
	SpatialNavigationAction: atom!("spatial-navigation-action"),
	SpatialNavigationContain: atom!("spatial-navigation-contain"),
	SpatialNavigationFunction: atom!("spatial-navigation-function"),

	// https://drafts.csswg.org/css-nesting-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-overflow-3/#property-index
	Overflow: atom!("overflow"),
	OverflowBlock: atom!("overflow-block"),
	// ! overflow-clip-margin redined in css-overflow-4
	OverflowInline: atom!("overflow-inline"),
	OverflowX: atom!("overflow-x"),
	OverflowY: atom!("overflow-y"),
	ScrollBehavior: atom!("scroll-behavior"),
	ScrollbarGutter: atom!("scrollbar-gutter"),
	// ! text-overflow redined in css-overflow-4

	// https://drafts.csswg.org/css-overflow-4/#property-index
	// (Yes this is really in the spec as -webkit-line-clamp)
	WebkitLineClamp: atom!("-webkit-line-clamp"),
	BlockEllipsis: atom!("block-ellipsis"),
	Continue: atom!("continue"),
	LineClamp: atom!("line-clamp"),
	MaxLines: atom!("max-lines"),
	OverflowClipMargin: atom!("overflow-clip-margin"),
	OverflowClipMarginBlock: atom!("overflow-clip-margin-block"),
	OverflowClipMarginBlockEnd: atom!("overflow-clip-margin-block-end"),
	OverflowClipMarginBlockStart: atom!("overflow-clip-margin-block-start"),
	OverflowClipMarginBottom: atom!("overflow-clip-margin-bottom"),
	OverflowClipMarginInline: atom!("overflow-clip-margin-inline"),
	OverflowClipMarginInlineEnd: atom!("overflow-clip-margin-inline-end"),
	OverflowClipMarginInlineStart: atom!("overflow-clip-margin-inline-start"),
	OverflowClipMarginLeft: atom!("overflow-clip-margin-left"),
	OverflowClipMarginRight: atom!("overflow-clip-margin-right"),
	OverflowClipMarginTop: atom!("overflow-clip-margin-top"),
	TextOverflow: atom!("text-overflow"),

	// https://drafts.csswg.org/css-overscroll-1/#property-index
	OverscrollBehavior: atom!("overscroll-behavior"),
	OverscrollBehaviorBlock: atom!("overscroll-behavior-block"),
	OverscrollBehaviorInline: atom!("overscroll-behavior-inline"),
	OverscrollBehaviorX: atom!("overscroll-behavior-x"),
	OverscrollBehaviorY: atom!("overscroll-behavior-y"),

	// https://drafts.csswg.org/css-page-3/#property-index
	Page: atom!("page"),

	// https://drafts.csswg.org/css-page-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-page-floats-3/#property-index
	Clear: atom!("clear"),
	Float: atom!("float"),
	FloatDefer: atom!("float-defer"),
	FloatOffset: atom!("float-offset"),
	FloatReference: atom!("float-reference"),

	// https://drafts.csswg.org/css-page-template-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-position-3/#property-index
	Bottom: atom!("bottom"),
	Inset: atom!("inset"),
	InsetBlock: atom!("inset-block"),
	InsetBlockEnd: atom!("inset-block-end"),
	InsetBlockStart: atom!("inset-block-start"),
	InsetInline: atom!("inset-inline"),
	InsetInlineEnd: atom!("inset-inline-end"),
	InsetInlineStart: atom!("inset-inline-start"),
	Left: atom!("left"),
	Position: atom!("position"),
	Right: atom!("right"),
	Top: atom!("top"),

	// https://drafts.csswg.org/css-preslev-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-print/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-pseudo-4/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-regions-1/#property-index
	FlowFrom: atom!("flow-from"),
	FlowInto: atom!("flow-into"),
	RegionFragment: atom!("region-fragment"),

	// https://drafts.csswg.org/css-rhythm-1/#property-index
	BlockStep: atom!("block-step"),
	BlockStepAlign: atom!("block-step-align"),
	BlockStepInsert: atom!("block-step-insert"),
	BlockStepRound: atom!("block-step-round"),
	BlockStepSize: atom!("block-step-size"),
	LineHeightStep: atom!("line-height-step"),

	// https://drafts.csswg.org/css-round-display-1/#property-index
	BorderBoundary: atom!("border-boundary"),
	ShapeInside: atom!("shape-inside"),

	// https://drafts.csswg.org/css-ruby-1/#property-index
	RubyAlign: atom!("ruby-align"),
	RubyMerge: atom!("ruby-merge"),
	RubyOverhang: atom!("ruby-overhang"),
	RubyPosition: atom!("ruby-position"),

	// https://drafts.csswg.org/css-scoping-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-scroll-anchoring-1/#property-index
	OverflowAnchor: atom!("overflow-anchor"),

	// https://drafts.csswg.org/css-scroll-snap-1/#property-index
	ScrollMargin: atom!("scroll-margin"),
	ScrollMarginBlock: atom!("scroll-margin-block"),
	ScrollMarginBlockEnd: atom!("scroll-margin-block-end"),
	ScrollMarginBlockStart: atom!("scroll-margin-block-start"),
	ScrollMarginBottom: atom!("scroll-margin-bottom"),
	ScrollMarginInline: atom!("scroll-margin-inline"),
	ScrollMarginInlineEnd: atom!("scroll-margin-inline-end"),
	ScrollMarginInlineStart: atom!("scroll-margin-inline-start"),
	ScrollMarginLeft: atom!("scroll-margin-left"),
	ScrollMarginRight: atom!("scroll-margin-right"),
	ScrollMarginTop: atom!("scroll-margin-top"),
	ScrollPadding: atom!("scroll-padding"),
	ScrollPaddingBlock: atom!("scroll-padding-block"),
	ScrollPaddingBlockEnd: atom!("scroll-padding-block-end"),
	ScrollPaddingBlockStart: atom!("scroll-padding-block-start"),
	ScrollPaddingBottom: atom!("scroll-padding-bottom"),
	ScrollPaddingInline: atom!("scroll-padding-inline"),
	ScrollPaddingInlineEnd: atom!("scroll-padding-inline-end"),
	ScrollPaddingInlineStart: atom!("scroll-padding-inline-start"),
	ScrollPaddingLeft: atom!("scroll-padding-left"),
	ScrollPaddingRight: atom!("scroll-padding-right"),
	ScrollPaddingTop: atom!("scroll-padding-top"),
	ScrollSnapAlign: atom!("scroll-snap-align"),
	ScrollSnapStop: atom!("scroll-snap-stop"),
	ScrollSnapType: atom!("scroll-snap-type"),

	// https://drafts.csswg.org/css-scroll-snap-2/#property-index
	ScrollStart: atom!("scroll-start"),
	ScrollStartBlock: atom!("scroll-start-block"),
	ScrollStartInline: atom!("scroll-start-inline"),
	ScrollStartTarget: atom!("scroll-start-target"),
	ScrollStartTargetBlock: atom!("scroll-start-target-block"),
	ScrollStartTargetInline: atom!("scroll-start-target-inline"),
	ScrollStartTargetX: atom!("scroll-start-target-x"),
	ScrollStartTargetY: atom!("scroll-start-target-y"),
	ScrollStartX: atom!("scroll-start-x"),
	ScrollStartY: atom!("scroll-start-y"),

	// https://drafts.csswg.org/css-scrollbars-1/#property-index
	ScrollbarColor: atom!("scrollbar-color"),
	ScrollbarWidth: atom!("scrollbar-width"),

	// https://drafts.csswg.org/css-shadow-parts-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-shapes-1/#property-index
	ShapeImageThreshold: atom!("shape-image-threshold"),
	ShapeMargin: atom!("shape-margin"),
	ShapeOutside: atom!("shape-outside"),

	// https://drafts.csswg.org/css-shapes-2/#property-index
	// ! shape-inside is redefined in css-round-display-1
	ShapePadding: atom!("shape-padding"),

	// https://drafts.csswg.org/css-size-adjust-1/#property-index
	TextSizeAdjust: atom!("text-size-adust"),

	// https://drafts.csswg.org/css-sizing-3/#property-index
	BoxSizing: atom!("box-sizing"),
	Height: atom!("height"),
	MaxHeight: atom!("max-height"),
	MaxWidth: atom!("max-width"),
	MinHeight: atom!("min-height"),
	MinWidth: atom!("min-width"),
	Width: atom!("width"),

	// https://drafts.csswg.org/css-sizing-4/#property-index
	AspecRatio: atom!("aspect-ratio"),
	ContainIntrinsicBlockSize: atom!("contain-intrinsic-block-size"),
	ContainIntrinsicHeight: atom!("contain-intrinsic-height"),
	ContainIntrinsicInlineSize: atom!("contain-intrinsic-inline-size"),
	ContainIntrinsicSize: atom!("contain-intrinsic-size"),
	ContainIntrinsicWidth: atom!("contain-intrinsic-width"),
	MinIntrinsicSizing: atom!("min-intrinsic-sizing"),

	// https://drafts.csswg.org/css-speech-1/#property-index
	Cue: atom!("cue"),
	CueAfter: atom!("cue-after"),
	CueBefore: atom!("cue-before"),
	Pause: atom!("pause"),
	PauseAfter: atom!("pause-after"),
	PauseBefore: atom!("pause-before"),
	Rest: atom!("rest"),
	RestAfter: atom!("rest-after"),
	RestBefore: atom!("rest-before"),
	Speak: atom!("speak"),
	SpeakAs: atom!("speak-as"),
	VoiceBalance: atom!("voice-balance"),
	VoiceDuration: atom!("voice-duration"),
	VoiceFamily: atom!("voice-family"),
	VoicePitch: atom!("voice-pitch"),
	VoiceRange: atom!("voice-range"),
	VoiceRate: atom!("voice-rate"),
	VoiceStress: atom!("voice-stress"),
	VoiceVolume: atom!("voice-volume"),

	// https://drafts.csswg.org/css-style-attr-1/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-syntax-3/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-tables-3/#property-index
	BorderCollapse: atom!("border-collapse"),
	BorderSpacing: atom!("border-spacing"),
	CaptionSide: atom!("caption-side"),
	EmptyCells: atom!("empty-cells"),
	TableLayout: atom!("table-layout"),

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
	HangingPunctuation: atom!("hanging-punctuation"),
	HyphenateCharacter: atom!("hyphenate-character"),
	HyphenateLimitChars: atom!("hyphenate-limit-chars"),
	HyphenateLimitLast: atom!("hyphenate-limit-last"),
	HyphenateLimitLines: atom!("hyphenate-limit-lines"),
	HyphenateLimitZone: atom!("hyphenate-limit-zone"),
	Hyphens: atom!("hyphens"),
	LetterSpacing: atom!("letter-spacing"),
	LineBreak: atom!("line-break"),
	LinePadding: atom!("line-padding"),
	OverflowWrap: atom!("overflow-wrap"),
	TabSize: atom!("tab-size"),
	TextAlign: atom!("text-align"),
	TextAlignAll: atom!("text-align-all"),
	TextAlignLast: atom!("text-align-last"),
	TextAutospace: atom!("text-autospace"),
	TextGroupAlign: atom!("text-group-align"),
	TextIndent: atom!("text-indent"),
	TextJustify: atom!("text-justify"),
	TextSpacing: atom!("text-spacing"),
	TextSpacingTrim: atom!("text-spacing-trim"),
	TextTransform: atom!("text-transform"),
	TextWrap: atom!("text-wrap"),
	WhiteSpace: atom!("white-space"),
	WhiteSpaceCollapse: atom!("white-space-collapse"),
	WhiteSpaceTrim: atom!("white-space-trim"),
	WordBoundaryDetection: atom!("word-boundary-detection"),
	WordBoundaryExpansion: atom!("word-boundary-expansion"),
	WordBreak: atom!("word-break"),
	WordSpacing: atom!("word-spacing"),
	WordWrap: atom!("word-wrap"),
	WrapAfter: atom!("wrap-after"),
	WrapBefore: atom!("wrap-before"),
	WrapInside: atom!("wrap-inside"),

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
	TextDecoration: atom!("text-decoration"),
	TextDecorationColor: atom!("text-decoration-color"),
	TextDecorationLine: atom!("text-decoration-line"),
	TextDecorationSkip: atom!("text-decoration-skip"),
	TextDecorationSkipInk: atom!("text-decoration-skip-ink"),
	TextDecorationSkipSelf: atom!("text-decoration-skip-self"),
	TextDecorationSkipSpaces: atom!("text-decoration-skip-spaces"),
	TextDecorationStyle: atom!("text-decoration-style"),
	TextDecorationThickness: atom!("text-decoration-thickness"),
	TextDecorationTrim: atom!("text-decoration-trim"),
	TextEmphasis: atom!("text-emphasis"),
	TextEmphasisColor: atom!("text-emphasis-color"),
	TextEmphasisPosition: atom!("text-emphasis-position"),
	TextEmphasisSkip: atom!("text-emphasis-skip"),
	TextEmphasisStyle: atom!("text-emphasis-style"),
	TextShadow: atom!("text-shadow"),
	TextUnderlineOffset: atom!("text-underline-offset"),
	TextUnderlinePosition: atom!("text-underline-position"),

	// https://drafts.csswg.org/css-transitions-1/#property-index
	Transition: atom!("transition"),
	TransitionDelay: atom!("transition-delay"),
	TransitionDuration: atom!("transition-duration"),
	TransitionProperty: atom!("transition-property"),
	TransitionTimingFunction: atom!("transition-timing-function"),

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
	AccentColor: atom!("accent-color"),
	Appearance: atom!("appearance"),
	Caret: atom!("caret"),
	CaretColor: atom!("caret-color"),
	CaretShape: atom!("caret-shape"),
	Cursor: atom!("cursor"),
	InputSecurity: atom!("input-security"),
	NavDown: atom!("nav-down"),
	NavLeft: atom!("nav-left"),
	NavRight: atom!("nav-right"),
	NavUp: atom!("nav-up"),
	Outline: atom!("outline"),
	OutlineColor: atom!("outline-color"),
	OutlineOffset: atom!("outline-offset"),
	OutlineStyle: atom!("outline-style"),
	OutlineWidth: atom!("outline-width"),
	PointerEvents: atom!("pointer-events"),
	Resize: atom!("resize"),
	UserSelect: atom!("user-select"),

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
	ViewTransitionName: atom!("view-transition-name"),

	// https://drafts.csswg.org/css-viewport/#property-index
	// <No properties>

	// https://drafts.csswg.org/css-will-change-1/#property-index
	WillChange: atom!("will-change"),

	// https://drafts.csswg.org/css-writing-modes-3/#property-index
	// ! direction is redefined in css-text-decor-4
	// ! glyph-orientation-horizontal is redefined in css-text-decor-4
	// ! text-combine-upright is redefined in css-text-decor-4
	// ! text-orientation is redefined in css-text-decor-4
	// ! unicode-bidi is redefined in css-text-decor-4
	// ! writing-mode is redefined in css-text-decor-4

	// https://drafts.csswg.org/css-writing-modes-4/#property-index
	Direction: atom!("direction"),
	GlyphOrientationHorizontal: atom!("glyph-orientation-horizontal"),
	TextCombineUpright: atom!("text-combine-upright"),
	TextOrientation: atom!("text-orientation"),
	UnicodeBidi: atom!("unicode-bidi"),
	WritingMode: atom!("writing-mode"),

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
	ZIndex: atom!("z-index"),

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
	AnimationRange: atom!("antimation-range"),
	AnimationRangeEnd: atom!("antimation-range-end"),
	AnimationRangeStart: atom!("antimation-range-start"),
	ScrollTimeline: atom!("scroll-timeline"),
	ScrollTimelineAxis: atom!("scroll-timeline-axis"),
	ScrollTimelineName: atom!("scroll-timeline-name"),
	TimelineScope: atom!("timeline-scope"),
	ViewTimeline: atom!("view-timeline"),
	ViewTimelineAxis: atom!("view-timeline-axis"),
	ViewTimelineInset: atom!("view-timeline-inset"),
	ViewTimelineName: atom!("view-timeline-name"),

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

	// https://drafts.fxtf.org/compositing-2/#property-index
	BackgroundBlendMode: atom!("background-blend-mode"),
	Isolation: atom!("isolation"),
	MixBlendMode: atom!("mix-blend-mode"),

	// https://drafts.fxtf.org/css-masking-1/#property-index
	Clip: atom!("clip"),
	ClipPath: atom!("clip-path"),
	ClipRule: atom!("clip-rule"),
	Mask: atom!("mask"),
	MaskBorder: atom!("mask-border"),
	MaskBorderMode: atom!("mask-border-mode"),
	MaskBorderOutset: atom!("mask-border-outset"),
	MaskBorderRepeat: atom!("mask-border-repeat"),
	MaskBorderSlice: atom!("mask-border-slice"),
	MaskBorderSource: atom!("mask-border-source"),
	MaskBorderWidth: atom!("mask-border-width"),
	MaskClip: atom!("mask-clip"),
	MaskImage: atom!("mask-image"),
	MaskMode: atom!("mask-mode"),
	MaskOrigin: atom!("mask-origin"),
	MaskPosition: atom!("mask-position"),
	MaskRepeat: atom!("mask-repeat"),
	MaskSize: atom!("mask-size"),
	MaskType: atom!("mask-type"),

	// https://drafts.fxtf.org/filter-effects/#property-index
	ColorInterpolationFilters: atom!("color-interpolation-filters"),
	Filter: atom!("filter"),
	FloodColor: atom!("flood-color"),
	FloodOpacity: atom!("flood-opacity"),
	LightingColor: atom!("lighting-color"),

	// https://drafts.fxtf.org/filter-effects-2/
	BackdropFilter: atom!("backdrop-filter"),

	// https://drafts.fxtf.org/fill-stroke/#property-index
	Fill: atom!("fill"),
	FillBreak: atom!("fill-break"),
	FillColor: atom!("fill-color"),
	FillImage: atom!("fill-image"),
	FillOpacity: atom!("fill-opacity"),
	FillOrigin: atom!("fill-origin"),
	FillPosition: atom!("fill-position"),
	FillRepeat: atom!("fill-repeat"),
	FillRule: atom!("fill-rule"),
	FillSize: atom!("fill-size"),
	Stroke: atom!("stroke"),
	StrokeAlign: atom!("stroke-align"),
	StrokeBreak: atom!("stroke-break"),
	StrokeColor: atom!("stroke-color"),
	StrokeDashCorner: atom!("stroke-dash-corner"),
	StrokeDashJustify: atom!("stroke-dash-justify"),
	StrokeDasharray: atom!("stroke-dasharray"),
	StrokeDashoffset: atom!("stroke-dashoffset"),
	StrokeImage: atom!("stroke-image"),
	StrokeLinecap: atom!("stroke-linecap"),
	StrokeLinejoin: atom!("stroke-linejoin"),
	StrokeMiterlimit: atom!("stroke-miterlimit"),
	StrokeOpacity: atom!("stroke-opacity"),
	StrokeOrigin: atom!("stroke-origin"),
	StrokePosition: atom!("stroke-position"),
	StrokeRepeat: atom!("stroke-repeat"),
	StrokeSize: atom!("stroke-size"),
	StrokeWidth: atom!("stroke-width"),

	// https://drafts.fxtf.org/motion-1/#property-index
	Offset: atom!("offset"),
	OffsetAnchor: atom!("offset-anchor"),
	OffsetDistance: atom!("offset-distance"),
	OffsetPath: atom!("offset-path"),
	OffsetPosition: atom!("offset-position"),
	OffsetRotate: atom!("offset-rotate"),


	// Non Standard Properties
	Zoom: atom!("zoom"),

	// Webkit NonStandards
	// WebkitTextSizeAdjust: atom!("-webkit-text-size-adjust"),
	// WebkitTextDecoration: atom!("-webkit-text-decoration"),
	// WebkitTapHighlightColor: atom!("-webkit-tap-highlight-color"),
	// WebkitTextDecorationSkipInk: atom!("-webkit-text-decoration-skip-ink"),
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<StyleProperty>(), 32);
		assert_eq!(size_of::<StyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<StyleProperty>(&allocator, "width: 1px", "width:1px");
		test_write::<StyleProperty>(&allocator, "width: min(1px, 2px)", "width:min(1px, 2px)");
	}
}
