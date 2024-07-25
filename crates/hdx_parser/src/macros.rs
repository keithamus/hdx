/// Returns an Unimplemented() Error given the parser context.
///
/// # Examples
///
/// ```
/// todo!(parser);
/// ```
#[macro_export]
macro_rules! todo {
	($parser: ident) => {
		Err($crate::diagnostics::Unimplemented($parser.span()))?
	};
}

/// Returns an Unexpected() Error given the parser context, and an optional token, which defaults
/// to the current token.
///
/// # Examples
///
/// ```
/// unexpected!(parser);
///
/// match parser.next() {
///     Token::Ident(_) => {...}
///     token => unexpected!(parser, token),
/// }
/// ```
#[macro_export]
macro_rules! unexpected {
	($parser: ident, $($token: tt)+) => {
		Err($crate::diagnostics::Unexpected($($token)+.clone(), $parser.span()))?
	};
	($parser: ident) => {
		Err($crate::diagnostics::Unexpected($parser.cur().clone(), $parser.span()))?
	};
}

/// Returns an UnexpectedIdent() Error given the parser context, and the given ident.
///
/// # Examples
///
/// ```
/// unexpected_ident!(parser, ident);
///
/// match parser.next() {
///     Token::Ident(ident) => unexpected_ident!(parser, ident),
/// }
/// ```
#[macro_export]
macro_rules! unexpected_ident {
	($parser: ident, $($atom: tt)+) => {
		Err($crate::diagnostics::UnexpectedIdent($($atom)+.clone(), $parser.span()))?
	};
}

/// Returns an UnexpectedFunction() Error given the parser context, and the given ident.
/// to the current token.
///
/// # Examples
///
/// ```no_run
/// unexpected_function!(parser, ident);
///
/// match parser.next() {
///     Token::Function(ident) => unexpected_function!(parser, ident),
/// }
/// ```
#[macro_export]
macro_rules! unexpected_function {
	($parser: ident, $($atom: tt)+) => {
		Err($crate::diagnostics::UnexpectedFunction($($atom)+.clone(), $parser.span()))?
	};
}

/// Returns whether the peeked token in the parser matches any of the given patterns.
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression.
///
/// # Examples
///
/// ```
/// assert!(peek!(parser, Kind::Comma));
/// ```
#[macro_export]
macro_rules! peek {
    ($parser: ident, 2, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $parser.peek_n(2).kind() {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
    ($parser: ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $parser.peek().kind() {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}

/// Advances the parser if the peeked token in the parser matches any of the given patterns,
/// effectively discarding the next token if matched. If the parser advanced, the return will be
/// true.
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern.
///
/// # Examples
///
/// ```
/// assert!(discard!(parser, Token::Comma));
///
/// assert!(discard!(parser, Token::Ident(atom) if atom.to_ascii_lowercase() == atom!("foo")));
/// ```
#[macro_export]
macro_rules! discard {
	($parser: ident, Include::$inc:ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
		match $parser.peek_with(Include::$inc).kind() {
			$pattern $(if $guard)? => {
				$parser.advance_with(Include::$inc);
				true
			},
			_ => false,
		}
	};
	($parser: ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
		match $parser.peek().kind() {
			$pattern $(if $guard)? => {
				$parser.advance();
				true
			},
			_ => false,
		}
	};
}

/// Matches a Token (from a Token returning parser method) against the given patterns. If no
/// patterns match then an Unexpected error will occur given the parser context and the token
/// being matched against.
///
/// Like in a `match` expression, the patterns can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern.
///
/// # Examples
///
/// ```
/// expect!(parser.next(), Kind::RightCurly);
///
/// expect!(parser.next_with(Include::Whitespace), Kind::Colon);
///
/// expect!(parser.next(), Kind::Comma | Kind::SemiColon);
/// ```
#[macro_export]
macro_rules! expect {
	($parser: ident.$method: ident($($args:tt)*), $pattern:pat $(if $guard:expr)? $(,)?) => {
		{
			let token = $parser.$method($($args)*);
			match token.kind() {
				$pattern $(if $guard)? => token,
				_ => $crate::unexpected!($parser, token),
			}
		}
	};
}

/// Matches a Token with a bound atom (from a Token returning parser method) against the given
/// patterns. Token should be limited to Token::Ident, Token::Function, or Token::AtKeyword.
/// The patterns will match against the atom, lowercased. If no patterns match then an
/// UnexpectedIdent error will be returned given the parser context and the author-cased atom.
/// If the Token is not the prescribed token, then an Unexpected error will be returned.
///
/// Like in a `match` expression, the patterns can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern.
///
/// By using a generic Token match statement plus a colon, multiple patterns can be given to
/// execute different march arms based on the resulting atom.
///
/// # Examples
///
/// ```
/// expect_ignore_case!(parser.next(), Token::Ident(atom!("foo"));
///
/// let thing = expect_ignore_case!{ parser.peek(), Token::Function(_):
///     atom!("foo") => Thing::Foo,
///     atom!("bar") => Thing::Bar,
/// }
/// ```
#[macro_export]
macro_rules! expect_ignore_case {
	($parser: ident.$method: ident($($args: tt)*), Kind::$tokenty: ident, $ident: ident) => {
		$crate::expect_ignore_case!{ $parser.$method($($args)*), Kind::$tokenty:
			atom if atom == $ident => {}
		}
	};
	($parser: ident.$method: ident($($args: tt)*), Kind::$tokenty: ident, $pat: pat) => {
		$crate::expect_ignore_case!{ $parser.$method($($args)*), Kind::$tokenty:
			$pat => {}
		}
	};
    ( $parser: ident.$method: ident($($args: tt)*), Kind::$tokenty: ident:
        $(
           $pattern:pat $(if $guard:expr)?  => $then: expr
        ),+
        $(,)?
    ) => {
		match $parser.$method($($args)*) {
			token if token.kind() == hdx_lexer::Kind::$tokenty => match $parser.parse_atom_lower(token) {
				$($pattern $( if $guard )? => $then,)+
				_ => $crate::unexpected_ident!($parser, $parser.parse_atom(token)),
			},
			token => $crate::unexpected!($parser, token),
		}
	};
}

/// Matches a Token's delimeter, expecting it to be the given value.
///
/// # Examples
///
/// ```
/// expect_delim!(parser.next(), '/');
///
/// expect_delim!(parser.next_with(Include::Whitespace), ':');
/// ```
#[macro_export]
macro_rules! expect_delim {
	($parser: ident.$method: ident($($args:tt)*), $val:literal $(if $guard:expr)? $(,)?) => {
		matches!($parser.$method($($args)*).char(), Some($val) $(if $guard)?)
	};
}

/// Matches a Token with a bound atom (from a Token returning parser method) against the given
/// patterns. Token should be limited to Token::Ident, Token::Function, or Token::AtKeyword.
/// The patterns will match against the atom, lowercased. If no patterns match, or the token
/// does not match, then `false` will be returned.
///
/// Like in a `match` expression, the patterns can be optionally followed by `if`
/// and a guard expression that has access to names bound by the pattern.
///
/// By using a generic Token match statement plus a colon, multiple patterns can be given to
/// execute different march arms based on the resulting atom.
///
/// # Examples
///
/// ```
/// if match_ignore_case!(parser.next(), Token::Ident(atom!("foo")) {
///     // ...
/// }
///
/// let thing = match_ignore_case!{ parser.peek(), Token::Function(_):
///     atom!("foo") => true
///     atom!("bar") => {
///         parser.advance();
///         false
///     },
/// }
/// ```
#[macro_export]
macro_rules! match_ignore_case {
	($parser: ident.$method: ident($($args: tt)*), Token::$tokenty: ident($ident: ident)) => {
		$crate::match_ignore_case!{ $parser.$method($($args)*), Token::$tokenty(_):
			atom if atom == $ident => true
		}
	};
	($parser: ident.$method: ident($($args: tt)*), Token::$tokenty: ident($atom: pat)) => {
		$crate::match_ignore_case!{ $parser.$method($($args)*), Token::$tokenty(_):
			$atom => true
		}
	};
    ( $parser: ident.$method: ident($($args: tt)*), Token::$tokenty: ident(_):
        $(
           $pattern:pat $(if $guard:expr)?  => $then: expr
        ),+
        $(,)?
    ) => {
		match $parser.$method($($args)*) {
			hdx_lexer::Token::$tokenty(ident) => match ident.to_ascii_lowercase() {
				$($pattern $( if $guard )? => $then,)+
				_ => false,
			},
			_ => false,
		}
	};
}

#[macro_export]
macro_rules! match_token_kind_and_char {
	($token:expr, $kind:expr, $char:expr) => {
		$token.kind() == $kind && $token.char() == Some($char)
	};
}
