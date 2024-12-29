use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{diagnostics, keyword_set, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

use crate::types::Image;

// https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Symbols<'a> {
	pub function: T![Function],
	pub symbols_type: Option<SymbolsType>,
	pub symbols: Vec<'a, Symbol<'a>>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for Symbols<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "symbols")
	}
}

impl<'a> Parse<'a> for Symbols<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if !p.eq_ignore_ascii_case(c, "symbols") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
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
keyword_set!(SymbolsType {
	Cyclic: "cyclic",
	Numeric: "numeric",
	Alphabetic: "alphabetic",
	Symbolic: "symbolic",
	Fixed: "fixed",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Symbols>(), 80);
		assert_eq!(std::mem::size_of::<Symbol>(), 208);
		assert_eq!(std::mem::size_of::<SymbolsType>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Symbols, "symbols(symbolic'+')");
		assert_parse!(Symbols, "symbols(symbolic'*''†''‡')");
	}
}
