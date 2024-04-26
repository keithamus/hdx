use hdx_derive::Atomizable;

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum MsPseudoElement {
	#[atomizable("-ms-backdrop")]
	Backdrop,
	#[atomizable("-ms-browse")]
	Browse,
	#[atomizable("-ms-check")]
	Check,
	#[atomizable("-ms-clear")]
	Clear,
	#[atomizable("-ms-expand")]
	Expand,
	#[atomizable("-ms-fill")]
	Fill,
	#[atomizable("-ms-fill-upper")]
	FillUpper,
	#[atomizable("-ms-fill-lower")]
	FillLower,
	#[atomizable("-ms-input-placeholder")]
	InputPlaceholder,
	#[atomizable("-ms-placeholder")]
	Placeholder,
	#[atomizable("-ms-reveal")]
	Reveal,
	#[atomizable("-ms-selection")]
	Selection,
	#[atomizable("-ms-thumb")]
	Thumb,
	#[atomizable("-ms-ticks-after")]
	TicksAfter,
	#[atomizable("-ms-ticks-before")]
	TicksBefore,
	#[atomizable("-ms-tooltip")]
	Tooltip,
	#[atomizable("-ms-track")]
	Track,
	#[atomizable("-ms-value")]
	Value,
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum MsPseudoClass {
	#[atomizable("-ms-fullscreen")]
	Fullscreen,
	#[atomizable("-ms-input-placeholder")]
	InputPlaceholder,
}
