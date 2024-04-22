use hdx_derive::Atomizable;

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum OPseudoElement {
	#[atomizable("-o-inner-spin-button")]
	InnerSpinButton,
	#[atomizable("-o-outer-spin-button")]
	OuterSpinButton,
	#[atomizable("-o-placeholder")]
	Placeholder,
	#[atomizable("-o-scrollbar")]
	Scrollbar,
	#[atomizable("-o-scrollbar-thumb")]
	ScrollbarThumb,
	#[atomizable("-o-scrollbar-track")]
	ScrollbarTrack,
	#[atomizable("-o-scrollbar-track-piece")]
	ScrollbarTrackPiece,
	#[atomizable("-o-selection")]
	Selection,
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum OPseudoClass {
	#[atomizable("-o-prefocus")]
	Prefocus,
}
