use crate::diagnostics;
use css_parse::{Build, Parse, Parser, Result as ParserResult, T, keyword_set};
use csskit_derives::{ToCursors, ToSpan, Visitable};

use super::{moz::MozPseudoClass, ms::MsPseudoClass, o::OPseudoClass, webkit::WebkitPseudoClass};

macro_rules! apply_pseudo_class {
	($macro: ident) => {
		$macro! {
			Active: "active",
			AnyLink: "any-link",
			Autofill: "autofill",
			Blank: "blank",
			Buffering: "buffering",
			Checked: "checked",
			Current: "current",
			Default: "default",
			Defined: "defined",
			Disabled: "disabled",
			Empty: "empty",
			Enabled: "enabled",
			First: "first",
			FirstChild: "first-child",
			FirstOfType: "first-of-type",
			Focus: "focus",
			FocusVisible: "focus-visible",
			FocusWithin: "focus-within",
			Fullscreen: "fullscreen",
			Future: "future",
			HasSlotted: "has-slotted",
			Host: "host",
			Heading: "heading",
			Hover: "hover",
			InRange: "in-range",
			Indeterminate: "indeterminate",
			Invalid: "invalid",
			LastChild: "last-child",
			LastOfType: "last-of-type",
			Left: "left",
			Link: "link",
			LocalLink: "local-link",
			Modal: "modal",
			Muted: "muted",
			OnlyChild: "only-child",
			OnlyOfType: "only-of-type",
			Open: "open",
			Optional: "optional",
			OutOfRange: "out-of-range",
			Past: "past",
			Paused: "paused",
			PictureInPicture: "picture-in-picture",
			PlaceholderShown: "placeholder-shown",
			Playing: "playing",
			PopoverOpen: "popover-open",
			ReadOnly: "read-only",
			ReadWrite: "read-write",
			Required: "required",
			Right: "right",
			Root: "root",
			Scope: "scope",
			Seeking: "seeking",
			Stalled: "stalled",
			Target: "target",
			TargetCurrent: "target-current",
			TargetWithin: "target-within",
			UserInvalid: "user-invalid",
			Valid: "valid",
			Visited: "visited",
			VolumeLocked: "volume-locked",
		}
	};
}

macro_rules! define_pseudo_class {
	( $($(#[$meta:meta])* $ident: ident: $str: tt $(,)*)+ ) => {
		#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
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

macro_rules! define_pseudo_class_keyword {
	( $($(#[$meta:meta])* $ident: ident: $str: tt $(,)*)+ ) => {
		keyword_set!(
			enum PseudoClassKeyword {
				$($ident: $str,)+
			}
		);
	}
}
apply_pseudo_class!(define_pseudo_class_keyword);

impl<'a> Parse<'a> for PseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let colon = p.parse::<T![:]>()?;
		let keyword = p.parse::<PseudoClassKeyword>();
		macro_rules! match_keyword {
			( $($(#[$meta:meta])* $ident: ident: $str: tt $(,)*)+ ) => {
				match keyword {
					$(Ok(PseudoClassKeyword::$ident(c)) => Ok(Self::$ident(colon, <T![Ident]>::build(p, c.into()))),)+
					Err(_) => {
						p.rewind(checkpoint);
						let c = p.peek_n(2);
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
						Err(diagnostics::UnexpectedPseudoClass(p.parse_str(c).into(), c))?
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PseudoClass>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PseudoClass, ":target");
		assert_parse!(PseudoClass, ":scope");
		assert_parse!(PseudoClass, ":valid");
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
