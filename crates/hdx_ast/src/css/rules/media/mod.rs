use smallvec::{smallvec, SmallVec};

use hdx_atom::{atom, Atom, Atomizable};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, discard, expect, expect_ignore_case, match_ignore_case, peek, todo, unexpected, unexpected_ident,
	AtRule, Parse, Parser, Result as ParserResult, RuleList, Spanned, Vec,
};
use hdx_writer::{write_css, write_list, CssWriter, OutputOption, Result as WriterResult, WriteCss};

use crate::css::stylesheet::Rule;

mod features;
use features::*;

// https://drafts.csswg.org/mediaqueries-4/
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MediaRule<'a> {
	pub query: Spanned<MediaQueryList>,
	pub rules: Spanned<MediaRules<'a>>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for MediaRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(parser.next(), Token::AtKeyword(atom!("media")));
		let span = parser.span();
		match Self::parse_at_rule(parser)? {
			(Some(query), Some(rules)) => Ok(Self { query, rules }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
		}
	}
}

impl<'a> AtRule<'a> for MediaRule<'a> {
	type Prelude = MediaQueryList;
	type Block = MediaRules<'a>;
}

impl<'a> WriteCss<'a> for MediaRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		if !sink.can_output(OutputOption::RedundantRules) && self.rules.node.0.is_empty() {
			return Ok(());
		}
		write_css!(sink, '@', atom!("media"));
		if matches!(self.query.node.0.first(), Some(Spanned { node: MediaQuery::Condition(_), .. })) {
			sink.write_whitespace()?;
		} else {
			sink.write_char(' ')?;
		}
		self.query.write_css(sink)?;
		sink.write_whitespace()?;
		sink.write_char('{')?;
		sink.write_newline()?;
		sink.indent();
		self.rules.write_css(sink)?;
		sink.write_newline()?;
		sink.dedent();
		sink.write_char('}')?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaRules<'a>(pub Vec<'a, Spanned<Rule<'a>>>);

impl<'a> Parse<'a> for MediaRules<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_rule_list(parser)?))
	}
}

impl<'a> RuleList<'a> for MediaRules<'a> {
	type Rule = Rule<'a>;
}

impl<'a> WriteCss<'a> for MediaRules<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut rules = self.0.iter().peekable();
		while let Some(rule) = rules.next() {
			rule.write_css(sink)?;
			if rules.peek().is_some() {
				sink.write_newline()?;
			}
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaQueryList(pub SmallVec<[Spanned<MediaQuery>; 1]>);

impl<'a> Parse<'a> for MediaQueryList {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut queries = smallvec![];
		loop {
			queries.push(MediaQuery::parse_spanned(parser)?);
			if !discard!(parser, Token::Comma) {
				return Ok(Self(queries));
			}
		}
	}
}

impl<'a> WriteCss<'a> for MediaQueryList {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_list!(sink, self.0,);
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
pub enum MediaQuery {
	Condition(MediaCondition),
	Typed(MediaType),
	NotTyped(MediaType),
	OnlyTyped(MediaType),
	TypedCondition(MediaType, MediaCondition),
	NotTypedCondition(MediaType, MediaCondition),
	OnlyTypedCondition(MediaType, MediaCondition),
}

impl<'a> Parse<'a> for MediaQuery {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut not = false;
		let mut only = false;
		let mut media_type = None;
		if peek!(parser, Token::LeftParen) {
			return Ok(Self::Condition(MediaCondition::parse(parser)?));
		}
		expect_ignore_case! { parser.next(), Token::Ident(_):
			atom!("not") => {
				not = true;
			},
			atom!("only") => {
				only = true;
			},
			atom => {
				if let Some(ty) = MediaType::from_atom(&atom) {
					media_type = Some(ty);
				} else {
					unexpected_ident!(parser, atom);
				}
			}
		}
		match parser.peek() {
			Token::Ident(ident) if only || not => {
				if let Some(ty) = MediaType::from_atom(ident) {
					parser.advance();
					media_type = Some(ty);
				} else {
					unexpected_ident!(parser, ident)
				}
			}
			Token::Ident(ident) if media_type.is_some() && matches!(ident.to_ascii_lowercase(), atom!("and")) => {
				// Must not advance because we need "and" to be consumed by MediaCondition
				return Ok(Self::TypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?));
			}
			token => {
				if let Some(mt) = media_type {
					return Ok(Self::Typed(mt));
				} else {
					unexpected!(parser, token)
				}
			}
		}
		match parser.next() {
			Token::Ident(ident) if matches!(ident.to_ascii_lowercase(), atom!("and")) => {
				// Must not advance because we need "and" to be consumed by MediaCondition
				if only {
					Ok(Self::OnlyTypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?))
				} else if not {
					Ok(Self::NotTypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?))
				} else {
					unexpected_ident!(parser, ident)
				}
			}
			_ if only => Ok(Self::OnlyTyped(media_type.unwrap())),
			_ if not => Ok(Self::NotTyped(media_type.unwrap())),
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for MediaQuery {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Condition(mc) => mc.write_css(sink)?,
			Self::Typed(mt) => mt.write_css(sink)?,
			Self::NotTyped(mt) => write_css!(sink, atom!("not"), (), mt),
			Self::OnlyTyped(mt) => write_css!(sink, atom!("only"), (), mt),
			Self::TypedCondition(mt, mc) => write_css!(sink, mt, (), mc),
			Self::NotTypedCondition(mt, mc) => write_css!(sink, atom!("not"), ' ', mt, (), mc),
			Self::OnlyTypedCondition(mt, mc) => write_css!(sink, atom!("only"), ' ', mt, (), mc),
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum MediaCondition {
	Is(MediaFeature),
	Not(MediaFeature),
	And(SmallVec<[MediaFeature; 4]>),
	Or(SmallVec<[MediaFeature; 4]>),
}

impl<'a> Parse<'a> for MediaCondition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if matches!(parser.peek(), Token::LeftParen) {
			if peek!(parser, 2, Token::LeftParen) {
				todo!(parser)
			} else {
				return Ok(Self::Is(MediaFeature::parse(parser)?));
			}
		}
		expect_ignore_case! { parser.peek(), Token::Ident(_):
			atom!("and") => {
				let mut features = smallvec![];
				loop {
					expect_ignore_case!(parser.next(), Token::Ident(atom!("and")));
					features.push(MediaFeature::parse(parser)?);
					if !match_ignore_case!(parser.peek(), Token::Ident(atom!("and"))) {
						return Ok(Self::And(features));
					}
				}
			},
			atom!("or") => {
				let mut features = smallvec![];
				loop {
					expect_ignore_case!(parser.next(), Token::Ident(atom!("or")));
					features.push(MediaFeature::parse(parser)?);
					if !match_ignore_case!(parser.peek(), Token::Ident(atom!("or"))) {
						return Ok(Self::And(features));
					}
				}
			},
			atom!("not") => {
				parser.advance();
				Ok(Self::Not(MediaFeature::parse(parser)?))
			},
		}
	}
}

impl<'a> WriteCss<'a> for MediaCondition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Is(feature) => feature.write_css(sink),
			Self::Not(feature) => {
				atom!("not").write_css(sink)?;
				sink.write_whitespace()?;
				feature.write_css(sink)
			}
			Self::And(features) => {
				let mut iter = features.iter().peekable();
				while let Some(feature) = iter.next() {
					atom!("and").write_css(sink)?;
					sink.write_whitespace()?;
					feature.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_whitespace()?;
					}
				}
				Ok(())
			}
			Self::Or(features) => {
				for feature in features.iter() {
					sink.write_char(' ')?;
					atom!("or").write_css(sink)?;
					sink.write_char(' ')?;
					feature.write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

macro_rules! media_feature {
	( $($name: ident($typ: ident): atom!($atom: tt)$(| $alts:pat)*,)+) => {
		// https://drafts.csswg.org/mediaqueries-5/#media-descriptor-table
		#[derive(Debug, PartialEq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum MediaFeature {
			$($name($typ),)+
		}
	}
}

apply_medias!(media_feature);

impl<'a> Parse<'a> for MediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser.next(), Token::LeftParen);
		macro_rules! match_media {
			( $($name: ident($typ: ident): atom!($atom: tt)$(| $alts:pat)*,)+) => {
				expect_ignore_case!{ parser.peek(), Token::Ident(_):
					$(atom!($atom)$(| $alts)* => Self::$name($typ::parse(parser)?),)+
				}
			}
		}
		let value = apply_medias!(match_media);
		expect!(parser.next(), Token::RightParen);
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for MediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('(')?;
		macro_rules! write_media {
			( $($name: ident($typ: ident): atom!($atom: tt)$(| $alts:pat)*,)+) => {
				match self {
					$(Self::$name(f) => f.write_css(sink)?,)+
				}
			}
		}
		apply_medias!(write_media);
		sink.write_char(')')
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
		}
	};
}
use apply_medias;

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum MediaType {
	All,
	Print,
	Screen,
	Custom(Atom),
}

impl Atomizable for MediaType {
	fn from_atom(atom: &Atom) -> Option<Self> {
		match atom.to_ascii_lowercase() {
			atom!("all") => Some(Self::All),
			atom!("print") => Some(Self::Print),
			atom!("screen") => Some(Self::Screen),
			// https://drafts.csswg.org/mediaqueries/#mq-syntax
			// The <media-type> production does not include the keywords only, not, and, or, and layer.
			atom!("only") | atom!("not") | atom!("and") | atom!("or") | atom!("layer") => None,
			custom => Some(Self::Custom(custom)),
		}
	}

	fn to_atom(&self) -> Atom {
		match self {
			Self::All => atom!("all"),
			Self::Print => atom!("print"),
			Self::Screen => atom!("screen"),
			Self::Custom(atom) => atom.clone(),
		}
	}
}

impl<'a> WriteCss<'a> for MediaType {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::All => atom!("all").write_css(sink),
			Self::Print => atom!("print").write_css(sink),
			Self::Screen => atom!("screen").write_css(sink),
			Self::Custom(atom) => atom.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MediaRule, 216);
		assert_size!(MediaQueryList, 168);
		assert_size!(MediaQuery, 144);
		assert_size!(MediaCondition, 120);
		assert_size!(MediaType, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MediaQuery, "print");
		assert_parse!(MediaQuery, "not embossed");
		assert_parse!(MediaQuery, "only screen");
		assert_parse!(MediaFeature, "(grid)");
		assert_parse!(MediaCondition, "and (grid)");
		assert_parse!(MediaQuery, "screen and (grid)");
		assert_parse!(MediaQuery, "screen and (hover) and (pointer)");
		assert_parse!(MediaQuery, "screen and (orientation: landscape)");
		assert_parse!(MediaRule, "@media print {\n\n}");
		assert_parse!(MediaRule, "@media print, (prefers-reduced-motion: reduce) {\n\n}");
		assert_parse!(MediaRule, "@media (min-width: 1200px) {\n\n}");
		assert_parse!(MediaRule, "@media (min-width: 1200px) {\n\tbody {\n\t\tcolor: red;\n\t}\n}");
		assert_parse!(MediaRule, "@media (min-width: 1200px) {\n@page {\n}\n}");
		// assert_parse!(MediaRule, "@media only screen and (max-device-width: 800px), only screen and (device-width: 1024px) and (device-height: 600px), only screen and (width: 1280px) and (orientation: landscape), only screen and (device-width: 800px), only screen and (max-width: 767px)");
	}

	#[test]
	fn test_minify() {
		// Drop redundant rules
		assert_minify!(MediaRule, "@media print {}", "");
	}
}
