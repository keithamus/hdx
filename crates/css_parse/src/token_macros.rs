use crate::{
	Cursor, CursorSink, Kind, KindSet, Parse, Parser, Peek, Result, Span, ToNormalisedValue, ToNumberValue, Token,
};

macro_rules! cursor_wrapped {
	($ident:ident) => {
		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl CursorSink) {
				s.append((*self).into());
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for $crate::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> Span {
				self.0.to_span()
			}
		}

		impl $crate::SemanticEq for $ident {
			fn semantic_eq(&self, s: &Self, source_text: &str) -> bool {
				self.0.semantic_eq(&s.0, source_text)
			}
		}
	};
}

/// Shared body for [define_kinds!] and [define_fixed_kinds!]; everything except the
/// `SemanticEq` impl, which differs between the two (see [define_fixed_kinds!]).
macro_rules! define_kind_common {
	($(#[$meta:meta])* $ident:ident) => {
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::Cursor);

		impl $ident {
			pub const fn dummy() -> Self {
				Self($crate::Cursor::dummy($crate::Token::dummy($crate::Kind::$ident)))
			}

			pub fn associated_whitespace(&self) -> $crate::AssociatedWhitespaceRules {
				self.0.token().associated_whitespace()
			}

			pub fn with_associated_whitespace(&self, rules: $crate::AssociatedWhitespaceRules) -> Self {
				Self(self.0.map_token(|t| t.with_associated_whitespace(rules)))
			}
		}

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			const PEEK_KINDSET: $crate::KindSet = $crate::KindSet::new(&[$crate::Kind::$ident]);
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse<I>(p: &mut $crate::Parser<'a, I>) -> $crate::Result<Self>
			where
				I: ::std::iter::Iterator<Item = $crate::Cursor> + ::std::clone::Clone,
			{
				let c = p.next();
				if Self::peek(p, c) { Ok(Self(c)) } else { Err($crate::Diagnostic::new(c, $crate::Diagnostic::unexpected))? }
			}
		}


		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for $crate::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}
	};
}

macro_rules! define_kinds {
	($($(#[$meta:meta])* $ident:ident,)*) => {
		$(
		define_kind_common!($(#[$meta])* $ident);

		impl $crate::SemanticEq for $ident {
			fn semantic_eq(&self, s: &Self, source_text: &str) -> bool {
				self.0.semantic_eq(&s.0, source_text)
			}
		}
		)*
	};
}

/// Like [define_kinds!], but for kinds whose content is entirely fixed by the Rust type - once
/// parsing succeeds there is no varying data left to compare (e.g. a [Comma] is always just a
/// `,`; the only bits that could otherwise differ are non-semantic associated-whitespace
/// formatting hints). `semantic_eq` for these kinds is therefore always `true`, skipping the
/// token comparison outright.
macro_rules! define_fixed_kinds {
	($($(#[$meta:meta])* $ident:ident,)*) => {
		$(
		define_kind_common!($(#[$meta])* $ident);

		impl $crate::SemanticEq for $ident {
			#[inline(always)]
			fn semantic_eq(&self, _: &Self, _source_text: &str) -> bool {
				true
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
		pub struct $ident($crate::Cursor);

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			const PEEK_KINDSET: $crate::KindSet = $crate::KindSet::new(&[$crate::Kind::$ident]);
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse<I>(p: &mut $crate::Parser<'a, I>) -> $crate::Result<Self>
			where
				I: ::std::iter::Iterator<Item = $crate::Cursor> + ::std::clone::Clone,
			{
				let c = p.next();
				if Self::peek(p, c) { Ok(Self(c)) } else { Err($crate::Diagnostic::new(c, $crate::Diagnostic::unexpected))? }
			}
		}


		impl From<$ident> for $crate::Kind {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0
			}
		}

		impl From<$ident> for $crate::Token {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl $crate::SemanticEq for $ident {
			fn semantic_eq(&self, s: &Self, source_text: &str) -> bool {
				self.0.semantic_eq(&s.0, source_text)
			}
		}

		impl $ident {
			/// Checks if the ident begins with two HYPHEN MINUS (`--`) characters.
			pub fn is_dashed_ident(&self) -> bool {
				self.0.token().is_dashed_ident()
			}

			pub const fn dummy() -> Self {
				Self($crate::Cursor::dummy($crate::Token::dummy($crate::Kind::$ident)))
			}
		}
		)*
	};
}

/// A macro for defining a struct which captures a [Kind::Delim][Kind::Delim] with a specific character.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// custom_delim!{
///   /// A £ character.
///   PoundSterling, '£'
/// }
///
/// assert_parse!(EmptyAtomSet::ATOMS, PoundSterling, "£");
/// ```
#[macro_export]
macro_rules! custom_delim {
	($(#[$meta:meta])* $ident:ident, $ch:literal) => {
		$(#[$meta])*
		#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		pub struct $ident($crate::T![Delim]);

		impl $ident {
			pub fn associated_whitespace(&self) -> $crate::AssociatedWhitespaceRules {
				self.0.associated_whitespace()
			}

			pub fn with_associated_whitespace(&self, rules: $crate::AssociatedWhitespaceRules) -> Self {
				Self(self.0.with_associated_whitespace(rules))
			}
		}

		impl $crate::ToCursors for $ident {
			fn to_cursors(&self, s: &mut impl $crate::CursorSink) {
				s.append((*self).into());
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek<I>(_: &$crate::Parser<'a, I>, c: $crate::Cursor) -> bool
			where
				I: ::std::iter::Iterator<Item = $crate::Cursor> + ::std::clone::Clone,
			{
				c == $crate::Kind::Delim && c == $ch
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse<I>(p: &mut $crate::Parser<'a, I>) -> $crate::Result<Self>
			where
				I: ::std::iter::Iterator<Item = $crate::Cursor> + ::std::clone::Clone,
			{
				use $crate::Peek;
				let delim = p.parse::<$crate::T![Delim]>()?;
				if Self::peek(p, delim.into()) {
					Ok(Self(delim))
				} else {
					Err($crate::Diagnostic::new(delim.into(), $crate::Diagnostic::unexpected))?
				}
			}
		}



		impl From<$ident> for $crate::Cursor {
			fn from(value: $ident) -> Self {
				value.0.into()
			}
		}

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span()
			}
		}

		impl PartialEq<char> for $ident {
			fn eq(&self, other: &char) -> bool {
				self.0 == *other
			}
		}

		impl $crate::SemanticEq for $ident {
			#[inline(always)]
			fn semantic_eq(&self, _: &Self, _source_text: &str) -> bool {
				// The character is fixed by the type itself (parsing only succeeds for
				// `$ch`), so there is nothing left to compare.
				true
			}
		}
	};
}

/// A macro for defining a struct which captures two adjacent [Kind::Delim][Kind::Delim] tokens, each with a
/// specific character.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// custom_double_delim!{
///   /// Two % adjacent symbols
///   DoublePercent, '%', '%'
/// }
///
/// assert_parse!(EmptyAtomSet::ATOMS, DoublePercent, "%%");
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
			fn peek<I>(p: &$crate::Parser<'a, I>, c: $crate::Cursor) -> bool
			where
				I: ::std::iter::Iterator<Item = $crate::Cursor> + ::std::clone::Clone,
			{
				c == $first && p.peek_n(2) == $second
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse<I>(p: &mut $crate::Parser<'a, I>) -> $crate::Result<Self>
			where
				I: ::std::iter::Iterator<Item = $crate::Cursor> + ::std::clone::Clone,
			{
				let first = p.parse::<$crate::T![Delim]>()?;
				if first != $first {
					let c: Cursor = first.into();
					Err($crate::Diagnostic::new(c, $crate::Diagnostic::expected_delim))?;
				}
				let skip = p.set_skip($crate::KindSet::NONE);
				let second = p.parse::<$crate::T![Delim]>();
				p.set_skip(skip);
				let second = second?;
				if second != $second {
					let c:Cursor = second.into();
					Err($crate::Diagnostic::new(c, $crate::Diagnostic::expected_delim))?;
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

		impl $crate::ToSpan for $ident {
			fn to_span(&self) -> $crate::Span {
				self.0.to_span() + self.1.to_span()
			}
		}

		impl $crate::SemanticEq for $ident {
			#[inline(always)]
			fn semantic_eq(&self, _: &Self, _source_text: &str) -> bool {
				// Both characters are fixed by the type itself (`$first` then `$second`), so
				// there is nothing left to compare.
				true
			}
		}
	};
}

define_kinds! {
	/// Represents a token with [Kind::Eof][Kind::Eof]. Use [T![Eof]][crate::T] to refer to this.
	Eof,

	/// Represents a token with [Kind::Comment][Kind::Comment]. Use [T![Comment]][crate::T] to refer to this.
	Comment,

	/// Represents a token with [Kind::CdcOrCdo][Kind::CdcOrCdo]. Use [T![CdcOrCdo]][crate::T] to refer to this.
	CdcOrCdo,

	/// Represents a token with [Kind::BadString][Kind::BadString]. Use [T![BadString]][crate::T] to refer to this.
	BadString,

	/// Represents a token with [Kind::BadUrl][Kind::BadUrl]. Use [T![BadUrl]][crate::T] to refer to this.[
	BadUrl,

	/// Represents a token with [Kind::Delim][Kind::Delim], can be any single character. Use [T![Delim]][crate::T] to refer to this.
	Delim,
}

define_fixed_kinds! {
	/// Represents a token with [Kind::Colon][Kind::Colon] - a `:` character. Use [T![:]][crate::T] to refer to this.
	Colon,

	/// Represents a token with [Kind::Semicolon][Kind::Semicolon] - a `;` character. Use [T![;]][crate::T] to refer to this.
	Semicolon,

	/// Represents a token with [Kind::Comma][Kind::Comma] - a `,` character. Use [T![,]][crate::T] to refer to this.
	Comma,

	/// Represents a token with [Kind::LeftCurly][Kind::LeftCurly] - a `{` character. Use [T!['{']][crate::T] to refer to this.
	LeftCurly,

	/// Represents a token with [Kind::LeftCurly][Kind::LeftCurly] - a `}` character. Use [T!['}']][crate::T] to refer to this.
	RightCurly,

	/// Represents a token with [Kind::LeftSquare][Kind::LeftSquare] - a `[` character. Use [T!['[']][crate::T] to refer to this.
	LeftSquare,

	/// Represents a token with [Kind::RightSquare][Kind::RightSquare] - a `]` character. Use [T![']']][crate::T] to refer to this.
	RightSquare,

	/// Represents a token with [Kind::LeftParen][Kind::LeftParen] - a `(` character. Use [T!['(']][crate::T] to refer to this.
	LeftParen,

	/// Represents a token with [Kind::RightParen][Kind::RightParen] - a `(` character. Use [T![')']][crate::T] to refer to this.
	RightParen,
}

impl PartialEq<char> for Delim {
	fn eq(&self, other: &char) -> bool {
		self.0 == *other
	}
}

define_kind_idents! {
	/// Represents a token with [Kind::Ident][Kind::Ident]. Use [T![Ident]][crate::T] to refer to this.
	Ident,

	/// Represents a token with [Kind::String][Kind::String]. Use [T![String]][crate::T] to refer to this.
	String,

	/// Represents a token with [Kind::Url][Kind::Url]. Use [T![Url]][crate::T] to refer to this.
	Url,

	/// Represents a token with [Kind::Function][Kind::Function]. Use [T![Function]][crate::T] to refer to this.
	Function,

	/// Represents a token with [Kind::AtKeyword][Kind::AtKeyword]. Use [T![AtKeyword]][crate::T] to refer to this.
	AtKeyword,

	/// Represents a token with [Kind::Hash][Kind::Hash]. Use [T![Hash]][crate::T] to refer to this.
	Hash,
}

/// Represents a token with [Kind::Whitespace]. Use [T![Whitespace]][crate::T] to refer to
/// this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Whitespace(Cursor);
cursor_wrapped!(Whitespace);

impl Whitespace {
	/// Returns a new [Whitespace] with the significance flag set to `significant`.
	///
	/// Whitespace parsed with [Parse] is significant by default: a [Whitespace] in an AST node carries meaning
	/// (a descendant combinator, or the space in `@charset "utf-8";`) and minifiers must keep it. Set this to
	/// `false` for whitespace kept only as trivia, which a minifier is free to remove.
	pub fn with_significant_whitespace(&self, significant: bool) -> Self {
		Self(self.0.map_token(|t| t.with_significant_whitespace(significant)))
	}
}

impl<'a> Peek<'a> for Whitespace {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Whitespace]);

	fn peek<I>(p: &Parser<'a, I>, _: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Whitespace needs to peek its own cursor because it was likely given one that skipped Whitespace.
		let c = p.peek_n_with_skip(1, KindSet::COMMENTS);
		c == Kind::Whitespace
	}
}

impl<'a> Parse<'a> for Whitespace {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Whitespace needs to implement parse so that it can change the skip-state to only ensuring Whitespace
		// is not ignored.
		let skip = p.set_skip(KindSet::COMMENTS);
		let c = p.next();
		p.set_skip(skip);
		if c != Kind::Whitespace {
			Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?
		}
		Ok(Self(c.map_token(|t| t.with_significant_whitespace(true))))
	}
}

/// Represents a token with [Kind::Ident] that also begins with two HYPHEN MINUS (`--`)
/// characters. Use [T![DashedIdent]][crate::T] to refer to this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DashedIdent(Ident);
cursor_wrapped!(DashedIdent);

impl<'a> Peek<'a> for DashedIdent {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);

	#[inline(always)]
	fn peek<I>(_: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == Kind::Ident && c.token().is_dashed_ident()
	}
}

impl<'a> Parse<'a> for DashedIdent {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		if Self::peek(p, c) {
			Ok(Self(Ident(c)))
		} else {
			Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?
		}
	}
}

/// Represents a token with [Kind::Dimension]. Use [T![Dimension]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Dimension(Cursor);
cursor_wrapped!(Dimension);

impl PartialEq<f32> for Dimension {
	fn eq(&self, other: &f32) -> bool {
		self.0.token().value() == *other
	}
}

impl<'a> Peek<'a> for Dimension {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Dimension]);
}

impl<'a> Parse<'a> for Dimension {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		if Self::peek(p, c) { Ok(Self(c)) } else { Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))? }
	}
}

impl From<Dimension> for f32 {
	fn from(val: Dimension) -> Self {
		val.0.token().value()
	}
}

impl ToNumberValue for Dimension {
	fn to_number_value(&self) -> Option<f32> {
		Some(self.0.token().value())
	}
}

impl Dimension {
	/// Returns the [f32] representation of the dimension's value.
	pub fn value(&self) -> f32 {
		self.0.token().value()
	}
}

/// Represents a token with [Kind::Number]. Use [T![Number]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Number(Cursor);
cursor_wrapped!(Number);

impl Number {
	pub const NUMBER_ZERO: Number = Number(Cursor::dummy(Token::NUMBER_ZERO));
	pub const ZERO: Number = Number(Cursor::dummy(Token::NUMBER_ZERO));

	/// Returns the [f32] representation of the number's value.
	pub fn value(&self) -> f32 {
		self.0.token().value()
	}

	pub fn is_int(&self) -> bool {
		self.0.token().is_int()
	}

	pub fn is_float(&self) -> bool {
		self.0.token().is_float()
	}

	pub fn has_sign(&self) -> bool {
		self.0.token().has_sign()
	}

	pub fn preserve_sign(self) -> Self {
		if self.has_sign() { Self(self.0.map_token(|t| t.with_sign_required())) } else { self }
	}
}

impl<'a> Peek<'a> for Number {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Number]);
}

impl<'a> Parse<'a> for Number {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		if Self::peek(p, c) { Ok(Self(c)) } else { Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))? }
	}
}

impl From<Number> for f32 {
	fn from(value: Number) -> Self {
		value.value()
	}
}

impl From<Number> for i32 {
	fn from(value: Number) -> Self {
		value.value() as i32
	}
}

impl PartialEq<f32> for Number {
	fn eq(&self, other: &f32) -> bool {
		self.value() == *other
	}
}

impl ToNumberValue for Number {
	fn to_number_value(&self) -> Option<f32> {
		Some(self.value())
	}
}

impl ToNormalisedValue for Number {
	fn to_normalised_value(&self) -> Option<f32> {
		self.to_number_value()
	}
}

/// Various [T!s][crate::T] representing a tokens with [Kind::Delim], but each represents a discrete character.
pub mod delim {
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `&`. Use [T![&]][crate::T] to
		/// refer to this.
		And, '&'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `@`. Use [T![@]][crate::T] to
		/// refer to this. Not to be conused with [T![AtKeyword]][crate::T] which represents a token with
		/// [Kind::AtKeyword][crate::Kind::AtKeyword].
		At, '@'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `^`. Use [T![^]][crate::T] to
		/// refer to this.
		Caret, '^'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `-`. Use [T![-]][crate::T] to
		/// refer to this.
		Dash, '-'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `$`. Use [T![$]][crate::T] to
		/// refer to this.
		Dollar, '$'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `.`. Use [T![.]][crate::T] to
		/// refer to this.
		Dot, '.'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `=`. Use [T![=]][crate::T] to
		/// refer to this.
		Eq, '='
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `>`. Use [T![>]][crate::T] to
		/// refer to this.
		Gt, '>'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `#`. Use [T![#]][crate::T] to
		/// refer to this. Not to be conused with [T![Hash]][crate::T] which represents a token with
		/// [Kind::Hash][crate::Kind::Hash].
		Hash, '#'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `<`. Use [T![<]][crate::T] to
		/// refer to this.
		Lt, '<'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `!`. Use [T![!]][crate::T] to
		/// refer to this.
		Bang, '!'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `|`. Use [T![|]][crate::T] to
		/// refer to this.
		Or, '|'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `%`. Use [T![%]][crate::T] to
		/// refer to this.
		Percent, '%'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `+`. Use [T![+]][crate::T] to
		/// refer to this.
		Plus, '+'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `?`. Use [T![?]][crate::T] to
		/// refer to this.
		Question, '?'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `/`. Use [T![/]][crate::T] to
		/// refer to this.
		Slash, '/'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `*`. Use [T![*]][crate::T] to
		/// refer to this.
		Star, '*'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `~`. Use [T![~]][crate::T] to
		/// refer to this.
		Tilde, '~'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char `_`. Use [T![_]][crate::T] to
		/// refer to this.
		Underscore, '_'
	}
	custom_delim! {
		/// Represents a token with [Kind::Delim][crate::Kind::Delim] that has the char ```. Use [T!['`']][crate::T] to
		/// refer to this.
		Backtick, '`'
	}
}

/// Various [T!s][crate::T] representing two consecutive tokens that cannot be separated by any other tokens. These are
/// convenient as it can be tricky to parse two consecutive tokens given the default behaviour of the parser is to skip
/// whitespace and comments.
pub mod double {
	use crate::{
		Cursor, CursorSink, Kind, KindSet, Parse, Parser, Peek, Result, SemanticEq, Span, T, ToCursors, ToSpan,
	};

	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `>` while the second has the char `=`, representing `>=`. Use
		/// [T![>=]][crate::T] to refer to this.
		GreaterThanEqual, '>', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `<` while the second has the char `=`, representing `<=`. Use
		/// [T![<=]][crate::T] to refer to this.
		LessThanEqual, '<', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `*` while the second has the char `|`, representing `*|`. Use
		/// [T![*|]][crate::T] to refer to this.
		StarPipe, '*', '|'
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `|` while the second has the char `|`, representing `||`. Use
		/// [T![||]][crate::T] to refer to this.
		PipePipe, '|', '|'
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `=` while the second has the char `=`, representing `==`. Use
		/// [T![==]][crate::T] to refer to this.
		EqualEqual, '=', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `~` while the second has the char `=`, representing `~=`. Use
		/// [T![~=]][crate::T] to refer to this.
		TildeEqual, '~', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `|` while the second has the char `=`, representing `|=`. Use
		/// [T![|=]][crate::T] to refer to this.
		PipeEqual, '|', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `^` while the second has the char `=`, representing `^=`. Use
		/// [T![\^=]][crate::T] to refer to this.
		CaretEqual, '^', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `$` while the second has the char `=`, representing `$=`. Use
		/// [T![$=]][crate::T] to refer to this.
		DollarEqual, '$', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `*` while the second has the char `=`, representing `*=`. Use
		/// [T![*=]][crate::T] to refer to this.
		StarEqual, '*', '='
	}
	custom_double_delim! {
		/// Represents a two consecutive tokens with [Kind::Delim][crate::Kind::Delim] that cannot be separated by any
		/// other token. The first token has the char `!` while the second has the char `=`, representing `!=`. Use
		/// [T![!=]][crate::T] to refer to this.
		BangEqual, '*', '='
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
		fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
		where
			I: Iterator<Item = Cursor> + Clone,
		{
			c == Kind::Colon && p.peek_n(2) == Kind::Colon
		}
	}

	impl<'a> Parse<'a> for ColonColon {
		fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
		where
			I: Iterator<Item = Cursor> + Clone,
		{
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

	impl ToSpan for ColonColon {
		fn to_span(&self) -> Span {
			self.0.to_span() + self.1.to_span()
		}
	}

	impl SemanticEq for ColonColon {
		#[inline(always)]
		fn semantic_eq(&self, _: &Self, _source_text: &str) -> bool {
			// Both `:` characters are fixed by the type itself, so there is nothing left to
			// compare.
			true
		}
	}
}

/// Represents any possible single token. Use [T![Any]][crate::T] to refer to this.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Any(Cursor);
cursor_wrapped!(Any);

impl<'a> Peek<'a> for Any {
	fn peek<I>(_: &Parser<'a, I>, _: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		true
	}
}

impl<'a> Parse<'a> for Any {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		Ok(Self(c))
	}
}

/// Represents a token with either [Kind::LeftCurly], [Kind::LeftParen] or [Kind::LeftSquare]. Use
/// [T![PairWiseStart]][crate::T] to refer to this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PairWiseStart(Cursor);
cursor_wrapped!(PairWiseStart);

impl PairWiseStart {
	pub fn kind(&self) -> Kind {
		self.0.token().kind()
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

impl<'a> Parse<'a> for PairWiseStart {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		if Self::peek(p, c) { Ok(Self(c)) } else { Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))? }
	}
}

/// Represents a token with either [Kind::RightCurly], [Kind::RightParen] or [Kind::RightSquare]. Use
/// [T![PairWiseEnd]][crate::T] to refer to this.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PairWiseEnd(Cursor);
cursor_wrapped!(PairWiseEnd);

impl PairWiseEnd {
	pub fn kind(&self) -> Kind {
		self.0.token().kind()
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

impl<'a> Parse<'a> for PairWiseEnd {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.next();
		if Self::peek(p, c) { Ok(Self(c)) } else { Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))? }
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
	[!=] => { $crate::token_macros::double::BangEqual };

	[Dimension::$ident: ident] => { $crate::token_macros::dimension::$ident };

	[!important] => { $crate::token_macros::double::BangImportant };

	[$ident:ident] => { $crate::token_macros::$ident }
}

#[cfg(test)]
mod fixed_kind_semantic_eq_tests {
	use super::*;
	use crate::SemanticEq;
	use css_lexer::{AssociatedWhitespaceRules, SourceOffset};

	// Colon, Semicolon, Comma, and the paren/curly/square brackets are "delim-like": they
	// share Delim's bit layout and can carry non-semantic associated-whitespace formatting
	// hints, which makes two otherwise-identical tokens compare unequal via plain `PartialEq`.
	// `semantic_eq` must ignore this entirely for these kinds, since there is no other varying
	// content once the type is known.
	#[test]
	fn fixed_punctuation_kinds_are_always_semantic_eq() {
		macro_rules! check {
			($ty:ident, $token:expr) => {{
				let plain = $ty(Cursor::new(SourceOffset(0), $token));
				let with_rule = $ty(Cursor::new(
					SourceOffset(0),
					$token.with_associated_whitespace(AssociatedWhitespaceRules::EnforceBefore),
				));
				assert_ne!(
					plain,
					with_rule,
					"associated whitespace should still affect PartialEq for {}",
					stringify!($ty)
				);
				assert!(
					plain.semantic_eq(&with_rule, ""),
					"{} should always be semantic_eq regardless of associated whitespace",
					stringify!($ty)
				);
			}};
		}
		check!(Colon, Token::COLON);
		check!(Semicolon, Token::SEMICOLON);
		check!(Comma, Token::COMMA);
		check!(LeftCurly, Token::LEFT_CURLY);
		check!(RightCurly, Token::RIGHT_CURLY);
		check!(LeftSquare, Token::LEFT_SQUARE);
		check!(RightSquare, Token::RIGHT_SQUARE);
		check!(LeftParen, Token::LEFT_PAREN);
		check!(RightParen, Token::RIGHT_PAREN);
	}
}
