use hdx_atom::{atom, Atomizable};
use hdx_parser::{diagnostics, Parse, Parser, Peek, Result as ParserResult, Token};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

pub(crate) use crate::css::units::*;
use crate::macros::keyword_typedef;

// https://drafts.csswg.org/css-display-4/#typedef-display-outside
// <display-outside>  = block | inline | run-in
keyword_typedef!(DisplayOutside { Block: atom!("block"), Inline: atom!("inline"), RunIn: atom!("run-in") });

// https://drafts.csswg.org/css-display-4/#typedef-display-inside
// <display-inside>   = flow | flow-root | table | flex | grid | ruby
keyword_typedef!(DisplayInside {
	Flow: atom!("flow"),
	FlowRoot: atom!("flow-root"),
	Table: atom!("table"),
	Flex: atom!("flex"),
	Grid: atom!("grid"),
	Ruby: atom!("ruby"),
});
