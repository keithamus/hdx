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
			fn peek(p: &$crate::Parser<'a>) -> Option<::hdx_lexer::Token> {
				let token = p.peek_next();
				if token.kind() == ::hdx_lexer::Kind::$ident {
					Some(token)
				} else {
					None
				}
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = p.next();
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
			fn peek(p: &$crate::Parser<'a>) -> Option<::hdx_lexer::Token> {
				p.peek::<$crate::T![Delim]>().filter(|token| matches!(token.char(), Some($ch)))
			}
		}

		impl<'a> $crate::Parse<'a> for $iden {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *p.parse::<$crate::T![Delim]>()?;
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
			fn peek(p: &$crate::Parser<'a>) -> Option<hdx_lexer::Token> {
				p.peek::<$crate::T![Dimension]>().filter(|token| {
					matches!(token.dimension_unit(), hdx_lexer::DimensionUnit::$ident)
						|| p.parse_atom_lower(*token) == ::hdx_atom::atom!($atom)
				})
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *p.parse::<$crate::T![Dimension]>()?;
				let atom = p.parse_atom_lower(token);
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
			fn peek(p: &$crate::Parser<'a>) -> Option<hdx_lexer::Token> {
				p.peek::<$crate::T![Ident]>().filter(|token| p.parse_atom_lower(*token) == ::hdx_atom::atom!($atom))
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *p.parse::<$crate::T![Ident]>()?;
				let atom = p.parse_atom_lower(token);
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
			fn peek(p: &$crate::Parser<'a>) -> Option<hdx_lexer::Token> {
				p.peek::<$crate::T![Function]>().filter(|token| p.parse_atom_lower(*token) == ::hdx_atom::atom!($atom))
			}
		}

		impl<'a> $crate::Parse<'a> for $ident {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let token = *p.parse::<$crate::T![Function]>()?;
				let atom = p.parse_atom_lower(token);
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
	fn peek(p: &crate::Parser<'a>) -> Option<Token> {
		Some(p.peek_next())
	}
}

impl<'a> Parse<'a> for Any {
	fn parse(p: &mut crate::Parser<'a>) -> Result<Self> {
		Ok(Self(p.next()))
	}
}

pub struct PairWise(Token);
delegate_to_token!(PairWise);

impl<'a> Peek<'a> for PairWise {
	fn peek(p: &crate::Parser<'a>) -> Option<Token> {
		let token = p.peek_next();
		if token.to_pairwise().is_some() {
			return Some(token);
		}
		None
	}
}

impl<'a> Parse<'a> for PairWise {
	fn parse(p: &mut crate::Parser<'a>) -> Result<Self> {
		let token = p.next();
		if token.to_pairwise().is_none() {
			Err(diagnostics::Unexpected(token, token.span()))?;
		}
		Ok(Self(token))
	}
}

#[macro_export]
macro_rules! T {
	[:] => { $crate::token::Colon };
	[;] => { $crate::token::Semicolon };
	[,] => { $crate::token::Comma };
	[' '] => { $crate::token::Whitespace };

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

	[Dimension::Cap] => { $crate::token::dimension::Cap };
	[Dimension::Ch] => { $crate::token::dimension::Ch };
	[Dimension::Cm] => { $crate::token::dimension::Cm };
	[Dimension::Cqb] => { $crate::token::dimension::Cqb };
	[Dimension::Cqh] => { $crate::token::dimension::Cqh };
	[Dimension::Cqi] => { $crate::token::dimension::Cqi };
	[Dimension::Cqmax] => { $crate::token::dimension::Cqmax };
	[Dimension::Cqmin] => { $crate::token::dimension::Cqmin };
	[Dimension::Cqw] => { $crate::token::dimension::Cqw };
	[Dimension::Deg] => { $crate::token::dimension::Deg };
	[Dimension::Dpcm] => { $crate::token::dimension::Dpcm };
	[Dimension::Dpi] => { $crate::token::dimension::Dpi };
	[Dimension::Dppx] => { $crate::token::dimension::Dppx };
	[Dimension::Dvh] => { $crate::token::dimension::Dvh };
	[Dimension::Dvw] => { $crate::token::dimension::Dvw };
	[Dimension::Em] => { $crate::token::dimension::Em };
	[Dimension::Ex] => { $crate::token::dimension::Ex };
	[Dimension::Fr] => { $crate::token::dimension::Fr };
	[Dimension::Grad] => { $crate::token::dimension::Grad };
	[Dimension::Hz] => { $crate::token::dimension::Hz };
	[Dimension::Ic] => { $crate::token::dimension::Ic };
	[Dimension::In] => { $crate::token::dimension::In };
	[Dimension::KHz] => { $crate::token::dimension::KHz };
	[Dimension::Lh] => { $crate::token::dimension::Lh };
	[Dimension::Lvh] => { $crate::token::dimension::Lvh };
	[Dimension::Lvw] => { $crate::token::dimension::Lvw };
	[Dimension::Mm] => { $crate::token::dimension::Mm };
	[Dimension::Ms] => { $crate::token::dimension::Ms };
	[Dimension::%] => { $crate::token::dimension::Percent };
	[Dimension::Percent] => { $crate::token::dimension::Percent };
	[Dimension::Pc] => { $crate::token::dimension::Pc };
	[Dimension::Pt] => { $crate::token::dimension::Pt };
	[Dimension::Px] => { $crate::token::dimension::Px };
	[Dimension::Q] => { $crate::token::dimension::Q };
	[Dimension::Rad] => { $crate::token::dimension::Rad };
	[Dimension::Rcap] => { $crate::token::dimension::Rcap };
	[Dimension::Rch] => { $crate::token::dimension::Rch };
	[Dimension::Rem] => { $crate::token::dimension::Rem };
	[Dimension::Rex] => { $crate::token::dimension::Rex };
	[Dimension::Ric] => { $crate::token::dimension::Ric };
	[Dimension::Rlh] => { $crate::token::dimension::Rlh };
	[Dimension::S] => { $crate::token::dimension::S };
	[Dimension::Svh] => { $crate::token::dimension::Svh };
	[Dimension::Svw] => { $crate::token::dimension::Svw };
	[Dimension::Turn] => { $crate::token::dimension::Turn };
	[Dimension::Vb] => { $crate::token::dimension::Vb };
	[Dimension::Vh] => { $crate::token::dimension::Vh };
	[Dimension::Vi] => { $crate::token::dimension::Vi };
	[Dimension::Vmax] => { $crate::token::dimension::Vmax };
	[Dimension::Vmin] => { $crate::token::dimension::Vmin };
	[Dimension::Vw] => { $crate::token::dimension::Vw };
	[Dimension::X] => { $crate::token::dimension::X };

	[$ident:ident] => { $crate::token::$ident }
}
