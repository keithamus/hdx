use hdx_atom::{atom, Atom, Atomizable};
use hdx_derive::{Atomizable, Writable};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{expect_ignore_case, unexpected, Parse, Parser, Result as ParserResult};
use hdx_writer::{OutputOption, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::types::Image;

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Symbols(pub SymbolsType, SmallVec<[Symbol; 0]>);

impl<'a> Parse<'a> for Symbols {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect_ignore_case!(parser.next(), Kind::Function, atom!("symbols"));
		let mut symbol_type = SymbolsType::default();
		let mut symbols = smallvec![];
		match parser.peek() {
			Token::Ident(atom) => {
				if let Some(st) = SymbolsType::from_atom(atom) {
					parser.next();
					symbol_type = st;
				}
			}
			Token::RightParen => {
				parser.next();
				return Ok(Self(symbol_type, symbols));
			}
			_ => {}
		}
		loop {
			match parser.peek().clone() {
				Token::String(atom, style) => {
					parser.next();
					symbols.push(Symbol::String(atom, style));
				}
				Token::Function(_) => {
					symbols.push(Symbol::Image(Image::parse(parser)?));
				}
				Token::RightParen => {
					parser.next();
					return Ok(Self(symbol_type, symbols));
				}
				token => unexpected!(parser, token),
			}
		}
	}
}

impl<'a> WriteCss<'a> for Symbols {
	fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> WriterResult {
		atom!("symbols").write_css(sink)?;
		sink.write_char('(')?;
		if self.0 != SymbolsType::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
			self.0.to_atom().write_css(sink)?;
			sink.write_char(' ')?;
		}
		let mut iter = self.1.iter().peekable();
		while let Some(w) = iter.next() {
			w.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(' ')?;
			}
		}
		sink.write_char(')')
	}
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Writable, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Symbol {
	#[writable(String)]
	String(Atom, QuoteStyle),
	Image(Image),
}

// https://drafts.csswg.org/css-counter-styles-3/#typedef-symbols-type
#[derive(Atomizable, Default, Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum SymbolsType {
	Cyclic,     // atom!("cyclic")
	Numeric,    // atom!("numeric")
	Alphabetic, // atom!("alphabetic")
	#[default]
	Symbolic, // atom!("symbolic")
	Fixed,      // atom!("fixed")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Symbols, 32);
		assert_size!(Symbol, 72);
		assert_size!(SymbolsType, 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Symbols, "symbols(symbolic '+')");
		assert_parse!(Symbols, "symbols(symbolic '*' '†' '‡')");
	}

	#[test]
	fn test_minify() {
		// Drops reundant "symbolic" default
		assert_minify!(Symbols, "symbols(symbolic '+')", "symbols(\"+\")");
		// Minifies UTF-8 escapes
		assert_minify!(
			Symbols,
			"symbols(cyclic '*' '\\2020' '\\2021' '\\A7')",
			"symbols(cyclic \"*\" \"†\" \"‡\" \"§\")"
		);
	}
}
