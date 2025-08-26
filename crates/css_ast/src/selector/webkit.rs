use css_parse::{Cursor, Parse, Parser, Result as ParserResult, T, diagnostics, pseudo_class, pseudo_element};
use csskit_derives::{ToCursors, Visitable};

use super::CompoundSelector;

pseudo_element!(
	/// <https://searchfox.org/wubkat/source/Source/WebCore/css/CSSPseudoSelectors.json>
	#[derive(Visitable)]
	#[visit(self)]
	pub enum WebkitPseudoElement {
		CalendarDatePickerIndicator: "-webkit-calendar-picker-indicator",
		CapsLockIndicator: "-webkit-caps-lock-indicator",
		ColorSwatch: "-webkit-color-swatch",
		ColorSwatchWrapper: "-webkit-color-swatch-wrapper",
		ContactsAutoFillButton: "-webkit-contacts-auto-fill-button",
		CredentialsAutoFillButton: "-webkit-credentials-auto-fill-button",
		CreditCardAutoFillButton: "-webkit-credit-card-auto-fill-button",
		DateAndTimeValue: "-webkit-date-and-time-value",
		DatetimeEdit: "-webkit-datetime-edit",
		DatetimeEditDayField: "-webkit-datetime-edit-day-field",
		DatetimeEditFieldsWrapper: "-webkit-datetime-edit-fields-wrapper",
		DatetimeEditHourField: "-webkit-datetime-edit-hour-field",
		DatetimeEditMeridiemField: "-webkit-datetime-edit-meridiem-field",
		DatetimeEditMillisecondField: "-webkit-datetime-edit-millisecond-field",
		DatetimeEditMinute: "-webkit-datetime-edit-minute",
		DatetimeEditMinuteField: "-webkit-datetime-edit-minute-field",
		DatetimeEditMonthField: "-webkit-datetime-edit-month-field",
		DatetimeEditSecondField: "-webkit-datetime-edit-second-field",
		DatetimeEditText: "-webkit-datetime-edit-text",
		DatetimeEditYearField: "-webkit-datetime-edit-year-field",
		DetailsMarker: "-webkit-details-marker",
		FileUploadButton: "-webkit-file-upload-button", // Alias for `:file-selector-button`
		GenericCueRoot: "-webkit-generic-cue-root",
		InputPlaceholder: "-webkit-input-placeholder", // Alias for `:placeholder`
		InnerSpinButton: "-webkit-inner-spin-button",
		ListButton: "-webkit-list-button",
		MediaTextTrackContainer: "-webkit-media-text-track-container",
		MediaTextTrackDisplay: "-webkit-media-text-track-display",
		MediaTextTrackDisplayBackdrop: "-webkit-media-text-track-display-backdrop",
		MediaTextTrackRegion: "-webkit-media-text-track-region",
		MediaTextTrackRegionContainer: "-webkit-media-text-track-region-container",
		MeterBar: "-webkit-meter-bar",
		MeterEvenLessGoodValue: "-webkit-meter-even-less-good-value",
		MeterInnerElement: "-webkit-meter-inner-element",
		MeterOptimumValue: "-webkit-meter-optimum-value",
		MeterSuboptimumValue: "-webkit-meter-suboptimum-value",
		OuterSpinButton: "-webkit-outer-spin-button", // Deprecated
		ProgressBar: "-webkit-progress-bar",
		ProgressInnerElement: "-webkit-progress-inner-element",
		ProgressValue: "-webkit-progress-value",
		Resizer: "-webkit-resizer",
		Scrollbar: "-webkit-scrollbar",
		ScrollbarButton: "-webkit-scrollbar-button",
		ScrollbarCorner: "-webkit-scrollbar-corner",
		ScrollbarThumb: "-webkit-scrollbar-thumb",
		ScrollbarTrack: "-webkit-scrollbar-track",
		ScrollbarTrackPiece: "-webkit-scrollbar-track-piece",
		SearchCancelButton: "-webkit-search-cancel-button",
		SearchDecoration: "-webkit-search-decoration",
		SearchResultsButton: "-webkit-search-results-button",
		SliderContainer: "-webkit-slider-container",
		SliderRunnableTrack: "-webkit-slider-runnable-track",
		SliderThumb: "-webkit-slider-thumb",
		PasswordAutoFillButton: "-webkit-password-auto-fill-button",
		TextfieldDecorationContainer: "-webkit-textfield-decoration-container",
		ValidationBubble: "-webkit-validation-bubble",
		ValidationBubbleArrow: "-webkit-validation-bubble-arrow",
		ValidationBubbleArrowClipper: "-webkit-validation-bubble-arrow-clipper",
		ValidationBubbleBody: "-webkit-validation-bubble-body",
		ValidationBubbleHeading: "-webkit-validation-bubble-heading",
		ValidationBubbleIcon: "-webkit-validation-bubble-icon",
		ValidationBubbleMessage: "-webkit-validation-bubble-message",
		ValidationBubbleTextBlock: "-webkit-validation-bubble-text-block",
	}
);

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum WebkitFunctionalPseudoElement<'a> {
	Distributed(WebkitDistrubutedFunctionalPseudoElement<'a>),
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoElement<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colons = p.parse::<T![::]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if p.eq_ignore_ascii_case(c, "-webkit-distributed") {
			let value = p.parse::<CompoundSelector>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Distributed(WebkitDistrubutedFunctionalPseudoElement { colons, function, value, close }))
		} else {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
	}
}

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub struct WebkitDistrubutedFunctionalPseudoElement<'a> {
	#[visit(skip)]
	pub colons: T![::],
	#[visit(skip)]
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	#[visit(skip)]
	pub close: Option<T![')']>,
}

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum WebkitFunctionalPseudoClass<'a> {
	Any(WebkitAnyFunctionalPseudoClass<'a>),
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoClass<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if p.eq_ignore_ascii_case(c, "-webkit-any") {
			let value = p.parse::<CompoundSelector>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			Ok(Self::Any(WebkitAnyFunctionalPseudoClass { colon, function, value, close }))
		} else {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
	}
}

#[derive(ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub struct WebkitAnyFunctionalPseudoClass<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	pub close: Option<T![')']>,
}

pseudo_class!(
	/// <https://searchfox.org/wubkat/source/Source/WebCore/css/CSSPseudoSelectors.json>
	#[derive(Visitable)]
	#[visit(self)]
	pub enum WebkitPseudoClass {
		AnimatingFullScreenTransition: "-webkit-animating-full-screen-transition",
		AnyLink: "-webkit-any-link",  // Alias for :any-link
		Autofill: "-webkit-autofill", // Alias for :autofill
		AutofillAndObscured: "-webkit-autofill-and-obscured",
		AutofillStrongPassword: "-webkit-autofill-strong-password",
		AutofillStrongPasswordViewable: "-webkit-autofill-strong-password-viewable",
		Drag: "-webkit-drag",
		FullPageMedia: "-webkit-full-page-media",
		FullScreen: "-webkit-full-screen",
		FullScreenAncestor: "-webkit-full-screen-ancestor",
		FullScreenControlsHidden: "-webkit-full-screen-controls-hidden",
		FullScreenDocument: "-webkit-full-screen-document",
	}
);
