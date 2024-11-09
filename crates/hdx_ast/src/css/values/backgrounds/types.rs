use crate::macros::keyword_typedef;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

pub(crate) use crate::css::types::*;
pub(crate) use crate::css::values::r#box::types::VisualBox;

mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(None, atom!("none"));
	custom_keyword!(RepeatX, atom!("repeat-x"));
	custom_keyword!(RepeatY, atom!("repeat-y"));
}

// https://drafts.csswg.org/css-backgrounds/#typedef-bg-image
// <bg-image> = <image> | none
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum BgImage<'a> {
	None,
	Image(Image<'a>),
}

impl<'a> Peek<'a> for BgImage<'a> {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<kw::None>().or_else(|| parser.peek::<Image>())
	}
}

impl<'a> Parse<'a> for BgImage<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::None>() {
			parser.hop(token);
			Ok(Self::None)
		} else {
			let image = parser.parse::<Image>()?;
			Ok(Self::Image(image))
		}
	}
}

impl<'a> WriteCss<'a> for BgImage<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::None => kw::None::atom().write_css(sink),
			Self::Image(image) => image.write_css(sink),
		}
	}
}

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat
// <repeat-style> = repeat-x | repeat-y | <repetition>{1,2}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum RepeatStyle {
	RepeatX,
	RepeatY,
	Repetition(Repetition, Option<Repetition>),
}

impl RepeatStyle {
	#[allow(non_upper_case_globals)]
	pub const Repeat: RepeatStyle = RepeatStyle::Repetition(Repetition::Repeat, None);
}

impl<'a> Peek<'a> for RepeatStyle {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<kw::RepeatX>().or_else(|| parser.peek::<kw::RepeatY>()).or_else(|| parser.peek::<Repetition>())
	}
}

impl<'a> Parse<'a> for RepeatStyle {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(token) = parser.peek::<kw::RepeatX>() {
			parser.hop(token);
			Ok(Self::RepeatX)
		} else if let Some(token) = parser.peek::<kw::RepeatY>() {
			parser.hop(token);
			Ok(Self::RepeatY)
		} else {
			let first = parser.parse::<Repetition>()?;
			let second = parser.parse_if_peek::<Repetition>()?;
			Ok(Self::Repetition(first, second))
		}
	}
}

impl<'a> WriteCss<'a> for RepeatStyle {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::RepeatX => write_css!(sink, kw::RepeatX::atom()),
			Self::RepeatY => write_css!(sink, kw::RepeatY::atom()),
			Self::Repetition(p1, p2) => write_css!(sink, p1, ' ', p2),
		}
		Ok(())
	}
}

// https://drafts.csswg.org/css-backgrounds-4/#typedef-repetition
// <repetition> = repeat | space | round | no-repeat
keyword_typedef!(Repetition {
	Repeat: atom!("repeat"),
	Space: atom!("space"),
	Round: atom!("round"),
	NoRepeat: atom!("no-repeat"),
});

// https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment
// <attachment> = scroll | fixed | local
keyword_typedef!(Attachment { Scroll: atom!("scroll"), Fixed: atom!("fixed"), Local: atom!("local") });

// https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-clip
// <bg-clip> = <visual-box> | border-area| text
// https://drafts.csswg.org/css-box-4/#typedef-visual-box
// <visual-box> = <visual-box> | margin-box
keyword_typedef!(BgClip {
	ContentBox: atom!("content-box"),
	LayoutBox: atom!("padding-box"),
	BorderBox: atom!("border-box"),
	BorderArea: atom!("border-area"),
	Text: atom!("text"),
});
