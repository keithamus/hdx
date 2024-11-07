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

// https://drafts.csswg.org/css-display-4/#typedef-display-listitem
// <display-listitem> = <display-outside>? && [ flow | flow-root ]? && list-item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum DisplayListitem {
	ListItem,
	Outside(Option<DisplayOutside>),
	Flow(Option<DisplayOutside>),
	FlowRoot(Option<DisplayOutside>),
}

impl<'a> Peek<'a> for DisplayListitem {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<Token![Ident]>().filter(|token| {
			matches!(
				parser.parse_atom_lower(*token),
				atom!("block")
					| atom!("inline")
					| atom!("run-in")
					| atom!("flow") | atom!("flow-root")
					| atom!("list-item")
			)
		})
	}
}

enum FlowOrFlowRoot {
	None,
	Flow,
	FlowRoot,
}

impl<'a> Parse<'a> for DisplayListitem {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut saw_list_item = false;
		let mut flow = FlowOrFlowRoot::None;
		let mut outside = None;
		loop {
			if let Some(token) = parser.peek::<Token![Ident]>() {
				let atom = parser.parse_atom_lower(token);

				if matches!(atom, atom!("list-item")) {
					if saw_list_item {
						Err(diagnostics::UnexpectedIdent(atom, token.span()))?
					}
					parser.hop(token);
					saw_list_item = true;
					continue;
				}

				if matches!(atom, atom!("flow") | atom!("flow-root")) {
					parser.hop(token);
					flow = if matches!(atom, atom!("flow")) { FlowOrFlowRoot::Flow } else { FlowOrFlowRoot::FlowRoot };
					continue;
				}

				if let Some(val) = DisplayOutside::from_atom(&atom) {
					if outside.is_none() {
						parser.hop(token);
						outside = Some(val);
						continue;
					}
				}

				return match flow {
					FlowOrFlowRoot::None => Ok(Self::Outside(outside)),
					FlowOrFlowRoot::Flow => Ok(Self::Flow(outside)),
					FlowOrFlowRoot::FlowRoot => Ok(Self::FlowRoot(outside)),
				};
			} else if saw_list_item {
				return Ok(Self::ListItem);
			} else {
				let token = parser.peek::<Token![Any]>().unwrap();
				Err(diagnostics::Unexpected(token, token.span()))?
			}
		}
	}
}

impl<'a> WriteCss<'a> for DisplayListitem {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::ListItem => write_css!(sink, atom!("list-item")),
			Self::Outside(outside) => write_css!(sink, outside, atom!("list-item")),
			Self::Flow(outside) => write_css!(sink, outside, atom!("flow"), atom!("list-item")),
			Self::FlowRoot(outside) => write_css!(sink, outside, atom!("flow-root"), atom!("list-item")),
		}
		Ok(())
	}
}

// https://drafts.csswg.org/css-display-4/#typedef-display-internal
// <display-internal> = table-row-group | table-header-group |
//                      table-footer-group | table-row | table-cell |
//                      table-column-group | table-column | table-caption |
//                      ruby-base | ruby-text | ruby-base-container |
//                      ruby-text-container
keyword_typedef!(DisplayInternal {
	TableRowGroup: atom!("table-row-group"),
	TableHeaderGroup: atom!("table-header-group"),
	TableFooterGroup: atom!("table-footer-group"),
	TableRow: atom!("table-row"),
	TableCell: atom!("table-cell"),
	TableColumnGroup: atom!("table-column-group"),
	TableColumn: atom!("table-column"),
	TableCaption: atom!("table-caption"),
	RubyBase: atom!("ruby-base"),
	RubyText: atom!("ruby-text"),
	RubyBaseContainer: atom!("ruby-base-container"),
	RubyTextContainer: atom!("ruby-text-container"),
});

// https://drafts.csswg.org/css-display-4/#typedef-display-box
// <display-box>      = contents | none
keyword_typedef!(DisplayBox { Contents: atom!("contents"), None: atom!("none") });

// https://drafts.csswg.org/css-display-4/#typedef-display-legacy
// <display-legacy>   = inline-block | inline-table | inline-flex | inline-grid
keyword_typedef!(DisplayLegacy {
	InlineBlock: atom!("inline-block"),
	InlineTable: atom!("inline-table"),
	InlineFlex: atom!("inline-flex"),
	InlineGrid: atom!("inline-grid"),
});
