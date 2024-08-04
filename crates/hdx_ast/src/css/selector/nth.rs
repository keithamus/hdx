use hdx_atom::atom;
use hdx_lexer::Include;
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
		}

		let mut b_sign = 0;
		let a = if let Some(token) = parser.peek::<Token![Dimension]>() {
			if token.is_float() {
				Err(diagnostics::ExpectedInt(parser.parse_number(token), token.span()))?
			}
			parser.hop(token);
			parser.parse_number(token) as i32
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
				atom!("-n") => {
					parser.hop(token);
					-1
				}
				atom!("n") => {
					parser.hop(token);
					1
				}
				atom!("n-") => {
					parser.hop(token);
					b_sign = -1;
					1
				}
				anb => {
					if anb.starts_with('-') {
						-1
					} else {
						1
					}
				}
			}
		} else {
			parser.parse::<Token![Delim]>()?;
			if let Some(next) = parser.peek_with::<Token![Ident]>(Include::Whitespace) {
				let anb = parser.parse_atom_lower(next);
				if !anb.starts_with('n') {
					Err(diagnostics::UnexpectedIdent(anb, next.span()))?
				}
				1
			} else {
				let token = parser.peek::<Token![Any]>().unwrap();
				Err(diagnostics::ExpectedIdent(token, token.span()))?
			}
		};
		let mut token = None;
		if let Some(t) = parser.peek::<Token![Ident]>() {
			parser.hop(t);
			token = Some(t);
		} else if let Some(t) = parser.peek::<Token![Dimension]>() {
			parser.hop(t);
			token = Some(t);
		}
		let b = if let Some(token) = token {
			let str = parser.parse_str(token);
			let mut chars = str.chars();
			if str.starts_with('-') {
				if a == -1 {
					chars.next();
				} else {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
			}
			if !matches!(chars.next(), Some('n') | Some('N')) {
				Err(diagnostics::UnexpectedIdent(parser.parse_atom(token), token.span()))?
			}
			if let Ok(i) = chars.as_str().parse::<i32>() {
				Some(i)
			} else if chars.as_str() != "" {
				Err(diagnostics::UnexpectedIdent(parser.parse_atom(token), token.span()))?
			} else {
				None
			}
		} else {
			None
		};
		if let Some(token) = parser.peek::<Token![Number]>() {
			if b.is_some() || token.is_float() || (!token.has_sign() || b_sign == 0) {
				Err(diagnostics::ExpectedInt(parser.parse_number(token), token.span()))?
			}
			parser.hop(token);
			if b_sign == 0 {
				b_sign = 1
			}
			let i = parser.parse_number(token);
			return Ok(Self(a, (i as i32) * b_sign));
		}
		if let Some(token) = parser.peek::<Token![Delim]>() {
			if b.is_none() {
				let char = token.char().unwrap();
				if char == '-' {
					b_sign = -1;
				} else if char == '+' {
					b_sign = 1;
				} else {
					Err(diagnostics::Unexpected(token, token.span()))?
				}
			}
			let num_token = *parser.parse::<Token![Number]>()?;
			if num_token.is_float() || num_token.has_sign() {
				Err(diagnostics::ExpectedInt(parser.parse_number(num_token), num_token.span()))?
			}
			let i = parser.parse_number(token);
			return Ok(Self(a, (i as i32) * b_sign));
		}
		Ok(Self(a, b.unwrap_or(0)))
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
		assert_parse!(Nth, "5");
		assert_parse!(Nth, "n");
		assert_parse!(Nth, "+n", "n");
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
