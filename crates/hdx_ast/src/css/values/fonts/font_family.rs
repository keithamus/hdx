use hdx_atom::{atom, Atom};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{unexpected, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::{Value, Writable};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFamily(pub SmallVec<[Spanned<SingleFontFamily>; 1]>);

#[derive(Writable, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleFontFamily {
	#[writable(String)]
	Named(Atom, QuoteStyle),
	Generic(Atom),
	// Generic Font Families
	Serif, // atom!("serif")
	#[default]
	SansSerif, // atom!("sans-serif")
	Cursive, // atom!("cursive")
	Fantasy, // atom!("fantasy")
	Monospace, // atom!("monospace")
	SystemUi, // atom!("system-ui")
	Math,  // atom!("math")
	Fangsong, // atom!("fangsong")
	Kai,   // atom!("kai")
	Nastaliq, // atom!("nastaliq")
	UiSerif, // atom!("ui-serif")
	UiMonospace, // atom!("ui-monospace")
	UiRounded, // atom!("ui-rounded")
	// <system-family-name>
	Caption,      // atom!("caption")
	Icon,         // atom!("icon")
	Menu,         // atom!("menu")
	MessageBox,   // atom!("message-box")
	SmallCaption, // atom!("small-caption")
	StatusBar,    // atom!("status-bar")
}

impl<'a> Value for FontFamily {}

impl<'a> Parse<'a> for SingleFontFamily {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let value = match parser.cur() {
			Token::Ident(ident) => {
				parser.advance();
				match ident.to_ascii_lowercase() {
					atom!("serif") => Self::Serif,
					atom!("sans-serif") => Self::SansSerif,
					atom!("cursive") => Self::Cursive,
					atom!("fantasy") => Self::Fantasy,
					atom!("monospace") => Self::Monospace,
					atom!("system-ui") => Self::SystemUi,
					atom!("math") => Self::Math,
					atom!("ui-serif") => Self::UiSerif,
					atom!("ui-monospace") => Self::UiMonospace,
					atom!("ui-rounded") => Self::UiRounded,
					atom!("caption") => Self::Caption,
					atom!("icon") => Self::Icon,
					atom!("menu") => Self::Menu,
					atom!("message-box") => Self::MessageBox,
					atom!("small-caption") => Self::SmallCaption,
					atom!("status-bar") => Self::StatusBar,
					_ => Self::Named(ident, QuoteStyle::None),
				}
			}
			Token::Function(atom!("generic")) => {
				parser.advance();
				match parser.cur() {
					Token::Ident(ident) => Self::Generic(ident),
					token => unexpected!(parser, token),
				}
			}
			Token::String(atom, quote) => {
				parser.advance();
				Self::Named(atom, quote)
			}
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> Parse<'a> for FontFamily {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			let value = SingleFontFamily::parse_spanned(parser)?;
			values.push(value);
			match parser.cur() {
				Token::Comma => {
					parser.advance();
				}
				_ => {
					break;
				}
			}
		}
		Ok(FontFamily(values))
	}
}

impl<'a> WriteCss<'a> for FontFamily {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(time) = iter.next() {
			time.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FontFamily, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FontFamily, "serif");
		assert_parse!(FontFamily, "Arial, sans-serif");
		assert_parse!(FontFamily, "'Gill Sans MS', Arial, system-ui, sans-serif");
	}

	#[test]
	fn test_minify() {
		assert_minify!(FontFamily, "Arial, sans-serif", "Arial,sans-serif");
		assert_minify!(
			FontFamily,
			"'Gill Sans MS', Arial, system-ui, sans-serif",
			"\"Gill Sans MS\",Arial,system-ui,sans-serif"
		);
	}
}
