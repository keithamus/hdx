use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::{Cursor, Span};
use hdx_parser::{
	diagnostics, keyword_typedef, AtRule, Build, ConditionalAtRule, CursorSink, Is, Parse, Parser, PreludeList,
	Result as ParserResult, RuleList, ToCursors, T,
};

use crate::css::{stylesheet::Rule, Visit, Visitable};

mod features;
use features::*;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(And, atom!("and"));
	custom_keyword!(Not, atom!("not"));
	custom_keyword!(Only, atom!("only"));
}

// https://drafts.csswg.org/mediaqueries-4/
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MediaRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub query: MediaQueryList<'a>,
	pub block: MediaRules<'a>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for MediaRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, query, block) = Self::parse_at_rule(p, Some(atom!("media")))?;
		if let Some(query) = query {
			Ok(Self { at_keyword, query, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for MediaRule<'a> {
	type Prelude = MediaQueryList<'a>;
	type Block = MediaRules<'a>;
}

impl<'a> ToCursors for MediaRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.query, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for MediaRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		todo!();
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaRules<'a> {
	pub open: T!['{'],
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for MediaRules<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for MediaRules<'a> {
	type Rule = Rule<'a>;
}

impl<'a> ToCursors for MediaRules<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaQueryList<'a>(pub Vec<'a, MediaQuery<'a>>);

impl<'a> PreludeList<'a> for MediaQueryList<'a> {
	type PreludeItem = MediaQuery<'a>;
}

impl<'a> Parse<'a> for MediaQueryList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> ToCursors for MediaQueryList<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for query in &self.0 {
			ToCursors::to_cursors(query, s);
		}
	}
}

keyword_typedef!(MediaPreCondition { Not: atom!("not"), Only: atom!("only") });

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum MediaType {
	All(T![Ident]),
	Print(T![Ident]),
	Screen(T![Ident]),
	Custom(T![Ident]),
}

impl<'a> Is<'a> for MediaType {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::is(p, c)
	}
}

impl<'a> Build<'a> for MediaType {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("all") => Self::All(<T![Ident]>::build(p, c)),
			atom!("print") => Self::All(<T![Ident]>::build(p, c)),
			atom!("screen") => Self::All(<T![Ident]>::build(p, c)),
			atom!("only") | atom!("not") | atom!("and") | atom!("or") | atom!("layer") => unreachable!(),
			_ => Self::Custom(<T![Ident]>::build(p, c)),
		}
	}
}

impl From<MediaType> for Cursor {
	fn from(value: MediaType) -> Cursor {
		match value {
			MediaType::All(c) => c.into(),
			MediaType::Print(c) => c.into(),
			MediaType::Screen(c) => c.into(),
			MediaType::Custom(c) => c.into(),
		}
	}
}

impl From<&MediaType> for Cursor {
	fn from(value: &MediaType) -> Cursor {
		(*value).into()
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaQuery<'a> {
	precondition: Option<MediaPreCondition>,
	media_type: Option<MediaType>,
	condition: Option<MediaCondition<'a>>,
}

impl<'a> Parse<'a> for MediaQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut precondition = None;
		let mut media_type = None;
		let mut condition = None;
		if p.peek::<T!['(']>() {
			condition = Some(p.parse::<MediaCondition<'a>>()?);
			return Ok(Self { precondition, media_type, condition });
		}
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		if MediaPreCondition::is(p, c) {
			precondition = Some(MediaPreCondition::build(p, c));
		} else if MediaType::is(p, c) {
			media_type = Some(MediaType::build(p, c));
		} else {
			Err(diagnostics::UnexpectedIdent(p.parse_atom(c), c.into()))?
		}
		if p.peek::<T![Ident]>() && precondition.is_some() {
			let ident = p.parse::<T![Ident]>()?;
			let c: Cursor = ident.into();
			if MediaType::is(p, c) {
				media_type = Some(MediaType::build(p, c));
			} else {
				Err(diagnostics::UnexpectedIdent(p.parse_atom(c), c.into()))?
			}
		}
		if p.peek::<kw::And>() || media_type.is_none() {
			condition = Some(p.parse::<MediaCondition>()?);
		}
		Ok(Self { precondition, media_type, condition })
	}
}

impl<'a> ToCursors for MediaQuery<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(precondition) = &self.precondition {
			ToCursors::to_cursors(precondition, s);
		}
		if let Some(media_type) = &self.media_type {
			s.append(media_type.into());
		}
		if let Some(condition) = &self.condition {
			ToCursors::to_cursors(condition, s);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum MediaCondition<'a> {
	Is(MediaFeature),
	Not(Box<MediaCondition<'a>>),
	And(Vec<'a, MediaFeature>),
	Or(Vec<'a, MediaFeature>),
}

impl<'a> ConditionalAtRule<'a> for MediaCondition<'a> {
	type Feature = MediaFeature;
	fn new_is(feature: MediaFeature) -> Self {
		Self::Is(feature)
	}
	fn new_not(condition: MediaCondition<'a>) -> Self {
		Self::Not(Box::new(condition))
	}
	fn new_and(feature: Vec<'a, MediaFeature>) -> Self {
		Self::And(feature)
	}
	fn new_or(feature: Vec<'a, MediaFeature>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for MediaCondition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

impl<'a> ToCursors for MediaCondition<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Is(c) => ToCursors::to_cursors(c, s),
			Self::Not(c) => ToCursors::to_cursors(c.as_ref(), s),
			Self::And(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
			Self::Or(cs) => {
				for c in cs {
					ToCursors::to_cursors(c, s);
				}
			}
		}
	}
}

macro_rules! media_feature {
	( $($name: ident($typ: ident): atom!($atom: tt)$(| $alts:pat)*,)+) => {
		// https://drafts.csswg.org/mediaqueries-5/#media-descriptor-table
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum MediaFeature {
			$($name($typ),)+
			Hack(HackMediaFeature),
		}
	}
}

apply_medias!(media_feature);

impl<'a> Parse<'a> for MediaFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<T![LeftParen]>()?;
		let checkpoint = p.checkpoint();
		if p.peek::<T![Ident]>() {
			let c = p.peek_n(1);
			macro_rules! match_media {
				( $($name: ident($typ: ident): atom!($atom: tt)$(| $alts:pat)*,)+) => {
					// Only peek at the token as the underlying media feature parser needs to parse the leading atom.
					{
						match p.parse_atom_lower(c) {
							$(atom!($atom)$(| $alts)* => $typ::parse(p).map(Self::$name),)+
							atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
						}
					}
				}
			}
			let value = apply_medias!(match_media).or_else(|err| {
				p.rewind(checkpoint);
				if let Ok(hack) = p.parse::<HackMediaFeature>() {
					Ok(Self::Hack(hack))
				} else {
					Err(err)
				}
			})?;
			p.parse::<T![')']>()?;
			Ok(value)
		} else {
			let c: Cursor = p.parse::<T![Any]>()?.into();
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
	}
}

impl<'a> ToCursors for MediaFeature {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		macro_rules! match_media {
			( $($name: ident($typ: ident): atom!($atom: tt)$(| $alts:pat)*,)+) => {
				match self {
					$(Self::$name(c) => ToCursors::to_cursors(c, s),)+
					Self::Hack(hack) => ToCursors::to_cursors(hack, s),
				}
			};
		}
		apply_medias!(match_media)
	}
}

macro_rules! apply_medias {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/mediaqueries/#media-descriptor-table

			AnyHover(AnyHoverMediaFeature): atom!("any-hover"),
			AnyPointer(AnyPointerMediaFeature): atom!("any-pointer"),
			AspectRatio(AspectRatioMediaFeature): atom!("aspect-ratio") | atom!("max-aspect-ratio") | atom!("min-aspect-ratio"),
			Color(ColorMediaFeature): atom!("color") | atom!("max-color") | atom!("min-color"),
			ColorGamut(ColorGamutMediaFeature): atom!("color-gamut"),
			ColorIndex(ColorIndexMediaFeature): atom!("color-index") | atom!("max-color-index") | atom!("min-color-index"),
			DeviceAspectRatio(DeviceAspectRatioMediaFeature): atom!("device-aspect-ratio") | atom!("max-device-aspect-ratio") | atom!("min-device-aspect-ratio"),
			DeviceHeight(DeviceHeightMediaFeature): atom!("device-height") | atom!("max-device-height") | atom!("min-device-height"),
			DeviceWidth(DeviceWidthMediaFeature): atom!("device-width") | atom!("max-device-width") | atom!("min-device-width"),
			DisplayMode(DisplayModeMediaFeature): atom!("display-mode"),
			DynamicRange(DynamicRangeMediaFeature): atom!("dynamic-range"),
			EnvironmentBlending(EnvironmentBlendingMediaFeature): atom!("environment-blending"),
			ForcedColors(ForcedColorsMediaFeature): atom!("forced-colors"),
			Grid(GridMediaFeature): atom!("grid"),
			Height(HeightMediaFeature): atom!("height") | atom!("max-height") | atom!("min-height"),
			HorizontalViewportSegments(HorizontalViewportSegmentsMediaFeature): atom!("horizontal-viewport-segments") | atom!("max-horizontal-viewport-segments") | atom!("min-horizontal-viewport-segments"),
			Hover(HoverMediaFeature): atom!("hover"),
			InvertedColors(InvertedColorsMediaFeature): atom!("inverted-colors"),
			Monochrome(MonochromeMediaFeature): atom!("monochrome") | atom!("max-monochrome") | atom!("min-monochrome"),
			NavControls(NavControlsMediaFeature): atom!("nav-controls"),
			Orientation(OrientationMediaFeature): atom!("orientation"),
			OverflowBlock(OverflowBlockMediaFeature): atom!("overflow-block"),
			OverflowInline(OverflowInlineMediaFeature): atom!("overflow-inline"),
			Pointer(PointerMediaFeature): atom!("pointer"),
			PrefersColorScheme(PrefersColorSchemeMediaFeature): atom!("prefers-color-scheme"),
			PrefersContrast(PrefersContrastMediaFeature): atom!("prefers-contrast"),
			PrefersReducedData(PrefersReducedDataMediaFeature): atom!("prefers-reduced-data"),
			PrefersReducedMotion(PrefersReducedMotionMediaFeature): atom!("prefers-reduced-motion"),
			PrefersReducedTransparency(PrefersReducedTransparencyMediaFeature): atom!("prefers-reduced-transparency"),
			Resolution(ResolutionMediaFeature): atom!("resolution") | atom!("max-resolution") | atom!("min-resolution"),
			Scan(ScanMediaFeature): atom!("scan"),
			Scripting(ScriptingMediaFeature): atom!("scripting"),
			Update(UpdateMediaFeature): atom!("update"),
			VerticalViewportSegments(VerticalViewportSegmentsMediaFeature): atom!("vertical-viewport-segments") | atom!("max-vertical-viewport-segments") | atom!("min-vertical-viewport-segments"),
			VideoColorGamut(VideoColorGamutMediaFeature): atom!("video-color-gamut"),
			VideoDynamicRange(VideoDynamicRangeMediaFeature): atom!("video-dynamic-range"),
			Width(WidthMediaFeature): atom!("width") | atom!("max-width") | atom!("min-width"),

			// https://searchfox.org/wubkat/source/Source/WebCore/css/query/MediaQueryFeatures.cpp#192
			WebkitAnimationMediaFeature(WebkitAnimationMediaFeature): atom!("-webkit-animation"),
			WebkitDevicePixelRatioMediaFeature(WebkitDevicePixelRatioMediaFeature): atom!("-webkit-device-pixel-ratio"),
			WebkitTransform2dMediaFeature(WebkitTransform2dMediaFeature): atom!("-webkit-transform-2d"),
			WebkitTransform3dMediaFeature(WebkitTransform3dMediaFeature): atom!("-webkit-transform-3d"),
			WebkitTransitionMediaFeature(WebkitTransitionMediaFeature): atom!("-webkit-transition"),
			WebkitVideoPlayableInlineMediaFeature(WebkitVideoPlayableInlineMediaFeature): atom!("-webkit-video-playable-inline"),

			// https://searchfox.org/mozilla-central/source/servo/components/style/gecko/media_features.rs#744
			MozDeviceOrientationMediaFeature(MozDeviceOrientationMediaFeature): atom!("-moz-device-orientation"),
			MozDevicePixelRatioMediaFeature(MozDevicePixelRatioMediaFeature): atom!("-moz-device-pixel-ratio") | atom!("max--moz-device-pixel-ratio") | atom!("min--moz-device-pixel-ratio"),
			MozMacGraphiteThemeMediaFeature(MozDevicePixelRatioMediaFeature): atom!("-moz-mac-graphite-theme"),
			MozMaemoClassicMediaFeature(MozDevicePixelRatioMediaFeature): atom!("-moz-maemo-classic"),
			MozImagesInMenusMediaFeature(MozDevicePixelRatioMediaFeature): atom!("-moz-images-in-menus"),
			MozOsVersionMenusMediaFeature(MozDevicePixelRatioMediaFeature): atom!("-moz-os-version"),

			// https://github.com/search?q=%2F%5C(-ms-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			MsHighContrastMediaFeature(MsHighContrastMediaFeature): atom!("-ms-high-contrast"),
			MsViewStateMediaFeature(MsViewStateMediaFeature): atom!("-ms-view-state"),
			MsImeAlignMediaFeature(MsImeAlignMediaFeature): atom!("-ms-ime-align"),
			MsDevicePixelRatioMediaFeature(MsDevicePixelRatioMediaFeature): atom!("-ms-device-pixel-ratio"),
			MsColumnCountMediaFeature(MsColumnCountMediaFeature): atom!("-ms-column-count"),

			// https://github.com/search?q=%2F%5C(-o-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			ODevicePixelRatioMediaFeature(ODevicePixelRatioMediaFeature): atom!("-o-device-pixel-ratio"),
		}
	};
}
use apply_medias;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MediaRule, 112);
		assert_size!(MediaQueryList, 32);
		assert_size!(MediaQuery, 136);
		assert_size!(MediaCondition, 104);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MediaQuery, "print");
		assert_parse!(MediaQuery, "not embossed");
		assert_parse!(MediaQuery, "only screen");
		// assert_parse!(MediaFeature, "(grid)", "grid");
		// assert_parse!(MediaQuery, "screen and (grid)");
		// assert_parse!(MediaQuery, "screen and (hover) and (pointer)");
		// assert_parse!(MediaQuery, "screen and (orientation: landscape)");
		// assert_parse!(MediaQuery, "(hover) and (pointer)");
		// assert_parse!(MediaQuery, "(hover) or (pointer)");
		// assert_parse!(MediaQuery, "not ((width: 2px) or (width: 3px))");
		// assert_parse!(MediaQuery, "not ((hover) or (pointer))");
		// assert_parse!(MediaQuery, "only (hover) or (pointer)");
		// assert_parse!(MediaRule, "@media print {\n\n}");
		// assert_parse!(MediaRule, "@media print, (prefers-reduced-motion: reduce) {\n\n}");
		// assert_parse!(MediaRule, "@media (min-width: 1200px) {\n\n}");
		// assert_parse!(MediaRule, "@media (min-width: 1200px) {\n\tbody {\n\t\tcolor: red;\n\t}\n}");
		// assert_parse!(MediaRule, "@media (min-width: 1200px) {\n@page {\n}\n}");
		// assert_parse!(MediaRule, "@media (max-width: 575.98px) and (prefers-reduced-motion: reduce) {\n\n}");
		// assert_parse!(MediaRule, "@media only screen and (max-device-width: 800px), only screen and (device-width: 1024px) and (device-height: 600px), only screen and (width: 1280px) and (orientation: landscape), only screen and (device-width: 800px), only screen and (max-width: 767px) {\n\n}");
		// assert_parse!(MediaRule, "@media(grid){a{padding:4px}}", "@media (grid: 0) {\n\ta {\n\t\tpadding: 4px;\n\t}\n}");
		// assert_parse!(
		// 	MediaRule,
		// 	"@media(grid){a{color-scheme:light}}",
		// 	"@media (grid: 0) {\n\ta {\n\t\tcolor-scheme: light;\n\t}\n}"
		// );

		// IE media hack
		// assert_parse!(MediaRule, "@media (min-width: 0\\0) {\n\n}");
	}

	// #[test]
	// fn test_errors() {
	// 	assert_parse_error!(MediaQuery, "(hover) and or (pointer)");
	// 	assert_parse_error!(MediaQuery, "(pointer) or and (pointer)");
	// 	assert_parse_error!(MediaQuery, "(pointer) not and (pointer)");
	// 	assert_parse_error!(MediaQuery, "only and (pointer)");
	// 	assert_parse_error!(MediaQuery, "not and (pointer)");
	// }
}
