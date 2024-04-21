use crate::Value;
use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, FromToken, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-color-adjust/#color-scheme-prop
#[derive(Value, Default, Debug, PartialEq, Hash)]
#[value(Inherits)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum ColorScheme {
	#[default]
	Normal,
	Defined(SmallVec<[ColorSchemeKeyword; 1]>),
	Only(SmallVec<[ColorSchemeKeyword; 1]>),
}

impl<'a> Parse<'a> for ColorScheme {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut only = false;
		let mut keywords = smallvec![];
		while let Token::Ident(ident) = parser.next() {
			match ident.to_ascii_lowercase() {
				atom!("normal") => return Ok(Self::Normal),
				atom!("only") => {
					if only {
						unexpected_ident!(parser, ident)
					}
					only = true;
				}
				atom!("light") => keywords.push(ColorSchemeKeyword::Light),
				atom!("dark") => keywords.push(ColorSchemeKeyword::Dark),
				_ => keywords.push(ColorSchemeKeyword::Custom(ident.clone())),
			}
		}
		if only && keywords.is_empty() {
			unexpected!(parser)
		}
		if only {
			Ok(Self::Only(keywords))
		} else {
			Ok(Self::Defined(keywords))
		}
	}
}

impl<'a> WriteCss<'a> for ColorScheme {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Normal => atom!("normal").write_css(sink),
			Self::Only(kw) | Self::Defined(kw) => {
				let mut iter = kw.iter().peekable();
				while let Some(selector) = iter.next() {
					selector.write_css(sink)?;
					if iter.peek().is_some() {
						sink.write_char(' ')?;
					}
				}
				if matches!(self, Self::Only(_)) {
					sink.write_char(' ')?;
					atom!("only").write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum ColorSchemeKeyword {
	Light,
	Dark,
	Custom(Atom),
}

impl FromToken for ColorSchemeKeyword {
	fn from_token(token: &Token) -> Option<Self> {
		match token {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("light") => Some(Self::Light),
				atom!("dark") => Some(Self::Dark),
				_ => Some(Self::Custom(ident.clone())),
			},
			_ => None,
		}
	}
}

impl<'a> WriteCss<'a> for ColorSchemeKeyword {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Light => atom!("light").write_css(sink),
			Self::Dark => atom!("dark").write_css(sink),
			Self::Custom(kw) => kw.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(ColorScheme, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ColorScheme, "normal");
		assert_parse!(ColorScheme, "light");
		assert_parse!(ColorScheme, "dark");
		assert_parse!(ColorScheme, "light dark");
		assert_parse!(ColorScheme, "dark only");
		assert_parse!(ColorScheme, "light dark magic");
		assert_parse!(ColorScheme, "light dark magic only");
		assert_parse!(ColorScheme, "light dark --other-custom only");
	}

	#[cfg(feature = "serde")]
	#[test]
	fn test_serializes() {
		assert_json!(ColorSchemeKeyword, "light", {
			"node": "light",
			"start": 0,
			"end": 5,
		});
		assert_json!(ColorScheme, "normal", {
			"node": { "type": "normal" },
			"start": 0,
			"end": 6,
		});
		assert_json!(ColorScheme, "light", {
			"node": {
				"type": "defined",
				"value": ["light"],
			},
			"start": 0,
			"end": 5,
		});
	}
}
