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
		Err($crate::diagnostics::Unimplemented(::hdx_lexer::Span::new($parser.offset(), $parser.offset())))?
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
/// assert!(discard!(parser, Comma));
/// ```
#[macro_export]
macro_rules! discard {
	($parser: ident, Include::$inc:ident, $tok:ident) => {
		if let Some(token) = $parser.peek_with::<$crate::Token![$tok]>(Include::$inc) {
			$parser.hop(token);
			true
		} else {
			false
		}
	};
	($parser: ident, $tok:ident) => {
		if let Some(token) = $parser.peek::<$crate::Token![$tok]>() {
			$parser.hop(token);
			true
		} else {
			false
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
/// if let Some(token) = parser.parse::<Token![Ident]>()? {
///   ...
/// } else {
///   unexpected!(parser, token),
/// }
/// ```
#[macro_export]
macro_rules! unexpected {
	($parser: ident, $($token: tt)+) => {
		{
			dbg!(format!("triggering unexpected {}, on {}, {}", $($token)+, file!(), line!()));
			Err($crate::diagnostics::Unexpected($($token)+, ($($token)+).span()))?
		}
	};
	($parser: ident) => {
		{
			let token = $parser.peek::<$crate::token::Any>().unwrap();
			dbg!(format!("triggering unexpected {}, on {}, {}", token, file!(), line!()));
			Err($crate::diagnostics::Unexpected(token, token.span()))?
		}
	};
}

/// Returns an UnexpectedIdent() Error given the parser context, and the given ident.
///
/// # Examples
///
/// ```
/// unexpected_ident!(parser, token, ident);
///
/// if let Some(token) = parser.parse::<Token![Ident]>()? {
///   match parser.parse_atom_lower(token) => {
///     atom!("foo") => { ... }
///     ident => unexpected_ident!(parser, token, ident),
///   }
/// }
/// ```
#[macro_export]
macro_rules! unexpected_ident {
	($parser: ident, $token: ident, $($atom: tt)+) => {
		Err($crate::diagnostics::UnexpectedIdent($($atom)+.clone(), $token.span()))?
	};
	($parser: ident, $token: ident) => {
		Err($crate::diagnostics::UnexpectedIdent($parser.parse_atom($token), $token.span()))?
	};
}

/// Returns an UnexpectedDimension() Error given the parser context, and the given ident.
///
/// # Examples
///
/// ```
/// unexpected_ident!(parser, atom);
///
/// if let Some(token) = parser.parse::<Token![Dimension]>()? {
///   match parser.parse_atom_lower(token) => {
///     atom!("foo") => { ... }
///     ident => unexpected_dimension!(parser, ident),
///   }
/// }
/// ```
#[macro_export]
macro_rules! unexpected_dimension {
	($parser: ident, $token: ident, $($atom: tt)+) => {
		Err($crate::diagnostics::UnexpectedDimension($($atom)+.clone(), $token.span()))?
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
/// if let Some(token) = parser.parse::<Token![Function]>()? {
///   match parser.parse_atom_lower(token) => {
///       atom!("foo") => { ... }
///       ident => unexpected_function!(parser, ident),
///     }
/// }
/// ```
#[macro_export]
macro_rules! unexpected_function {
	($parser: ident, $token: ident, $($atom: tt)+) => {
		Err($crate::diagnostics::UnexpectedFunction($($atom)+.clone(), $token.span()))?
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
        match $parser.peek_next() {
            t if t.kind() == hdx_lexer::Kind::$kind && matches!($parser.parse_atom_lower(t), $atom) $(&& $guard)? => true,
            _ => false
        }
    };
}

/// Matches the next token as an Ident, expecting it to be the given atom.
/// If it matches, it will advance the parser forward.
///
/// # Examples
///
/// ```
/// expect_keyword!(parser, atom!("at"));
///
/// let atom = atom!("at")
/// expect_keyword!(parser, atom);
/// ```
#[macro_export]
macro_rules! expect_keyword {
	($parser: ident, $atom:ident) => {
		let token = *$parser.parse::<$crate::Token![Ident]>()?;
		let atom = $parser.parse_atom_lower(token);
		if atom != $atom {
			$crate::unexpected_ident!($parser, token, atom)
		}
	};
	($parser: ident, $($atom: tt)+) => {
		let token = *$parser.parse::<$crate::Token![Ident]>()?;
		let atom = $parser.parse_atom_lower(token);
		if atom != $($atom)+ {
			$crate::unexpected_ident!($parser, token, atom)
		}
	};
}
