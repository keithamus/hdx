use css_lexer::{Cursor, DimensionUnit, Kind, KindSet, Span, Token};

use crate::{diagnostics, Build, Parse, Parser, Peek, Result};

macro_rules! define_kinds {
	($($(#[$meta:meta])* $ident:ident,)*) => {
		$(
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::css_lexer::Cursor);

		impl $ident {
			pub const fn dummy() -> Self {
				Self(::css_lexer::Cursor::dummy(::css_lexer::Token::dummy(::css_lexer::Kind::$ident)))
			}
		}

		impl From<$ident> for ::css_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::css_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl From<$ident> for ::css_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.token()
			}
		}

		impl From<&$ident> for ::css_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0.token()
			}
		}

		impl From<$ident> for ::css_lexer::Span {
			fn from(value: $ident) -> Self {
				value.0.span()
			}
		}

		impl From<&$ident> for ::css_lexer::Span {
			fn from(value: &$ident) -> Self {
				value.0.span()
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(_: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::$ident
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> Self {
				Self(c)
			}
		}
		)*
	};
}

macro_rules! define_kind_idents {
	($($(#[$meta:meta])* $ident:ident,)*) => {
		$(
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::css_lexer::Cursor);

		impl From<$ident> for ::css_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::css_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl From<$ident> for ::css_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.token()
			}
		}

		impl From<&$ident> for ::css_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0.token()
			}
		}

		impl From<$ident> for ::css_lexer::Span {
			fn from(value: $ident) -> Self {
				value.0.span()
			}
		}

		impl From<&$ident> for ::css_lexer::Span {
			fn from(value: &$ident) -> Self {
				value.0.span()
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(_: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::$ident
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> Self {
				Self(c)
			}
		}

		impl $ident {
			/// Checks if the ident begins with two HYPHEN MINUS (`--`) characters.
			pub fn is_dashed_ident(&self) -> bool {
				self.0.token().is_dashed_ident()
			}

			pub const fn dummy() -> Self {
				Self(::css_lexer::Cursor::dummy(::css_lexer::Token::dummy(::css_lexer::Kind::$ident)))
			}
		}
		)*
	};
}

/// A macro for defining a struct which captures a [Kind::Delim][css_lexer::Kind::Delim] with a specific character.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// custom_delim!{
///   /// A £ character.
///   PoundSterling, '£'
/// }
///
/// assert_parse!(PoundSterling, "£");
/// ```
#[macro_export]
macro_rules! custom_delim {
	($(#[$meta:meta])* $ident:ident, $ch:literal) => {
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::T![Delim]);

		impl From<$ident> for ::css_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<&$ident> for ::css_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for ::css_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<&$ident> for ::css_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for ::css_lexer::Span {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<&$ident> for ::css_lexer::Span {
			fn from(value: &$ident) -> Self {
				value.0.into()
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(_: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::Delim && c == $ch
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(p: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> Self {
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

#[doc(hidden)]
#[macro_export]
macro_rules! custom_dimension {
	($(#[$meta:meta])*$ident: ident, $str: tt) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(::css_lexer::Cursor);

		impl $ident {
			/// Returns the [f32] representation of the dimension's value.
			pub fn value(&self) -> f32 {
				self.0.token().value()
			}

			pub const fn dummy() -> Self {
				Self(::css_lexer::Cursor::dummy(::css_lexer::Token::dummy(::css_lexer::Kind::Dimension)))
			}
		}

		impl From<$ident> for ::css_lexer::Token {
			fn from(value: $ident) -> Self {
				value.0.token()
			}
		}

		impl From<&$ident> for ::css_lexer::Token {
			fn from(value: &$ident) -> Self {
				value.0.token()
			}
		}

		impl From<$ident> for ::css_lexer::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<&$ident> for ::css_lexer::Cursor {
			fn from(value: &$ident) -> Self {
				value.0
			}
		}

		impl PartialEq<f32> for $ident {
			fn eq(&self, other: &f32) -> bool {
				self.value() == *other
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(p: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::Dimension
					&& (c == ::css_lexer::DimensionUnit::$ident || p.eq_ignore_ascii_case(c, $str))
			}
		}

		impl<'a> $crate::Build<'a> for $ident {
			fn build(_: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> Self {
				Self(c)
			}
		}

		impl From<$ident> for i32 {
			fn from(value: $ident) -> Self {
				value.value() as i32
			}
		}

		impl From<&$ident> for i32 {
			fn from(value: &$ident) -> Self {
				value.value() as i32
			}
		}

		impl From<$ident> for f32 {
			fn from(value: $ident) -> Self {
				value.value()
			}
		}

		impl From<&$ident> for f32 {
			fn from(value: &$ident) -> Self {
				value.value()
			}
		}
	};
}

/// A macro for defining a struct which captures two adjacent [Kind::Delim][css_lexer::Kind::Delim] tokens, each with a
/// specific character.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// custom_double_delim!{
///   /// Two % adjacent symbols
///   DoublePercent, '%', '%'
/// }
///
/// assert_parse!(DoublePercent, "%%");
/// ```
#[macro_export]
macro_rules! custom_double_delim {
	($(#[$meta:meta])*$ident: ident, $first: literal, $second: literal) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident(pub $crate::T![Delim], pub $crate::T![Delim]);

		impl $ident {
			pub const fn dummy() -> Self {
				Self(<$crate::T![Delim]>::dummy(), <$crate::T![Delim]>::dummy())
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(p: &$crate::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				c == $first && p.peek_n(2) == $second
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let first = p.parse::<$crate::T![Delim]>()?;
				if first != $first {
					let c: css_lexer::Cursor = first.into();
					Err($crate::diagnostics::ExpectedDelim(c.into(), c.into()))?;
				}
				let skip = p.set_skip(css_lexer::KindSet::NONE);
				let second = p.parse::<$crate::T![Delim]>();
				p.set_skip(skip);
				let second = second?;
				if second != $second {
					let c: css_lexer::Cursor = second.into();
					Err($crate::diagnostics::ExpectedDelim(c.into(), c.into()))?;
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

		impl From<$ident> for ::css_lexer::Span {
			fn from(value: $ident) -> Self {
				Into::<::css_lexer::Span>::into(value.0) + Into::<::css_lexer::Span>::into(value.1)
			}
		}

		impl From<&$ident> for ::css_lexer::Span {
			fn from(value: &$ident) -> Self {
				Into::<::css_lexer::Span>::into(value.0) + Into::<::css_lexer::Span>::into(value.1)
			}
		}
	};
}

/// A macro for defining an enum which captures a token with [Kind::Ident][css_lexer::Kind::Ident] that matches one of
/// the variant names in the enum.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// keyword_set!(
///   /// Some docs on this type...
///   Keywords {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(Keywords, "FoO");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(Keywords, "baR");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(Keywords, "bing");
///
/// assert_parse_error!(Keywords, "oof");
/// ```
#[macro_export]
macro_rules! keyword_set {
	($(#[$meta:meta])* $name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $name {
			$($variant(::css_lexer::Cursor)),+
		}
		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::Ident && Self::MAP.get(&p.parse_str_lower(c)).is_some()
			}
		}
		impl<'a> $crate::Build<'a> for $name {
			fn build(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> Self {
				let val = Self::MAP.get(&p.parse_str_lower(c)).unwrap();
				match val {
					$(Self::$variant(_) => Self::$variant(c),)+
				}
			}
		}
		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
					$($variant_str => $name::$variant(::css_lexer::Cursor::dummy(::css_lexer::Token::dummy(::css_lexer::Kind::Ident)))),+
			};
		}

		impl From<$name> for css_lexer::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Token {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t).into(),)+
				}
			}
		}

		impl From<$name> for css_lexer::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}

		impl From<&$name> for css_lexer::Cursor {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t),)+
				}
			}
		}

		impl From<$name> for css_lexer::Span {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => (t.span()),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Span {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (t.span()),)+
				}
			}
		}
	}
}

/// A macro for defining an enum which captures a token with [Kind::Function][css_lexer::Kind::Function] that matches
/// one of the variant names in the enum.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// function_set!(
///   /// Some docs on this type...
///   Functions {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(Functions, "FoO(");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(Functions, "baR(");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(Functions, "bing(");
///
/// assert_parse_error!(Functions, "oof(");
/// ```
#[macro_export]
macro_rules! function_set {
	($(#[$meta:meta])*$name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $name {
			$($variant(::css_lexer::Cursor)),+
		}
		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::Function && Self::MAP.get(p.parse_str_lower(c)).is_some()
			}
		}
		impl<'a> $crate::Build<'a> for $name {
			fn build(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> Self {
				let val = Self::MAP.get(p.parse_str_lower(c)).unwrap();
				match val {
					$(Self::$variant(_) => Self::$variant(c),)+
				}
			}
		}
		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
				$($variant_str => $name::$variant(::css_lexer::Cursor::dummy(::css_lexer::Token::dummy(::css_lexer::Kind::Function)))),+
			};
		}

		impl From<$name> for css_lexer::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Token {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t).into(),)+
				}
			}
		}

		impl From<$name> for css_lexer::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}

		impl From<&$name> for css_lexer::Cursor {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t),)+
				}
			}
		}

		impl From<$name> for css_lexer::Span {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => (t.span()),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Span {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (t.span()),)+
				}
			}
		}
	}
}

/// A macro for defining an enum which captures a token with [Kind::AtKeyword][css_lexer::Kind::AtKeyword] that matches one of
/// the variant names in the enum.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// atkeyword_set!(
///   /// Some docs on this type...
///   Keywords {
///     Foo: "foo",
///     Bar: "bar",
///     Baz: "baz"
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(Keywords, "@FoO");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(Keywords, "@baR");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(Keywords, "@bing");
///
/// assert_parse_error!(Keywords, "@oof");
/// ```
#[macro_export]
macro_rules! atkeyword_set {
	($(#[$meta:meta])*$name: ident { $( $variant: ident: $variant_str: tt$(,)?)+ }) => {
		$(#[$meta])*
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub enum $name {
			$($variant(::css_lexer::Cursor)),+
		}
		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> bool {
				c == ::css_lexer::Kind::AtKeyword && Self::MAP.get(&p.parse_str_lower(c)).is_some()
			}
		}
		impl<'a> $crate::Build<'a> for $name {
			fn build(p: &$crate::Parser<'a>, c: css_lexer::Cursor) -> Self {
				let val = Self::MAP.get(&p.parse_str_lower(c)).unwrap();
				match val {
					$(Self::$variant(_) => Self::$variant(c),)+
				}
			}
		}
		impl $name {
			const MAP: phf::Map<&'static str, $name> = phf::phf_map! {
					$($variant_str => $name::$variant(::css_lexer::Cursor::dummy(::css_lexer::Token::dummy(::css_lexer::Kind::AtKeyword)))),+
			};
		}

		impl From<$name> for css_lexer::Token {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t.into(),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Token {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t).into(),)+
				}
			}
		}

		impl From<$name> for css_lexer::Cursor {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => t,)+
				}
			}
		}

		impl From<&$name> for css_lexer::Cursor {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (*t),)+
				}
			}
		}

		impl From<$name> for css_lexer::Span {
			fn from(value: $name) -> Self {
				match value {
					$($name::$variant(t) => (t.span()),)+
				}
			}
		}

		impl From<&$name> for css_lexer::Span {
			fn from(value: &$name) -> Self {
				match value {
					$($name::$variant(t) => (t.span()),)+
				}
			}
		}
	}
}

define_kinds! {
	/// Represents a token with [Kind::Eof][css_lexer::Kind::Eof]. Use [T![Eof]][crate::T] to refer to this.
	Eof,

	/// Represents a token with [Kind::Comment][css_lexer::Kind::Comment]. Use [T![Comment]][crate::T] to refer to this.
	Comment,

	/// Represents a token with [Kind::CdcOrCdo][css_lexer::Kind::CdcOrCdo]. Use [T![CdcOrCdo]][crate::T] to refer to this.
	CdcOrCdo,

	/// Represents a token with [Kind::BadString][css_lexer::Kind::BadString]. Use [T![BadString]][crate::T] to refer to this.
	BadString,

	/// Represents a token with [Kind::BadUrl][css_lexer::Kind::BadUrl]. Use [T![BadUrl]][crate::T] to refer to this.[
	BadUrl,

	/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim], can be any single character. Use [T![Delim]][crate::T] to refer to this.
	Delim,

	/// Represents a token with [Kind::Colon][css_lexer::Kind::Colon] - a `:` character. Use [T![:]][crate::T] to refer to this.
	Colon,

	/// Represents a token with [Kind::Semicolon][css_lexer::Kind::Semicolon] - a `;` character. Use [T![;]][crate::T] to refer to this.
	Semicolon,

	/// Represents a token with [Kind::Comma][css_lexer::Kind::Comma] - a `,` character. Use [T![,]][crate::T] to refer to this.
	Comma,

	/// Represents a token with [Kind::LeftCurly][css_lexer::Kind::LeftCurly] - a `{` character. Use [T!['{']][crate::T] to refer to this.
	LeftCurly,

	/// Represents a token with [Kind::LeftCurly][css_lexer::Kind::LeftCurly] - a `}` character. Use [T!['}']][crate::T] to refer to this.
	RightCurly,

	/// Represents a token with [Kind::LeftSquare][css_lexer::Kind::LeftSquare] - a `[` character. Use [T!['[']][crate::T] to refer to this.
	LeftSquare,

	/// Represents a token with [Kind::RightSquare][css_lexer::Kind::RightSquare] - a `]` character. Use [T![']']][crate::T] to refer to this.
	RightSquare,

	/// Represents a token with [Kind::LeftParen][css_lexer::Kind::LeftParen] - a `(` character. Use [T!['(']][crate::T] to refer to this.
	LeftParen,

	/// Represents a token with [Kind::RightParen][css_lexer::Kind::RightParen] - a `(` character. Use [T![')']][crate::T] to refer to this.
	RightParen,
}

impl PartialEq<char> for Delim {
	fn eq(&self, other: &char) -> bool {
		self.0 == *other
	}
}

define_kind_idents! {
	/// Represents a token with [Kind::Ident][css_lexer::Kind::Ident]. Use [T![Ident]][crate::T] to refer to this.
	Ident,

	/// Represents a token with [Kind::String][css_lexer::Kind::String]. Use [T![String]][crate::T] to refer to this.
	String,

	/// Represents a token with [Kind::Url][css_lexer::Kind::Url]. Use [T![Url]][crate::T] to refer to this.
	Url,

	/// Represents a token with [Kind::Function][css_lexer::Kind::Function]. Use [T![Function]][crate::T] to refer to this.
	Function,

	/// Represents a token with [Kind::AtKeyword][css_lexer::Kind::AtKeyword]. Use [T![AtKeyword]][crate::T] to refer to this.
	AtKeyword,

	/// Represents a token with [Kind::Hash][css_lexer::Kind::Hash]. Use [T![Hash]][crate::T] to refer to this.
	Hash,
}

/// Represents a token with [Kind::Whitespace]. Use [T![Whitespace]][crate::T] to refer to
/// this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Whitespace(Cursor);

impl From<Whitespace> for Cursor {
	fn from(value: Whitespace) -> Self {
		value.0
	}
}

impl From<&Whitespace> for Cursor {
	fn from(value: &Whitespace) -> Self {
		value.0
	}
}

impl From<Whitespace> for Token {
	fn from(value: Whitespace) -> Self {
		value.0.token()
	}
}

impl From<&Whitespace> for Token {
	fn from(value: &Whitespace) -> Self {
		value.0.token()
	}
}

impl From<Whitespace> for Span {
	fn from(value: Whitespace) -> Self {
		value.0.span()
	}
}

impl From<&Whitespace> for Span {
	fn from(value: &Whitespace) -> Self {
		value.0.span()
	}
}

impl<'a> Peek<'a> for Whitespace {
	fn peek(p: &Parser<'a>, _: Cursor) -> bool {
		// Whitespace needs to peek its own cursor because it was likely given one that skipped Whitespace.
		let c = p.peek_next_including_whitespace();
		c == Kind::Whitespace
	}
}

impl<'a> Parse<'a> for Whitespace {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		// Whitespace needs to implement parse so that it can change the skip-state to only ensuring Whitespace
		// is not ignored.
		let skip = p.set_skip(KindSet::COMMENTS);
		let c = p.next();
		p.set_skip(skip);
		if c != Kind::Whitespace {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		Ok(Self(c))
	}
}

/// Represents a token with [Kind::Ident] that also begins with two HYPHEN MINUS (`--`)
/// characters. Use [T![DashedIdent]][crate::T] to refer to this.
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

impl From<DashedIdent> for Span {
	fn from(value: DashedIdent) -> Self {
		let t: Cursor = value.into();
		t.into()
	}
}

impl From<&DashedIdent> for Span {
	fn from(value: &DashedIdent) -> Self {
		let t: Cursor = value.into();
		t.into()
	}
}

impl<'a> Peek<'a> for DashedIdent {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Ident && c.token().is_dashed_ident()
	}
}

impl<'a> Build<'a> for DashedIdent {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(Ident::build(p, c))
	}
}

/// Represents a token with [Kind::Dimension]. Use [T![Dimension]][crate::T] to refer to this.
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

impl<'a> Peek<'a> for Dimension {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
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

impl Dimension {
	/// Returns the [f32] representation of the dimension's value.
	pub fn value(&self) -> f32 {
		self.0.token().value()
	}

	/// Returns the [DimensionUnit].
	///
	/// If the dimension unit is custom (e.g. dashed), has escape characters, or is not a recognised CSS Dimension, this
	/// will return [DimensionUnit::Unknown].
	pub fn dimension_unit(&self) -> DimensionUnit {
		self.0.token().dimension_unit()
	}
}

/// Represents a token with [Kind::Number]. Use [T![Number]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Number(Cursor);

impl Number {
	pub const NUMBER_ZERO: Number = Number(Cursor::dummy(Token::NUMBER_ZERO));

	/// Returns the [f32] representation of the number's value.
	pub fn value(&self) -> f32 {
		self.0.token().value()
	}
}

impl From<Number> for Cursor {
	fn from(value: Number) -> Self {
		value.0
	}
}

impl From<&Number> for Cursor {
	fn from(value: &Number) -> Self {
		value.0
	}
}

impl From<Number> for Token {
	fn from(value: Number) -> Self {
		value.0.token()
	}
}

impl From<&Number> for Token {
	fn from(value: &Number) -> Self {
		value.0.token()
	}
}

impl From<Number> for Span {
	fn from(value: Number) -> Self {
		value.0.span()
	}
}

impl From<&Number> for Span {
	fn from(value: &Number) -> Self {
		value.0.span()
	}
}

impl Number {
	pub const ZERO: Number = Number(Cursor::dummy(Token::NUMBER_ZERO));
}

impl<'a> Peek<'a> for Number {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Kind::Number
	}
}

impl<'a> Build<'a> for Number {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c)
	}
}

impl From<Number> for f32 {
	fn from(value: Number) -> Self {
		value.0.token().value()
	}
}

impl From<Number> for i32 {
	fn from(value: Number) -> Self {
		value.0.token().value() as i32
	}
}

impl PartialEq<f32> for Number {
	fn eq(&self, other: &f32) -> bool {
		self.0.token().value() == *other
	}
}

/// Various [T!s][crate::T] representing a tokens with [Kind::Delim], but each represents a discrete character.
pub mod delim {
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `&`. Use [T![&]][crate::T] to
		/// refer to this.
		And, '&'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `@`. Use [T![@]][crate::T] to
		/// refer to this. Not to be conused with [T![AtKeyword]][crate::T] which represents a token with
		/// [Kind::AtKeyword][css_lexer::Kind::AtKeyword].
		At, '@'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `^`. Use [T![^]][crate::T] to
		/// refer to this.
		Caret, '^'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `-`. Use [T![-]][crate::T] to
		/// refer to this.
		Dash, '-'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `$`. Use [T![$]][crate::T] to
		/// refer to this.
		Dollar, '$'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `.`. Use [T![.]][crate::T] to
		/// refer to this.
		Dot, '.'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `=`. Use [T![=]][crate::T] to
		/// refer to this.
		Eq, '='
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `>`. Use [T![>]][crate::T] to
		/// refer to this.
		Gt, '>'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `#`. Use [T![#]][crate::T] to
		/// refer to this. Not to be conused with [T![Hash]][crate::T] which represents a token with
		/// [Kind::Hash][css_lexer::Kind::Hash].
		Hash, '#'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `<`. Use [T![<]][crate::T] to
		/// refer to this.
		Lt, '<'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `!`. Use [T![!]][crate::T] to
		/// refer to this.
		Bang, '!'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `|`. Use [T![|]][crate::T] to
		/// refer to this.
		Or, '|'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `%`. Use [T![%]][crate::T] to
		/// refer to this.
		Percent, '%'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `+`. Use [T![+]][crate::T] to
		/// refer to this.
		Plus, '+'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `?`. Use [T![?]][crate::T] to
		/// refer to this.
		Question, '?'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `/`. Use [T![/]][crate::T] to
		/// refer to this.
		Slash, '/'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `*`. Use [T![*]][crate::T] to
		/// refer to this.
		Star, '*'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `~`. Use [T![~]][crate::T] to
		/// refer to this.
		Tilde, '~'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char `_`. Use [T![_]][crate::T] to
		/// refer to this.
		Underscore, '_'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][css_lexer::Kind::Delim] that has the char ```. Use [T!['`']][crate::T] to
		/// refer to this.
		Backtick, '`'
	}
}

/// Various [T!s][crate::T] representing two consecutive tokens that cannot be separated by any other tokens. These are
/// convenient as it can be tricky to parse two consecutive tokens given the default behaviour of the parser is to skip
/// whitespace and comments.
pub mod double {
	use css_lexer::Span;
	use css_lexer::{Cursor, Kind, KindSet};

	use crate::{CursorSink, Parse, Parser, Peek, Result, ToCursors, T};

	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `>` while the second has the char `=`, representing `>=`. Use
		/// [T![>=]][crate::T] to refer to this.
		GreaterThanEqual, '>', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `<` while the second has the char `=`, representing `<=`. Use
		/// [T![<=]][crate::T] to refer to this.
		LessThanEqual, '<', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `*` while the second has the char `|`, representing `*|`. Use
		/// [T![*|]][crate::T] to refer to this.
		StarPipe, '*', '|'
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `|` while the second has the char `|`, representing `||`. Use
		/// [T![||]][crate::T] to refer to this.
		PipePipe, '|', '|'
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `=` while the second has the char `=`, representing `==`. Use
		/// [T![==]][crate::T] to refer to this.
		EqualEqual, '=', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `~` while the second has the char `=`, representing `~=`. Use
		/// [T![~=]][crate::T] to refer to this.
		TildeEqual, '~', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `|` while the second has the char `=`, representing `|=`. Use
		/// [T![|=]][crate::T] to refer to this.
		PipeEqual, '|', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `^` while the second has the char `=`, representing `^=`. Use
		/// [T![\^=]][crate::T] to refer to this.
		CaretEqual, '^', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `$` while the second has the char `=`, representing `$=`. Use
		/// [T![$=]][crate::T] to refer to this.
		DollarEqual, '$', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][css_lexer::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `*` while the second has the char `=`, representing `*=`. Use
		/// [T![*=]][crate::T] to refer to this.
		StarEqual, '*', '='
	}

	/// Represents a two consecutive tokens with [Kind::Colon] that cannot be separated by any other token, representing
	/// `::`. Use [T![::]][crate::T] to refer to this.
	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub struct ColonColon(T![:], T![:]);

	impl ColonColon {
		pub const fn dummy() -> Self {
			Self(<T![:]>::dummy(), <T![:]>::dummy())
		}
	}

	impl<'a> Peek<'a> for ColonColon {
		fn peek(p: &Parser<'a>, c: Cursor) -> bool {
			c == Kind::Colon && p.peek_n(2) == Kind::Colon
		}
	}

	impl<'a> Parse<'a> for ColonColon {
		fn parse(p: &mut Parser<'a>) -> Result<Self> {
			let first = p.parse::<T![:]>()?;
			let skip = p.set_skip(KindSet::NONE);
			let second = p.parse::<T![:]>();
			p.set_skip(skip);
			Ok(Self(first, second?))
		}
	}

	impl ToCursors for ColonColon {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			s.append(self.0.into());
			s.append(self.1.into());
		}
	}

	impl From<ColonColon> for Span {
		fn from(value: ColonColon) -> Self {
			Into::<Span>::into(value.0) + Into::<Span>::into(value.1)
		}
	}

	impl From<&ColonColon> for Span {
		fn from(value: &ColonColon) -> Self {
			Into::<Span>::into(value.0) + Into::<Span>::into(value.1)
		}
	}
}

/// Dimension specific [T!s][crate::T]. These are all [Kind::Dimension], but each represents
/// a discrete dimension unit.
pub mod dimension {
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cap`. Use
		/// [T![Dimension::Cap]][crate::T] to refer to this.
		Cap, "cap"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `ch`. Use
		/// [T![Dimension::Ch]][crate::T] to refer to this.
		Ch, "ch"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cm`. Use
		/// [T![Dimension::Cm]][crate::T] to refer to this.
		Cm, "cm"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cqb`. Use
		/// [T![Dimension::Cqb]][crate::T] to refer to this.
		Cqb, "cqb"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cqh`. Use
		/// [T![Dimension::Cqh]][crate::T] to refer to this.
		Cqh, "cqh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cqi`. Use
		/// [T![Dimension::Cqi]][crate::T] to refer to this.
		Cqi, "cqi"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cqmax`. Use
		/// [T![Dimension::Cqmax]][crate::T] to refer to this.
		Cqmax, "cqmax"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cqmin`. Use
		/// [T![Dimension::Cqmin]][crate::T] to refer to this.
		Cqmin, "cqmin"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `cqw`. Use
		/// [T![Dimension::Cqw]][crate::T] to refer to this.
		Cqw, "cqw"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `deg`. Use
		/// [T![Dimension::Deg]][crate::T] to refer to this.
		Deg, "deg"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dpcm`. Use
		/// [T![Dimension::Dpcm]][crate::T] to refer to this.
		Dpcm, "dpcm"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dpi`. Use
		/// [T![Dimension::Dpi]][crate::T] to refer to this.
		Dpi, "dpi"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dppx`. Use
		/// [T![Dimension::Dppx]][crate::T] to refer to this.
		Dppx, "dppx"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dvb`. Use
		/// [T![Dimension::Dvb]][crate::T] to refer to this.
		Dvb, "dvb"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dvh`. Use
		/// [T![Dimension::Dvh]][crate::T] to refer to this.
		Dvh, "dvh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dvi`. Use
		/// [T![Dimension::Dvi]][crate::T] to refer to this.
		Dvi, "dvi"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dvmax`. Use
		/// [T![Dimension::Dvmax]][crate::T] to refer to this.
		Dvmax, "dvmax"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dvmin`. Use
		/// [T![Dimension::Dvmin]][crate::T] to refer to this.
		Dvmin, "dvmin"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `dvw`. Use
		/// [T![Dimension::Dvw]][crate::T] to refer to this.
		Dvw, "dvw"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `em`. Use
		/// [T![Dimension::Em]][crate::T] to refer to this.
		Em, "em"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `ex`. Use
		/// [T![Dimension::Ex]][crate::T] to refer to this.
		Ex, "ex"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `fr`. Use
		/// [T![Dimension::Fr]][crate::T] to refer to this.
		Fr, "fr"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `grad`. Use
		/// [T![Dimension::Grad]][crate::T] to refer to this.
		Grad, "grad"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `hz`. Use
		/// [T![Dimension::Hz]][crate::T] to refer to this.
		Hz, "hz"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `ic`. Use
		/// [T![Dimension::Ic]][crate::T] to refer to this.
		Ic, "ic"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `in`. Use
		/// [T![Dimension::In]][crate::T] to refer to this.
		In, "in"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `khz`. Use
		/// [T![Dimension::Khz]][crate::T] to refer to this.
		Khz, "khz"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lh`. Use
		/// [T![Dimension::Lh]][crate::T] to refer to this.
		Lh, "lh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lvb`. Use
		/// [T![Dimension::Lvb]][crate::T] to refer to this.
		Lvb, "lvb"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lvh`. Use
		/// [T![Dimension::Lvh]][crate::T] to refer to this.
		Lvh, "lvh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lvi`. Use
		/// [T![Dimension::Lvi]][crate::T] to refer to this.
		Lvi, "lvi"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lvmax`. Use
		/// [T![Dimension::Lvmax]][crate::T] to refer to this.
		Lvmax, "lvmax"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lvmin`. Use
		/// [T![Dimension::Lvmin]][crate::T] to refer to this.
		Lvmin, "lvmin"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `lvw`. Use
		/// [T![Dimension::Lvw]][crate::T] to refer to this.
		Lvw, "lvw"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `mm`. Use
		/// [T![Dimension::Mm]][crate::T] to refer to this.
		Mm, "mm"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `ms`. Use
		/// [T![Dimension::Ms]][crate::T] to refer to this.
		Ms, "ms"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `pc`. Use
		/// [T![Dimension::Pc]][crate::T] to refer to this.
		Pc, "pc"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `%`. Use
		/// [T![Dimension::%]][crate::T] to refer to this.
		Percent, "%"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `pt`. Use
		/// [T![Dimension::Pt]][crate::T] to refer to this.
		Pt, "pt"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `px`. Use
		/// [T![Dimension::Px]][crate::T] to refer to this.
		Px, "px"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `q`. Use
		/// [T![Dimension::Q]][crate::T] to refer to this.
		Q, "q"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `rad`. Use
		/// [T![Dimension::Rad]][crate::T] to refer to this.
		Rad, "rad"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `rcap`. Use
		/// [T![Dimension::Rcap]][crate::T] to refer to this.
		Rcap, "rcap"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `rch`. Use
		/// [T![Dimension::Rch]][crate::T] to refer to this.
		Rch, "rch"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `rem`. Use
		/// [T![Dimension::Rem]][crate::T] to refer to this.
		Rem, "rem"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `rex`. Use
		/// [T![Dimension::Rex]][crate::T] to refer to this.
		Rex, "rex"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `ric`. Use
		/// [T![Dimension::Ric]][crate::T] to refer to this.
		Ric, "ric"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `rlh`. Use
		/// [T![Dimension::Rlh]][crate::T] to refer to this.
		Rlh, "rlh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `s`. Use
		/// [T![Dimension::S]][crate::T] to refer to this.
		S, "s"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `svb`. Use
		/// [T![Dimension::Svb]][crate::T] to refer to this.
		Svb, "svb"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `svh`. Use
		/// [T![Dimension::Svh]][crate::T] to refer to this.
		Svh, "svh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `svi`. Use
		/// [T![Dimension::Svi]][crate::T] to refer to this.
		Svi, "svi"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `svmax`. Use
		/// [T![Dimension::Svmax]][crate::T] to refer to this.
		Svmax, "svmax"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `svmin`. Use
		/// [T![Dimension::Svmin]][crate::T] to refer to this.
		Svmin, "svmin"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `svw`. Use
		/// [T![Dimension::Svw]][crate::T] to refer to this.
		Svw, "svw"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `turn`. Use
		/// [T![Dimension::Turn]][crate::T] to refer to this.
		Turn, "turn"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `vb`. Use
		/// [T![Dimension::Vb]][crate::T] to refer to this.
		Vb, "vb"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `vh`. Use
		/// [T![Dimension::Vh]][crate::T] to refer to this.
		Vh, "vh"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `vi`. Use
		/// [T![Dimension::Vi]][crate::T] to refer to this.
		Vi, "vi"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `vmax`. Use
		/// [T![Dimension::Vmax]][crate::T] to refer to this.
		Vmax, "vmax"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `vmin`. Use
		/// [T![Dimension::Vmin]][crate::T] to refer to this.
		Vmin, "vmin"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `vw`. Use
		/// [T![Dimension::Vw]][crate::T] to refer to this.
		Vw, "vw"
	}
	custom_dimension! {
		/// Represents a token with [Kind::Dimension][css_lexer::Kind::Dimension] where the dimension unit was `x`. Use
		/// [T![Dimension::X]][crate::T] to refer to this.
		X, "x"
	}
}

/// Represents any possible single token. Use [T![Any]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> Peek<'a> for Any {
	fn peek(_: &Parser<'a>, _: Cursor) -> bool {
		true
	}
}

impl<'a> Build<'a> for Any {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c)
	}
}

/// Represents a token with either [Kind::LeftCurly], [Kind::LeftParen] or [Kind::LeftSquare]. Use
/// [T![PairWiseStart]][crate::T] to refer to this.
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

	pub fn end(&self) -> Kind {
		match self.kind() {
			Kind::LeftCurly => Kind::RightCurly,
			Kind::LeftParen => Kind::RightParen,
			Kind::LeftSquare => Kind::RightSquare,
			k => k,
		}
	}
}

impl<'a> Peek<'a> for PairWiseStart {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly, Kind::LeftSquare, Kind::LeftParen]);
}

impl<'a> Build<'a> for PairWiseStart {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c.token())
	}
}

/// Represents a token with either [Kind::RightCurly], [Kind::RightParen] or [Kind::RightSquare]. Use
/// [T![PairWiseEnd]][crate::T] to refer to this.
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

	pub fn start(&self) -> Kind {
		match self.kind() {
			Kind::RightCurly => Kind::LeftCurly,
			Kind::RightParen => Kind::LeftParen,
			Kind::RightSquare => Kind::LeftSquare,
			k => k,
		}
	}
}

impl<'a> Peek<'a> for PairWiseEnd {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::RightCurly, Kind::RightSquare, Kind::RightParen]);
}

impl<'a> Build<'a> for PairWiseEnd {
	fn build(_: &Parser<'a>, c: Cursor) -> Self {
		Self(c.token())
	}
}

/// The [T!][crate::T] macro expands to the name of a type representing the Token of the same name. These can be used in struct
/// fields to type child nodes.
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
	[!] => { $crate::token_macros::delim::Bang };
	[|] => { $crate::token_macros::delim::Or };
	[%] => { $crate::token_macros::delim::Percent };
	[+] => { $crate::token_macros::delim::Plus };
	[?] => { $crate::token_macros::delim::Question };
	[/] => { $crate::token_macros::delim::Slash };
	[*] => { $crate::token_macros::delim::Star };
	[~] => { $crate::token_macros::delim::Tilde };
	[_] => { $crate::token_macros::delim::Underscore };
	['`'] => { $crate::token_macros::delim::Backtick };

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

#[cfg(test)]
mod tests {
	use crate::Parser;
	use bumpalo::Bump;
	use css_lexer::{Cursor, DimensionUnit};

	#[test]
	fn test_custom_dimension() {
		custom_dimension!(Px, "px");
		let allocator = Bump::new();
		let mut p = Parser::new(&allocator, "1px");
		let result = p.parse_entirely::<Px>();
		assert!(matches!(result.output, Some(Px(_))));
		let c: Cursor = result.output.unwrap().into();
		assert!(c.token().value() == 1.0);
		assert!(c.token().dimension_unit() == DimensionUnit::Px);
		let mut p = Parser::new(&allocator, "1rem");
		let result = p.parse_entirely::<Px>();
		assert!(result.output.is_none());
	}
}
