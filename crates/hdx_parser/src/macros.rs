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
