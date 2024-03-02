use hdx_parser::{Features, Parse, Parser};
use hdx_writer::{BaseCssWriter, OutputOption, WriteCss};
use oxc_allocator::Allocator;

#[cfg(test)]
macro_rules! assert_size {
	($ty: ty, $i: literal) => {
		assert_eq!(::std::mem::size_of::<$ty>(), $i);
	}
}
pub(crate) use assert_size;

#[cfg(test)]
pub fn test_write_with_options<'a, T: Parse<'a> + WriteCss<'a>>(
	allocator: &'a Allocator,
	source_text: &'a str,
	expected: &'a str,
	opts: OutputOption,
) {
	let mut string = String::new();
	let mut writer = BaseCssWriter::new(&mut string, opts);
	let parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely_with::<T>();
	if !result.errors.is_empty() {
		panic!("\n\nFailed to parse ({:?}) saw error {:?}", source_text, result.errors[0]);
	}
	result.output.unwrap().write_css(&mut writer).unwrap();
	if expected != string {
		panic!("\n\nParsed output did not match expected format:\n\n  parser output:  {:?}\n       expected:  {:?}\n", string, expected);
	}
}

#[cfg(test)]
macro_rules! assert_parse {
	($ty: ty, $str: literal, $str2: literal) => {
		let allocator = oxc_allocator::Allocator::default();
		$crate::test_helpers::test_write_with_options::<$ty>(&allocator, $str, $str2, hdx_writer::OutputOption::all());
	};
	($ty: ty, $str: literal) => {
		let allocator = oxc_allocator::Allocator::default();
		$crate::test_helpers::test_write_with_options::<$ty>(&allocator, $str, $str, hdx_writer::OutputOption::all());
	}
}
#[cfg(test)]
pub(crate) use assert_parse;

#[cfg(test)]
macro_rules! assert_minify {
	($ty: ty, $str: literal, $str2: literal) => {
		let allocator = oxc_allocator::Allocator::default();
		$crate::test_helpers::test_write_with_options::<$ty>(&allocator, $str, $str2, hdx_writer::OutputOption::none());
	}
}
#[cfg(test)]
pub(crate) use assert_minify;

#[cfg(test)]
pub fn test_error<'a, T: Parse<'a> + WriteCss<'a>>(allocator: &'a Allocator, source_text: &'a str) {
	let parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely_with::<T>();
	if result.errors.is_empty() {
		panic!("Expected errors but ({:?}) parsed without error.", source_text);
	}
	assert!(!result.errors.is_empty());
}

#[cfg(test)]
macro_rules! assert_parse_error {
	($ty: ty, $str: literal) => {
		let allocator = oxc_allocator::Allocator::default();
		$crate::test_helpers::test_error::<$ty>(&allocator, $str);
	}
}
#[cfg(test)]
pub(crate) use assert_parse_error;

#[cfg(feature = "serde")]
pub fn test_serialize<'a, T: Parse<'a> + WriteCss<'a> + serde::Serialize>(
	allocator: &'a Allocator,
	source_text: &'a str,
	expected: serde_json::Value,
) {
	use serde_json::{from_str, to_string, Value};
	let parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely_with::<T>();
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



#[cfg(test)]
macro_rules! assert_json {
	($ty: ty, $str: literal == $($json:tt)+) => {
		let allocator = oxc_allocator::Allocator::default();
		$crate::test_helpers::test_serialize::<$ty>(&allocator, $str, ::serde_json::json!($($json)+));
	}
}
#[cfg(test)]
pub(crate) use assert_json;
    
