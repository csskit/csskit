use super::prelude::*;
use crate::CompoundSelector;

pseudo_element!(
	/// <https://searchfox.org/wubkat/source/Source/WebCore/css/CSSPseudoSelectors.json>
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitPseudoElement {
		Backdrop: CssAtomSet::_WebkitBackdrop,
		CalendarDatePickerIndicator: CssAtomSet::_WebkitCalendarPickerIndicator,
		CapsLockIndicator: CssAtomSet::_WebkitCapsLockIndicator,
		ColorSwatch: CssAtomSet::_WebkitColorSwatch,
		ColorSwatchWrapper: CssAtomSet::_WebkitColorSwatchWrapper,
		ContactsAutoFillButton: CssAtomSet::_WebkitContactsAutoFillButton,
		CredentialsAutoFillButton: CssAtomSet::_WebkitCredentialsAutoFillButton,
		CreditCardAutoFillButton: CssAtomSet::_WebkitCreditCardAutoFillButton,
		DateAndTimeValue: CssAtomSet::_WebkitDateAndTimeValue,
		DatetimeEdit: CssAtomSet::_WebkitDatetimeEdit,
		DatetimeEditDayField: CssAtomSet::_WebkitDatetimeEditDayField,
		DatetimeEditFieldsWrapper: CssAtomSet::_WebkitDatetimeEditFieldsWrapper,
		DatetimeEditHourField: CssAtomSet::_WebkitDatetimeEditHourField,
		DatetimeEditMeridiemField: CssAtomSet::_WebkitDatetimeEditMeridiemField,
		DatetimeEditMillisecondField: CssAtomSet::_WebkitDatetimeEditMillisecondField,
		DatetimeEditMinute: CssAtomSet::_WebkitDatetimeEditMinute,
		DatetimeEditMinuteField: CssAtomSet::_WebkitDatetimeEditMinuteField,
		DatetimeEditMonthField: CssAtomSet::_WebkitDatetimeEditMonthField,
		DatetimeEditSecondField: CssAtomSet::_WebkitDatetimeEditSecondField,
		DatetimeEditText: CssAtomSet::_WebkitDatetimeEditText,
		DatetimeEditYearField: CssAtomSet::_WebkitDatetimeEditYearField,
		DetailsMarker: CssAtomSet::_WebkitDetailsMarker,
		FileUploadButton: CssAtomSet::_WebkitFileUploadButton, // Alias for `:file-selector-button`
		GenericCueRoot: CssAtomSet::_WebkitGenericCueRoot,
		InnerSpinButton: CssAtomSet::_WebkitInnerSpinButton,
		InputPlaceholder: CssAtomSet::_WebkitInputPlaceholder, // Alias for `:placeholder`
		ListButton: CssAtomSet::_WebkitListButton,
		MediaControls: CssAtomSet::_WebkitMediaControls,
		MediaTextTrackContainer: CssAtomSet::_WebkitMediaTextTrackContainer,
		MediaTextTrackDisplay: CssAtomSet::_WebkitMediaTextTrackDisplay,
		MediaTextTrackDisplayBackdrop: CssAtomSet::_WebkitMediaTextTrackDisplayBackdrop,
		MediaTextTrackRegion: CssAtomSet::_WebkitMediaTextTrackRegion,
		MediaTextTrackRegionContainer: CssAtomSet::_WebkitMediaTextTrackRegionContainer,
		MeterBar: CssAtomSet::_WebkitMeterBar,
		MeterEvenLessGoodValue: CssAtomSet::_WebkitMeterEvenLessGoodValue,
		MeterInnerElement: CssAtomSet::_WebkitMeterInnerElement,
		MeterOptimumValue: CssAtomSet::_WebkitMeterOptimumValue,
		MeterSuboptimumValue: CssAtomSet::_WebkitMeterSuboptimumValue,
		OuterSpinButton: CssAtomSet::_WebkitOuterSpinButton, // Deprecated
		PasswordAutoFillButton: CssAtomSet::_WebkitPasswordAutoFillButton,
		ProgressBar: CssAtomSet::_WebkitProgressBar,
		ProgressInnerElement: CssAtomSet::_WebkitProgressInnerElement,
		ProgressValue: CssAtomSet::_WebkitProgressValue,
		Resizer: CssAtomSet::_WebkitResizer,
		Scrollbar: CssAtomSet::_WebkitScrollbar,
		ScrollbarButton: CssAtomSet::_WebkitScrollbarButton,
		ScrollbarCorner: CssAtomSet::_WebkitScrollbarCorner,
		ScrollbarThumb: CssAtomSet::_WebkitScrollbarThumb,
		ScrollbarTrack: CssAtomSet::_WebkitScrollbarTrack,
		ScrollbarTrackPiece: CssAtomSet::_WebkitScrollbarTrackPiece,
		SearchCancelButton: CssAtomSet::_WebkitSearchCancelButton,
		SearchDecoration: CssAtomSet::_WebkitSearchDecoration,
		SearchResultsButton: CssAtomSet::_WebkitSearchResultsButton,
		SearchResultsDecoration: CssAtomSet::_WebkitSearchResultsDecoration,
		Selection: CssAtomSet::_WebkitSelection,
		SliderContainer: CssAtomSet::_WebkitSliderContainer,
		SliderRunnableTrack: CssAtomSet::_WebkitSliderRunnableTrack,
		SliderThumb: CssAtomSet::_WebkitSliderThumb,
		StrongPasswordAutoFillButton: CssAtomSet::_WebkitStrongPasswordAutoFillButton,
		TextfieldDecorationContainer: CssAtomSet::_WebkitTextfieldDecorationContainer,
		ValidationBubble: CssAtomSet::_WebkitValidationBubble,
		ValidationBubbleArrow: CssAtomSet::_WebkitValidationBubbleArrow,
		ValidationBubbleArrowClipper: CssAtomSet::_WebkitValidationBubbleArrowClipper,
		ValidationBubbleBody: CssAtomSet::_WebkitValidationBubbleBody,
		ValidationBubbleHeading: CssAtomSet::_WebkitValidationBubbleHeading,
		ValidationBubbleIcon: CssAtomSet::_WebkitValidationBubbleIcon,
		ValidationBubbleMessage: CssAtomSet::_WebkitValidationBubbleMessage,
		ValidationBubbleTextBlock: CssAtomSet::_WebkitValidationBubbleTextBlock,
	}
);

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum WebkitFunctionalPseudoElement<'a> {
	Distributed(WebkitDistrubutedFunctionalPseudoElement<'a>),
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoElement<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let colons = p.parse::<T![::]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if p.equals_atom(c, &CssAtomSet::_WebkitDistributed) {
			let value = p.parse::<CompoundSelector>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Distributed(WebkitDistrubutedFunctionalPseudoElement { colons, function, value, close }))
		} else {
			Err(Diagnostic::new(c, Diagnostic::unexpected_function))?
		}
	}
}

#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitDistrubutedFunctionalPseudoElement<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub colons: T![::],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: Option<T![')']>,
}

#[node]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum WebkitFunctionalPseudoClass<'a> {
	Any(WebkitAnyFunctionalPseudoClass<'a>),
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoClass<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if p.equals_atom(c, &CssAtomSet::_WebkitAny) {
			let value = p.parse::<CompoundSelector>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Any(WebkitAnyFunctionalPseudoClass { colon, function, value, close }))
		} else {
			Err(Diagnostic::new(c, Diagnostic::unexpected_function))?
		}
	}
}

#[node]
#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct WebkitAnyFunctionalPseudoClass<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	pub close: Option<T![')']>,
}

pseudo_class!(
	/// <https://searchfox.org/wubkat/source/Source/WebCore/css/CSSPseudoSelectors.json>
	//  <https://searchfox.org/wubkat/source/Source/WebCore/css/SelectorChecker.cpp#1263-1273>
	#[node]
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WebkitPseudoClass {
		AnimatingFullScreenTransition: CssAtomSet::_WebkitAnimatingFullScreenTransition,
		AnyLink: CssAtomSet::_WebkitAnyLink,  // Alias for :anyLink
		Autofill: CssAtomSet::_WebkitAutofill, // Alias for :autofill
		AutofillAndObscured: CssAtomSet::_WebkitAutofillAndObscured,
		AutofillStrongPassword: CssAtomSet::_WebkitAutofillStrongPassword,
		AutofillStrongPasswordViewable: CssAtomSet::_WebkitAutofillStrongPasswordViewable,
		CornerPresent: CssAtomSet::CornerPresent,
		Decrement: CssAtomSet::Decrement,
		DoubleButton: CssAtomSet::DoubleButton,
		Drag: CssAtomSet::_WebkitDrag,
		End: CssAtomSet::End,
		FullPageMedia: CssAtomSet::_WebkitFullPageMedia,
		FullScreen: CssAtomSet::_WebkitFullScreen,
		FullScreenAncestor: CssAtomSet::_WebkitFullScreenAncestor,
		FullScreenControlsHidden: CssAtomSet::_WebkitFullScreenControlsHidden,
		FullScreenDocument: CssAtomSet::_WebkitFullScreenDocument,
		Horizontal: CssAtomSet::Horizontal,
		Increment: CssAtomSet::Increment,
		NoButton: CssAtomSet::NoButton,
		SingleButton: CssAtomSet::SingleButton,
		Start: CssAtomSet::Start,
		Vertical: CssAtomSet::Vertical,
		WindowInactive: CssAtomSet::WindowInactive,
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, SelectorList};
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WebkitPseudoElement, "::-webkit-search-results-decoration");
		assert_parse!(CssAtomSet::ATOMS, WebkitPseudoClass, ":horizontal");
		assert_parse!(CssAtomSet::ATOMS, WebkitPseudoClass, ":window-inactive");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::-webkit-scrollbar-button:horizontal:decrement");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::-webkit-scrollbar-thumb:window-inactive");
	}
}
