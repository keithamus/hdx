use super::prelude::*;
use css_parse::Box;

mod features;
pub use features::*;

/// <https://drafts.csswg.org/mediaqueries-4/>
#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.media"))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Media)]
pub struct MediaRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Media)]
	pub name: T![AtKeyword],
	pub prelude: MediaQueryList<'a>,
	#[metadata(block)]
	pub block: MediaRuleBlock<'a>,
}

#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MediaRuleBlock<'a>(pub Block<'a, StyleValue<'a>, Rule<'a>, CssMetadata>);
#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MediaQueryList<'a>(pub CommaSeparated<'a, MediaQuery<'a>, 1>);

#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MediaType {
	#[atom(CssAtomSet::All)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	All(T![Ident]),
	#[atom(CssAtomSet::Print)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Print(T![Ident]),
	#[atom(CssAtomSet::Screen)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Screen(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Custom(T![Ident]),
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MediaPreCondition {
	#[atom(CssAtomSet::Not)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Not(T![Ident]),
	#[atom(CssAtomSet::Only)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Only(T![Ident]),
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MediaQuery<'a> {
	precondition: Option<MediaPreCondition>,
	media_type: Option<MediaType>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	and: Option<T![Ident]>,
	condition: Option<MediaCondition<'a>>,
}

impl<'a> Peek<'a> for MediaQuery<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::LeftParen]);
}

impl<'a> Parse<'a> for MediaQuery<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut precondition = None;
		let mut media_type = None;
		let mut and = None;
		let mut condition = None;
		if p.peek::<T!['(']>() {
			condition = Some(p.parse::<MediaCondition<'a>>()?);
			return Ok(Self { precondition, media_type, and, condition });
		}
		let c = p.peek_n(1);
		if MediaPreCondition::peek(p, c) {
			precondition = Some(p.parse::<MediaPreCondition>()?);
		} else if MediaType::peek(p, c) {
			media_type = Some(p.parse::<MediaType>()?);
		} else {
			Err(Diagnostic::new(c, Diagnostic::expected_ident))?
		}
		if p.peek::<T![Ident]>() && precondition.is_some() {
			let c: Cursor = p.peek_n(1);
			if MediaType::peek(p, c) {
				media_type = Some(p.parse::<MediaType>()?);
			} else {
				Err(Diagnostic::new(c, Diagnostic::expected_ident))?
			}
		}
		let c = p.peek_n(1);
		if c == Kind::Ident && p.equals_atom(c, &CssAtomSet::And) {
			and = Some(p.parse::<T![Ident]>()?);
			condition = Some(p.parse::<MediaCondition>()?);
		}
		Ok(Self { precondition, media_type, and, condition })
	}
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MediaCondition<'a> {
	Is(MediaFeature<'a>),
	Not(T![Ident], MediaFeature<'a>),
	And(Vec<'a, (MediaFeature<'a>, Option<T![Ident]>)>),
	Or(Vec<'a, (MediaFeature<'a>, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for MediaCondition<'a> {
	type FeatureCondition = MediaFeature<'a>;
	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: MediaFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: MediaFeature<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (MediaFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (MediaFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for MediaCondition<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

macro_rules! media_feature {
	( $($name: ident($typ: ty): $pat: pat,)+) => {
		/// <https://drafts.csswg.org/mediaqueries-5/#media-descriptor-table>
		#[node]
		#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		#[derive(csskit_derives::NodeWithMetadata)]
		pub enum MediaFeature<'a> {
			$($name($typ),)+
			#[cfg_attr(feature = "visitable", visit(skip))]
			Custom(CustomMediaFeature),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Hack(HackMediaFeature),
		}
	}
}

apply_medias!(media_feature);

impl<'a> Peek<'a> for MediaFeature<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen]);
}

impl<'a> Parse<'a> for MediaFeature<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let checkpoint = p.checkpoint();
		let mut c = p.peek_n(2);
		macro_rules! match_media {
			( $($name: ident($typ: ty): $pat: pat,)+) => {
				// Only peek at the token as the underlying media feature parser needs to parse the leading ident.
				{
					match p.to_atom::<CssAtomSet>(c) {
						$($pat => <$typ>::parse(p).map(Self::$name),)+
						_ => Err(Diagnostic::new(c, Diagnostic::expected_ident))?
					}
				}
			}
		}
		if c == Kind::Ident && c.token().is_dashed_ident() {
			return p.parse::<CustomMediaFeature>().map(Self::Custom);
		}
		if c == Kind::Ident {
			let value = apply_medias!(match_media).or_else(|err| {
				p.rewind(checkpoint);
				if let Ok(hack) = p.parse::<HackMediaFeature>() { Ok(Self::Hack(hack)) } else { Err(err) }
			})?;
			Ok(value)
		} else {
			// Styles like (1em < width < 1em), (1em <= width <= 1em) or (1/1 <= aspect-ratio <= 2/1)
			c = css_parse::peek_ranged_feature_name(p);
			if c != Kind::Ident {
				c = p.next();
				Err(Diagnostic::new(c, Diagnostic::unexpected))?
			}
			apply_medias!(match_media)
		}
	}
}

macro_rules! apply_medias {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/mediaqueries/#media-descriptor-table

			AnyHover(AnyHoverMediaFeature): CssAtomSet::AnyHover,
			AnyPointer(AnyPointerMediaFeature): CssAtomSet::AnyPointer,
			AspectRatio(Box<'a, AspectRatioMediaFeature<'a>>): CssAtomSet::AspectRatio | CssAtomSet::MinAspectRatio | CssAtomSet::MaxAspectRatio,
			Color(ColorMediaFeature<'a>): CssAtomSet::Color | CssAtomSet::MaxColor | CssAtomSet::MinColor,
			ColorGamut(ColorGamutMediaFeature): CssAtomSet::ColorGamut,
			ColorIndex(ColorIndexMediaFeature<'a>): CssAtomSet::ColorIndex | CssAtomSet::MaxColorIndex | CssAtomSet::MinColorIndex,
			DeviceAspectRatio(Box<'a, DeviceAspectRatioMediaFeature<'a>>): CssAtomSet::DeviceAspectRatio | CssAtomSet::MaxDeviceAspectRatio | CssAtomSet::MinDeviceAspectRatio,
			DeviceHeight(DeviceHeightMediaFeature<'a>): CssAtomSet::DeviceHeight | CssAtomSet::MaxDeviceHeight | CssAtomSet::MinDeviceHeight,
			DeviceWidth(DeviceWidthMediaFeature<'a>): CssAtomSet::DeviceWidth | CssAtomSet::MaxDeviceWidth | CssAtomSet::MinDeviceWidth,
			DisplayMode(DisplayModeMediaFeature): CssAtomSet::DisplayMode,
			DynamicRange(DynamicRangeMediaFeature): CssAtomSet::DynamicRange,
			EnvironmentBlending(EnvironmentBlendingMediaFeature): CssAtomSet::EnvironmentBlending,
			ForcedColors(ForcedColorsMediaFeature): CssAtomSet::ForcedColors,
			Grid(GridMediaFeature): CssAtomSet::Grid,
			Height(HeightMediaFeature<'a>): CssAtomSet::Height | CssAtomSet::MaxHeight | CssAtomSet::MinHeight,
			HorizontalViewportSegments(HorizontalViewportSegmentsMediaFeature<'a>): CssAtomSet::HorizontalViewportSegments | CssAtomSet::MaxHorizontalViewportSegments | CssAtomSet::MinHorizontalViewportSegments,
			Hover(HoverMediaFeature): CssAtomSet::Hover,
			InvertedColors(InvertedColorsMediaFeature): CssAtomSet::InvertedColors,
			Monochrome(MonochromeMediaFeature<'a>): CssAtomSet::Monochrome | CssAtomSet::MaxMonochrome | CssAtomSet::MinMonochrome,
			NavControls(NavControlsMediaFeature): CssAtomSet::NavControls,
			Orientation(OrientationMediaFeature): CssAtomSet::Orientation,
			OverflowBlock(OverflowBlockMediaFeature): CssAtomSet::OverflowBlock,
			OverflowInline(OverflowInlineMediaFeature): CssAtomSet::OverflowInline,
			Pointer(PointerMediaFeature): CssAtomSet::Pointer,
			PrefersColorScheme(PrefersColorSchemeMediaFeature): CssAtomSet::PrefersColorScheme,
			PrefersContrast(PrefersContrastMediaFeature): CssAtomSet::PrefersContrast,
			PrefersReducedData(PrefersReducedDataMediaFeature): CssAtomSet::PrefersReducedData,
			PrefersReducedMotion(PrefersReducedMotionMediaFeature): CssAtomSet::PrefersReducedMotion,
			PrefersReducedTransparency(PrefersReducedTransparencyMediaFeature): CssAtomSet::PrefersReducedTransparency,
			Resolution(ResolutionMediaFeature<'a>): CssAtomSet::Resolution | CssAtomSet::MaxResolution | CssAtomSet::MinResolution,
			Scan(ScanMediaFeature): CssAtomSet::Scan,
			Scripting(ScriptingMediaFeature): CssAtomSet::Scripting,
			Update(UpdateMediaFeature): CssAtomSet::Update,
			VerticalViewportSegments(VerticalViewportSegmentsMediaFeature<'a>): CssAtomSet::VerticalViewportSegments | CssAtomSet::MaxVerticalViewportSegments | CssAtomSet::MinVerticalViewportSegments,
			VideoColorGamut(VideoColorGamutMediaFeature): CssAtomSet::VideoColorGamut,
			VideoDynamicRange(VideoDynamicRangeMediaFeature): CssAtomSet::VideoDynamicRange,
			Width(WidthMediaFeature<'a>): CssAtomSet::Width | CssAtomSet::MaxWidth | CssAtomSet::MinWidth,

			// https://searchfox.org/wubkat/source/Source/WebCore/css/query/MediaQueryFeatures.cpp#192
			WebkitAnimationMediaFeature(WebkitAnimationMediaFeature): CssAtomSet::_WebkitAnimation,
			WebkitDevicePixelRatioMediaFeature(WebkitDevicePixelRatioMediaFeature<'a>): CssAtomSet::_WebkitDevicePixelRatio,
			WebkitTransform2dMediaFeature(WebkitTransform2dMediaFeature): CssAtomSet::_WebkitTransform2d,
			WebkitTransform3dMediaFeature(WebkitTransform3dMediaFeature): CssAtomSet::_WebkitTransform3d,
			WebkitTransitionMediaFeature(WebkitTransitionMediaFeature): CssAtomSet::_WebkitTransition,
			WebkitVideoPlayableInlineMediaFeature(WebkitVideoPlayableInlineMediaFeature): CssAtomSet::_WebkitVideoPlayableInline,

			// https://searchfox.org/mozilla-central/source/servo/components/style/gecko/media_features.rs#744
			MozDeviceOrientationMediaFeature(MozDeviceOrientationMediaFeature): CssAtomSet::_MozDeviceOrientation,
			MozDevicePixelRatioMediaFeature(MozDevicePixelRatioMediaFeature<'a>): CssAtomSet::_MozDevicePixelRatio | CssAtomSet::_MozMaxDevicePixelRatio | CssAtomSet::_MozMinDevicePixelRatio,
			MozMacGraphiteThemeMediaFeature(MozMacGraphiteThemeMediaFeature): CssAtomSet::_MozMacGraphiteTheme,
			MozMaemoClassicMediaFeature(MozMaemoClassicMediaFeature): CssAtomSet::_MozMaemoClassicTheme,
			MozImagesInMenusMediaFeature(MozImagesInMenusMediaFeature): CssAtomSet::_MozImagesInMenus,
			MozOsVersionMenusMediaFeature(MozOsVersionMediaFeature): CssAtomSet::_MozOsVersion,

			// https://github.com/search?q=%2F%5C(-ms-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			MsHighContrastMediaFeature(MsHighContrastMediaFeature): CssAtomSet::_MsHighContrast,
			MsViewStateMediaFeature(MsViewStateMediaFeature): CssAtomSet::_MsViewState,
			MsImeAlignMediaFeature(MsImeAlignMediaFeature): CssAtomSet::_MsImeAlign,
			MsDevicePixelRatioMediaFeature(MsDevicePixelRatioMediaFeature<'a>): CssAtomSet::_MsDevicePixelRatio,
			MsColumnCountMediaFeature(MsColumnCountMediaFeature<'a>): CssAtomSet::_MsColumnCount,

			// https://github.com/search?q=%2F%5C(-o-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			ODevicePixelRatioMediaFeature(ODevicePixelRatioMediaFeature<'a>): CssAtomSet::_ODevicePixelRatio,
		}
	};
}
use apply_medias;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"print",
			MediaQuery { precondition: None, media_type: Some(MediaType::Print(_)), and: None, condition: None }
		);
		assert_parse!(CssAtomSet::ATOMS, MediaQueryList, "print, tv");
		assert_parse!(CssAtomSet::ATOMS, MediaQueryList, "(--modern)");
		assert_parse!(CssAtomSet::ATOMS, MediaQueryList, "screen and (--modern)");
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"not embossed",
			MediaQuery {
				precondition: Some(MediaPreCondition::Not(_)),
				media_type: Some(MediaType::Custom(_)),
				and: None,
				condition: None
			}
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"only screen",
			MediaQuery {
				precondition: Some(MediaPreCondition::Only(_)),
				media_type: Some(MediaType::Screen(_)),
				and: None,
				condition: None
			}
		);
		assert_parse!(CssAtomSet::ATOMS, MediaFeature, "(grid)", MediaFeature::Grid(_));
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"screen and (grid)",
			MediaQuery {
				precondition: None,
				media_type: Some(MediaType::Screen(_)),
				and: Some(_),
				condition: Some(MediaCondition::Is(MediaFeature::Grid(_))),
			}
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"screen and (hover)and (pointer)",
			MediaQuery {
				precondition: None,
				media_type: Some(MediaType::Screen(_)),
				and: Some(_),
				condition: Some(MediaCondition::And(_))
			}
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"screen and (orientation:landscape)",
			MediaQuery {
				precondition: None,
				media_type: Some(MediaType::Screen(_)),
				and: Some(_),
				condition: Some(MediaCondition::Is(MediaFeature::Orientation(_))),
			}
		);
		assert_parse!(CssAtomSet::ATOMS, MediaQuery, "(hover)and (pointer)");
		assert_parse!(CssAtomSet::ATOMS, MediaQuery, "(hover)or (pointer)");
		assert_parse!(CssAtomSet::ATOMS, MediaFeature, "(aspect-ratio:16/9)", MediaFeature::AspectRatio(_));
		assert_parse!(CssAtomSet::ATOMS, MediaFeature, "(1/1<aspect-ratio<16/9)", MediaFeature::AspectRatio(_));
		assert_parse!(CssAtomSet::ATOMS, MediaFeature, "(1/1<=aspect-ratio<=16/9)", MediaFeature::AspectRatio(_));
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaFeature,
			"(min-device-aspect-ratio:16/9)",
			MediaFeature::DeviceAspectRatio(_)
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaFeature,
			"(min-resolution:2dppx)",
			MediaFeature::Resolution(ResolutionMediaFeature::Min(..))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaFeature,
			"(2dppx<resolution)",
			MediaFeature::Resolution(ResolutionMediaFeature::Right(..))
		);
		assert_parse!(CssAtomSet::ATOMS, MediaFeature, "(update:fast)", MediaFeature::Update(_));
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(aspect-ratio:16/9){a{color:red}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(update:none)and (resolution>=2x){}");
		// assert_parse!(CssAtomSet::ATOMS, MediaQuery, "not ((width: 2px) or (width: 3px))");
		// assert_parse!(CssAtomSet::ATOMS, MediaQuery, "not ((hover) or (pointer))");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media print{}");
		// assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media print,(prefers-reduced-motion: reduce){}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:1200px){}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(max-width:calc(48rem - 0.02px)){}");
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaFeature,
			"(calc(1px + 2px)<width<=max(40em,50vw))",
			MediaFeature::Width(_)
		);
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:1200px){body{color:red;}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:1200px){@page{}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media screen{body{color:black}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(max-width:575.98px)and (prefers-reduced-motion:reduce){}");
		// assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media only screen and(max-device-width:800px),only screen and (device-width:1024px) and (device-height: 600px),only screen and (width:1280px) and (orientation:landscape), only screen and (device-width:800px), only screen and (max-width:767px) {}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(grid){a{padding:4px}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:0){background:white}");
		// assert_parse!(
		// 	MediaRule,
		// 	"@media(grid){a{color-scheme:light}}",
		// 	"@media (grid: 0) {\n\ta {\n\t\tcolor-scheme: light;\n\t}\n}"
		// );

		// IE media hack
		// assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media (min-width: 0\\0) {\n\n}");
	}

	// #[test]
	// fn test_errors() {
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "(hover) and or (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "(pointer) or and (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "(pointer) not and (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "only and (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "not and (pointer)");
	// }

	#[test]
	#[cfg(feature = "visitable")]
	fn test_media_rule_visits() {
		use crate::assert_visits;
		assert_visits!(
			"@media (min-width: 768px) { body { color: red; } }",
			MediaRule,
			MediaCondition,
			WidthMediaFeature,
			Length,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			StyleValue,
			ColorStyleValue,
			Color
		);
		assert_visits!(
			"@media screen and (min-width: 768px) { body { color: red; } }",
			MediaRule,
			MediaType,
			MediaCondition,
			WidthMediaFeature,
			Length,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			StyleValue,
			ColorStyleValue,
			Color
		);
		assert_visits!(
			"@media only screen { body { color: red; } }",
			MediaRule,
			MediaPreCondition,
			MediaType,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			StyleValue,
			ColorStyleValue,
			Color
		);
	}
}
