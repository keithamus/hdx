use hdx_parser::{Parse, Parser, Peek, Result as ParserResult, T};
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
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<func::SnapBlock>()
	}
}

impl<'a> Parse<'a> for SnapBlock {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<func::SnapBlock>()?;
		let length = p.parse::<LengthPercentage>()?;
		let keyword = if let Some(token) = p.peek::<T![,]>() {
			p.hop(token);
			Some(p.parse::<SnapBlockKeyword>()?)
		} else {
			None
		};
		p.parse::<T![RightParen]>()?;
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
	fn peek(p: &Parser<'a>) -> Option<hdx_lexer::Token> {
		p.peek::<func::SnapInline>()
	}
}

impl<'a> Parse<'a> for SnapInline {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<func::SnapInline>()?;
		let length = p.parse::<LengthPercentage>()?;
		let keyword = if let Some(token) = p.peek::<T![,]>() {
			p.hop(token);
			Some(p.parse::<SnapInlineKeyword>()?)
		} else {
			None
		};
		p.parse::<T![RightParen]>()?;
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
