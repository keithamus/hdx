use hdx_parser::Token;
use hdx_parser::{Parse, Parser, Peek, Result as ParserResult};
use hdx_writer::{write_css, CssWriter, Result as WriterResult, WriteCss};

pub(crate) use crate::css::units::*;
use crate::macros::keyword_typedef;

mod func {
	use hdx_parser::custom_function;

	custom_function!(SnapBlock, atom!("snap-block"));
	custom_function!(SnapInline, atom!("snap-inline"));
}

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block
// snap-block() = snap-block( <length> , [ start | end | near ]? )
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapBlock(LengthPercentage, Option<SnapBlockKeyword>);

impl<'a> Peek<'a> for SnapBlock {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<func::SnapBlock>()
	}
}

impl<'a> Parse<'a> for SnapBlock {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		parser.parse::<func::SnapBlock>()?;
		let length = parser.parse::<LengthPercentage>()?;
		let keyword = if let Some(token) = parser.peek::<Token![,]>() {
			parser.hop(token);
			Some(parser.parse::<SnapBlockKeyword>()?)
		} else {
			None
		};
		parser.parse::<Token![RightParen]>()?;
		Ok(Self(length, keyword))
	}
}

impl<'a> WriteCss<'a> for SnapBlock {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_css!(sink, func::SnapBlock::atom(), '(', self.0);
		if let Some(keyword) = self.1 {
			write_css!(sink, ',', (), keyword);
		}
		')'.write_css(sink)
	}
}

keyword_typedef!(SnapBlockKeyword { Start: atom!("start"), End: atom!("end"), Near: atom!("near") });

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-inline
// snap-inline() = snap-inline( <length> , [ left | right | near ]? )
#[derive(Debug, Clone, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapInline(LengthPercentage, Option<SnapInlineKeyword>);

impl<'a> Peek<'a> for SnapInline {
	fn peek(parser: &Parser<'a>) -> Option<hdx_lexer::Token> {
		parser.peek::<func::SnapInline>()
	}
}

impl<'a> Parse<'a> for SnapInline {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		parser.parse::<func::SnapInline>()?;
		let length = parser.parse::<LengthPercentage>()?;
		let keyword = if let Some(token) = parser.peek::<Token![,]>() {
			parser.hop(token);
			Some(parser.parse::<SnapInlineKeyword>()?)
		} else {
			None
		};
		parser.parse::<Token![RightParen]>()?;
		Ok(Self(length, keyword))
	}
}

impl<'a> WriteCss<'a> for SnapInline {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		write_css!(sink, func::SnapInline::atom(), '(', self.0);
		if let Some(keyword) = self.1 {
			write_css!(sink, ',', (), keyword);
		}
		')'.write_css(sink)
	}
}

keyword_typedef!(SnapInlineKeyword { Left: atom!("left"), Right: atom!("right"), Near: atom!("near") });
