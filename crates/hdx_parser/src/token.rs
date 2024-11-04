use hdx_lexer::{Kind, Token};

use crate::{
	diagnostics,
	traits::{Parse, Peek},
	Result,
};

#[macro_export]
macro_rules! delegate_to_token {
	($ident:ident) => {
		impl std::ops::Deref for $ident {
			type Target = hdx_lexer::Token;
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
		pub struct $ident(::hdx_lexer::Token);
		delegate_to_token!($ident);

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(parser: &$crate::Parser<'a>) -> Option<::hdx_lexer::Token> {
				let token = parser.peek_next();
				if token.kind() == ::hdx_lexer::Kind::$ident {
					Some(token)
				} else {
					None
				}
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = parser.next();
				if token.kind() == ::hdx_lexer::Kind::$ident {
					Ok(Self(token))
				} else {
					let expected_token = ::hdx_lexer::Token::new(Kind::$ident, 0, token.offset(), token.len());
					Err($crate::diagnostics::ExpectedToken(expected_token, token, token.span()))?
				}
			}
		}
	};
}

#[macro_export]
macro_rules! custom_delim {
	($iden:ident, $ch:literal) => {
		pub struct $iden(::hdx_lexer::Token);
		delegate_to_token!($iden);

		impl<'a> $crate::Peek<'a> for $iden {
			fn peek(parser: &$crate::Parser<'a>) -> Option<::hdx_lexer::Token> {
				parser.peek::<$crate::Token![Delim]>().filter(|token| matches!(token.char(), Some($ch)))
			}
		}

		impl<'a> $crate::Parse<'a> for $iden {
			fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *parser.parse::<$crate::Token![Delim]>()?;
				if !matches!(token.char(), Some($ch)) {
					Err($crate::diagnostics::ExpectedDelim(token, token.span()))?;
				}
				Ok(Self(token))
			}
		}
	};
}

#[macro_export]
macro_rules! custom_dimension {
	($ident: ident, atom!($atom: tt)) => {
		pub struct $ident(::hdx_lexer::Token);
		$crate::delegate_to_token!($ident);

		impl $ident {
			#[allow(unused)]
			pub fn atom() -> ::hdx_atom::Atom {
				::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(parser: &$crate::Parser<'a>) -> Option<hdx_lexer::Token> {
				parser.peek::<$crate::Token![Dimension]>().filter(|token| {
					matches!(token.dimension_unit(), hdx_lexer::DimensionUnit::$ident)
						|| parser.parse_atom_lower(*token) == ::hdx_atom::atom!($atom)
				})
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *parser.parse::<$crate::Token![Dimension]>()?;
				let atom = parser.parse_atom_lower(token);
				if atom != ::hdx_atom::atom!($atom) {
					Err($crate::diagnostics::UnexpectedDimension(atom, token.span()))?
				}
				Ok(Self(token))
			}
		}
	};
}

#[macro_export]
macro_rules! custom_keyword {
	($ident: ident, atom!($atom: tt)) => {
		pub struct $ident(pub ::hdx_lexer::Token);
		$crate::delegate_to_token!($ident);

		impl $ident {
			#[allow(unused)]
			pub fn atom() -> ::hdx_atom::Atom {
				::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(parser: &$crate::Parser<'a>) -> Option<hdx_lexer::Token> {
				parser
					.peek::<$crate::Token![Ident]>()
					.filter(|token| parser.parse_atom_lower(*token) == ::hdx_atom::atom!($atom))
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *parser.parse::<$crate::Token![Ident]>()?;
				let atom = parser.parse_atom_lower(token);
				if atom != ::hdx_atom::atom!($atom) {
					Err($crate::diagnostics::UnexpectedIdent(atom, token.span()))?
				}
				Ok(Self(token))
			}
		}
	};
}

#[macro_export]
macro_rules! custom_function {
	($ident: ident, atom!($atom: tt)) => {
		pub struct $ident(::hdx_lexer::Token);
		$crate::delegate_to_token!($ident);

		impl $ident {
			pub fn atom() -> ::hdx_atom::Atom {
				::hdx_atom::atom!($atom)
			}
		}

		impl<'a> $crate::Peek<'a> for $ident {
			fn peek(parser: &$crate::Parser<'a>) -> Option<hdx_lexer::Token> {
				parser
					.peek::<$crate::Token![Function]>()
					.filter(|token| parser.parse_atom_lower(*token) == ::hdx_atom::atom!($atom))
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(parser: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *parser.parse::<$crate::Token![Function]>()?;
				let atom = parser.parse_atom_lower(token);
				if atom != ::hdx_atom::atom!($atom) {
					Err($crate::diagnostics::ExpectedFunctionOf(::hdx_atom::atom!($atom), atom, token.span()))?
				}
				Ok(Self(token))
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
	custom_dimension!(Dvh, atom!("dvh"));
	custom_dimension!(Dvw, atom!("dvw"));
	custom_dimension!(Em, atom!("em"));
	custom_dimension!(Ex, atom!("ex"));
	custom_dimension!(Fr, atom!("fr"));
	custom_dimension!(Grad, atom!("grad"));
	custom_dimension!(Hz, atom!("hz"));
	custom_dimension!(Ic, atom!("ic"));
	custom_dimension!(In, atom!("in"));
	custom_dimension!(KHz, atom!("khz"));
	custom_dimension!(Lh, atom!("lh"));
	custom_dimension!(Lvh, atom!("lvh"));
	custom_dimension!(Lvw, atom!("lvw"));
	custom_dimension!(Mm, atom!("mm"));
	custom_dimension!(Ms, atom!("ms"));
	custom_dimension!(Percent, atom!("%"));
	custom_dimension!(Pc, atom!("pc"));
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
	custom_dimension!(Svh, atom!("svh"));
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

pub struct Any(Token);
delegate_to_token!(Any);

impl<'a> Peek<'a> for Any {
	fn peek(parser: &crate::Parser<'a>) -> Option<Token> {
		Some(parser.peek_next())
	}
}

impl<'a> Parse<'a> for Any {
	fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
		Ok(Self(parser.next()))
	}
}

pub struct PairWise(Token);
delegate_to_token!(PairWise);

impl<'a> Peek<'a> for PairWise {
	fn peek(parser: &crate::Parser<'a>) -> Option<Token> {
		let token = parser.peek_next();
		if token.to_pairwise().is_some() {
			return Some(token);
		}
		None
	}
}

impl<'a> Parse<'a> for PairWise {
	fn parse(parser: &mut crate::Parser<'a>) -> Result<Self> {
		let token = parser.next();
		if token.to_pairwise().is_none() {
			Err(diagnostics::Unexpected(token, token.span()))?;
		}
		Ok(Self(token))
	}
}

#[macro_export]
macro_rules! Token {
	[:] => { $crate::token::Colon };
	[;] => { $crate::token::Semicolon };
	[,] => { $crate::token::Comma };
	[ ] => { $crate::token::Whitespace };

	[&] => { $crate::token::delim::And };
	[@] => { $crate::token::delim::At };
	[^] => { $crate::token::delim::Caret };
	[-] => { $crate::token::delim::Dash };
	[$] => { $crate::token::delim::Dollar };
	[.] => { $crate::token::delim::Dot };
	[=] => { $crate::token::delim::Eq };
	[>] => { $crate::token::delim::Gt };
	[#] => { $crate::token::delim::Hash };
	[<] => { $crate::token::delim::Lt };
	[!] => { $crate::token::delim::Not };
	[|] => { $crate::token::delim::Or };
	[%] => { $crate::token::delim::Percent };
	[+] => { $crate::token::delim::Plus };
	[?] => { $crate::token::delim::Question };
	[/] => { $crate::token::delim::Slash };
	[*] => { $crate::token::delim::Star };
	[~] => { $crate::token::delim::Tilde };
	[_] => { $crate::token::delim::Underscore };

	[$ident:ident] => { $crate::token::$ident }
}

#[macro_export]
macro_rules! Delim {
	[:] => { $crate::token::Colon };
	[;] => { $crate::token::Semicolon };
	[,] => { $crate::token::Comma };

	[&] => { $crate::token::delim::And };
	[@] => { $crate::token::delim::At };
	[^] => { $crate::token::delim::Caret };
	[-] => { $crate::token::delim::Dash };
	[$] => { $crate::token::delim::Dollar };
	[.] => { $crate::token::delim::Dot };
	[=] => { $crate::token::delim::Eq };
	[>] => { $crate::token::delim::Gt };
	[#] => { $crate::token::delim::Hash };
	[<] => { $crate::token::delim::Lt };
	[!] => { $crate::token::delim::Not };
	[|] => { $crate::token::delim::Or };
	[%] => { $crate::token::delim::Percent };
	[+] => { $crate::token::delim::Plus };
	[?] => { $crate::token::delim::Question };
	[/] => { $crate::token::delim::Slash };
	[*] => { $crate::token::delim::Star };
	[~] => { $crate::token::delim::Tilde };
	[_] => { $crate::token::delim::Underscore };
}

#[macro_export]
macro_rules! Dimension {
	[Cap] => { $crate::token::dimension::Cap };
	[Ch] => { $crate::token::dimension::Ch };
	[Cm] => { $crate::token::dimension::Cm };
	[Cqb] => { $crate::token::dimension::Cqb };
	[Cqh] => { $crate::token::dimension::Cqh };
	[Cqi] => { $crate::token::dimension::Cqi };
	[Cqmax] => { $crate::token::dimension::Cqmax };
	[Cqmin] => { $crate::token::dimension::Cqmin };
	[Cqw] => { $crate::token::dimension::Cqw };
	[Deg] => { $crate::token::dimension::Deg };
	[Dpcm] => { $crate::token::dimension::Dpcm };
	[Dpi] => { $crate::token::dimension::Dpi };
	[Dppx] => { $crate::token::dimension::Dppx };
	[Dvh] => { $crate::token::dimension::Dvh };
	[Dvw] => { $crate::token::dimension::Dvw };
	[Em] => { $crate::token::dimension::Em };
	[Ex] => { $crate::token::dimension::Ex };
	[Fr] => { $crate::token::dimension::Fr };
	[Grad] => { $crate::token::dimension::Grad };
	[Hz] => { $crate::token::dimension::Hz };
	[Ic] => { $crate::token::dimension::Ic };
	[In] => { $crate::token::dimension::In };
	[KHz] => { $crate::token::dimension::KHz };
	[Lh] => { $crate::token::dimension::Lh };
	[Lvh] => { $crate::token::dimension::Lvh };
	[Lvw] => { $crate::token::dimension::Lvw };
	[Mm] => { $crate::token::dimension::Mm };
	[Ms] => { $crate::token::dimension::Ms };
	[%] => { $crate::token::dimension::Percent };
	[Percent] => { $crate::token::dimension::Percent };
	[Pc] => { $crate::token::dimension::Pc };
	[Pt] => { $crate::token::dimension::Pt };
	[Px] => { $crate::token::dimension::Px };
	[Q] => { $crate::token::dimension::Q };
	[Rad] => { $crate::token::dimension::Rad };
	[Rcap] => { $crate::token::dimension::Rcap };
	[Rch] => { $crate::token::dimension::Rch };
	[Rem] => { $crate::token::dimension::Rem };
	[Rex] => { $crate::token::dimension::Rex };
	[Ric] => { $crate::token::dimension::Ric };
	[Rlh] => { $crate::token::dimension::Rlh };
	[S] => { $crate::token::dimension::S };
	[Svh] => { $crate::token::dimension::Svh };
	[Svw] => { $crate::token::dimension::Svw };
	[Turn] => { $crate::token::dimension::Turn };
	[Vb] => { $crate::token::dimension::Vb };
	[Vh] => { $crate::token::dimension::Vh };
	[Vi] => { $crate::token::dimension::Vi };
	[Vmax] => { $crate::token::dimension::Vmax };
	[Vmin] => { $crate::token::dimension::Vmin };
	[Vw] => { $crate::token::dimension::Vw };
	[X] => { $crate::token::dimension::X };
}
