use css_parse::{pseudo_class, pseudo_element};
use hdx_proc_macro::visit;

use crate::{Visit, Visitable};

#[visit]
pseudo_element!(OPseudoElement {
	InnerSpinButton: "-o-inner-spin-button",
	OuterSpinButton: "-o-outer-spin-button",
	Placeholder: "-o-placeholder",
	Scrollbar: "-o-scrollbar",
	ScrollbarThumb: "-o-scrollbar-thumb",
	ScrollbarTrack: "-o-scrollbar-track",
	ScrollbarTrackPiece: "-o-scrollbar-track-piece",
	Selection: "-o-selection",
});

impl<'a> Visitable<'a> for OPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_o_pseudo_element(self);
	}
}

#[visit]
pseudo_class!(OPseudoClass { Prefocus: "-o-prefocus" });

impl<'a> Visitable<'a> for OPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_o_pseudo_class(self);
	}
}
