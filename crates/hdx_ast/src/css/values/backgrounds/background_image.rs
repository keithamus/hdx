use hdx_atom::atom;
use hdx_derive::{Value, Writable};
use hdx_lexer::{Kind, Token};
use hdx_parser::{discard, Parse, Parser, Result as ParserResult, Spanned};
use smallvec::{smallvec, SmallVec};

use crate::css::types::Image;

#[derive(Value, Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BackgroundImage(pub SmallVec<[Spanned<SingleBackgroundImage>; 1]>);

impl<'a> Parse<'a> for BackgroundImage {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			let value = SingleBackgroundImage::parse_spanned(parser)?;
			values.push(value);
			if !discard!(parser, Kind::Comma) {
				break;
			}
		}
		Ok(Self(values))
	}
}

#[derive(Writable, Default, Debug, PartialEq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleBackgroundImage {
	#[default]
	None,
	Image(Image),
}
impl<'a> Parse<'a> for SingleBackgroundImage {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(match parser.peek().clone() {
			Token::Ident(ident) if ident.to_ascii_lowercase() == atom!("none") => {
				parser.next();
				Self::None
			}
			_ => Self::Image(Image::parse(parser)?),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(BackgroundImage, 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BackgroundImage, "none");
		assert_parse!(BackgroundImage, "none, none, none");
		assert_parse!(BackgroundImage, "none, url(foo.png), linear-gradient(to bottom, red, blue)");
	}

	#[test]
	fn test_minify() {
		assert_minify!(BackgroundImage, "none", "none");
		assert_minify!(BackgroundImage, "none, none, none", "none,none,none");
	}
}
