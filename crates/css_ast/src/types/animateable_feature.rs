use super::prelude::*;

// https://drafts.csswg.org/css-will-change-1/#typedef-animateable-feature
// <animateable-feature> = scroll-position | contents | <custom-ident>
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum AnimateableFeature {
	#[atom(CssAtomSet::ScrollPosition)]
	ScrollPosition(T![Ident]),
	#[atom(CssAtomSet::Contents)]
	Contents(T![Ident]),

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
	#[atom(CssAtomSet::BackdropFilter)]
	BackdropFilter(T![Ident]),
	#[atom(CssAtomSet::ClipPath)]
	ClipPath(T![Ident]),
	#[atom(CssAtomSet::Contain)]
	Contain(T![Ident]),
	#[atom(CssAtomSet::Filter)]
	Filter(T![Ident]),
	#[atom(CssAtomSet::Isolation)]
	Isolation(T![Ident]),
	#[atom(CssAtomSet::MixBlendMode)]
	MixBlendMode(T![Ident]),
	#[atom(CssAtomSet::OffsetPath)]
	OffsetPath(T![Ident]),
	#[atom(CssAtomSet::Opacity)]
	Opacity(T![Ident]),
	#[atom(CssAtomSet::Perspective)]
	Perspective(T![Ident]),
	#[atom(CssAtomSet::Position)]
	Position(T![Ident]),
	#[atom(CssAtomSet::Rotate)]
	Rotate(T![Ident]),
	#[atom(CssAtomSet::Scale)]
	Scale(T![Ident]),
	#[atom(CssAtomSet::Transform)]
	Transform(T![Ident]),
	#[atom(CssAtomSet::TransformStyle)]
	TransformStyle(T![Ident]),
	#[atom(CssAtomSet::Translate)]
	Translate(T![Ident]),
	#[atom(CssAtomSet::ZIndex)]
	ZIndex(T![Ident]),

	// Chrome also supports
	#[atom(CssAtomSet::ViewTransitionName)]
	ViewTransitionName(T![Ident]),

	// Chrome & Safari (but not Firefox) support
	#[atom(CssAtomSet::Mask)]
	Mask(T![Ident]),
	#[atom(CssAtomSet::OffsetPosition)]
	OffsetPosition(T![Ident]),
	#[atom(CssAtomSet::_WebkitBoxReflect)]
	WebkitBoxReflect(T![Ident]),
	#[atom(CssAtomSet::_WebkitMaskBoxImage)]
	WebkitMaskBoxImage(T![Ident]),

	// Safari also supports
	#[atom(CssAtomSet::MaskBorder)]
	MaskBorder(T![Ident]),
	#[atom(CssAtomSet::_WebkitMask)]
	WebkitMask(T![Ident]),
	#[atom(CssAtomSet::_WebkitPerspective)]
	WebkitPerspective(T![Ident]),
	#[atom(CssAtomSet::_WebkitBackdropFilter)]
	WebkitBackdropFilter(T![Ident]),
	#[atom(CssAtomSet::_WebkitOverflowScrolling)]
	WebkitOverflowScrolling(T![Ident]),

	// Firefox & Safari also supports:
	#[atom(CssAtomSet::MaskImage)]
	MaskImage(T![Ident]),

	CustomIdent(T![Ident]),
}
