use hdx_atom::atom;
use hdx_lexer::{Include, Kind};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, Token};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Nth(i32, i32);

impl<'a> Parse<'a> for Nth {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<Token![Number]>() {
			parser.hop(token);
			return Ok(Self(0, parser.parse_number(token) as i32));
		} else if let Some(token) = parser.peek::<Token![Ident]>() {
			match parser.parse_atom_lower(token) {
				atom!("even") => {
					parser.hop(token);
					return Ok(Self(2, 0));
				}
				atom!("odd") => {
					parser.hop(token);
					return Ok(Self(2, 1));
				}
				_ => {}
			}
		}

		let a;
		let mut b_sign = 0;
		let mut token = *parser.parse::<Token![Any]>()?;
		if matches!(token.char(), Some('+')) {
			token = *parser.parse_with::<Token![Any]>(Include::Whitespace)?;
		}
		if !matches!(token.kind(), Kind::Number | Kind::Dimension | Kind::Ident) {
			Err(diagnostics::Unexpected(token, token.span()))?
		}
		if token.is_float() {
			Err(diagnostics::ExpectedInt(parser.parse_number(token), token.span()))?
		}
		match parser.parse_atom(token) {
			atom!("-n") | atom!("-N") => {
				if token.is_int() {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
				a = -1;
			}
			atom!("n") | atom!("N") => {
				a = if token.is_int() { parser.parse_number(token) as i32 } else { 1 };
			}
			atom!("n-") | atom!("N-") => {
				b_sign = -1;
				a = if token.is_int() { parser.parse_number(token) as i32 } else { 1 };
			}
			anb => {
				let mut chars = anb.chars();
				let mut c = chars.next();
				a = if token.is_int() {
					parser.parse_number(token) as i32
				} else if matches!(c, Some('-')) {
					c = chars.next();
					-1
				} else {
					1
				};
				if !matches!(c, Some('n') | Some('N')) {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
				if let Ok(b) = chars.as_str().parse::<i32>() {
					return Ok(Self(a, b));
				} else if !chars.as_str().is_empty() {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
			}
		}

		if b_sign == 0 {
			if let Some(token) = parser.peek::<Token![+]>() {
				b_sign = 1;
				parser.hop(token);
			} else if let Some(token) = parser.peek::<Token![-]>() {
				b_sign = -1;
				parser.hop(token);
			}
		}

		let b = if let Some(token) = parser.peek::<Token![Number]>() {
			if token.is_float() {
				Err(diagnostics::ExpectedInt(parser.parse_number(token), token.span()))?
			}
			if token.has_sign() && b_sign != 0 {
				Err(diagnostics::ExpectedUnsigned(parser.parse_number(token), token.span()))?
			}
			if b_sign == 0 {
				b_sign = 1;
			}
			let i = parser.parse_number(token);
			parser.hop(token);
			(i.abs() as i32) * b_sign
		} else {
			0
		};
		Ok(Self(a, b))
	}
}

impl<'a> WriteCss<'a> for Nth {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self(2, 0) => atom!("even").write_css(sink),
			Self(2, 1) => atom!("odd").write_css(sink),
			Self(0, 0) => sink.write_char('0'),
			Self(1, 0) => sink.write_char('n'),
			Self(-1, 0) => sink.write_str("-n"),
			Self(a, 0) => sink.write_str(&format!("{}n", a)),
			Self(0, b) => sink.write_str(&format!("{}", b)),
			Self(1, b) => sink.write_str(&format!("n{:+}", b)),
			Self(-1, b) => sink.write_str(&format!("-n{:+}", b)),
			Self(a, b) => sink.write_str(&format!("{}n{:+}", a, b)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Nth, 8);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Nth, "odd");
		assert_parse!(Nth, "ODD", "odd");
		assert_parse!(Nth, "eVeN", "even");
		assert_parse!(Nth, "5");
		assert_parse!(Nth, "n");
		assert_parse!(Nth, "+n", "n");
		assert_parse!(Nth, "+N", "n");
		assert_parse!(Nth, "-n");
		assert_parse!(Nth, "+5", "5");
		assert_parse!(Nth, "5n");
		assert_parse!(Nth, "+5n", "5n");
		assert_parse!(Nth, "-5n");
		assert_parse!(Nth, "n-4");
		assert_parse!(Nth, "-n-4");
		assert_parse!(Nth, "+n-4", "n-4");
		assert_parse!(Nth, "+n+4", "n+4");
		assert_parse!(Nth, "+n-123456789", "n-123456789");
		assert_parse!(Nth, "2n", "even");
		assert_parse!(Nth, "2n+1", "odd");
		assert_parse!(Nth, "+2n+1", "odd");
		assert_parse!(Nth, "-2n+1");
		assert_parse!(Nth, "-2n-1");
		assert_parse!(Nth, "+2n-1", "2n-1");
		assert_parse!(Nth, "3n+4");
		assert_parse!(Nth, "3n+1");
		assert_parse!(Nth, "n + 3", "n+3");
		assert_parse!(Nth, "-n+3");
		assert_parse!(Nth, "-n+3");

		// Ported from https://github.com/web-platform-tests/wpt/blob/c1247636413abebe66ca11a2ca3476de771c99cb/css/selectors/parsing/parse-anplusb.html
		assert_parse!(Nth, "1n+0", "n");
		assert_parse!(Nth, "n+0", "n");
		assert_parse!(Nth, "n", "n");
		assert_parse!(Nth, "-n+0", "-n");
		assert_parse!(Nth, "-n", "-n");
		assert_parse!(Nth, "N", "n");
		assert_parse!(Nth, "+n+3", "n+3");
		assert_parse!(Nth, " +n + 7 ", "n+7");
		assert_parse!(Nth, "  N- 123", "n-123");
		assert_parse!(Nth, "n- 10", "n-10");
		assert_parse!(Nth, "-n\n- 1", "-n-1");
		assert_parse!(Nth, " 23n\n\n+\n\n123 ", "23n+123");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Nth, "3n + -6");
		assert_parse_error!(Nth, "3 n");
		assert_parse_error!(Nth, "+ 2n");
		assert_parse_error!(Nth, "+ 2");

		// Ported from https://github.com/web-platform-tests/wpt/blob/c1247636413abebe66ca11a2ca3476de771c99cb/css/selectors/parsing/parse-anplusb.html
		assert_parse_error!(Nth, "n- 1 2");
		assert_parse_error!(Nth, "n-b1");
		assert_parse_error!(Nth, "n-+1");
		assert_parse_error!(Nth, "n-1n");
		assert_parse_error!(Nth, "-n -b1");
		assert_parse_error!(Nth, "-1n- b1");
		assert_parse_error!(Nth, "-n-13b1");
		assert_parse_error!(Nth, "-n-+1");
		assert_parse_error!(Nth, "-n+n");
		assert_parse_error!(Nth, "+ 1n");
		assert_parse_error!(Nth, "  n +12 3");
		assert_parse_error!(Nth, "  12 n ");
		assert_parse_error!(Nth, "+12n-0+1");
		assert_parse_error!(Nth, "+12N -- 1");
		assert_parse_error!(Nth, "+12 N ");
		assert_parse_error!(Nth, "+ n + 7");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serializes() {
		assert_json!(Nth, "odd", { "node": [2, 1], "start": 0, "end": 3 });
		assert_json!(Nth, "3n+1", { "node": [3, 1], "start": 0, "end": 4 });
	}
}
