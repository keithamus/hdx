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
/// assert!(discard!(parser, Kind::Comma));
///
/// assert!(discard!(parser, Kind::Ident if atom.to_ascii_lowercase() == atom!("foo")));
/// ```
#[macro_export]
macro_rules! discard {
	($parser: ident, Include::$inc:ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
		match $parser.peek_with(Include::$inc).kind() {
			$pattern $(if $guard)? => {
				$parser.next_with(Include::$inc);
				true
			},
			_ => false,
		}
	};
	($parser: ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
		match $parser.peek().kind() {
			$pattern $(if $guard)? => {
				$parser.next();
				true
			},
			_ => false,
		}
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
///     t if t.kind() == Kind::Ident => {...}
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
///     t if t.kind() == Kind::Ident => match parser.parse_atom_lower(t) => {
///       atom!("foo") => { ... }
///       ident => unexpected_ident!(parser, ident),
///     }
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
///     t if t.kind() == Kind::Ident => match parser.parse_atom_lower(t) => {
///       atom!("foo") => { ... }
///       ident => unexpected_function!(parser, ident),
///     }
/// }
/// ```
#[macro_export]
macro_rules! unexpected_function {
	($parser: ident, $($atom: tt)+) => {
		Err($crate::diagnostics::UnexpectedFunction($($atom)+.clone(), $parser.span()))?
	};
}

/// Returns whether the peeked token in the parser matches the given Kind (and
/// any additional guards).
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

/// Returns whether the peeked token matches a given Ident. Token should be
/// limited to Token::Ident, Token::Function, or Token::AtKeyword. The patterns
/// will match against the atom, lowercased. If either the Token or the (lower)
/// Ident do not match then `false` will be returned. If both match, `true` will
/// be returned.
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression.
///
/// # Examples
///
/// ```
/// assert!(peek_ignore_case!(parser, Kind::Ident, atom!("foo")));
/// ```
#[macro_export]
macro_rules! peek_ignore_case {
    ($parser: ident, 2, Kind::$kind:ident, $atom:pat $(if $guard:expr)? $(,)?) => {
        match $parser.peek_n(2) {
            t if t.kind() == hdx_lexer::Kind::$kind && matches!($parser.parse_atom_lower(t), $atom) $(&& $guard)? => true,
            _ => false
        }
    };
    ($parser: ident, Kind::$kind:ident, $atom:pat $(if $guard:expr)? $(,)?) => {
        match $parser.peek() {
            t if t.kind() == hdx_lexer::Kind::$kind && matches!($parser.parse_atom_lower(t), $atom) $(&& $guard)? => true,
            _ => false
        }
    };
}

/// Returns whether the peeked token in the parser matches the given kind, with
/// the given delimiter char() (and any additional guards).
///
/// Like in a `match` expression, the pattern can be optionally followed by `if`
/// and a guard expression.
///
/// # Examples
///
/// ```
/// assert!(peek_delim!(parser, '!'));
/// ```
#[macro_export]
macro_rules! peek_delim {
    ($parser: ident, 2, $char:literal $(if $guard:expr)? $(,)?) => {
        match $parser.peek_n(2) {
            t if t.kind() == hdx_lexer::Kind::Delim && matches!(t.char(), Some($char)) $(&& $guard)? => true,
            _ => false
        }
    };
    ($parser: ident, $char:literal $(if $guard:expr)? $(,)?) => {
        match $parser.peek() {
            t if t.kind() == hdx_lexer::Kind::Delim && matches!(t.char(), Some($char)) $(&& $guard)? => true,
            _ => false
        }
    };
}

/// Matches a token (from a Token returning parser method) against the given patterns. If no
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
			match $parser.$method($($args)*) {
				t if matches!(t.kind(), $pattern) $(&& $guard)? => t,
				t => $crate::unexpected!($parser, t),
			}
		}
	};
}

/// Matches a token against the given atoms. Token should be limited to
/// Token::Ident, Token::Function, or Token::AtKeyword. The patterns will match
/// against the atom, lowercased. If no patterns match then an UnexpectedIdent
/// error will be returned given the parser context and the author-cased atom.
/// If the Token is not the prescribed token, then an Unexpected error will be
/// returned.
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
/// expect_ignore_case!(parser.next(), Token::Ident, atom!("foo"));
///
/// let thing = expect_ignore_case!{ parser.peek(), Kind::Function:
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
		if !matches!($parser.$method($($args)*).char(), Some($val) $(if $guard)?) {
			$crate::unexpected!($parser),
		}
	};
}

/// Matches a token's atom against the given atom. Token should be limited to
/// Token::Ident, Token::Function, or Token::AtKeyword. The patterns will match
/// against the atom, lowercased. If no patterns match, or the token does not
/// match, then `false` will be returned. If the Token kind and (lowercased)
/// atom match then `true` will be returned.
///
/// Like in a `match` expression, the patterns can be optionally followed by
/// `if` and a guard expression that has access to names bound by the pattern.
///
/// By using a generic Token match statement plus a colon, multiple patterns can
/// be given to execute different march arms based on the resulting atom.
///
/// # Examples
///
/// ```
/// if match_ignore_case!(parser.next(), Kind::Ident, atom!("foo")) {
///     // ...
/// }
///
/// let thing = match_ignore_case!{ parser.peek(), Kind::Function:
///     atom!("foo") => true
///     atom!("bar") => {
///         parser.next();
///         false
///     },
/// }
/// ```
#[macro_export]
macro_rules! match_ignore_case {
	($parser: ident.$method: ident($($args: tt)*), Kind::$tokenty: ident, $ident: ident) => {
		$crate::match_ignore_case!{ $parser.$method($($args)*), Kind::$tokenty:
			atom if atom == $ident => true
		}
	};
	($parser: ident.$method: ident($($args: tt)*), Kind::$tokenty: ident, $atom: pat) => {
		$crate::match_ignore_case!{ $parser.$method($($args)*), Kind::$tokenty:
			$atom => true
		}
	};
	( $parser: ident.$method: ident($($args: tt)*), Kind::$tokenty: ident:
			$(
					$pattern:pat $(if $guard:expr)? => $then: expr
			),+
			$(,)?
	) => {
		match $parser.$method($($args)*) {
			t if t.kind() == hdx_lexer::Kind::$tokenty => match $parser.parse_atom_lower(t) {
				$($pattern $( if $guard )? => $then,)+
				_ => false,
			},
			_ => false,
		}
	};
}

/// Matches a token's char() value agains the given chars. Token is also
/// matched to ensure it is only Kind::Delim. The patterns will match against
/// the char(). If no patterns match, or the token does not match Kind::Delim,
/// then `false` will be returned.
///
/// Like in a `match` expression, the patterns can be optionally followed by
/// `if` and a guard expression that has access to names bound by the pattern.
///
/// By using a generic Token match statement plus a colon, multiple patterns
/// can be given to execute different march arms based on the resulting atom.
///
/// # Examples
///
/// ```
/// if match_delim!(parser.next(), Token::Ident(atom!("foo")) {
///     // ...
/// }
///
/// let thing = match_delim!{ parser.peek(), Token::Function(_):
///     atom!("foo") => true
///     atom!("bar") => {
///         parser.next();
///         false
///     },
/// }
/// ```
#[macro_export]
macro_rules! match_delim {
	($parser: ident.$method: ident($($args: tt)*), $char: literal) => {
		$crate::match_delim!{ $parser.$method($($args)*):
			 $char => true
		}
	};
	( $parser: ident.$method: ident($($args: tt)*):
			$(
				$char:literal $(if $guard:expr)?  => $then: expr,
			)+
			None => $else: expr$(,)?
	) => {
		match $parser.$method($($args)*) {
			t if t.kind() == hdx_lexer::Kind::Delim => match t.char().unwrap() {
		    $($char $( if $guard )? => $then,)+
				None => $else,
			},
			_ => $else,
		}
	};
}
