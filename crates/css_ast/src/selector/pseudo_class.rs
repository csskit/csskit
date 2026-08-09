use super::prelude::*;

use super::{moz::MozPseudoClass, ms::MsPseudoClass, o::OPseudoClass, webkit::WebkitPseudoClass};

macro_rules! apply_pseudo_class {
	($macro: ident) => {
		$macro! {
			Active: CssAtomSet::Active,
			ActiveViewTransition: CssAtomSet::ActiveViewTransition,
			AnimatedImage: CssAtomSet::AnimatedImage,
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
			FirstOfPage: CssAtomSet::FirstOfPage,
			FirstOfType: CssAtomSet::FirstOfType,
			Focus: CssAtomSet::Focus,
			FocusVisible: CssAtomSet::FocusVisible,
			FocusWithin: CssAtomSet::FocusWithin,
			Fullscreen: CssAtomSet::Fullscreen,
			Future: CssAtomSet::Future,
			HasSlotted: CssAtomSet::HasSlotted,
			Heading: CssAtomSet::Heading,
			HighValue: CssAtomSet::HighValue,
			Host: CssAtomSet::Host,
			Hover: CssAtomSet::Hover,
			InRange: CssAtomSet::InRange,
			Indeterminate: CssAtomSet::Indeterminate,
			InterestSource: CssAtomSet::InterestSource,
			InterestTarget: CssAtomSet::InterestTarget,
			Invalid: CssAtomSet::Invalid,
			LastChild: CssAtomSet::LastChild,
			LastOfPage: CssAtomSet::LastOfPage,
			LastOfType: CssAtomSet::LastOfType,
			Left: CssAtomSet::Left,
			Link: CssAtomSet::Link,
			LocalLink: CssAtomSet::LocalLink,
			LowValue: CssAtomSet::LowValue,
			Modal: CssAtomSet::Modal,
			Muted: CssAtomSet::Muted,
			NavSource: CssAtomSet::NavSource,
			OnlyChild: CssAtomSet::OnlyChild,
			OnlyOfType: CssAtomSet::OnlyOfType,
			Open: CssAtomSet::Open,
			OptimalValue: CssAtomSet::OptimalValue,
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
			Snapped: CssAtomSet::Snapped,
			SnappedBlock: CssAtomSet::SnappedBlock,
			SnappedInline: CssAtomSet::SnappedInline,
			SnappedX: CssAtomSet::SnappedX,
			SnappedY: CssAtomSet::SnappedY,
			Stalled: CssAtomSet::Stalled,
			Target: CssAtomSet::Target,
			TargetAfter: CssAtomSet::TargetAfter,
			TargetBefore: CssAtomSet::TargetBefore,
			TargetCurrent: CssAtomSet::TargetCurrent,
			Unchecked: CssAtomSet::Unchecked,
			UserInvalid: CssAtomSet::UserInvalid,
			UserValid: CssAtomSet::UserValid,
			Valid: CssAtomSet::Valid,
			Visited: CssAtomSet::Visited,
			VolumeLocked: CssAtomSet::VolumeLocked,
		}
	};
}

macro_rules! define_pseudo_class {
	( $($(#[$meta:meta])* $ident: ident: $pat: pat $(,)*)+ ) => {
		#[node]
		#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		pub enum PseudoClass {
			$($(#[$meta])* $ident(#[semantic_eq(skip)] T![:], T![Ident]),)+
			Webkit(WebkitPseudoClass),
			Moz(MozPseudoClass),
			Ms(MsPseudoClass),
			O(OPseudoClass),
		}
	};
}
apply_pseudo_class!(define_pseudo_class);

impl<'a> Parse<'a> for PseudoClass {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PseudoClass, ":target");
		assert_parse!(CssAtomSet::ATOMS, PseudoClass, ":scope");
		assert_parse!(CssAtomSet::ATOMS, PseudoClass, ":valid");
	}

	#[test]
	fn unknown_pseudo_class_names_only_the_bad_token() {
		let source = ":nonsense";
		let arena = css_parse::Arena::default();
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, source);
		let result = css_parse::Parser::new(&arena, source, lexer).parse_entirely::<PseudoClass>();
		let error = result.errors.first().expect("a diagnostic");
		let meta = (error.formatter)(error, source);
		assert_eq!(meta.message, "Unexpected pseudo selector ':nonsense'");
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
