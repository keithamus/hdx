use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet, Span};
use hdx_parser::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

use super::CompoundSelector;

// https://searchfox.org/wubkat/source/Source/WebCore/css/CSSPseudoSelectors.json
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum WebkitPseudoElement {
	CalendarDatePickerIndicator(T![::], T![Ident]),
	CapsLockIndicator(T![::], T![Ident]),
	ColorSwatch(T![::], T![Ident]),
	ColorSwatchWrapper(T![::], T![Ident]),
	ContactsAutoFillButton(T![::], T![Ident]),
	CredentialsAutoFillButton(T![::], T![Ident]),
	CreditCardAutoFillButton(T![::], T![Ident]),
	DateAndTimeValue(T![::], T![Ident]),
	DatetimeEdit(T![::], T![Ident]),
	DatetimeEditDayField(T![::], T![Ident]),
	DatetimeEditFieldsWrapper(T![::], T![Ident]),
	DatetimeEditHourField(T![::], T![Ident]),
	DatetimeEditMeridiemField(T![::], T![Ident]),
	DatetimeEditMillisecondField(T![::], T![Ident]),
	DatetimeEditMinute(T![::], T![Ident]),
	DatetimeEditMinuteField(T![::], T![Ident]),
	DatetimeEditMonthField(T![::], T![Ident]),
	DatetimeEditSecondField(T![::], T![Ident]),
	DatetimeEditText(T![::], T![Ident]),
	DatetimeEditYearField(T![::], T![Ident]),
	DetailsMarker(T![::], T![Ident]),
	FileUploadButton(T![::], T![Ident]), // Alias for `:file-selector-button`
	GenericCueRoot(T![::], T![Ident]),
	InputPlaceholder(T![::], T![Ident]), // Alias for `:placeholder`
	InnerSpinButton(T![::], T![Ident]),
	ListButton(T![::], T![Ident]),
	MediaTextTrackContainer(T![::], T![Ident]),
	MediaTextTrackDisplay(T![::], T![Ident]),
	MediaTextTrackDisplayBackdrop(T![::], T![Ident]),
	MediaTextTrackRegion(T![::], T![Ident]),
	MediaTextTrackRegionContainer(T![::], T![Ident]),
	MeterBar(T![::], T![Ident]),
	MeterEvenLessGoodValue(T![::], T![Ident]),
	MeterInnerElement(T![::], T![Ident]),
	MeterOptimumValue(T![::], T![Ident]),
	MeterSuboptimumValue(T![::], T![Ident]),
	OuterSpinButton(T![::], T![Ident]), // Deprecated
	ProgressBar(T![::], T![Ident]),
	ProgressInnerElement(T![::], T![Ident]),
	ProgressValue(T![::], T![Ident]),
	Resizer(T![::], T![Ident]),
	Scrollbar(T![::], T![Ident]),
	ScrollbarButton(T![::], T![Ident]),
	ScrollbarCorner(T![::], T![Ident]),
	ScrollbarThumb(T![::], T![Ident]),
	ScrollbarTrack(T![::], T![Ident]),
	ScrollbarTrackPiece(T![::], T![Ident]),
	SearchCancelButton(T![::], T![Ident]),
	SearchDecoration(T![::], T![Ident]),
	SearchResultsButton(T![::], T![Ident]),
	SliderContainer(T![::], T![Ident]),
	SliderRunnableTrack(T![::], T![Ident]),
	SliderThumb(T![::], T![Ident]),
	PasswordAutoFillButton(T![::], T![Ident]),
	TextfieldDecorationContainer(T![::], T![Ident]),
	ValidationBubble(T![::], T![Ident]),
	ValidationBubbleArrow(T![::], T![Ident]),
	ValidationBubbleArrowClipper(T![::], T![Ident]),
	ValidationBubbleBody(T![::], T![Ident]),
	ValidationBubbleHeading(T![::], T![Ident]),
	ValidationBubbleIcon(T![::], T![Ident]),
	ValidationBubbleMessage(T![::], T![Ident]),
	ValidationBubbleTextBlock(T![::], T![Ident]),
}

impl<'a> Parse<'a> for WebkitPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let double_colon = p.parse::<T![::]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let double_colon = double_colon?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-webkit-calendar-picker-indicator") => Self::CalendarDatePickerIndicator(double_colon, ident),
			atom!("-webkit-caps-lock-indicator") => Self::CapsLockIndicator(double_colon, ident),
			atom!("-webkit-color-swatch") => Self::ColorSwatch(double_colon, ident),
			atom!("-webkit-color-swatch-wrapper") => Self::ColorSwatchWrapper(double_colon, ident),
			atom!("-webkit-contacts-auto-fill-button") => Self::ContactsAutoFillButton(double_colon, ident),
			atom!("-webkit-credentials-auto-fill-button") => Self::CredentialsAutoFillButton(double_colon, ident),
			atom!("-webkit-credit-card-auto-fill-button") => Self::CreditCardAutoFillButton(double_colon, ident),
			atom!("-webkit-date-and-time-value") => Self::DateAndTimeValue(double_colon, ident),
			atom!("-webkit-datetime-edit") => Self::DatetimeEdit(double_colon, ident),
			atom!("-webkit-datetime-edit-day-field") => Self::DatetimeEditDayField(double_colon, ident),
			atom!("-webkit-datetime-edit-fields-wrapper") => Self::DatetimeEditFieldsWrapper(double_colon, ident),
			atom!("-webkit-datetime-edit-hour-field") => Self::DatetimeEditHourField(double_colon, ident),
			atom!("-webkit-datetime-edit-meridiem-field") => Self::DatetimeEditMeridiemField(double_colon, ident),
			atom!("-webkit-datetime-edit-millisecond-field") => Self::DatetimeEditMillisecondField(double_colon, ident),
			atom!("-webkit-datetime-edit-minute") => Self::DatetimeEditMinute(double_colon, ident),
			atom!("-webkit-datetime-edit-minute-field") => Self::DatetimeEditMinuteField(double_colon, ident),
			atom!("-webkit-datetime-edit-month-field") => Self::DatetimeEditMonthField(double_colon, ident),
			atom!("-webkit-datetime-edit-second-field") => Self::DatetimeEditSecondField(double_colon, ident),
			atom!("-webkit-datetime-edit-text") => Self::DatetimeEditText(double_colon, ident),
			atom!("-webkit-datetime-edit-year-field") => Self::DatetimeEditYearField(double_colon, ident),
			atom!("-webkit-details-marker") => Self::DetailsMarker(double_colon, ident),
			atom!("-webkit-file-upload-button") => Self::FileUploadButton(double_colon, ident), // Alias for `:file-selector-button`
			atom!("-webkit-generic-cue-root") => Self::GenericCueRoot(double_colon, ident),
			atom!("-webkit-input-placeholder") => Self::InputPlaceholder(double_colon, ident), // Alias for `:placeholder`
			atom!("-webkit-inner-spin-button") => Self::InnerSpinButton(double_colon, ident),
			atom!("-webkit-list-button") => Self::ListButton(double_colon, ident),
			atom!("-webkit-media-text-track-container") => Self::MediaTextTrackContainer(double_colon, ident),
			atom!("-webkit-media-text-track-display") => Self::MediaTextTrackDisplay(double_colon, ident),
			atom!("-webkit-media-text-track-display-backdrop") => {
				Self::MediaTextTrackDisplayBackdrop(double_colon, ident)
			}
			atom!("-webkit-media-text-track-region") => Self::MediaTextTrackRegion(double_colon, ident),
			atom!("-webkit-media-text-track-region-container") => {
				Self::MediaTextTrackRegionContainer(double_colon, ident)
			}
			atom!("-webkit-meter-bar") => Self::MeterBar(double_colon, ident),
			atom!("-webkit-meter-even-less-good-value") => Self::MeterEvenLessGoodValue(double_colon, ident),
			atom!("-webkit-meter-inner-element") => Self::MeterInnerElement(double_colon, ident),
			atom!("-webkit-meter-optimum-value") => Self::MeterOptimumValue(double_colon, ident),
			atom!("-webkit-meter-suboptimum-value") => Self::MeterSuboptimumValue(double_colon, ident),
			atom!("-webkit-outer-spin-button") => Self::OuterSpinButton(double_colon, ident), // Deprecated
			atom!("-webkit-progress-bar") => Self::ProgressBar(double_colon, ident),
			atom!("-webkit-progress-inner-element") => Self::ProgressInnerElement(double_colon, ident),
			atom!("-webkit-progress-value") => Self::ProgressValue(double_colon, ident),
			atom!("-webkit-resizer") => Self::Resizer(double_colon, ident),
			atom!("-webkit-scrollbar") => Self::Scrollbar(double_colon, ident),
			atom!("-webkit-scrollbar-button") => Self::ScrollbarButton(double_colon, ident),
			atom!("-webkit-scrollbar-corner") => Self::ScrollbarCorner(double_colon, ident),
			atom!("-webkit-scrollbar-thumb") => Self::ScrollbarThumb(double_colon, ident),
			atom!("-webkit-scrollbar-track") => Self::ScrollbarTrack(double_colon, ident),
			atom!("-webkit-scrollbar-track-piece") => Self::ScrollbarTrackPiece(double_colon, ident),
			atom!("-webkit-search-cancel-button") => Self::SearchCancelButton(double_colon, ident),
			atom!("-webkit-search-decoration") => Self::SearchDecoration(double_colon, ident),
			atom!("-webkit-search-results-button") => Self::SearchResultsButton(double_colon, ident),
			atom!("-webkit-slider-container") => Self::SliderContainer(double_colon, ident),
			atom!("-webkit-slider-runnable-track") => Self::SliderRunnableTrack(double_colon, ident),
			atom!("-webkit-slider-thumb") => Self::SliderThumb(double_colon, ident),
			atom!("-webkit-password-auto-fill-button") => Self::PasswordAutoFillButton(double_colon, ident),
			atom!("-webkit-textfield-decoration-container") => Self::TextfieldDecorationContainer(double_colon, ident),
			atom!("-webkit-validation-bubble") => Self::ValidationBubble(double_colon, ident),
			atom!("-webkit-validation-bubble-arrow") => Self::ValidationBubbleArrow(double_colon, ident),
			atom!("-webkit-validation-bubble-arrow-clipper") => Self::ValidationBubbleArrowClipper(double_colon, ident),
			atom!("-webkit-validation-bubble-body") => Self::ValidationBubbleBody(double_colon, ident),
			atom!("-webkit-validation-bubble-heading") => Self::ValidationBubbleHeading(double_colon, ident),
			atom!("-webkit-validation-bubble-icon") => Self::ValidationBubbleIcon(double_colon, ident),
			atom!("-webkit-validation-bubble-message") => Self::ValidationBubbleMessage(double_colon, ident),
			atom!("-webkit-validation-bubble-text-block") => Self::ValidationBubbleTextBlock(double_colon, ident),
			atom => Err(diagnostics::UnexpectedFunction(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors for WebkitPseudoElement {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::CalendarDatePickerIndicator(colon, ident)
			| Self::CapsLockIndicator(colon, ident)
			| Self::ColorSwatch(colon, ident)
			| Self::ColorSwatchWrapper(colon, ident)
			| Self::ContactsAutoFillButton(colon, ident)
			| Self::CredentialsAutoFillButton(colon, ident)
			| Self::CreditCardAutoFillButton(colon, ident)
			| Self::DateAndTimeValue(colon, ident)
			| Self::DatetimeEdit(colon, ident)
			| Self::DatetimeEditDayField(colon, ident)
			| Self::DatetimeEditFieldsWrapper(colon, ident)
			| Self::DatetimeEditHourField(colon, ident)
			| Self::DatetimeEditMeridiemField(colon, ident)
			| Self::DatetimeEditMillisecondField(colon, ident)
			| Self::DatetimeEditMinute(colon, ident)
			| Self::DatetimeEditMinuteField(colon, ident)
			| Self::DatetimeEditMonthField(colon, ident)
			| Self::DatetimeEditSecondField(colon, ident)
			| Self::DatetimeEditText(colon, ident)
			| Self::DatetimeEditYearField(colon, ident)
			| Self::DetailsMarker(colon, ident)
			| Self::FileUploadButton(colon, ident)
			| Self::GenericCueRoot(colon, ident)
			| Self::InputPlaceholder(colon, ident)
			| Self::InnerSpinButton(colon, ident)
			| Self::ListButton(colon, ident)
			| Self::MediaTextTrackContainer(colon, ident)
			| Self::MediaTextTrackDisplay(colon, ident)
			| Self::MediaTextTrackDisplayBackdrop(colon, ident)
			| Self::MediaTextTrackRegion(colon, ident)
			| Self::MediaTextTrackRegionContainer(colon, ident)
			| Self::MeterBar(colon, ident)
			| Self::MeterEvenLessGoodValue(colon, ident)
			| Self::MeterInnerElement(colon, ident)
			| Self::MeterOptimumValue(colon, ident)
			| Self::MeterSuboptimumValue(colon, ident)
			| Self::OuterSpinButton(colon, ident)
			| Self::ProgressBar(colon, ident)
			| Self::ProgressInnerElement(colon, ident)
			| Self::ProgressValue(colon, ident)
			| Self::Resizer(colon, ident)
			| Self::Scrollbar(colon, ident)
			| Self::ScrollbarButton(colon, ident)
			| Self::ScrollbarCorner(colon, ident)
			| Self::ScrollbarThumb(colon, ident)
			| Self::ScrollbarTrack(colon, ident)
			| Self::ScrollbarTrackPiece(colon, ident)
			| Self::SearchCancelButton(colon, ident)
			| Self::SearchDecoration(colon, ident)
			| Self::SearchResultsButton(colon, ident)
			| Self::SliderContainer(colon, ident)
			| Self::SliderRunnableTrack(colon, ident)
			| Self::SliderThumb(colon, ident)
			| Self::PasswordAutoFillButton(colon, ident)
			| Self::TextfieldDecorationContainer(colon, ident)
			| Self::ValidationBubble(colon, ident)
			| Self::ValidationBubbleArrow(colon, ident)
			| Self::ValidationBubbleArrowClipper(colon, ident)
			| Self::ValidationBubbleBody(colon, ident)
			| Self::ValidationBubbleHeading(colon, ident)
			| Self::ValidationBubbleIcon(colon, ident)
			| Self::ValidationBubbleMessage(colon, ident)
			| Self::ValidationBubbleTextBlock(colon, ident) => {
				ToCursors::to_cursors(colon, s);
				s.append(ident.into());
			}
		}
	}
}

impl<'a> Visitable<'a> for WebkitPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_webkit_pseudo_element(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
		match p.parse_atom_lower(c) {
			atom!("-webkit-distributed") => {
				let value = p.parse::<CompoundSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Distributed(WebkitDistrubutedFunctionalPseudoElement { colons, function, value, close }))
			}
			atom => Err(diagnostics::UnexpectedFunction(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors for WebkitFunctionalPseudoElement<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Distributed(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl<'a> Visitable<'a> for WebkitFunctionalPseudoElement<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_webkit_functional_pseudo_element(self);
		match self {
			Self::Distributed(pseudo) => {
				pseudo.value.accept(v);
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct WebkitDistrubutedFunctionalPseudoElement<'a> {
	pub colons: T![::],
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for WebkitDistrubutedFunctionalPseudoElement<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.colons, s);
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(open) = self.close {
			s.append(open.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum WebkitFunctionalPseudoClass<'a> {
	Any(WebkitAnyFunctionalPseudoClass<'a>),
}

impl<'a> Visitable<'a> for WebkitFunctionalPseudoClass<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_webkit_functional_pseudo_class(self);
		match self {
			Self::Any(pseudo) => {
				pseudo.value.accept(v);
			}
		}
	}
}

impl<'a> Parse<'a> for WebkitFunctionalPseudoClass<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		match p.parse_atom_lower(c) {
			atom!("-webkit-any") => {
				let value = p.parse::<CompoundSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Ok(Self::Any(WebkitAnyFunctionalPseudoClass { colon, function, value, close }))
			}
			atom => Err(diagnostics::UnexpectedFunction(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors for WebkitFunctionalPseudoClass<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Any(c) => ToCursors::to_cursors(c, s),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct WebkitAnyFunctionalPseudoClass<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: CompoundSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for WebkitAnyFunctionalPseudoClass<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.colon, s);
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(open) = self.close {
			s.append(open.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum WebkitPseudoClass {
	AnimatingFullScreenTransition(T![:], T![Ident]),
	AnyLink(T![:], T![Ident]),  // Alias for :any-link
	Autofill(T![:], T![Ident]), // Alias for :autofill
	AutofillAndObscured(T![:], T![Ident]),
	AutofillStrongPassword(T![:], T![Ident]),
	AutofillStrongPasswordViewable(T![:], T![Ident]),
	Drag(T![:], T![Ident]),
	FullPageMedia(T![:], T![Ident]),
	FullScreen(T![:], T![Ident]),
	FullScreenAncestor(T![:], T![Ident]),
	FullScreenControlsHidden(T![:], T![Ident]),
	FullScreenDocument(T![:], T![Ident]),
}

impl<'a> Parse<'a> for WebkitPseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colon = p.parse::<T![:]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colon = colon?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-webkit-animating-full-screen-transition") => Self::AnimatingFullScreenTransition(colon, ident),
			atom!("-webkit-any-link") => Self::AnyLink(colon, ident), // Alias for :any-link
			atom!("-webkit-autofill") => Self::Autofill(colon, ident), // Alias for :autofill
			atom!("-webkit-autofill-and-obscured") => Self::AutofillAndObscured(colon, ident),
			atom!("-webkit-autofill-strong-password") => Self::AutofillStrongPassword(colon, ident),
			atom!("-webkit-autofill-strong-password-viewable") => Self::AutofillStrongPasswordViewable(colon, ident),
			atom!("-webkit-drag") => Self::Drag(colon, ident),
			atom!("-webkit-full-page-media") => Self::FullPageMedia(colon, ident),
			atom!("-webkit-full-screen") => Self::FullScreen(colon, ident),
			atom!("-webkit-full-screen-ancestor") => Self::FullScreenAncestor(colon, ident),
			atom!("-webkit-full-screen-controls-hidden") => Self::FullScreenControlsHidden(colon, ident),
			atom!("-webkit-full-screen-document") => Self::FullScreenDocument(colon, ident),
			atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors for WebkitPseudoClass {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::AnimatingFullScreenTransition(colon, ident)
			| Self::AnyLink(colon, ident)
			| Self::Autofill(colon, ident)
			| Self::AutofillAndObscured(colon, ident)
			| Self::AutofillStrongPassword(colon, ident)
			| Self::AutofillStrongPasswordViewable(colon, ident)
			| Self::Drag(colon, ident)
			| Self::FullPageMedia(colon, ident)
			| Self::FullScreen(colon, ident)
			| Self::FullScreenAncestor(colon, ident)
			| Self::FullScreenControlsHidden(colon, ident)
			| Self::FullScreenDocument(colon, ident) => {
				ToCursors::to_cursors(colon, s);
				s.append(ident.into());
			}
		}
	}
}

impl From<&WebkitPseudoClass> for Span {
	fn from(value: &WebkitPseudoClass) -> Self {
		match value {
			WebkitPseudoClass::AnimatingFullScreenTransition(colon, ident)
			| WebkitPseudoClass::AnyLink(colon, ident)
			| WebkitPseudoClass::Autofill(colon, ident)
			| WebkitPseudoClass::AutofillAndObscured(colon, ident)
			| WebkitPseudoClass::AutofillStrongPassword(colon, ident)
			| WebkitPseudoClass::AutofillStrongPasswordViewable(colon, ident)
			| WebkitPseudoClass::Drag(colon, ident)
			| WebkitPseudoClass::FullPageMedia(colon, ident)
			| WebkitPseudoClass::FullScreen(colon, ident)
			| WebkitPseudoClass::FullScreenAncestor(colon, ident)
			| WebkitPseudoClass::FullScreenControlsHidden(colon, ident)
			| WebkitPseudoClass::FullScreenDocument(colon, ident) => Into::<Span>::into(colon) + ident.into(),
		}
	}
}

impl<'a> Visitable<'a> for WebkitPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_webkit_pseudo_class(self);
	}
}
