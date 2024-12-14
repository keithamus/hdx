use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{
	diagnostics, keyword_typedef, Build, CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, T,
};

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum BgImage<'a> {
	None(T![Ident]),
	Image(Image<'a>),
}

impl<'a> Peek<'a> for BgImage<'a> {
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<kw::None>() || p.peek::<Image>()
	}
}

impl<'a> Parse<'a> for BgImage<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<kw::None>() {
			Ok(Self::None(p.parse::<T![Ident]>()?))
		} else {
			let image = p.parse::<Image>()?;
			Ok(Self::Image(image))
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
	fn peek(p: &Parser<'a>) -> bool {
		p.peek::<kw::RepeatX>() || p.peek::<kw::RepeatY>() || p.peek::<Repetition>()
	}
}

impl<'a> Parse<'a> for RepeatStyle {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		match p.parse_atom_lower(c) {
			atom!("repeat-x") => Ok(Self::RepeatX(<T![Ident]>::build(p, c))),
			atom!("repeat-y") => Ok(Self::RepeatY(<T![Ident]>::build(p, c))),
			atom!("repeat") | atom!("space") | atom!("round") | atom!("no-repeat") => {
				let first = Repetition::build(p, c);
				let second = p.parse_if_peek::<Repetition>()?;
				Ok(Self::Repetition(first, second))
			}
			atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
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
