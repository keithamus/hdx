use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{Build, Is, Parser, T};

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

impl<'a> Is<'a> for AnimateableFeature {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::is(p, c)
	}
}

impl<'a> Build<'a> for AnimateableFeature {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		let ident = <T![Ident]>::build(p, c);
		match p.parse_atom_lower(c) {
			atom!("-webkit-backdrop-filter") => Self::WebkitBackdropFilter(ident),
			atom!("-webkit-box-reflex") => Self::WebkitBoxReflect(ident),
			atom!("-webkit-mask") => Self::WebkitMask(ident),
			atom!("-webkit-mask-box-image") => Self::WebkitMaskBoxImage(ident),
			atom!("-webkit-overflow-scrolling") => Self::WebkitOverflowScrolling(ident),
			atom!("-webkit-perspective") => Self::WebkitPerspective(ident),
			atom!("backdrop-filter") => Self::BackdropFilter(ident),
			atom!("clip-path") => Self::ClipPath(ident),
			atom!("contain") => Self::Contain(ident),
			atom!("filter") => Self::Filter(ident),
			atom!("isolation") => Self::Isolation(ident),
			atom!("mask") => Self::Mask(ident),
			atom!("mask-border") => Self::MaskBorder(ident),
			atom!("mask-image") => Self::MaskImage(ident),
			atom!("mix-blend-mode") => Self::MixBlendMode(ident),
			atom!("offset-path") => Self::OffsetPath(ident),
			atom!("offset-position") => Self::OffsetPosition(ident),
			atom!("opacity") => Self::Opacity(ident),
			atom!("perspective") => Self::Perspective(ident),
			atom!("position") => Self::Position(ident),
			atom!("rotate") => Self::Rotate(ident),
			atom!("scale") => Self::Scale(ident),
			atom!("transform") => Self::Transform(ident),
			atom!("transform-style") => Self::TransformStyle(ident),
			atom!("translate") => Self::Translate(ident),
			atom!("view-transition-name") => Self::ViewTransitionName(ident),
			atom!("z-index") => Self::ZIndex(ident),
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
