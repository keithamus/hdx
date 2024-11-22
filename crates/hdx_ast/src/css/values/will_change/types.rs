use hdx_atom::{atom, Atom};
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, T};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-will-change-1/#typedef-animateable-feature
// <animateable-feature> = scroll-position | contents | <custom-ident>
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum AnimateableFeature {
	ScrollPosition,
	Contents,
	CustomIdent(Atom),

	// These are known "custom idents" that Firefox, Safari and WebKit support.
	// See https://searchfox.org/mozilla-central/source/servo/components/style/values/specified/box.rs#1001-1025
	// and also https://searchfox.org/mozilla-central/source/servo/components/style/values/specified/box.rs#1033-1053
	// for Firefox.
	//
	// See https://searchfox.org/wubkat/source/Source/WebCore/rendering/style/WillChangeData.cpp for Safari
	//
	// See https://source.chromium.org/search?q=%22WillChangeProperties().Contains%22
	// and also https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/style/computed_style.cc;l=1366-1400
	// for Chromium

	// Shared
	BackdropFilter,
	ClipPath,
	Contain,
	Filter,
	Isolation,
	MixBlendMode,
	OffsetPath,
	Opacity,
	Perspective,
	Position,
	Rotate,
	Scale,
	Transform,
	TransformStyle,
	Translate,
	ZIndex,

	// Chrome also supports
	ViewTransitionName,

	// Chrome & Safari (but not Firefox) support
	Mask,
	OffsetPosition,
	WebkitBoxReflect,
	WebkitMaskBoxImage,

	// Safari also supports
	MaskBorder,
	WebkitMask,
	WebkitPerspective,
	WebkitBackdropFilter,
	WebkitOverflowScrolling,

	// Firefox & Safari also supports:
	MaskImage,
}

impl<'a> Peek<'a> for AnimateableFeature {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<T![Ident]>()
	}
}

impl<'a> Parse<'a> for AnimateableFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let token = *parser.parse::<T![Ident]>()?;
		Ok(match parser.parse_atom_lower(token) {
			atom!("-webkit-backdrop-filter") => Self::WebkitBackdropFilter,
			atom!("-webkit-box-reflex") => Self::WebkitBoxReflect,
			atom!("-webkit-mask") => Self::WebkitMask,
			atom!("-webkit-mask-box-image") => Self::WebkitMaskBoxImage,
			atom!("-webkit-overflow-scrolling") => Self::WebkitOverflowScrolling,
			atom!("-webkit-perspective") => Self::WebkitPerspective,
			atom!("backdrop-filter") => Self::BackdropFilter,
			atom!("clip-path") => Self::ClipPath,
			atom!("contain") => Self::Contain,
			atom!("filter") => Self::Filter,
			atom!("isolation") => Self::Isolation,
			atom!("mask") => Self::Mask,
			atom!("mask-border") => Self::MaskBorder,
			atom!("mask-image") => Self::MaskImage,
			atom!("mix-blend-mode") => Self::MixBlendMode,
			atom!("offset-path") => Self::OffsetPath,
			atom!("offset-position") => Self::OffsetPosition,
			atom!("opacity") => Self::Opacity,
			atom!("perspective") => Self::Perspective,
			atom!("position") => Self::Position,
			atom!("rotate") => Self::Rotate,
			atom!("scale") => Self::Scale,
			atom!("transform") => Self::Transform,
			atom!("transform-style") => Self::TransformStyle,
			atom!("translate") => Self::Translate,
			atom!("view-transition-name") => Self::ViewTransitionName,
			atom!("z-index") => Self::ZIndex,
			atom => Self::CustomIdent(atom),
		})
	}
}

impl<'a> WriteCss<'a> for AnimateableFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::CustomIdent(a) => a.write_css(sink),
			Self::ScrollPosition => atom!("scroll-position").write_css(sink),
			Self::Contents => atom!("contents").write_css(sink),
			Self::BackdropFilter => atom!("backdrop-filter").write_css(sink),
			Self::ClipPath => atom!("clip-path").write_css(sink),
			Self::Contain => atom!("contain").write_css(sink),
			Self::Filter => atom!("filter").write_css(sink),
			Self::Isolation => atom!("isolation").write_css(sink),
			Self::Mask => atom!("mask").write_css(sink),
			Self::MaskBorder => atom!("mask-border").write_css(sink),
			Self::MaskImage => atom!("mask-image").write_css(sink),
			Self::MixBlendMode => atom!("mix-blend-mode").write_css(sink),
			Self::OffsetPath => atom!("offset-path").write_css(sink),
			Self::OffsetPosition => atom!("offset-position").write_css(sink),
			Self::Opacity => atom!("opacity").write_css(sink),
			Self::Perspective => atom!("perspective").write_css(sink),
			Self::Position => atom!("position").write_css(sink),
			Self::Rotate => atom!("rotate").write_css(sink),
			Self::Scale => atom!("scale").write_css(sink),
			Self::Transform => atom!("transform").write_css(sink),
			Self::TransformStyle => atom!("transform-style").write_css(sink),
			Self::Translate => atom!("translate").write_css(sink),
			Self::ViewTransitionName => atom!("view-transition-name").write_css(sink),
			Self::WebkitBackdropFilter => atom!("-webkit-backdrop-filter").write_css(sink),
			Self::WebkitBoxReflect => atom!("-webkit-box-reflect").write_css(sink),
			Self::WebkitMask => atom!("-webkit-mask").write_css(sink),
			Self::WebkitMaskBoxImage => atom!("-webkit-mask-box-image").write_css(sink),
			Self::WebkitOverflowScrolling => atom!("-webkit-overflow-scrolling").write_css(sink),
			Self::WebkitPerspective => atom!("-webkit-perspective").write_css(sink),
			Self::ZIndex => atom!("z-index").write_css(sink),
		}
	}
}
