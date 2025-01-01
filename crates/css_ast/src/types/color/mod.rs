mod color_function;
mod named;
mod system;

use css_lexer::Cursor;
use css_parse::{diagnostics, keyword_set, Build, Parse, Parser, Peek, Result as ParserResult, ToCursors, T};

pub use color_function::*;
pub use named::*;
pub use system::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Color {
	Currentcolor(T![Ident]),
	Transparent(T![Ident]),
	System(SystemColor),
	Hex(T![Hash]),
	Named(NamedColor),
	Function(ColorFunction),
	// TODO: need bumpalo::Box PartialEq, or bumpalo::Box serde
	// Relative(Box<'a, Color<'a>>, ColorFunction),
	// Mix(ColorMixSyntax, Box<'a, Color<'a>>, u8, Box<'a, Color<'a>>),
}

impl Color {
	// Alias CanvasText for #[initial()]
	// #[allow(non_upper_case_globals)]
	// pub const Canvastext: Color = Color::System(SystemColor::CanvasText);
}

keyword_set!(ColorKeyword { Currentcolor: "currentcolor", Transparent: "transparent" });

impl<'a> ToCursors for Color {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		match self {
			Self::Currentcolor(t) => s.append((*t).into()),
			Self::Transparent(t) => s.append((*t).into()),
			Self::System(t) => s.append((*t).into()),
			Self::Hex(t) => s.append((*t).into()),
			Self::Named(t) => s.append((*t).into()),
			Self::Function(func) => ToCursors::to_cursors(func, s),
		}
	}
}

impl<'a> Peek<'a> for Color {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Hash]>::peek(p, c) || <T![Function]>::peek(p, c) || <T![Ident]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for Color {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Hash]>() {
			Ok(Self::Hex(p.parse::<T![Hash]>()?))
		} else if p.peek::<T![Ident]>() {
			let c = p.peek_n(1);
			let color_keyword = p.parse_if_peek::<ColorKeyword>()?;
			let ident = <T![Ident]>::build(p, c);
			match color_keyword {
				Some(ColorKeyword::Currentcolor(_)) => Ok(Self::Currentcolor(ident)),
				Some(ColorKeyword::Transparent(_)) => Ok(Self::Transparent(ident)),
				_ => {
					if let Some(named) = p.parse_if_peek::<NamedColor>()? {
						Ok(Self::Named(named))
					} else {
						p.parse::<SystemColor>().map(Self::System)
					}
				}
			}
		} else if p.peek::<ColorFunction>() {
			p.parse::<ColorFunction>().map(Color::Function)
		} else {
			Err(diagnostics::Unimplemented(p.peek_n(1).into()))?
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Color>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Color, "currentcolor");
		assert_parse!(Color, "#fff");
		assert_parse!(Color, "red");
		assert_parse!(Color, "#ababab");
		assert_parse!(Color, "rgb(255 255 255)");
		assert_parse!(Color, "rgb(255,255,255)");
		assert_parse!(Color, "rgba(255,255,255,0.5)");
		assert_parse!(Color, "rgb(29 164 192/95%)");
		assert_parse!(Color, "rgb(255 255 255/0.5)");
		assert_parse!(Color, "rgb(255 20% 12.2/0.5)");
		assert_parse!(Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(Color, "hwb(740deg 20% 30%/50%)");
		assert_parse!(Color, "lch(20% 30% 740deg/50%)");
	}

	#[test]
	fn test_recoverable_writes() {
		// Missing /
		assert_parse!(Color, "rgb(255 255 255 0.5)");
		// Mixed legacy values
		assert_parse!(Color, "rgba(255,20%,255,0.5)");
		// Trailing comma
		assert_parse!(Color, "rgb(255,255,255,)");
		// Using legacy comma syntax but with /
		assert_parse!(Color, "rgb(255,255,255/0.5)");
		// Using both legacy commas and also a /
		assert_parse!(Color, "rgba(255,255,255,/0.5)");
		// Missing a comma
		assert_parse!(Color, "rgb(29,164 192,95%)");
	}

	#[test]
	fn test_errors() {
		// Using degrees for RGB
		assert_parse_error!(Color, "rgba(250deg, 255, 255 / 0.5)");
		// Using % for first component in hsl
		assert_parse_error!(Color, "hsl(250%, 255, 255)");
		// Using % for first component in lch
		assert_parse_error!(Color, "lch(250%, 255, 255)");
		// Using degrees for wrong component in hsl
		assert_parse_error!(Color, "hsl(250, 255deg, 255)");
		// Using degrees for wrong component in lch
		assert_parse_error!(Color, "lch(250, 255deg, 255)");
	}
}
