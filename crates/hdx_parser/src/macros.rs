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
