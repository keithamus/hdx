use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{diagnostics, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{bitmask, Value};

// https://drafts.csswg.org/css-display-4/#propdef-display
#[derive(Value, Default)]
#[bitmask(u8)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum Display {
	// The anatomy of the u8 for Display values is:
	//
	//    |--------- First two bits are special flags
	//    |  |------ Next two bits are Outside values
	//    |  |  |--- Next four bits are Inside/Internal values
	//    v  v  v
	// 0b 00 00 0000
	//    ^  ^  ^
	//    |  |  |--- Inside:   0001 = Flow
	//    |  |  |              0010 = Flow-Root
	//    |  |  |              0011 = Flex
	//    |  |  |              0100 = Grid
	//    |  |  |              0101 = Ruby
	//    |  |  |              1000 = Table
	//    |  |  |
	//    |  |  |--- Internal: 0100 = ruby-base
	//    |  |                 0101 = ruby-text
	//    |  |                 0110 = ruby-base-container
	//    |  |                 0111 = ruby-text-container
	//    |  |                 1000 = table-row-group
	//    |  |                 1001 = table-header-group
	//    |  |                 1010 = table-footer-group
	//    |  |                 1011 = table-row
	//    |  |                 1100 = table-cell
	//    |  |                 1101 = table-column-group
	//    |  |                 1110 = table-column
	//    |  |                 1111 = table-caption
	//    |  |
	//    |  |
	//    |  |------ Outside:  01 = Block
	//    |                    10 = Inline
	//    |                    11 = Run-in
	//    |
	//    |--------- Special:  00 = N/A
	//                         01 = list-item
	//                         10 = <display-legacy>
	//                         11 = <display-internal>
	// Special sentinel values also exist:
	// <display-box>
	// 0b11_11_1111 -> Contents
	// 0b00_00_0000 -> None

	// <display-legacy>
	InlineBlock = 0b1010_0010, // Legacy "inline flow-root"
	InlineTable = 0b1010_1000, // Legacy "inline table"
	InlineFlex = 0b1010_0011,  // Legacy "inline flex"
	InlineGrid = 0b1010_0100,  // Legacy "inline grid"
	// <display-box>
	None = 0b0000_0000,
	Contents = 0b1111_1111,
	// <display-internal>
	TableRowGroup = 0b1100_1000,
	TableHeaderGroup = 0b1100_1001,
	TableFooterGroup = 0b1100_1010,
	TableRow = 0b1100_1011,
	TableCell = 0b1100_1100,
	TableColumnGroup = 0b1100_1101,
	TableColumn = 0b1100_1110,
	TableCaption = 0b1100_1111,
	RubyBase = 0b1100_0100,
	RubyText = 0b1100_0101,
	RubyBaseContainer = 0b1100_0110,
	RubyTextContainer = 0b1100_0111,
	// <display-listitem>
	ListItem = 0b0100_0000,
	// <display-outside>
	Block = 0b0001_0000,
	#[default]
	Inline = 0b0010_0000,
	RunIn = 0b0011_0000,
	// <display-inside>
	Flow = 0b0000_0001,
	FlowRoot = 0b0000_0010,
	Flex = 0b0000_0011,
	Grid = 0b0000_0100,
	Ruby = 0b0000_0101,
	Table = 0b0000_0110,
}

impl Display {
	#[inline]
	fn outside_bits(&self) -> Self {
		Self { bits: self.bits & 0b0011_0000 }
	}

	#[inline]
	fn inside_bits(&self) -> Self {
		Self { bits: self.bits & 0b0000_1111 }
	}

	#[inline]
	fn has_inside(&self) -> bool {
		self.bits & 0b0000_1111 > 0 && self.bits & 0b1000_0000 == 0
	}

	#[inline]
	fn has_outside(&self) -> bool {
		self.bits & 0b0011_0000 > 0
	}

	#[inline]
	fn has_list_item(&self) -> bool {
		self.bits & 0b1100_0000 == 0b0100_0000
	}

	#[inline]
	fn valid_list_item(&self) -> bool {
		self.bits & 0b0000_1100 == 0 && self.bits & 0b0000_0011 > 0 && self.bits & 0b0000_0011 != 0b0000_0011
	}

	#[inline]
	fn is_legacy(&self) -> bool {
		self.bits & 0b1100_0000 == 0b1000_0000
	}

	#[inline]
	fn is_internal(&self) -> bool {
		self.bits & 0b1100_0000 == 0b1100_0000
	}

	#[inline]
	fn is_table(&self) -> bool {
		self.bits & 0b1100_1000 == 0b1100_1000 || self.bits & 0b0000_1000 == 0b0000_1000
	}

	#[inline]
	fn is_ruby(&self) -> bool {
		self.bits & 0b1100_0100 == 0b1100_0100 || self.bits & 0b0000_0101 == 0b0000_0101
	}

	#[inline]
	fn outside_to_atom(&self) -> Option<Atom> {
		match self.outside_bits() {
			Display::RunIn => Some(atom!("run-in")),
			Display::Block => Some(atom!("block")),
			Display::Inline => Some(atom!("inline")),
			_ => None,
		}
	}

	#[inline]
	fn inside_to_atom(&self) -> Option<Atom> {
		match self.inside_bits() {
			Display::Flow => Some(atom!("flow")),
			Display::FlowRoot => Some(atom!("flow-root")),
			Display::Flex => Some(atom!("flex")),
			Display::Grid => Some(atom!("grid")),
			Display::Ruby => Some(atom!("ruby")),
			Display::Table => Some(atom!("table")),
			_ => None,
		}
	}
}

impl<'a> Parse<'a> for Display {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let span = parser.span();
		// Certain values can only be used in a "standalone way" and so complete the
		// value:
		let single_value = match parser.cur() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				// <display-box>
				atom!("none") => Some(Display::None),
				atom!("contents") => Some(Display::Contents),
				// <display-legacy>
				atom!("inline-block") => Some(Display::InlineBlock),
				atom!("inline-table") => Some(Display::InlineTable),
				atom!("inline-flex") => Some(Display::InlineFlex),
				atom!("inline-grid") => Some(Display::InlineGrid),
				// <display-internal>
				atom!("table-row-group") => Some(Display::TableRowGroup),
				atom!("table-header-group") => Some(Display::TableHeaderGroup),
				atom!("table-footer-group") => Some(Display::TableFooterGroup),
				atom!("table-row") => Some(Display::TableRow),
				atom!("table-cell") => Some(Display::TableCell),
				atom!("table-column-group") => Some(Display::TableColumnGroup),
				atom!("table-column") => Some(Display::TableColumn),
				atom!("table-caption") => Some(Display::TableCaption),
				atom!("ruby-base") => Some(Display::RubyBase),
				atom!("ruby-text") => Some(Display::RubyText),
				atom!("ruby-base-container") => Some(Display::RubyBaseContainer),
				atom!("ruby-text-container") => Some(Display::RubyTextContainer),
				_ => None,
			},
			_ => None,
		};
		if let Some(value) = single_value {
			parser.advance();
			return Ok(value);
		}

		// If a legacy/internal/box value is not applied then it must be a pair/triplet
		let mut value = Display::None;
		loop {
			match parser.cur() {
				Token::Ident(atom) => match atom.to_ascii_lowercase() {
					// <display-outside>
					atom!("block") if !value.has_outside() => value |= Display::Block,
					atom!("inline") if !value.has_outside() => value |= Display::Inline,
					atom!("run-in") if !value.has_outside() => value |= Display::RunIn,
					// <display-inside>
					atom!("flow") if !value.has_inside() => value |= Display::Flow,
					atom!("flow-root") if !value.has_inside() => value |= Display::FlowRoot,
					atom!("flex") if !value.has_inside() => value |= Display::Flex,
					atom!("grid") if !value.has_inside() => value |= Display::Grid,
					atom!("ruby") if !value.has_inside() => value |= Display::Ruby,
					atom!("table") if !value.has_inside() => value |= Display::Table,
					// <display-listitem>
					atom!("list-item") if !value.has_list_item() => value |= Display::ListItem,

					atom => unexpected_ident!(parser, atom),
				},
				_ => break,
			}
			parser.advance();
		}
		if value.has_list_item() && !value.valid_list_item() {
			Err(diagnostics::DisplayHasInvalidListItemCombo(value.inside_to_atom().unwrap(), span.end(parser.pos())))?;
		}
		Ok(value)
	}
}

impl<'a> WriteCss<'a> for Display {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			&Self::None => sink.write_str("none"),
			&Self::Contents => sink.write_str("contents"),
			&Self::InlineBlock if self.is_legacy() => sink.write_str("inline-block"),
			&Self::InlineTable if self.is_legacy() => sink.write_str("inline-table"),
			&Self::InlineFlex if self.is_legacy() => sink.write_str("inline-flex"),
			&Self::InlineGrid if self.is_legacy() => sink.write_str("inline-grid"),
			&Self::TableRowGroup if self.is_internal() => sink.write_str("table-row-group"),
			&Self::TableHeaderGroup if self.is_internal() => sink.write_str("table-header-group"),
			&Self::TableFooterGroup if self.is_internal() => sink.write_str("table-footer-group"),
			&Self::TableRow if self.is_internal() => sink.write_str("table-row"),
			&Self::TableCell if self.is_internal() => sink.write_str("table-cell"),
			&Self::TableColumnGroup if self.is_internal() => sink.write_str("table-column-group"),
			&Self::TableColumn if self.is_internal() => sink.write_str("table-column"),
			&Self::TableCaption if self.is_internal() => sink.write_str("table-caption"),
			&Self::RubyBase if self.is_internal() => sink.write_str("ruby-base"),
			&Self::RubyText if self.is_internal() => sink.write_str("ruby-text"),
			&Self::RubyBaseContainer if self.is_internal() => sink.write_str("ruby-base-container"),
			&Self::RubyTextContainer if self.is_internal() => sink.write_str("ruby-text-container"),

			_ => {
				if let Some(outside) = self.outside_to_atom() {
					outside.write_css(sink)?;
					if self.has_inside() || self.has_list_item() {
						sink.write_char(' ')?;
					}
				}
				if let Some(inside) = self.inside_to_atom() {
					inside.write_css(sink)?;
					if self.has_list_item() {
						sink.write_char(' ')?;
					}
				}
				if self.has_list_item() {
					sink.write_str("list-item")?;
				}
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Display>(), 1);
	}

	#[test]
	fn test_to_atoms() {
		assert_eq!(Display::FlowRoot.inside_to_atom(), Some(atom!("flow-root")));
		assert_eq!(Display::Flex.inside_to_atom(), Some(atom!("flex")));
		assert_eq!((Display::Block | Display::Flex).inside_to_atom(), Some(atom!("flex")));
		assert_eq!((Display::RunIn | Display::Flow).inside_to_atom(), Some(atom!("flow")));
	}

	#[test]
	fn test_variants() {
		let allocator = Allocator::default();
		// Parsing a display value should be written identically
		test_write::<Display>(&allocator, "none", "none");
		test_write::<Display>(&allocator, "contents", "contents");
		test_write::<Display>(&allocator, "block flow", "block flow");
		test_write::<Display>(&allocator, "block flow-root", "block flow-root");
		test_write::<Display>(&allocator, "inline flow", "inline flow");
		test_write::<Display>(&allocator, "inline flow-root", "inline flow-root");
		test_write::<Display>(&allocator, "run-in flow", "run-in flow");
		test_write::<Display>(&allocator, "block flow list-item", "block flow list-item");
		test_write::<Display>(&allocator, "inline flow list-item", "inline flow list-item");
		test_write::<Display>(&allocator, "block flex", "block flex");
		test_write::<Display>(&allocator, "inline flex", "inline flex");
		test_write::<Display>(&allocator, "block grid", "block grid");
		test_write::<Display>(&allocator, "inline grid", "inline grid");
		test_write::<Display>(&allocator, "inline ruby", "inline ruby");
		test_write::<Display>(&allocator, "block ruby", "block ruby");
		test_write::<Display>(&allocator, "block table", "block table");
		test_write::<Display>(&allocator, "inline table", "inline table");
	}
}
