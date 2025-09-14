use crate::diagnostics;
use css_parse::{Parse, Parser, Result, T};
use csskit_derives::{IntoCursor, Peek, ToCursors, Visitable};

// https://drafts.csswg.org/css-will-change-1/#typedef-animateable-feature
// <animateable-feature> = scroll-position | contents | <custom-ident>
#[derive(IntoCursor, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
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

impl<'a> Parse<'a> for AnimateableFeature {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let ident = p.parse::<T![Ident]>()?;
			let feature = Self::MAP.get(p.parse_str_lower(ident.into()));
			match feature {
				Some(Self::WebkitBackdropFilter(_)) => Ok(Self::WebkitBackdropFilter(ident)),
				Some(Self::WebkitBoxReflect(_)) => Ok(Self::WebkitBoxReflect(ident)),
				Some(Self::WebkitMask(_)) => Ok(Self::WebkitMask(ident)),
				Some(Self::WebkitMaskBoxImage(_)) => Ok(Self::WebkitMaskBoxImage(ident)),
				Some(Self::WebkitOverflowScrolling(_)) => Ok(Self::WebkitOverflowScrolling(ident)),
				Some(Self::WebkitPerspective(_)) => Ok(Self::WebkitPerspective(ident)),
				Some(Self::BackdropFilter(_)) => Ok(Self::BackdropFilter(ident)),
				Some(Self::ClipPath(_)) => Ok(Self::ClipPath(ident)),
				Some(Self::Contain(_)) => Ok(Self::Contain(ident)),
				Some(Self::Filter(_)) => Ok(Self::Filter(ident)),
				Some(Self::Isolation(_)) => Ok(Self::Isolation(ident)),
				Some(Self::Mask(_)) => Ok(Self::Mask(ident)),
				Some(Self::MaskBorder(_)) => Ok(Self::MaskBorder(ident)),
				Some(Self::MaskImage(_)) => Ok(Self::MaskImage(ident)),
				Some(Self::MixBlendMode(_)) => Ok(Self::MixBlendMode(ident)),
				Some(Self::OffsetPath(_)) => Ok(Self::OffsetPath(ident)),
				Some(Self::OffsetPosition(_)) => Ok(Self::OffsetPosition(ident)),
				Some(Self::Opacity(_)) => Ok(Self::Opacity(ident)),
				Some(Self::Perspective(_)) => Ok(Self::Perspective(ident)),
				Some(Self::Position(_)) => Ok(Self::Position(ident)),
				Some(Self::Rotate(_)) => Ok(Self::Rotate(ident)),
				Some(Self::Scale(_)) => Ok(Self::Scale(ident)),
				Some(Self::Transform(_)) => Ok(Self::Transform(ident)),
				Some(Self::TransformStyle(_)) => Ok(Self::TransformStyle(ident)),
				Some(Self::Translate(_)) => Ok(Self::Translate(ident)),
				Some(Self::ViewTransitionName(_)) => Ok(Self::ViewTransitionName(ident)),
				Some(Self::ZIndex(_)) => Ok(Self::ZIndex(ident)),
				_ => Ok(Self::CustomIdent(ident)),
			}
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}
