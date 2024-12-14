use bumpalo::Bump;
use hdx_parser::{Features, Parse, Parser, ToCursors};

#[cfg(test)]
macro_rules! assert_size {
	($ty: ty, $i: literal) => {
		assert_eq!(::std::mem::size_of::<$ty>(), $i);
	};
}
pub(crate) use assert_size;

#[cfg(test)]
pub fn test_write_with_options<'a, T: Parse<'a> + ToCursors>(
	allocator: &'a Bump,
	source_text: &'a str,
	expected: &'a str,
	file: &str,
	line: u32,
) {
	let mut parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely::<T>();
	if !result.errors.is_empty() {
		panic!("\n\nParse on {}:{} failed. ({:?}) saw error {:?}", file, line, source_text, result.errors[0]);
	}
	let mut actual = bumpalo::collections::String::new_in(allocator);
	let mut cursors = hdx_parser::CursorStream::new(&allocator);
	result.write(&mut cursors, &mut actual).unwrap();
	if expected != actual {
		panic!("\n\nParse on {}:{} failed: did not match expected format:\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: {:?}\n", file, line, source_text, actual, expected);
	}
}

#[cfg(test)]
macro_rules! assert_parse {
	($ty: ty, $str: literal, $str2: literal) => {
		let allocator = bumpalo::Bump::default();
		$crate::test_helpers::test_write_with_options::<$ty>(&allocator, $str, $str2, file!(), line!());
	};
	($ty: ty, $str: literal) => {
		let allocator = bumpalo::Bump::default();
		$crate::test_helpers::test_write_with_options::<$ty>(&allocator, $str, $str, file!(), line!());
	};
}
#[cfg(test)]
pub(crate) use assert_parse;

#[cfg(test)]
pub fn test_error<'a, T: Parse<'a> + ToCursors>(allocator: &'a Bump, source_text: &'a str, file: &str, line: u32) {
	let mut parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely::<T>();
	if result.errors.is_empty() {
		let mut actual = bumpalo::collections::String::new_in(allocator);
	let mut cursors = hdx_parser::CursorStream::new(&allocator);
		result.write(&mut cursors, &mut actual).unwrap();
		panic!("\n\nParse on {}:{} passed. Expected errors but it passed without error.\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: (Error)", file, line, source_text, actual);
	}
	assert!(!result.errors.is_empty());
}

#[cfg(test)]
macro_rules! assert_parse_error {
	($ty: ty, $str: literal) => {
		let allocator = bumpalo::Bump::default();
		$crate::test_helpers::test_error::<$ty>(&allocator, $str, file!(), line!());
	};
}
#[cfg(test)]
pub(crate) use assert_parse_error;

#[cfg(feature = "serde")]
pub fn test_serialize<'a, T: Parse<'a> + ToCursors + serde::Serialize>(
	allocator: &'a Bump,
	source_text: &'a str,
	expected: serde_json::Value,
) {
	use serde_json::{from_str, to_string, Value};
	let mut parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely::<T>();
	if let Some(res) = result.output {
		let actual = from_str::<Value>(&to_string(&res).unwrap()).unwrap();
		assert_eq!(expected, actual);
	} else {
		if !result.errors.is_empty() {
			panic!("Failed to parse ({:?}) saw error {:?}", source_text, result.errors[0]);
		}
		panic!("Failed to parse ({:?})", source_text);
	}
}
