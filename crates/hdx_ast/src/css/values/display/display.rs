use bitmask_enum::bitmask;
use hdx_atom::{atom, Atom};
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{diagnostics, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

// https://drafts.csswg.org/css-display-4/#propdef-display
#[derive(Value, Default)]
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
		// valid list item must be either isolated, or with a <display-outside> or flow/flow-root
		self.bits & 0b1000_1100 == 0
			&& (self.bits & 0b0011_0011 > 0 || self.bits == Self::ListItem.bits)
			&& !self.contains(Self::Flex)
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
			Self::RunIn => Some(atom!("run-in")),
			Self::Block => Some(atom!("block")),
			Self::Inline => Some(atom!("inline")),
			_ => None,
		}
	}

	#[inline]
	fn inside_to_atom(&self) -> Option<Atom> {
		match self.inside_bits() {
			Self::Flow => Some(atom!("flow")),
			Self::FlowRoot => Some(atom!("flow-root")),
			Self::Flex => Some(atom!("flex")),
			Self::Grid => Some(atom!("grid")),
			Self::Ruby => Some(atom!("ruby")),
			Self::Table => Some(atom!("table")),
			_ => None,
		}
	}
}

impl<'a> Parse<'a> for Display {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let span = parser.span();
		// Certain values can only be used in a "standalone way" and so complete the
		// value:
		let single_value = match parser.peek() {
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				// <display-box>
				atom!("none") => Some(Self::None),
				atom!("contents") => Some(Self::Contents),
				// <display-legacy>
				atom!("inline-block") => Some(Self::InlineBlock),
				atom!("inline-table") => Some(Self::InlineTable),
				atom!("inline-flex") => Some(Self::InlineFlex),
				atom!("inline-grid") => Some(Self::InlineGrid),
				// <display-internal>
				atom!("table-row-group") => Some(Self::TableRowGroup),
				atom!("table-header-group") => Some(Self::TableHeaderGroup),
				atom!("table-footer-group") => Some(Self::TableFooterGroup),
				atom!("table-row") => Some(Self::TableRow),
				atom!("table-cell") => Some(Self::TableCell),
				atom!("table-column-group") => Some(Self::TableColumnGroup),
				atom!("table-column") => Some(Self::TableColumn),
				atom!("table-caption") => Some(Self::TableCaption),
				atom!("ruby-base") => Some(Self::RubyBase),
				atom!("ruby-text") => Some(Self::RubyText),
				atom!("ruby-base-container") => Some(Self::RubyBaseContainer),
				atom!("ruby-text-container") => Some(Self::RubyTextContainer),
				_ => None,
			},
			_ => None,
		};
		if let Some(value) = single_value {
			parser.advance();
			return Ok(value);
		}

		// If a legacy/internal/box value is not applied then it must be a pair/triplet
		let mut value = Self::None;
		while let Token::Ident(atom) = parser.next() {
			match atom.to_ascii_lowercase() {
				// <display-outside>
				atom!("block") if !value.has_outside() => value |= Self::Block,
				atom!("inline") if !value.has_outside() => value |= Self::Inline,
				atom!("run-in") if !value.has_outside() => value |= Self::RunIn,
				// <display-inside>
				atom!("flow") if !value.has_inside() => value |= Self::Flow,
				atom!("flow-root") if !value.has_inside() => value |= Self::FlowRoot,
				atom!("flex") if !value.has_inside() => value |= Self::Flex,
				atom!("grid") if !value.has_inside() => value |= Self::Grid,
				atom!("ruby") if !value.has_inside() => value |= Self::Ruby,
				atom!("table") if !value.has_inside() => value |= Self::Table,
				// <display-listitem>
				atom!("list-item") if !value.has_list_item() => value |= Self::ListItem,
				_ => unexpected_ident!(parser, atom),
			}
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
			&Self::None => atom!("none").write_css(sink),
			&Self::Contents => atom!("contents").write_css(sink),
			&Self::InlineBlock if self.is_legacy() => atom!("inline-block").write_css(sink),
			&Self::InlineTable if self.is_legacy() => atom!("inline-table").write_css(sink),
			&Self::InlineFlex if self.is_legacy() => atom!("inline-flex").write_css(sink),
			&Self::InlineGrid if self.is_legacy() => atom!("inline-grid").write_css(sink),
			&Self::TableRowGroup if self.is_internal() => atom!("table-row-group").write_css(sink),
			&Self::TableHeaderGroup if self.is_internal() => atom!("table-header-group").write_css(sink),
			&Self::TableFooterGroup if self.is_internal() => atom!("table-footer-group").write_css(sink),
			&Self::TableRow if self.is_internal() => atom!("table-row").write_css(sink),
			&Self::TableCell if self.is_internal() => atom!("table-cell").write_css(sink),
			&Self::TableColumnGroup if self.is_internal() => atom!("table-column-group").write_css(sink),
			&Self::TableColumn if self.is_internal() => atom!("table-column").write_css(sink),
			&Self::TableCaption if self.is_internal() => atom!("table-caption").write_css(sink),
			&Self::RubyBase if self.is_internal() => atom!("ruby-base").write_css(sink),
			&Self::RubyText if self.is_internal() => atom!("ruby-text").write_css(sink),
			&Self::RubyBaseContainer if self.is_internal() => atom!("ruby-base-container").write_css(sink),
			&Self::RubyTextContainer if self.is_internal() => atom!("ruby-text-container").write_css(sink),
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
					atom!("list-item").write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Display, 1);
	}

	#[test]
	fn test_to_atoms() {
		assert_eq!(Display::FlowRoot.inside_to_atom(), Some(atom!("flow-root")));
		assert_eq!(Display::Flex.inside_to_atom(), Some(atom!("flex")));
		assert_eq!((Display::Block | Display::Flex).inside_to_atom(), Some(atom!("flex")));
		assert_eq!((Display::RunIn | Display::Flow).inside_to_atom(), Some(atom!("flow")));
	}

	#[test]
	fn test_writes() {
		// Parsing a display value should be written identically
		assert_parse!(Display, "none");
		assert_parse!(Display, "contents");
		assert_parse!(Display, "list-item");
		assert_parse!(Display, "block flow");
		assert_parse!(Display, "block flow-root");
		assert_parse!(Display, "inline flow");
		assert_parse!(Display, "inline flow-root");
		assert_parse!(Display, "run-in flow");
		assert_parse!(Display, "block flow list-item");
		assert_parse!(Display, "inline flow list-item");
		assert_parse!(Display, "block flex");
		assert_parse!(Display, "inline flex");
		assert_parse!(Display, "block grid");
		assert_parse!(Display, "inline grid");
		assert_parse!(Display, "inline ruby");
		assert_parse!(Display, "block ruby");
		assert_parse!(Display, "block table");
		assert_parse!(Display, "inline table");
	}

	#[test]
	fn test_errors() {
		// Parsing a display value should be written identically
		assert_parse_error!(Display, "none contents");
		assert_parse_error!(Display, "block contents");
		assert_parse_error!(Display, "list-item table");
		assert_parse_error!(Display, "list-item flex");
		assert_parse_error!(Display, "list-item grid");
		assert_parse_error!(Display, "list-item ruby");
		assert_parse_error!(Display, "ruby list-item");
		assert_parse_error!(Display, "block block");
		assert_parse_error!(Display, "flow flow-root");
		assert_parse_error!(Display, "inline inline");
		assert_parse_error!(Display, "inline inline-table");
		assert_parse_error!(Display, "block inline-grid");
	}
}
