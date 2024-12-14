use bumpalo::collections::Vec;
use hdx_parser::{keyword_typedef, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use crate::css::types::Image;

mod func {
	use hdx_parser::custom_function;
	custom_function!(Symbols, atom!("symbols"));
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Symbols<'a> {
	pub function: func::Symbols,
	pub symbols_type: Option<SymbolsType>,
	pub symbols: Vec<'a, Symbol<'a>>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for Symbols<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<func::Symbols>()
	}
}

impl<'a> Parse<'a> for Symbols<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<func::Symbols>()?;
		let mut symbols = Vec::new_in(p.bump());
		if let Some(close) = p.parse_if_peek::<T![')']>()? {
			return Ok(Self { function, symbols_type: None, symbols, close: Some(close) });
		}
		let symbols_type = p.parse_if_peek::<SymbolsType>()?;
		loop {
			if p.at_end() {
				return Ok(Self { function, symbols_type, symbols, close: None });
			}
			if let Some(close) = p.parse_if_peek::<T![')']>()? {
				return Ok(Self { function, symbols_type, symbols, close: Some(close) });
			}
			if p.peek::<T![String]>() {
				symbols.push(Symbol::String(p.parse::<T![String]>()?));
			} else {
				symbols.push(Symbol::Image(p.parse::<Image>()?));
			}
		}
	}
}

impl<'a> ToCursors for Symbols<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.function.into());
		if let Some(symbols_type) = self.symbols_type {
			s.append(symbols_type.into());
		}
		for symbol in &self.symbols {
			ToCursors::to_cursors(symbol, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Symbol<'a> {
	String(T![String]),
	Image(Image<'a>),
}

impl<'a> ToCursors for Symbol<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::String(c) => s.append(c.into()),
			Self::Image(c) => ToCursors::to_cursors(c, s),
		}
	}
}

// https://drafts.csswg.org/css-counter-styles-3/#typedef-symbols-type
keyword_typedef!(SymbolsType {
	Cyclic: atom!("cyclic"),
	Numeric: atom!("numeric"),
	Alphabetic: atom!("alphabetic"),
	Symbolic: atom!("symbolic"),
	Fixed: atom!("fixed"),
});

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Symbols, 72);
		assert_size!(Symbol, 184);
		assert_size!(SymbolsType, 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Symbols, "symbols(symbolic'+')");
		assert_parse!(Symbols, "symbols(symbolic'*''†''‡')");
	}
}
