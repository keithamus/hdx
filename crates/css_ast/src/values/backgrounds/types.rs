use css_lexer::Cursor;
use css_parse::{
	diagnostics, keyword_set, Build, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T,
};

pub(crate) use crate::types::*;
pub(crate) use crate::values::r#box::types::VisualBox;

// https://drafts.csswg.org/css-backgrounds/#typedef-bg-image
// <bg-image> = <image> | none
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum BgImage<'a> {
	None(T![Ident]),
	Image(Image<'a>),
}

impl<'a> Peek<'a> for BgImage<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<Image>::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Parse<'a> for BgImage<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Image>() {
			let image = p.parse::<Image>()?;
			Ok(Self::Image(image))
		} else {
			let ident = p.parse::<T![Ident]>()?;
			let c: Cursor = ident.into();
			if !p.eq_ignore_ascii_case(c, "none") {
				Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), ident.into()))?;
			}
			Ok(Self::None(ident))
		}
	}
}

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat
// <repeat-style> = repeat-x | repeat-y | <repetition>{1,2}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum RepeatStyle {
	RepeatX(T![Ident]),
	RepeatY(T![Ident]),
	Repetition(Repetition, Option<Repetition>),
}

impl<'a> Peek<'a> for RepeatStyle {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<Repetition>::peek(p, c) || (<T![Ident]>::peek(p, c) && matches!(p.parse_str_lower(c), "repeat-x" | "repeat-y"))
	}
}

impl<'a> Parse<'a> for RepeatStyle {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		match p.parse_str_lower(c) {
			"repeat-x" => Ok(Self::RepeatX(<T![Ident]>::build(p, c))),
			"repeat-y" => Ok(Self::RepeatY(<T![Ident]>::build(p, c))),
			_ if <Repetition>::peek(p, c) => {
				let first = Repetition::build(p, c);
				let second = p.parse_if_peek::<Repetition>()?;
				Ok(Self::Repetition(first, second))
			}
			_ => Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?,
		}
	}
}

impl<'a> ToCursors for RepeatStyle {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::RepeatX(c) => s.append(c.into()),
			Self::RepeatY(c) => s.append(c.into()),
			Self::Repetition(p1, p2) => {
				s.append(p1.into());
				if let Some(p2) = p2 {
					s.append(p2.into());
				}
			}
		}
	}
}

// https://drafts.csswg.org/css-backgrounds-4/#typedef-repetition
// <repetition> = repeat | space | round | no-repeat
keyword_set!(Repetition { Repeat: "repeat", Space: "space", Round: "round", NoRepeat: "no-repeat" });

// https://drafts.csswg.org/css-backgrounds-3/#typedef-attachment
// <attachment> = scroll | fixed | local
keyword_set!(Attachment { Scroll: "scroll", Fixed: "fixed", Local: "local" });

// https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-clip
// <bg-clip> = <visual-box> | border-area| text
// https://drafts.csswg.org/css-box-4/#typedef-visual-box
// <visual-box> = <visual-box> | margin-box
keyword_set!(BgClip {
	ContentBox: "content-box",
	LayoutBox: "padding-box",
	BorderBox: "border-box",
	BorderArea: "border-area",
	Text: "text",
});
