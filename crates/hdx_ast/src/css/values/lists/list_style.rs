use hdx_atom::atom;
use hdx_derive::Value;
use hdx_lexer::Token;
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult};

use crate::macros::*;

use super::{ListStyleImage, ListStylePosition, ListStyleType};

// https://drafts.csswg.org/css-lists/#list-style-property
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ListStyle(pub ListStylePosition, pub ListStyleImage, pub ListStyleType);

impl<'a> Parse<'a> for ListStyle {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut first = None;
		let mut second = ListStyleImage::None;
		let mut third = ListStyleType::None;
		let mut nones = 0;
		loop {
			if matches!(parser.peek(), Token::Semicolon | Token::Eof | Token::RightParen | Token::RightCurly) {
				break;
			}
			if matches!(parser.peek(), Token::Ident(atom) if atom.to_ascii_lowercase() == atom!("none")) {
				parser.advance();
				nones += 1;
				if nones > 2 {
					unexpected!(parser);
				}
				continue;
			}
			if first.is_none() {
				first = ListStylePosition::try_parse(parser).ok();
				if first.is_some() {
					continue;
				}
			}
			if second == ListStyleImage::None && nones < 2 {
				if third != ListStyleType::None && nones == 1 {
					unexpected!(parser);
				}
				if let Ok(val) = ListStyleImage::try_parse(parser) {
					second = val;
					continue;
				}
			}
			if third == ListStyleType::None && nones < 2 {
				if second != ListStyleImage::None && nones == 1 {
					unexpected!(parser);
				}
				if let Ok(val) = ListStyleType::try_parse(parser) {
					third = val;
					continue;
				}
			}
			break;
		}
		if first.is_none() && second == ListStyleImage::None && third == ListStyleType::None && nones == 0 {
			hdx_parser::unexpected!(parser);
		}
		Ok(Self(first.unwrap_or(ListStylePosition::default()), second, third))
	}
}

write_simple_shorthand!(ListStyle, ListStylePosition, ListStyleImage, ListStyleType);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ListStyle, 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ListStyle, "none", "outside none none");
		assert_parse!(ListStyle, "outside url(bullet.png) none");
		assert_parse!(ListStyle, "none url(bullet.png)", "outside url(bullet.png) none");
		assert_parse!(ListStyle, "disc", "outside none disc");
		assert_parse!(ListStyle, "symbols('+')", "outside none symbols(symbolic '+')");
		assert_parse!(ListStyle, "outside radial-gradient(closest-corner circle, white, #7f4e20, white) none");
		// Duplicate side is allowed because the second one will be a custom CounteStyle
		assert_parse!(ListStyle, "outside outside", "outside none outside");
	}

	#[test]
	fn test_minify() {
		assert_minify!(ListStyle, "outside none none", "none");
		assert_minify!(ListStyle, "outside symbols('+')", "symbols(\"+\")");
	}

	#[test]
	fn test_errors() {
		// Empty!
		assert_parse_error!(ListStyle, "");
		// 3 nones
		assert_parse_error!(ListStyle, "none none none");
		// Duplicate type
		assert_parse_error!(ListStyle, "disc disc");
		// Duplicate image
		assert_parse_error!(ListStyle, "url(bullet.png) url(dot.png)");
		// Invalid None
		assert_parse_error!(ListStyle, "none disc url(bullet.png)");
	}
}
