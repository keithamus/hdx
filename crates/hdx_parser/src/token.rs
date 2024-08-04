use hdx_lexer::{Kind, PairWise, QuoteStyle, Span, Token};

use crate::{
	traits::{Parse, Peek},
	Result,
};

macro_rules! delegate_to_token {
	($ident:ident) => {
		impl $ident {
			#[inline]
			pub fn span(&self) -> Span {
				self.0.span()
			}

			#[inline]
			pub fn kind(&self) -> Kind {
				self.0.kind()
			}

			#[inline]
			pub fn len(&self) -> u32 {
				self.0.len()
			}

			#[inline]
			pub fn is_empty(&self) -> bool {
				self.0.is_empty()
			}

			#[inline]
			pub fn is_ident_like(&self) -> bool {
				self.0.is_empty()
			}

			#[inline]
			pub fn char(&self) -> Option<char> {
				self.0.char()
			}

			#[inline]
			pub fn is_int(&self) -> bool {
				self.0.is_int()
			}

			#[inline]
			pub fn is_float(&self) -> bool {
				self.0.is_float()
			}

			#[inline]
			pub fn has_sign(&self) -> bool {
				self.0.has_sign()
			}

			#[inline]
			pub fn numeric_len(&self) -> u32 {
				self.0.numeric_len()
			}

			#[inline]
			pub fn quote_style(&self) -> QuoteStyle {
				self.0.quote_style()
			}

			#[inline]
			pub fn string_has_closing_quote(&self) -> bool {
				self.0.string_has_closing_quote()
			}

			#[inline]
			pub fn can_escape(&self) -> bool {
				self.0.can_escape()
			}

			#[inline]
			pub fn contains_escape_chars(&self) -> bool {
				self.0.contains_escape_chars()
			}

			#[inline]
			pub fn is_dashed_ident(&self) -> bool {
				self.0.is_dashed_ident()
			}

			#[inline]
			pub fn is_lower_case(&self) -> bool {
				self.0.is_lower_case()
			}

			#[inline]
			pub fn is_trivia(&self) -> bool {
				self.0.is_trivia()
			}

			#[inline]
			pub fn url_has_leading_space(&self) -> bool {
				self.0.url_has_leading_space()
			}

			#[inline]
			pub fn url_has_closing_paren(&self) -> bool {
				self.0.url_has_closing_paren()
			}

			#[inline]
			pub fn contains_newline(&self) -> bool {
				self.0.contains_newline()
			}

			#[inline]
			pub fn contains_tab(&self) -> bool {
				self.0.contains_tab()
			}

			#[inline]
			pub fn hash_is_id_like(&self) -> bool {
				self.0.hash_is_id_like()
			}

			#[inline]
			pub fn is_bad(&self) -> bool {
				self.0.is_bad()
			}

			#[inline]
			pub fn is_cdc(&self) -> bool {
				self.0.is_cdc()
			}

			#[inline]
			pub fn to_pairwise(&self) -> Option<PairWise> {
				self.0.to_pairwise()
			}
		}

		impl From<Token> for $ident {
			fn from(t: Token) -> Self {
				Self(t)
			}
		}

		impl std::ops::Deref for $ident {
			type Target = Token;
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl core::fmt::Debug for $ident {
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				self.0.fmt(f)
			}
		}

		impl std::fmt::Display for $ident {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				self.0.fmt(f)
			}
		}

		#[cfg(feature = "serde")]
		impl serde::ser::Serialize for $ident {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				self.0.serialize::<S>(serializer)
			}
		}
	};
}

macro_rules! kind {
	($ident:ident) => {
		pub struct $ident(Token);
		delegate_to_token!($ident);

		impl<'a> Peek<'a> for $ident {
			fn peek(parser: &$crate::Parser<'a>) -> Option<Token> {
				parser.peek_kind(Kind::$ident)
			}
		}

		impl<'a> Parse<'a> for $ident {
			fn parse(parser: &mut $crate::Parser<'a>) -> Result<Self> {
				parser.next_kind(Kind::$ident).map(|t| t.into())
			}
		}
	};
}

kind!(Ident);
kind!(Eof);
kind!(Whitespace);
kind!(Comment);
kind!(CdcOrCdo);
kind!(Number);
kind!(Dimension);
kind!(BadString);
kind!(BadUrl);
kind!(Function);
kind!(AtKeyword);
kind!(Hash);
kind!(String);
kind!(Url);
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

macro_rules! make_delim {
	($iden:ident, $ch:literal) => {
		pub struct $iden(Token);
		delegate_to_token!($iden);

		impl<'a> Peek<'a> for $iden {
			fn peek(parser: &crate::Parser<'a>) -> Option<Token> {
				parser.peek_kind(Kind::Delim).filter(|t| t.char().unwrap() == $ch)
			}
		}

		impl<'a> Parse<'a> for $iden {
			fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
				Ok(parser
					.next_kind(Kind::Delim)
					.and_then(|t| {
						if t.char().unwrap() == $ch {
							Ok(t)
						} else {
							Err($crate::diagnostics::ExpectedDelim(t, t.span()))?
						}
					})?
					.into())
			}
		}
	};
}

make_delim!(DelimAnd, '&');
make_delim!(DelimAt, '@');
make_delim!(DelimCaret, '^');
make_delim!(DelimDash, '-');
make_delim!(DelimDollar, '$');
make_delim!(DelimDot, '.');
make_delim!(DelimEq, '=');
make_delim!(DelimGt, '>');
make_delim!(DelimHash, '#');
make_delim!(DelimLt, '<');
make_delim!(DelimNot, '!');
make_delim!(DelimOr, '|');
make_delim!(DelimPercent, '%');
make_delim!(DelimPlus, '+');
make_delim!(DelimQuestion, '?');
make_delim!(DelimSlash, '/');
make_delim!(DelimStar, '*');
make_delim!(DelimTilde, '~');
make_delim!(DelimUnderscore, '_');

pub struct Any(Token);
impl<'a> Peek<'a> for Any {
	fn peek(parser: &crate::Parser<'a>) -> Option<Token> {
		Some(parser.peek_n(1))
	}
}

impl<'a> Parse<'a> for Any {
	fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
		Ok(Self(parser.next()))
	}
}

impl std::ops::Deref for Any {
	type Target = Token;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[macro_export]
macro_rules! Token {
	[:] => { $crate::token::Colon };
	[;] => { $crate::token::Semicolon };
	[,] => { $crate::token::Comma };

	[$ident:ident] => { $crate::token::$ident }
}

#[macro_export]
macro_rules! Delim {
	[:] => { $crate::token::Colon };
	[;] => { $crate::token::Semicolon };
	[,] => { $crate::token::Comma };

	[&] => { $crate::token::DelimAnd };
	[@] => { $crate::token::DelimAt };
	[^] => { $crate::token::DelimCaret };
	[-] => { $crate::token::DelimDash };
	[$] => { $crate::token::DelimDollar };
	[.] => { $crate::token::DelimDot };
	[=] => { $crate::token::DelimEq };
	[>] => { $crate::token::DelimGt };
	[#] => { $crate::token::DelimHash };
	[<] => { $crate::token::DelimLt };
	[!] => { $crate::token::DelimNot };
	[|] => { $crate::token::DelimOr };
	[%] => { $crate::token::DelimPercent };
	[+] => { $crate::token::DelimPlus };
	[?] => { $crate::token::DelimQuestion };
	[/] => { $crate::token::DelimSlash };
	[*] => { $crate::token::DelimStar };
	[~] => { $crate::token::DelimTilde };
	[_] => { $crate::token::DelimUnderscore };
}
