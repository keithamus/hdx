use hdx_atom::{atom, Atomizable};
use hdx_derive::{Atomizable, Writable};
use hdx_lexer::QuoteStyle;
use hdx_parser::{discard, Parse, Parser, Peek, Result as ParserResult, Token};
use hdx_writer::{OutputOption, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

use crate::css::types::Image;

mod func {
	use hdx_parser::custom_function;
	custom_function!(Symbols, atom!("symbols"));
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Symbols<'a>(pub SymbolsType, SmallVec<[Symbol<'a>; 0]>);

impl<'a> Peek<'a> for Symbols<'a> {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<Token![Function]>()
	}
}

impl<'a> Parse<'a> for Symbols<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		parser.parse::<func::Symbols>()?;
		let mut symbol_type = SymbolsType::default();
		let mut symbols = smallvec![];
		if discard!(parser, RightParen) {
			return Ok(Self(symbol_type, symbols));
		}
		if let Some(token) = parser.peek::<Token![Ident]>() {
			if let Some(st) = SymbolsType::from_atom(&parser.parse_atom(token)) {
				parser.hop(token);
				symbol_type = st;
			}
		}
		loop {
			if discard!(parser, RightParen) {
				return Ok(Self(symbol_type, symbols));
			}
			if let Some(token) = parser.peek::<Token![String]>() {
				parser.hop(token);
				symbols.push(Symbol::String(parser.parse_str(token), token.quote_style()));
			} else {
				symbols.push(Symbol::Image(Image::parse(parser)?));
			}
		}
	}
}

impl<'a> WriteCss<'a> for Symbols<'a> {
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
pub enum Symbol<'a> {
	#[writable(String)]
	String(&'a str, QuoteStyle),
	Image(Image<'a>),
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
