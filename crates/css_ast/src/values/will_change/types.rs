use css_lexer::Cursor;
use css_parse::{Build, Parser, Peek, T};

// https://drafts.csswg.org/css-will-change-1/#typedef-animateable-feature
// <animateable-feature> = scroll-position | contents | <custom-ident>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum AnimateableFeature {
	ScrollPosition(T![Ident]),
	Contents(T![Ident]),
	CustomIdent(T![Ident]),

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
	BackdropFilter(T![Ident]),
	ClipPath(T![Ident]),
	Contain(T![Ident]),
	Filter(T![Ident]),
	Isolation(T![Ident]),
	MixBlendMode(T![Ident]),
	OffsetPath(T![Ident]),
	Opacity(T![Ident]),
	Perspective(T![Ident]),
	Position(T![Ident]),
	Rotate(T![Ident]),
	Scale(T![Ident]),
	Transform(T![Ident]),
	TransformStyle(T![Ident]),
	Translate(T![Ident]),
	ZIndex(T![Ident]),

	// Chrome also supports
	ViewTransitionName(T![Ident]),

	// Chrome & Safari (but not Firefox) support
	Mask(T![Ident]),
	OffsetPosition(T![Ident]),
	WebkitBoxReflect(T![Ident]),
	WebkitMaskBoxImage(T![Ident]),

	// Safari also supports
	MaskBorder(T![Ident]),
	WebkitMask(T![Ident]),
	WebkitPerspective(T![Ident]),
	WebkitBackdropFilter(T![Ident]),
	WebkitOverflowScrolling(T![Ident]),

	// Firefox & Safari also supports:
	MaskImage(T![Ident]),
}

impl AnimateableFeature {
	const MAP: phf::Map<&'static str, AnimateableFeature> = phf::phf_map! {
			"-webkit-backdrop-filter" => Self::WebkitBackdropFilter(<T![Ident]>::dummy()),
			"-webkit-box-reflex" => Self::WebkitBoxReflect(<T![Ident]>::dummy()),
			"-webkit-mask" => Self::WebkitMask(<T![Ident]>::dummy()),
			"-webkit-mask-box-image" => Self::WebkitMaskBoxImage(<T![Ident]>::dummy()),
			"-webkit-overflow-scrolling" => Self::WebkitOverflowScrolling(<T![Ident]>::dummy()),
			"-webkit-perspective" => Self::WebkitPerspective(<T![Ident]>::dummy()),
			"backdrop-filter" => Self::BackdropFilter(<T![Ident]>::dummy()),
			"clip-path" => Self::ClipPath(<T![Ident]>::dummy()),
			"contain" => Self::Contain(<T![Ident]>::dummy()),
			"filter" => Self::Filter(<T![Ident]>::dummy()),
			"isolation" => Self::Isolation(<T![Ident]>::dummy()),
			"mask" => Self::Mask(<T![Ident]>::dummy()),
			"mask-border" => Self::MaskBorder(<T![Ident]>::dummy()),
			"mask-image" => Self::MaskImage(<T![Ident]>::dummy()),
			"mix-blend-mode" => Self::MixBlendMode(<T![Ident]>::dummy()),
			"offset-path" => Self::OffsetPath(<T![Ident]>::dummy()),
			"offset-position" => Self::OffsetPosition(<T![Ident]>::dummy()),
			"opacity" => Self::Opacity(<T![Ident]>::dummy()),
			"perspective" => Self::Perspective(<T![Ident]>::dummy()),
			"position" => Self::Position(<T![Ident]>::dummy()),
			"rotate" => Self::Rotate(<T![Ident]>::dummy()),
			"scale" => Self::Scale(<T![Ident]>::dummy()),
			"transform" => Self::Transform(<T![Ident]>::dummy()),
			"transform-style" => Self::TransformStyle(<T![Ident]>::dummy()),
			"translate" => Self::Translate(<T![Ident]>::dummy()),
			"view-transition-name" => Self::ViewTransitionName(<T![Ident]>::dummy()),
			"z-index" => Self::ZIndex(<T![Ident]>::dummy()),
	};
}

impl<'a> Peek<'a> for AnimateableFeature {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c)
	}
}

impl<'a> Build<'a> for AnimateableFeature {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		let ident = <T![Ident]>::build(p, c);
		let feature = Self::MAP.get(&p.parse_str_lower(c));
		match feature {
			Some(Self::WebkitBackdropFilter(_)) => Self::WebkitBackdropFilter(ident),
			Some(Self::WebkitBoxReflect(_)) => Self::WebkitBoxReflect(ident),
			Some(Self::WebkitMask(_)) => Self::WebkitMask(ident),
			Some(Self::WebkitMaskBoxImage(_)) => Self::WebkitMaskBoxImage(ident),
			Some(Self::WebkitOverflowScrolling(_)) => Self::WebkitOverflowScrolling(ident),
			Some(Self::WebkitPerspective(_)) => Self::WebkitPerspective(ident),
			Some(Self::BackdropFilter(_)) => Self::BackdropFilter(ident),
			Some(Self::ClipPath(_)) => Self::ClipPath(ident),
			Some(Self::Contain(_)) => Self::Contain(ident),
			Some(Self::Filter(_)) => Self::Filter(ident),
			Some(Self::Isolation(_)) => Self::Isolation(ident),
			Some(Self::Mask(_)) => Self::Mask(ident),
			Some(Self::MaskBorder(_)) => Self::MaskBorder(ident),
			Some(Self::MaskImage(_)) => Self::MaskImage(ident),
			Some(Self::MixBlendMode(_)) => Self::MixBlendMode(ident),
			Some(Self::OffsetPath(_)) => Self::OffsetPath(ident),
			Some(Self::OffsetPosition(_)) => Self::OffsetPosition(ident),
			Some(Self::Opacity(_)) => Self::Opacity(ident),
			Some(Self::Perspective(_)) => Self::Perspective(ident),
			Some(Self::Position(_)) => Self::Position(ident),
			Some(Self::Rotate(_)) => Self::Rotate(ident),
			Some(Self::Scale(_)) => Self::Scale(ident),
			Some(Self::Transform(_)) => Self::Transform(ident),
			Some(Self::TransformStyle(_)) => Self::TransformStyle(ident),
			Some(Self::Translate(_)) => Self::Translate(ident),
			Some(Self::ViewTransitionName(_)) => Self::ViewTransitionName(ident),
			Some(Self::ZIndex(_)) => Self::ZIndex(ident),
			_ => Self::CustomIdent(ident),
		}
	}
}

impl From<AnimateableFeature> for Cursor {
	fn from(value: AnimateableFeature) -> Self {
		match value {
			AnimateableFeature::ScrollPosition(c) => c.into(),
			AnimateableFeature::Contents(c) => c.into(),
			AnimateableFeature::CustomIdent(c) => c.into(),
			AnimateableFeature::BackdropFilter(c) => c.into(),
			AnimateableFeature::ClipPath(c) => c.into(),
			AnimateableFeature::Contain(c) => c.into(),
			AnimateableFeature::Filter(c) => c.into(),
			AnimateableFeature::Isolation(c) => c.into(),
			AnimateableFeature::MixBlendMode(c) => c.into(),
			AnimateableFeature::OffsetPath(c) => c.into(),
			AnimateableFeature::Opacity(c) => c.into(),
			AnimateableFeature::Perspective(c) => c.into(),
			AnimateableFeature::Position(c) => c.into(),
			AnimateableFeature::Rotate(c) => c.into(),
			AnimateableFeature::Scale(c) => c.into(),
			AnimateableFeature::Transform(c) => c.into(),
			AnimateableFeature::TransformStyle(c) => c.into(),
			AnimateableFeature::Translate(c) => c.into(),
			AnimateableFeature::ZIndex(c) => c.into(),
			AnimateableFeature::ViewTransitionName(c) => c.into(),
			AnimateableFeature::Mask(c) => c.into(),
			AnimateableFeature::OffsetPosition(c) => c.into(),
			AnimateableFeature::WebkitBoxReflect(c) => c.into(),
			AnimateableFeature::WebkitMaskBoxImage(c) => c.into(),
			AnimateableFeature::MaskBorder(c) => c.into(),
			AnimateableFeature::WebkitMask(c) => c.into(),
			AnimateableFeature::WebkitPerspective(c) => c.into(),
			AnimateableFeature::WebkitBackdropFilter(c) => c.into(),
			AnimateableFeature::WebkitOverflowScrolling(c) => c.into(),
			AnimateableFeature::MaskImage(c) => c.into(),
		}
	}
}
