use hdx_atom::atom;
use hdx_derive::Atomizable;
use hdx_parser::{expect_ignore_case, todo, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://searchfox.org/wubkat/source/Source/WebCore/css/CSSPseudoSelectors.json
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum WebkitPseudoElement {
	#[atomizable("-webkit-calendar-picker-indicator")]
	CalendarDatePickerIndicator,
	#[atomizable("-webkit-caps-lock-indicator")]
	CapsLockIndicator,
	#[atomizable("-webkit-color-swatch")]
	ColorSwatch,
	#[atomizable("-webkit-color-swatch-wrapper")]
	ColorSwatchWrapper,
	#[atomizable("-webkit-contacts-auto-fill-button")]
	ContactsAutoFillButton,
	#[atomizable("-webkit-credentials-auto-fill-button")]
	CredentialsAutoFillButton,
	#[atomizable("-webkit-credit-card-auto-fill-button")]
	CreditCardAutoFillButton,
	#[atomizable("-webkit-date-and-time-value")]
	DateAndTimeValue,
	#[atomizable("-webkit-datetime-edit")]
	DatetimeEdit,
	#[atomizable("-webkit-datetime-edit-day-field")]
	DatetimeEditDayField,
	#[atomizable("-webkit-datetime-edit-fields-wrapper")]
	DatetimeEditFieldsWrapper,
	#[atomizable("-webkit-datetime-edit-hour-field")]
	DatetimeEditHourField,
	#[atomizable("-webkit-datetime-edit-meridiem-field")]
	DatetimeEditMeridiemField,
	#[atomizable("-webkit-datetime-edit-millisecond-field")]
	DatetimeEditMillisecondField,
	#[atomizable("-webkit-datetime-edit-minute")]
	DatetimeEditMinute,
	#[atomizable("-webkit-datetime-edit-minute-field")]
	DatetimeEditMinuteField,
	#[atomizable("-webkit-datetime-edit-month-field")]
	DatetimeEditMonthField,
	#[atomizable("-webkit-datetime-edit-second-field")]
	DatetimeEditSecondField,
	#[atomizable("-webkit-datetime-edit-text")]
	DatetimeEditText,
	#[atomizable("-webkit-datetime-edit-year-field")]
	DatetimeEditYearField,
	#[atomizable("-webkit-details-marker")]
	DetailsMarker,
	#[atomizable("-webkit-file-upload-button")]
	FileUploadButton, // Alias for `:file-selector-button`
	#[atomizable("-webkit-generic-cue-root")]
	GenericCueRoot,
	#[atomizable("-webkit-input-placeholder")]
	InputPlaceholder, // Alias for `:placeholder`
	#[atomizable("-webkit-inner-spin-button")]
	InnerSpinButton,
	#[atomizable("-webkit-list-button")]
	ListButton,
	#[atomizable("-webkit-media-text-track-container")]
	MediaTextTrackContainer,
	#[atomizable("-webkit-media-text-track-display")]
	MediaTextTrackDisplay,
	#[atomizable("-webkit-media-text-track-display-backdrop")]
	MediaTextTrackDisplayBackdrop,
	#[atomizable("-webkit-media-text-track-region")]
	MediaTextTrackRegion,
	#[atomizable("-webkit-media-text-track-region-container")]
	MediaTextTrackRegionContainer,
	#[atomizable("-webkit-meter-bar")]
	MeterBar,
	#[atomizable("-webkit-meter-even-less-good-value")]
	MeterEvenLessGoodValue,
	#[atomizable("-webkit-meter-inner-element")]
	MeterInnerElement,
	#[atomizable("-webkit-meter-optimum-value")]
	MeterOptimumValue,
	#[atomizable("-webkit-meter-suboptimum-value")]
	MeterSuboptimumValue,
	#[atomizable("-webkit-outer-spin-button")]
	OuterSpinButton, // Deprecated
	#[atomizable("-webkit-progress-bar")]
	ProgressBar,
	#[atomizable("-webkit-progress-inner-element")]
	ProgressInnerElement,
	#[atomizable("-webkit-progress-value")]
	ProgressValue,
	#[atomizable("-webkit-resizer")]
	Resizer,
	#[atomizable("-webkit-scrollbar")]
	Scrollbar,
	#[atomizable("-webkit-scrollbar-button")]
	ScrollbarButton,
	#[atomizable("-webkit-scrollbar-corner")]
	ScrollbarCorner,
	#[atomizable("-webkit-scrollbar-thumb")]
	ScrollbarThumb,
	#[atomizable("-webkit-scrollbar-track")]
	ScrollbarTrack,
	#[atomizable("-webkit-scrollbar-track-piece")]
	ScrollbarTrackPiece,
	#[atomizable("-webkit-search-cancel-button")]
	SearchCancelButton,
	#[atomizable("-webkit-search-decoration")]
	SearchDecoration,
	#[atomizable("-webkit-search-results-button")]
	SearchResultsButton,
	#[atomizable("-webkit-slider-container")]
	SliderContainer,
	#[atomizable("-webkit-slider-runnable-track")]
	SliderRunnableTrack,
	#[atomizable("-webkit-slider-thumb")]
	SliderThumb,
	#[atomizable("-webkit-password-auto-fill-button")]
	PasswordAutoFillButton,
	#[atomizable("-webkit-textfield-decoration-container")]
	TextfieldDecorationContainer,
	#[atomizable("-webkit-validation-bubble")]
	ValidationBubble,
	#[atomizable("-webkit-validation-bubble-arrow")]
	ValidationBubbleArrow,
	#[atomizable("-webkit-validation-bubble-arrow-clipper")]
	ValidationBubbleArrowClipper,
	#[atomizable("-webkit-validation-bubble-body")]
	ValidationBubbleBody,
	#[atomizable("-webkit-validation-bubble-heading")]
	ValidationBubbleHeading,
	#[atomizable("-webkit-validation-bubble-icon")]
	ValidationBubbleIcon,
	#[atomizable("-webkit-validation-bubble-message")]
	ValidationBubbleMessage,
	#[atomizable("-webkit-validation-bubble-text-block")]
	ValidationBubbleTextBlock,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum WebkitFunctionalPseudoElement {
	Distributed(()), // atom!("-webkit-distributed")
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoElement {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case! { parser.next(), Token::Function(_):
			atom!("-webkit-distributed") => todo!(parser),
		}
	}
}

impl<'a> WriteCss<'a> for WebkitFunctionalPseudoElement {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		std::todo!("Cannot write non-standard webkit functional pseudos yet")
	}
}

// TODO: functional pseudos
// -webkit-any() alias of :is()
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum WebkitFunctionalPseudoClass {
	Any(()),
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoClass {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case! { parser.next(), Token::Function(_):
			atom!("-webkit-any") => todo!(parser),
		}
	}
}

impl<'a> WriteCss<'a> for WebkitFunctionalPseudoClass {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		std::todo!("Cannot write webkit functional pseudo class yet")
	}
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum WebkitPseudoClass {
	#[atomizable("-webkit-any-link")]
	AnyLink, // Alias for :any-link
	#[atomizable("-webkit-autofill")]
	Autofill, // Alias for :autofill
	#[atomizable("-webkit-autofill-and-obscured")]
	AutofillAndObscured,
	#[atomizable("-webkit-autofill-strong-password")]
	AutofillStrongPassword,
	#[atomizable("-webkit-autofill-strong-password-viewable")]
	AutofillStrongPasswordViewable,
	#[atomizable("-webkit-drag")]
	Drag,
}
