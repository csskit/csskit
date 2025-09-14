use crate::{MozPseudoElement, MsPseudoElement, OPseudoElement, WebkitPseudoElement, diagnostics};
use css_parse::{KindSet, Parse, Parser, Result as ParserResult, T, keyword_set, pseudo_class};
use csskit_derives::{ToCursors, ToSpan, Visitable};

macro_rules! apply_pseudo_element {
	($macro: ident) => {
		$macro! {
			After: "after",
			Backdrop: "backdrop",
			Before: "before",
			Checkmark: "checkmark",
			Column: "column",
			Cue: "cue",
			DetailsContent: "details-content",
			FileSelectorButton: "file-selector-button",
			FirstLetter: "first-letter",
			FirstLine: "first-line",
			GrammarError: "grammar-error",
			Marker: "marker",
			PickerIcon: "picker-icon",
			Placeholder: "placeholder",
			ScrollMarker: "scroll-marker",
			ScrollMarkerGroup: "scroll-marker-group",
			Selection: "selection",
			SpellingError: "spelling-error",
			TargetText: "target-text",
			ViewTransition: "view-transition",
		}
	};
}

macro_rules! define_pseudo_element {
	( $($(#[$meta:meta])* $ident: ident: $str: tt $(,)*)+ ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		#[visit(self)]
		pub enum PseudoElement {
			$($(#[$meta])* $ident(T![::], T![Ident]),)+
			Webkit(WebkitPseudoElement),
			Moz(MozPseudoElement),
			Ms(MsPseudoElement),
			O(OPseudoElement),
		}
	};
}
apply_pseudo_element!(define_pseudo_element);

macro_rules! define_pseudo_element_keyword {
	( $($(#[$meta:meta])* $ident: ident: $str: tt $(,)*)+ ) => {
		keyword_set!(pub enum PseudoElementKeyword {
			$($ident: $str,)+
		});
	};
}
apply_pseudo_element!(define_pseudo_element_keyword);

impl<'a> Parse<'a> for PseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let skip = p.set_skip(KindSet::NONE);
		let colons = p.parse::<T![::]>();
		let keyword = p.parse::<PseudoElementKeyword>();
		p.set_skip(skip);
		let colons = colons?;
		macro_rules! match_keyword {
			( $($(#[$meta:meta])* $ident: ident: $str: tt $(,)*)+ ) => {
				match keyword {
					$(Ok(PseudoElementKeyword::$ident(ident)) => Ok(Self::$ident(colons, ident)),)+
					Err(_) => {
						p.rewind(checkpoint);
						let c = p.peek_n(2);
						if let Ok(psuedo) = p.try_parse::<WebkitPseudoElement>() {
							return Ok(Self::Webkit(psuedo));
						}
						if let Ok(psuedo) = p.try_parse::<MozPseudoElement>() {
							return Ok(Self::Moz(psuedo));
						}
						if let Ok(psuedo) = p.try_parse::<MsPseudoElement>() {
							return Ok(Self::Ms(psuedo));
						}
						if let Ok(psuedo) = p.try_parse::<OPseudoElement>() {
							return Ok(Self::O(psuedo));
						}
						Err(diagnostics::UnexpectedPseudoElement(p.to_source_cursor(c).to_string(), c))?
					}
				}
			}
		}
		apply_pseudo_element!(match_keyword)
	}
}

pseudo_class!(
	#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
	#[derive(Visitable)]
	#[visit(self)]
	pub enum LegacyPseudoElement {
		After: "after",
		Before: "before",
		FirstLetter: "first-letter",
		FirstLine: "first-line",
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PseudoElement>(), 44);
		assert_eq!(std::mem::size_of::<LegacyPseudoElement>(), 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PseudoElement, "::after");
		assert_parse!(PseudoElement, "::first-letter");
		assert_parse!(PseudoElement, "::view-transition");
		assert_parse!(LegacyPseudoElement, ":after");
	}

	#[cfg(feature = "css_feature_data")]
	#[test]
	fn test_feature_data() {
		use crate::assert_feature_id;
		assert_feature_id!("::after", PseudoElement, "css.selectors.after");
		assert_feature_id!("::view-transition", PseudoElement, "css.selectors.view-transition");
		assert_feature_id!("::spelling-error", PseudoElement, "css.selectors.spelling-error");
		assert_feature_id!(":after", LegacyPseudoElement, "css.selectors.after");
		assert_feature_id!(":before", LegacyPseudoElement, "css.selectors.before");
	}
}
