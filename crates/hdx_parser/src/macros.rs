/// Returns an Unimplemented() Error given the parser context.
///
/// # Examples
///
/// ```
/// todo!(parser);
/// ```
#[macro_export]
macro_rules! todo {
	($p: ident) => {
		Err($crate::diagnostics::Unimplemented(::hdx_lexer::Span::new($p.offset(), $p.offset())))?
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
	($p: ident, Include::$inc:ident, $tok:ident) => {
		if let Some(token) = $p.peek_with::<$crate::T![$tok]>(Include::$inc) {
			$p.hop(token);
			true
		} else {
			false
		}
	};
	($p: ident, $tok:ident) => {
		if let Some(token) = $p.peek::<$crate::T![$tok]>() {
			$p.hop(token);
			true
		} else {
			false
		}
	};
}
