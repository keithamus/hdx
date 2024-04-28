use hdx_atom::{atom, Atom};
use hdx_derive::{Value, Writable};
use hdx_lexer::{QuoteStyle, Token};
use hdx_parser::{discard, expect, unexpected, Parse, Parser, Result as ParserResult, Spanned};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-duration
#[derive(Value, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FontFamily(pub SmallVec<[Spanned<SingleFontFamily>; 1]>);

#[derive(Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleFontFamily {
	#[writable(String)]
	Named(Atom, QuoteStyle),
	#[writable(as_function = "generic")]
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

impl<'a> Parse<'a> for SingleFontFamily {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let value = match parser.next() {
			Token::String(atom, quote) => Self::Named(atom.clone(), *quote),
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
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
				_ => Self::Named(ident.clone(), QuoteStyle::None),
			},
			Token::Function(atom!("generic")) => match parser.next().clone() {
				Token::Ident(ident) => {
					expect!(parser.next(), Token::RightParen);
					Self::Generic(ident)
				}
				token => unexpected!(parser, token),
			},
			token => unexpected!(parser, token),
		};
		Ok(value)
	}
}

impl<'a> Parse<'a> for FontFamily {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			values.push(SingleFontFamily::parse_spanned(parser)?);
			if !discard!(parser, Token::Comma) {
				break;
			}
		}
		Ok(Self(values))
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
		assert_parse!(FontFamily, "generic(foo)");
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
