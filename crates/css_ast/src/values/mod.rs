mod align;
mod anchor_position;
mod animation_triggers;
mod animations;
mod backgrounds;
mod borders;
mod r#box;
mod r#break;
mod cascade;
mod color;
mod color_adjust;
mod color_hdr;
mod compositing;
mod conditional;
mod contain;
mod content;
mod css2;
mod display;
mod exclusions;
mod fill_stroke;
mod filter_effects;
mod flexbox;
mod fonts;
mod forms;
mod gaps;
mod gcpm;
mod grid;
mod image_animation;
mod images;
mod inline;
mod line_grid;
mod link_params;
mod lists;
mod logical;
mod masking;
mod motion;
mod moz;
mod ms;
mod multicol;
mod nav;
mod non_standard;
mod o;
mod overflow;
mod overscroll;
mod page;
mod page_floats;
mod pointer_animations;
mod pointer_events;
mod position;
mod regions;
mod rhythm;
mod round_display;
mod ruby;
mod scroll_anchoring;
mod scroll_animations;
mod scroll_snap;
mod scrollbars;
mod shaders;
mod shapes;
mod size_adjust;
mod sizing;
mod speech;
mod svg_painting;
mod tables;
mod text;
mod text_decor;
mod transforms;
mod transitions;
mod ui;
mod value;
#[allow(clippy::module_inception)]
mod values;
mod variables;
mod view_transitions;
mod viewport;
mod webkit;
mod will_change;
mod writing_modes;

#[allow(ambiguous_glob_reexports)]
pub use align::*;
pub use anchor_position::*;
pub use animation_triggers::*;
pub use animations::*;
pub use backgrounds::*;
pub use borders::*;
pub use r#box::*;
pub use r#break::*;
pub use cascade::*;
pub use color::*;
pub use color_adjust::*;
pub use color_hdr::*;
pub use compositing::*;
pub use conditional::*;
pub use contain::*;
pub use content::*;
pub use css2::*;
pub use display::*;
pub use exclusions::*;
pub use fill_stroke::*;
pub use filter_effects::*;
pub use flexbox::*;
pub use fonts::*;
pub use forms::*;
pub use gaps::*;
pub use gcpm::*;
pub use grid::*;
pub use image_animation::*;
pub use images::*;
pub use inline::*;
pub use line_grid::*;
// TODO: link_params isn't supported by any engines yet.
#[allow(unused)]
pub use link_params::*;
pub use lists::*;
pub use logical::*;
pub use masking::*;
pub use motion::*;
pub use moz::*;
pub use ms::*;
pub use multicol::*;
pub use nav::*;
pub use non_standard::*;
pub use o::*;
pub use overflow::*;
pub use overscroll::*;
pub use page::*;
pub use page_floats::*;
pub use pointer_animations::*;
pub use pointer_events::*;
pub use position::*;
pub use regions::*;
pub use rhythm::*;
pub use round_display::*;
pub use ruby::*;
pub use scroll_anchoring::*;
pub use scroll_animations::*;
pub use scroll_snap::*;
pub use scrollbars::*;
#[allow(unused)]
pub use shaders::*;
pub use shapes::*;
pub use size_adjust::*;
pub use sizing::*;
pub use speech::*;
pub use svg_painting::*;
pub use tables::*;
pub use text::*;
pub use text_decor::*;
pub use transforms::*;
pub use transitions::*;
pub use ui::*;
pub use value::{
	CalcableSubstitutionFunction, CalcableValue, KeywordSubstitutionFunction, KeywordValue,
	NumericSubstitutionFunction, NumericValue, QuerySubstitutionFunction, QueryValue, SubstitutionFunction, Value,
};
pub(crate) use value::{impl_value_slot_parse, is_substitution_function};
pub use values::*;
// TODO: not sure if this is needed yet.
#[allow(unused)]
pub use variables::*;
pub use view_transitions::*;
pub use viewport::*;
pub use webkit::*;
pub use will_change::*;
pub use writing_modes::*;

// Re-export counter_style rules
pub use crate::rules::{
	AdditiveSymbolsStyleValue, FallbackStyleValue, NegativeStyleValue, PadStyleValue, PrefixStyleValue,
	RangeStyleValue, SpeakAsStyleValue, SuffixStyleValue, SymbolsStyleValue, SystemStyleValue,
};

mod prelude {
	pub(crate) use crate::CssAtomSet;
	pub(crate) use crate::traits::*;
	pub(crate) use csskit_derives::*;
	pub(crate) use csskit_proc_macro::*;
}
