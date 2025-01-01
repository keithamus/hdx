/// (Requires feature "testing") Given a Node, and a string, this will expand to code that sets up a parser, and parses the given string against the
/// given node. If the parse failed this macro will [panic] with a readable failure. It then writes the result out using
/// [CursorFmtSink], writing the parsed Node back out to a string. If resulting string from the given string, then the
/// macro will [panic] with a readable failure.
///
/// In rare cases it might be necessary to ensure the resulting string _differs_ from the input, for example if a
/// grammar is serialized in a specific order but allows parsing in any order (many style values do this). In these
/// cases, a second string can be provided which will be asserted gainst the output instead.
///
/// ```
/// use css_parse::*;
/// assert_parse!(T![Ident], "foo");
/// ```
#[macro_export]
macro_rules! assert_parse {
	($ty: ty, $str: literal, $str2: literal) => {
		let source_text = $str;
		let expected = $str2;
		let bump = ::bumpalo::Bump::default();
		let mut parser = $crate::Parser::new(&bump, &source_text);
		let result = parser.parse_entirely::<$ty>();
		if !result.errors.is_empty() {
			panic!("\n\nParse on {}:{} failed. ({:?}) saw error {:?}", file!(), line!(), source_text, result.errors[0]);
		}
		let mut actual = ::bumpalo::collections::String::new_in(&bump);
		let mut cursors = $crate::CursorFmtSink::new(&source_text, &mut actual);
		{
			use $crate::ToCursors;
			result.to_cursors(&mut cursors);
		}
		if expected != actual {
			panic!("\n\nParse on {}:{} failed: did not match expected format:\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: {:?}\n", file!(), line!(), source_text, actual, expected);
		}
	};
	($ty: ty, $str: literal) => {
		assert_parse!($ty, $str, $str);
	};
}
#[cfg(test)]
pub(crate) use assert_parse;

/// (Requires feature "testing") Given a Node, and a string, this will expand to code that sets up a parser, and parses the given string against the
/// given node. If the parse succeeded this macro will [panic] with a readable failure.
///
/// In rare cases it might be necessary to ensure the resulting string _differs_ from the input, for example if a
/// grammar is serialized in a specific order but allows parsing in any order (many style values do this). In these
/// cases, a second string can be provided which will be asserted gainst the output instead.
///
/// ```
/// use css_parse::*;
/// assert_parse_error!(T![Ident], "1");
/// ```
#[macro_export]
macro_rules! assert_parse_error {
	($ty: ty, $str: literal) => {
		let source_text = $str;
		let bump = ::bumpalo::Bump::default();
		let mut parser = $crate::Parser::new(&bump, source_text);
		let result = parser.parse_entirely::<$ty>();
		if result.errors.is_empty() {
			let mut actual = ::bumpalo::collections::String::new_in(&bump);
			let mut cursors = $crate::CursorFmtSink::new(&source_text, &mut actual);
			use $crate::ToCursors;
			result.to_cursors(&mut cursors);
			panic!("\n\nParse on {}:{} passed. Expected errors but it passed without error.\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: (Error)", file!(), line!(), source_text, actual);
		}
		assert!(!result.errors.is_empty());
	};
}
#[cfg(test)]
pub(crate) use assert_parse_error;
