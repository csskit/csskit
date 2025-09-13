use crate::{CssAtomSet, CssDiagnostic};
use css_parse::{Diagnostic, Parse, Parser, Result as ParserResult, T};
use csskit_derives::{Peek, ToCursors, ToSpan, Visitable};

use super::{moz::MozPseudoClass, ms::MsPseudoClass, o::OPseudoClass, webkit::WebkitPseudoClass};

macro_rules! apply_pseudo_class {
	($macro: ident) => {
		$macro! {
			Active: CssAtomSet::Active,
			AnyLink: CssAtomSet::AnyLink,
			Autofill: CssAtomSet::Autofill,
			Blank: CssAtomSet::Blank,
			Buffering: CssAtomSet::Buffering,
			Checked: CssAtomSet::Checked,
			Current: CssAtomSet::Current,
			Default: CssAtomSet::Default,
			Defined: CssAtomSet::Defined,
			Disabled: CssAtomSet::Disabled,
			Empty: CssAtomSet::Empty,
			Enabled: CssAtomSet::Enabled,
			First: CssAtomSet::First,
			FirstChild: CssAtomSet::FirstChild,
			FirstOfType: CssAtomSet::FirstOfType,
			Focus: CssAtomSet::Focus,
			FocusVisible: CssAtomSet::FocusVisible,
			FocusWithin: CssAtomSet::FocusWithin,
			Fullscreen: CssAtomSet::Fullscreen,
			Future: CssAtomSet::Future,
			HasSlotted: CssAtomSet::HasSlotted,
			Host: CssAtomSet::Host,
			Heading: CssAtomSet::Heading,
			Hover: CssAtomSet::Hover,
			InRange: CssAtomSet::InRange,
			Indeterminate: CssAtomSet::Indeterminate,
			Invalid: CssAtomSet::Invalid,
			LastChild: CssAtomSet::LastChild,
			LastOfType: CssAtomSet::LastOfType,
			Left: CssAtomSet::Left,
			Link: CssAtomSet::Link,
			LocalLink: CssAtomSet::LocalLink,
			Modal: CssAtomSet::Modal,
			Muted: CssAtomSet::Muted,
			OnlyChild: CssAtomSet::OnlyChild,
			OnlyOfType: CssAtomSet::OnlyOfType,
			Open: CssAtomSet::Open,
			Optional: CssAtomSet::Optional,
			OutOfRange: CssAtomSet::OutOfRange,
			Past: CssAtomSet::Past,
			Paused: CssAtomSet::Paused,
			PictureInPicture: CssAtomSet::PictureInPicture,
			PlaceholderShown: CssAtomSet::PlaceholderShown,
			Playing: CssAtomSet::Playing,
			PopoverOpen: CssAtomSet::PopoverOpen,
			ReadOnly: CssAtomSet::ReadOnly,
			ReadWrite: CssAtomSet::ReadWrite,
			Required: CssAtomSet::Required,
			Right: CssAtomSet::Right,
			Root: CssAtomSet::Root,
			Scope: CssAtomSet::Scope,
			Seeking: CssAtomSet::Seeking,
			Stalled: CssAtomSet::Stalled,
			Target: CssAtomSet::Target,
			TargetCurrent: CssAtomSet::TargetCurrent,
			TargetWithin: CssAtomSet::TargetWithin,
			UserInvalid: CssAtomSet::UserInvalid,
			Valid: CssAtomSet::Valid,
			Visited: CssAtomSet::Visited,
			VolumeLocked: CssAtomSet::VolumeLocked,
		}
	};
}

macro_rules! define_pseudo_class {
	( $($(#[$meta:meta])* $ident: ident: $pat: pat $(,)*)+ ) => {
		#[derive(Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		#[visit(self)]
		pub enum PseudoClass {
			$($(#[$meta])* $ident(T![:], T![Ident]),)+
			Webkit(WebkitPseudoClass),
			Moz(MozPseudoClass),
			Ms(MsPseudoClass),
			O(OPseudoClass),
		}
	};
}
apply_pseudo_class!(define_pseudo_class);

impl<'a> Parse<'a> for PseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(2);
		macro_rules! match_keyword {
			( $($(#[$meta:meta])* $ident: ident: $pat: pat $(,)*)+ ) => {
				match p.to_atom::<CssAtomSet>(c) {
					$($pat => {
						let colon = p.parse::<T![:]>()?;
						let ident = p.parse::<T![Ident]>()?;
						Ok(Self::$ident(colon, ident))
					})+
					_ => {
						if let Ok(psuedo) = p.try_parse::<WebkitPseudoClass>() {
							return Ok(Self::Webkit(psuedo));
						}
						if let Ok(psuedo) = p.try_parse::<MozPseudoClass>() {
							return Ok(Self::Moz(psuedo));
						}
						if let Ok(psuedo) = p.try_parse::<MsPseudoClass>() {
							return Ok(Self::Ms(psuedo));
						}
						if let Ok(psuedo) = p.try_parse::<OPseudoClass>() {
							return Ok(Self::O(psuedo));
						}
						Err(Diagnostic::new(c, Diagnostic::unexpected_pseudo_class))?
					}
				}
			};
		}
		apply_pseudo_class!(match_keyword)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PseudoClass>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PseudoClass, ":target");
		assert_parse!(CssAtomSet::ATOMS, PseudoClass, ":scope");
		assert_parse!(CssAtomSet::ATOMS, PseudoClass, ":valid");
	}

	#[cfg(feature = "css_feature_data")]
	#[test]
	fn test_feature_data() {
		use crate::assert_feature_id;
		assert_feature_id!(":hover", PseudoClass, "css.selectors.hover");
		assert_feature_id!(":future", PseudoClass, "css.selectors.future");
		assert_feature_id!(":volume-locked", PseudoClass, "css.selectors.volume-locked");
	}
}
