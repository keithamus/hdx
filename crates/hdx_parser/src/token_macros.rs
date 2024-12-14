use hdx_lexer::{Cursor, Kind, KindSet, Token};
use miette::Result;

use crate::{diagnostics, Build, Is, Parse, Parser, Peek};

macro_rules! kind {
	($ident:ident) => {
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::hdx_lexer::Token);

		impl From<$ident> for ::hdx_lexer::Cursor {
			fn from(value: $ident) -> Self {
				Cursor::dummy(value.0)
			}
		}

		impl From<&$ident> for ::hdx_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				Cursor::dummy(value.0)
			}
		}

		impl From<$ident> for ::hdx_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::hdx_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl<'a> $crate::Is<'a> for $ident {
			fn is(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> bool {
				c == ::hdx_lexer::Kind::$ident
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> Self {
				Self(c.token())
			}
		}
	};
}

macro_rules! kind_ident {
	($ident:ident) => {
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::hdx_lexer::Cursor);

		impl From<$ident> for ::hdx_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::hdx_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl From<$ident> for ::hdx_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<&$ident> for ::hdx_lexer::Token {
			fn from(value: &$ident) -> Self {
				let t: Token = value.into();
				t
			}
		}

		impl<'a> $crate::Is<'a> for $ident {
			fn is(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> bool {
				c == ::hdx_lexer::Kind::$ident
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> Self {
				Self(c)
			}
		}
	};
}

#[macro_export]
macro_rules! custom_delim {
	($ident:ident, $ch:literal) => {
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::T![Delim]);

		impl From<$ident> for ::hdx_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<&$ident> for ::hdx_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for ::hdx_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<&$ident> for ::hdx_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0.into()
			}
		}

		impl<'a> $crate::Is<'a> for $ident {
			fn is(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> bool {
				c == ::hdx_lexer::Kind::Delim && c == $ch
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(p: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> Self {
				Self(<$crate::T![Delim]>::build(p, c))
			}
		}

		impl PartialEq<char> for $ident {
			fn eq(&self, other: &char) -> bool {
				self.0 == *other
			}
		}
	};
}

#[macro_export]
macro_rules! custom_dimension {
	($ident: ident, atom!($atom: tt)) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::hdx_lexer::Token);

		impl Default for $ident {
			fn default() -> Self {
				Self(::hdx_lexer::Token::new_dimension(
					false,
					false,
					1,
					hdx_atom::atom!($atom).len() as u32,
					0.0,
					::hdx_lexer::DimensionUnit::$ident,
				))
			}
		}

		impl From<$ident> for ::hdx_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::hdx_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl From<$ident> for ::hdx_lexer::Cursor {
			fn from(value: $ident) -> Self {
				::hdx_lexer::Cursor::dummy(value.0)
			}
		}

		impl From<&$ident> for ::hdx_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				::hdx_lexer::Cursor::dummy(value.0)
			}
		}

		impl PartialEq<f32> for $ident {
			fn eq(&self, other: &f32) -> bool {
				self.0.value() == *other
			}
		}

		impl PartialEq<::hdx_atom::Atom> for $ident {
			fn eq(&self, other: &::hdx_atom::Atom) -> bool {
				::hdx_atom::atom!($atom) == *other
			}
		}

		impl<'a> $crate::Is<'a> for $ident {
			fn is(p: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> bool {
				c == ::hdx_lexer::Kind::Dimension
					&& (c == ::hdx_lexer::DimensionUnit::$ident || p.parse_atom_lower(c) == ::hdx_atom::atom!($atom))
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> Self {
				Self(c.token())
			}
		}

		impl From<$ident> for i32 {
			fn from(value: $ident) -> Self {
				value.0.value() as i32
			}
		}

		impl From<&$ident> for i32 {
			fn from(value: &$ident) -> Self {
				value.0.value() as i32
			}
		}

		impl From<$ident> for f32 {
			fn from(value: $ident) -> Self {
				value.0.value()
			}
		}

		impl From<&$ident> for f32 {
			fn from(value: &$ident) -> Self {
				value.0.value()
			}
		}
	};
}

#[macro_export]
macro_rules! custom_keyword {
	($ident: ident, atom!($atom: tt)) => {
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::hdx_lexer::Cursor);

		impl core::convert::From<$ident> for ::hdx_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl core::convert::From<&$ident> for ::hdx_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl core::convert::From<$ident> for ::hdx_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl core::convert::From<&$ident> for ::hdx_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0.into()
			}
		}

		impl ::hdx_atom::Atomizable for $ident {
			fn from_atom(atom: &hdx_atom::Atom) -> Option<Self> {
				if *atom == ::hdx_atom::atom!($atom) {
					Some(Self(::hdx_lexer::Cursor::dummy(::hdx_lexer::Token::new_ident(
						false,
						false,
						false,
						hdx_atom::atom!($atom).len() as u32,
					))))
				} else {
					Option::None
				}
			}

			fn to_atom(&self) -> ::hdx_atom::Atom {
				::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Is<'a> for $ident {
			fn is(p: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> bool {
				<$crate::T![Ident]>::is(p, c) && p.parse_atom_lower(c) == ::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> Self {
				Self(c)
			}
		}
	};
}

#[macro_export]
macro_rules! custom_function {
	($ident: ident, atom!($atom: tt)) => {
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::hdx_lexer::Cursor);

		impl From<$ident> for ::hdx_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for ::hdx_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::hdx_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl ::hdx_atom::Atomizable for $ident {
			fn from_atom(atom: &hdx_atom::Atom) -> Option<Self> {
				if *atom == ::hdx_atom::atom!($atom) {
					Some(Self(::hdx_lexer::Cursor::dummy(::hdx_lexer::Token::new_function(
						false,
						false,
						false,
						::hdx_atom::atom!($atom).len() as u32 + 1,
					))))
				} else {
					None
				}
			}

			fn to_atom(&self) -> ::hdx_atom::Atom {
				::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Is<'a> for $ident {
			fn is(p: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> bool {
				c == ::hdx_lexer::Kind::Function && p.parse_atom_lower(c) == ::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(p: &$crate::Parser<'a>, c: ::hdx_lexer::Cursor) -> Self {
				Self(c)
			}
		}
	};
}

#[macro_export]
macro_rules! custom_double_delim {
	($ident: ident, $first: literal, $second: literal) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::T![Delim], $crate::T![Delim]);

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(p: &$crate::Parser<'a>) -> bool {
				let first = p.peek_n(1);
				if first == $first {
					return p.peek_n(2) == $second;
				}
				false
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let start_offset = p.offset();
				let first = p.parse::<$crate::T![Delim]>()?;
				if first != $first {
					Err($crate::diagnostics::ExpectedDelim(first.0.kind(), start_offset.as_span(first.0)))?;
				}
				let start_offset = p.offset();
				let skip = p.set_skip(hdx_lexer::KindSet::NONE);
				let second = p.parse::<$crate::T![Delim]>();
				p.set_skip(skip);
				let second = second?;
				if second != $second {
					Err($crate::diagnostics::ExpectedDelim(second.0.kind(), start_offset.as_span(second.0)))?;
				}
				Ok(Self(first, second))
			}
		}

		impl<'a> $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append(self.0.into());
				s.append(self.1.into());
			}
		}
	};
}

#[macro_export]
macro_rules! keyword_typedef {
	($name: ident { $( $variant: ident: atom!($variant_atom: tt)),+ $(,)* }) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
		pub enum $name {
			$($variant(hdx_lexer::Cursor)),+
		}

		impl<'a> hdx_parser::Is<'a> for $name {
			fn is(p: &hdx_parser::Parser<'a>, c: hdx_lexer::Cursor) -> bool {
				<hdx_parser::T![Ident]>::is(p, c) && matches!(p.parse_atom_lower(c), $(hdx_atom::atom!($variant_atom))|+)
			}
		}

		impl<'a> hdx_parser::Build<'a> for $name {
			fn build(p: &hdx_parser::Parser<'a>, c: hdx_lexer::Cursor) -> Self {
				match p.parse_atom_lower(c) {
					$(hdx_atom::atom!($variant_atom) => Self::$variant(c),)+
					_ => unreachable!(),
				}
			}
		}

		impl From<$name> for hdx_lexer::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<&$name> for hdx_lexer::Token {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t).into(),)+
				}
			}
		}

		impl From<$name> for hdx_lexer::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}

		impl From<&$name> for hdx_lexer::Cursor {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t),)+
				}
			}
		}
	}
}

kind!(Eof);
kind!(Comment);
kind!(CdcOrCdo);
kind!(BadString);
kind!(BadUrl);
kind!(Delim);
kind!(Colon);
kind!(Semicolon);
kind!(Comma);
kind!(LeftCurly);
kind!(RightCurly);
kind!(LeftSquare);
kind!(RightSquare);
kind!(LeftParen);
kind!(RightParen);

impl PartialEq<char> for Delim {
	fn eq(&self, other: &char) -> bool {
		self.0 == *other
	}
}

kind_ident!(Ident);
kind_ident!(String);
kind_ident!(Url);
kind_ident!(Function);
kind_ident!(AtKeyword);
kind_ident!(Hash);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Whitespace(Token);

impl From<Whitespace> for Cursor {
	fn from(value: Whitespace) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<&Whitespace> for Cursor {
	fn from(value: &Whitespace) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<Whitespace> for Token {
	fn from(value: Whitespace) -> Self {
		value.0
	}
}

impl From<&Whitespace> for Token {
	fn from(value: &Whitespace) -> Self {
		value.0
	}
}

impl<'a> Peek<'a> for Whitespace {
	fn peek(p: &Parser<'a>) -> bool {
		let c = p.peek_next_including_whitespace();
		c == Kind::Whitespace
	}
}

impl<'a> Parse<'a> for Whitespace {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let skip = p.set_skip(KindSet::COMMENTS);
		let c = p.next();
		p.set_skip(skip);
		if c != Kind::Whitespace {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		Ok(Self(c.token()))
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DashedIdent(Ident);

impl From<DashedIdent> for Cursor {
	fn from(value: DashedIdent) -> Self {
		value.0.into()
	}
}

impl From<&DashedIdent> for Cursor {
	fn from(value: &DashedIdent) -> Self {
		let t: Cursor = value.into();
		t
	}
}

impl<'a> Is<'a> for DashedIdent {
	fn is(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Ident && c.token().is_dashed_ident()
	}
}

impl<'a> Build<'a> for DashedIdent {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(Ident::build(p, c))
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Dimension(Cursor);

impl From<Dimension> for Cursor {
	fn from(value: Dimension) -> Self {
		value.0
	}
}

impl From<&Dimension> for Cursor {
	fn from(value: &Dimension) -> Self {
		value.0
	}
}

impl PartialEq<f32> for Dimension {
	fn eq(&self, other: &f32) -> bool {
		self.0.token().value() == *other
	}
}

impl<'a> Is<'a> for Dimension {
	fn is(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Dimension
	}
}

impl<'a> Build<'a> for Dimension {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c)
	}
}

impl From<Dimension> for f32 {
	fn from(value: Dimension) -> Self {
		value.0.token().value()
	}
}

impl From<&Dimension> for f32 {
	fn from(value: &Dimension) -> Self {
		value.0.token().value()
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Number(Token);

impl Number {
	pub const NUMBER_ZERO: Number = Number(Token::NUMBER_ZERO);
}

impl Default for Number {
	fn default() -> Self {
		Self(Token::new_number(false, false, 1, 0.0))
	}
}

impl From<Number> for Cursor {
	fn from(value: Number) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<&Number> for Cursor {
	fn from(value: &Number) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<Number> for Token {
	fn from(value: Number) -> Self {
		value.0
	}
}

impl From<&Number> for Token {
	fn from(value: &Number) -> Self {
		value.0
	}
}

impl Number {
	pub const ZERO: Number = Number(Token::NUMBER_ZERO);

	pub fn new(n: f32) -> Self {
		Self(Token::new_number(false, false, 0, n))
	}
}

impl<'a> Is<'a> for Number {
	fn is(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Number
	}
}

impl<'a> Build<'a> for Number {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c.token())
	}
}

impl From<Number> for f32 {
	fn from(value: Number) -> Self {
		value.0.value()
	}
}

impl From<Number> for i32 {
	fn from(value: Number) -> Self {
		value.0.value() as i32
	}
}

impl PartialEq<f32> for Number {
	fn eq(&self, other: &f32) -> bool {
		self.0.value() == *other
	}
}

pub mod delim {
	custom_delim!(And, '&');
	custom_delim!(At, '@');
	custom_delim!(Caret, '^');
	custom_delim!(Dash, '-');
	custom_delim!(Dollar, '$');
	custom_delim!(Dot, '.');
	custom_delim!(Eq, '=');
	custom_delim!(Gt, '>');
	custom_delim!(Hash, '#');
	custom_delim!(Lt, '<');
	custom_delim!(Not, '!');
	custom_delim!(Or, '|');
	custom_delim!(Percent, '%');
	custom_delim!(Plus, '+');
	custom_delim!(Question, '?');
	custom_delim!(Slash, '/');
	custom_delim!(Star, '*');
	custom_delim!(Tilde, '~');
	custom_delim!(Underscore, '_');
}

pub mod double {
	custom_double_delim!(GreaterThanEqual, '>', '=');
	custom_double_delim!(LessThanEqual, '<', '=');
	custom_double_delim!(StarPipe, '*', '|');
	custom_double_delim!(PipePipe, '|', '|');
	custom_double_delim!(EqualEqual, '=', '=');
	custom_double_delim!(TildeEqual, '~', '=');
	custom_double_delim!(PipeEqual, '|', '=');
	custom_double_delim!(CaretEqual, '^', '=');
	custom_double_delim!(DollarEqual, '$', '=');
	custom_double_delim!(StarEqual, '*', '=');

	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub struct ColonColon(crate::T![:], crate::T![:]);

	impl<'a> crate::Peek<'a> for ColonColon {
		fn peek(p: &crate::Parser<'a>) -> bool {
			let first = p.peek_n(1);
			if first == hdx_lexer::Kind::Colon {
				return p.peek_n(2) == hdx_lexer::Kind::Colon;
			}
			false
		}
	}

	impl<'a> crate::Parse<'a> for ColonColon {
		fn parse(p: &mut crate::Parser<'a>) -> crate::Result<Self> {
			let first = p.parse::<crate::T![:]>()?;
			let skip = p.set_skip(hdx_lexer::KindSet::NONE);
			let second = p.parse::<crate::T![:]>();
			p.set_skip(skip);
			Ok(Self(first, second?))
		}
	}

	impl crate::ToCursors for ColonColon {
		fn to_cursors(&self, s: &mut impl crate::CursorSink) {
			s.append(self.0.into());
			s.append(self.1.into());
		}
	}
}

pub mod dimension {
	custom_dimension!(Cap, atom!("cap"));
	custom_dimension!(Ch, atom!("ch"));
	custom_dimension!(Cm, atom!("cm"));
	custom_dimension!(Cqb, atom!("cqb"));
	custom_dimension!(Cqh, atom!("cqh"));
	custom_dimension!(Cqi, atom!("cqi"));
	custom_dimension!(Cqmax, atom!("cqmax"));
	custom_dimension!(Cqmin, atom!("cqmin"));
	custom_dimension!(Cqw, atom!("cqw"));
	custom_dimension!(Deg, atom!("deg"));
	custom_dimension!(Dpcm, atom!("dpcm"));
	custom_dimension!(Dpi, atom!("dpi"));
	custom_dimension!(Dppx, atom!("dppx"));
	custom_dimension!(Dvb, atom!("dvb"));
	custom_dimension!(Dvh, atom!("dvh"));
	custom_dimension!(Dvi, atom!("dvi"));
	custom_dimension!(Dvmax, atom!("dvmax"));
	custom_dimension!(Dvmin, atom!("dvmin"));
	custom_dimension!(Dvw, atom!("dvw"));
	custom_dimension!(Em, atom!("em"));
	custom_dimension!(Ex, atom!("ex"));
	custom_dimension!(Fr, atom!("fr"));
	custom_dimension!(Grad, atom!("grad"));
	custom_dimension!(Hz, atom!("hz"));
	custom_dimension!(Ic, atom!("ic"));
	custom_dimension!(In, atom!("in"));
	custom_dimension!(Khz, atom!("khz"));
	custom_dimension!(Lh, atom!("lh"));
	custom_dimension!(Lvb, atom!("lvb"));
	custom_dimension!(Lvh, atom!("lvh"));
	custom_dimension!(Lvi, atom!("lvi"));
	custom_dimension!(Lvmax, atom!("lvmax"));
	custom_dimension!(Lvmin, atom!("lvmin"));
	custom_dimension!(Lvw, atom!("lvw"));
	custom_dimension!(Mm, atom!("mm"));
	custom_dimension!(Ms, atom!("ms"));
	custom_dimension!(Pc, atom!("pc"));
	custom_dimension!(Percent, atom!("%"));
	custom_dimension!(Pt, atom!("pt"));
	custom_dimension!(Px, atom!("px"));
	custom_dimension!(Q, atom!("q"));
	custom_dimension!(Rad, atom!("rad"));
	custom_dimension!(Rcap, atom!("rcap"));
	custom_dimension!(Rch, atom!("rch"));
	custom_dimension!(Rem, atom!("rem"));
	custom_dimension!(Rex, atom!("rex"));
	custom_dimension!(Ric, atom!("ric"));
	custom_dimension!(Rlh, atom!("rlh"));
	custom_dimension!(S, atom!("s"));
	custom_dimension!(Svb, atom!("svb"));
	custom_dimension!(Svh, atom!("svh"));
	custom_dimension!(Svi, atom!("svi"));
	custom_dimension!(Svmax, atom!("svmax"));
	custom_dimension!(Svmin, atom!("svmin"));
	custom_dimension!(Svw, atom!("svw"));
	custom_dimension!(Turn, atom!("turn"));
	custom_dimension!(Vb, atom!("vb"));
	custom_dimension!(Vh, atom!("vh"));
	custom_dimension!(Vi, atom!("vi"));
	custom_dimension!(Vmax, atom!("vmax"));
	custom_dimension!(Vmin, atom!("vmin"));
	custom_dimension!(Vw, atom!("vw"));
	custom_dimension!(X, atom!("x"));
}

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Any(Cursor);

impl From<Any> for Cursor {
	fn from(value: Any) -> Self {
		value.0
	}
}

impl From<&Any> for Cursor {
	fn from(value: &Any) -> Self {
		value.0
	}
}

impl<'a> Is<'a> for Any {
	fn is(_: &Parser<'a>, _: Cursor) -> bool {
		true
	}
}

impl<'a> Build<'a> for Any {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c)
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PairWiseStart(Token);

impl From<PairWiseStart> for Cursor {
	fn from(value: PairWiseStart) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<&PairWiseStart> for Cursor {
	fn from(value: &PairWiseStart) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<PairWiseStart> for Token {
	fn from(value: PairWiseStart) -> Self {
		value.0
	}
}

impl From<&PairWiseStart> for Token {
	fn from(value: &PairWiseStart) -> Self {
		value.0
	}
}

impl PairWiseStart {
	pub fn kind(&self) -> Kind {
		self.0.kind()
	}
}

impl<'a> Is<'a> for PairWiseStart {
	fn is(_: &Parser<'a>, c: Cursor) -> bool {
		let kindset = KindSet::new(&[Kind::LeftCurly, Kind::LeftSquare, Kind::LeftParen]);
		c == kindset
	}
}

impl<'a> Build<'a> for PairWiseStart {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c.token())
	}
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PairWiseEnd(Token);

impl From<PairWiseEnd> for Cursor {
	fn from(value: PairWiseEnd) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<&PairWiseEnd> for Cursor {
	fn from(value: &PairWiseEnd) -> Self {
		Cursor::dummy(value.0)
	}
}

impl From<PairWiseEnd> for Token {
	fn from(value: PairWiseEnd) -> Self {
		value.0
	}
}

impl From<&PairWiseEnd> for Token {
	fn from(value: &PairWiseEnd) -> Self {
		value.0
	}
}

impl PairWiseEnd {
	pub fn kind(&self) -> Kind {
		self.0.kind()
	}
}

impl<'a> Is<'a> for PairWiseEnd {
	fn is(_: &Parser<'a>, c: Cursor) -> bool {
		let kindset = KindSet::new(&[Kind::RightCurly, Kind::RightSquare, Kind::RightParen]);
		c == kindset
	}
}

impl<'a> Build<'a> for PairWiseEnd {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c.token())
	}
}

#[macro_export]
macro_rules! T {
	[:] => { $crate::token_macros::Colon };
	[;] => { $crate::token_macros::Semicolon };
	[,] => { $crate::token_macros::Comma };
	['{'] => { $crate::token_macros::LeftCurly };
	['}'] => { $crate::token_macros::RightCurly };
	['['] => { $crate::token_macros::LeftSquare };
	[']'] => { $crate::token_macros::RightSquare };
	['('] => { $crate::token_macros::LeftParen };
	[')'] => { $crate::token_macros::RightParen };
	[' '] => { $crate::token_macros::Whitespace };

	[&] => { $crate::token_macros::delim::And };
	[@] => { $crate::token_macros::delim::At };
	[^] => { $crate::token_macros::delim::Caret };
	[-] => { $crate::token_macros::delim::Dash };
	[$] => { $crate::token_macros::delim::Dollar };
	[.] => { $crate::token_macros::delim::Dot };
	[=] => { $crate::token_macros::delim::Eq };
	[>] => { $crate::token_macros::delim::Gt };
	[#] => { $crate::token_macros::delim::Hash };
	[<] => { $crate::token_macros::delim::Lt };
	[!] => { $crate::token_macros::delim::Not };
	[|] => { $crate::token_macros::delim::Or };
	[%] => { $crate::token_macros::delim::Percent };
	[+] => { $crate::token_macros::delim::Plus };
	[?] => { $crate::token_macros::delim::Question };
	[/] => { $crate::token_macros::delim::Slash };
	[*] => { $crate::token_macros::delim::Star };
	[~] => { $crate::token_macros::delim::Tilde };
	[_] => { $crate::token_macros::delim::Underscore };

	[>=] => { $crate::token_macros::double::GreaterThanEqual };
	[<=] => { $crate::token_macros::double::LessThanEqual };
	[*|] => { $crate::token_macros::double::StarPipe };
	[::] => { $crate::token_macros::double::ColonColon };
	[||] => { $crate::token_macros::double::PipePipe };
	[==] => { $crate::token_macros::double::EqualEqual };
	[~=] => { $crate::token_macros::double::TildeEqual };
	[|=] => { $crate::token_macros::double::PipeEqual };
	[^=] => { $crate::token_macros::double::CaretEqual };
	["$="] => { $crate::token_macros::double::DollarEqual };
	[*=] => { $crate::token_macros::double::StarEqual };

	[Dimension::$ident: ident] => { $crate::token_macros::dimension::$ident };
	[Dimension::%] => { $crate::token_macros::dimension::Percent };

	[!important] => { $crate::token_macros::double::BangImportant };

	[$ident:ident] => { $crate::token_macros::$ident }
}
