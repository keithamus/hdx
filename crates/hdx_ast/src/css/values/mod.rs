use hdx_derive::Value;
use hdx_parser::{todo, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

/// Values
pub mod align;
pub mod anchor_position;
pub mod animations;
pub mod backgrounds;
pub mod borders;
pub mod r#box;
pub mod r#break;
pub mod cascade;
pub mod color;
pub mod color_adjust;
pub mod color_hdr;
pub mod conditional;
pub mod contain;
pub mod content;
pub mod display;
pub mod exclusions;
pub mod flexbox;
pub mod fonts;
pub mod gcpm;
pub mod grid;
pub mod images;
pub mod inline;
pub mod line_grid;
pub mod link_params;
pub mod lists;
pub mod logical;
pub mod multicol;
pub mod nav;
pub mod overflow;
pub mod overscroll;
pub mod page;
pub mod page_floats;
pub mod position;
pub mod regions;
pub mod rhythm;
pub mod round_display;
pub mod ruby;
pub mod scroll_anchoring;
pub mod scroll_animations;
pub mod scroll_snap;
pub mod scrollbars;
pub mod shapes;
pub mod size_adjust;
pub mod sizing;
pub mod speech;
pub mod tables;
pub mod text;
pub mod text_decor;
pub mod transforms;
pub mod transitions;
pub mod ui;
#[allow(clippy::module_inception)]
pub mod values;
pub mod variables;
pub mod view_transitions;
pub mod viewport;
pub mod will_change;
pub mod writing_modes;

#[allow(ambiguous_glob_reexports)]
pub use align::*;
pub use anchor_position::*;
pub use animations::*;
pub use backgrounds::*;
pub use borders::*;
pub use cascade::*;
pub use color::*;
pub use color_adjust::*;
pub use color_hdr::*;
pub use conditional::*;
pub use contain::*;
pub use content::*;
pub use display::*;
pub use exclusions::*;
pub use flexbox::*;
pub use fonts::*;
pub use gcpm::*;
pub use grid::*;
pub use images::*;
pub use inline::*;
pub use line_grid::*;
pub use link_params::*;
pub use lists::*;
pub use logical::*;
pub use multicol::*;
pub use nav::*;
pub use overflow::*;
pub use overscroll::*;
pub use page::*;
pub use page_floats::*;
pub use position::*;
pub use r#box::*;
pub use r#break::*;
pub use regions::*;
pub use rhythm::*;
pub use round_display::*;
pub use ruby::*;
pub use scroll_anchoring::*;
pub use scroll_animations::*;
pub use scroll_snap::*;
pub use scrollbars::*;
pub use shapes::*;
pub use size_adjust::*;
pub use sizing::*;
pub use speech::*;
pub use tables::*;
pub use text::*;
pub use text_decor::*;
pub use transforms::*;
pub use transitions::*;
pub use ui::*;
pub use values::*;
pub use variables::*;
pub use view_transitions::*;
pub use viewport::*;
pub use will_change::*;
pub use writing_modes::*;

// TODO!
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Todo {
	#[default]
	Todo,
}

impl<'a> Parse<'a> for Todo {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		todo!(parser)
	}
}

impl<'a> WriteCss<'a> for Todo {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> WriterResult {
		Err(std::fmt::Error)
	}
}
