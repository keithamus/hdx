use hdx_atom::atom;
use hdx_lexer::{Include, Token};
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Nth(i32, i32);

impl<'a> Parse<'a> for Nth {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut b_sign = 0;
		let a = match parser.peek().clone() {
			Token::Number(b, t) if !t.is_float() => {
				parser.advance();
				return Ok(Self(0, b as i32));
			}
			Token::Dimension(a, _, t) if !t.is_float() => a as i32,
			Token::Ident(atom) => match atom.to_ascii_lowercase() {
				atom!("even") => {
					parser.advance();
					return Ok(Self(2, 0));
				}
				atom!("odd") => {
					parser.advance();
					return Ok(Self(2, 1));
				}
				atom!("-n") => {
					parser.advance();
					-1
				}
				atom!("n") => {
					parser.advance();
					1
				}
				atom!("n-") => {
					parser.advance();
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
			},
			Token::Delim('+') => {
				parser.advance();
				match parser.peek_with(Include::Whitespace) {
					Token::Ident(anb) => {
						if !anb.starts_with('n') && !anb.starts_with('N') {
							unexpected_ident!(parser, anb);
						}
						1
					}
					token => unexpected!(parser, token),
				}
			}
			token => unexpected!(parser, token),
		};
		let b = match parser.peek().clone() {
			Token::Ident(atom!("n"))
			| Token::Ident(atom!("N"))
			| Token::Dimension(_, atom!("n"), _)
			| Token::Dimension(_, atom!("N"), _) => {
				parser.advance();
				None
			}
			Token::Ident(atom) | Token::Dimension(_, atom, _) => {
				parser.advance();
				let mut chars = atom.chars();
				if atom.starts_with('-') {
					if a == -1 {
						chars.next();
					} else {
						unexpected!(parser);
					}
				}
				if !matches!(chars.next(), Some('n') | Some('N')) {
					unexpected_ident!(parser, atom)
				}
				if let Ok(i) = chars.as_str().parse::<i32>() {
					Some(i)
				} else if chars.as_str() != "" {
					unexpected_ident!(parser, atom)
				} else {
					None
				}
			}
			_ => None,
		};
		match parser.peek().clone() {
			Token::Number(i, t) if b.is_none() && !t.is_float() => {
				parser.advance();
				if t.is_signed() && b_sign != 0 {
					unexpected!(parser)
				}
				if b_sign == 0 {
					b_sign = 1
				}
				Ok(Self(a, (i as i32) * b_sign))
			}
			token @ Token::Delim('+') | token @ Token::Delim('-') if b.is_none() => {
				parser.advance();
				if matches!(token, Token::Delim('-')) {
					b_sign = -1;
				} else {
					b_sign = 1;
				}
				match parser.next().clone() {
					Token::Number(i, t) if !t.is_float() && !t.is_signed() => Ok(Self(a, (i as i32) * b_sign)),
					token => unexpected!(parser, token),
				}
			}
			_ => Ok(Self(a, b.unwrap_or(0))),
		}
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
