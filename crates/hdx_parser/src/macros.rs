#[macro_export]
macro_rules! peek {
    ($parser: ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $parser.peek() {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}

#[macro_export]
macro_rules! unexpected {
	($parser: ident, $token: ident) => {
		Err($crate::diagnostics::Unexpected($token, $parser.span()))?
	};
	($parser: ident) => {
		Err($crate::diagnostics::Unexpected($parser.cur(), $parser.span()))?
	};
}

#[macro_export]
macro_rules! unexpected_ident {
	($parser: ident, $atom: ident) => {
		Err($crate::diagnostics::UnexpectedIdent($atom, $parser.span()))?
	};
}

#[macro_export]
macro_rules! expect {
	($parser: ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
		match $parser.cur() {
			$pattern $(if $guard)? => {},
			token => $crate::unexpected!($parser, token),
		}
	};
}

#[macro_export]
macro_rules! expect_ignore_case {
	($parser: ident, $tokenty: ident, $ident: ident) => {
		match $parser.cur() {
			Token::$tokenty(ident) => match ident.to_ascii_lowercase() {
				i if i == $ident => {}
				_ => $crate::unexpected_ident!($parser, ident),
			},
			token => $crate::unexpected!($parser, token),
		}
	};
	($parser: ident, $ident: ident) => {
		match $parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				i if i == $ident => {}
				_ => $crate::unexpected_ident!($parser, ident),
			},
			token => $crate::unexpected!($parser, token),
		}
	};
	($parser: ident, $tokenty: ident, $ident: pat) => {
		match $parser.cur() {
			Token::$tokenty(ident) => match ident.to_ascii_lowercase() {
				$ident => {}
				_ => $crate::unexpected_ident!($parser, ident),
			},
			token => $crate::unexpected!($parser, token),
		}
	};
	($parser: ident, $ident: pat) => {
		match $parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				$ident => {}
				_ => $crate::unexpected_ident!($parser, ident),
			},
			token => $crate::unexpected!($parser, token),
		}
	};
}

#[macro_export]
macro_rules! match_ident_ignore_case {
	($parser: ident, $ident: pat) => {
		match $parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				$ident => true,
				_ => false,
			},
			token => false,
		}
	};
}

#[macro_export]
macro_rules! discard {
	($parser: ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
		match $parser.cur() {
			$pattern $(if $guard)? => {
				$parser.advance();
			},
			_ => {},
		}
	};
}
