use crate::{CssAtomSet, CssDiagnostic, MozPseudoElement, MsPseudoElement, OPseudoElement, WebkitPseudoElement};
use css_lexer::Kind;
use css_parse::{Cursor, Diagnostic, KindSet, Parse, Parser, Peek, Result as ParserResult, T, pseudo_class};
use csskit_derives::{SemanticEq, ToCursors, ToSpan};

macro_rules! apply_pseudo_element {
	($macro: ident) => {
		$macro! {
			After: CssAtomSet::After,
			Backdrop: CssAtomSet::Backdrop,
			Before: CssAtomSet::Before,
			Checkmark: CssAtomSet::Checkmark,
			Column: CssAtomSet::Column,
			Cue: CssAtomSet::Cue,
			DetailsContent: CssAtomSet::DetailsContent,
			FileSelectorButton: CssAtomSet::FileSelectorButton,
			FirstLetter: CssAtomSet::FirstLetter,
			FirstLine: CssAtomSet::FirstLine,
			GrammarError: CssAtomSet::GrammarError,
			Marker: CssAtomSet::Marker,
			PickerIcon: CssAtomSet::PickerIcon,
			Placeholder: CssAtomSet::Placeholder,
			ScrollMarker: CssAtomSet::ScrollMarker,
			ScrollMarkerGroup: CssAtomSet::ScrollMarkerGroup,
			Selection: CssAtomSet::Selection,
			SpellingError: CssAtomSet::SpellingError,
			TargetText: CssAtomSet::TargetText,
			ViewTransition: CssAtomSet::ViewTransition,
		}
	};
}

macro_rules! define_pseudo_element {
	( $($(#[$meta:meta])* $ident: ident: $pat: pat $(,)*)+ ) => {
		#[derive(ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
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

impl<'a> Peek<'a> for PseudoElement {
	fn peek<I>(p: &Parser<'a, I>, _: css_lexer::Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.peek::<T![::]>() && p.peek_n(3) == Kind::Ident
	}
}

impl<'a> Parse<'a> for PseudoElement {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(3);
		macro_rules! match_keyword {
			( $($(#[$meta:meta])* $ident: ident: $pat: pat $(,)*)+ ) => {
				match p.to_atom::<CssAtomSet>(c) {
					$($pat => {
						let skip = p.set_skip(KindSet::NONE);
						let colons = p.parse::<T![::]>();
						let ident = p.parse::<T![Ident]>();
						p.set_skip(skip);
						Ok(Self::$ident(colons?, ident?))
					})+
					_ => {
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
						Err(Diagnostic::new(c, Diagnostic::unexpected_pseudo_element))?
					}
				}
			}
		}
		apply_pseudo_element!(match_keyword)
	}
}

pseudo_class!(
	#[derive(ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.selectors"))]
	pub enum LegacyPseudoElement {
		After: CssAtomSet::After,
		Before: CssAtomSet::Before,
		FirstLetter: CssAtomSet::FirstLetter,
		FirstLine: CssAtomSet::FirstLine,
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PseudoElement>(), 44);
		assert_eq!(std::mem::size_of::<LegacyPseudoElement>(), 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PseudoElement, "::after");
		assert_parse!(CssAtomSet::ATOMS, PseudoElement, "::first-letter");
		assert_parse!(CssAtomSet::ATOMS, PseudoElement, "::view-transition");
		assert_parse!(CssAtomSet::ATOMS, LegacyPseudoElement, ":after");
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
