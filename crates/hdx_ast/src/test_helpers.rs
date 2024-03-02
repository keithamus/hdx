use hdx_parser::{Features, Parse, Parser};
use hdx_writer::{BaseCssWriter, OutputOption, WriteCss};
use oxc_allocator::Allocator;

#[cfg(test)]
pub fn test_write_with_options<'a, T: Parse<'a> + WriteCss<'a>>(allocator: &'a Allocator, source_text: &'a str, expected: &'a str, opts: OutputOption) {
	let mut string = String::new();
	let mut writer = BaseCssWriter::new(&mut string, opts);
	let parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely_with::<T>();
	if !result.errors.is_empty() {
		panic!("Failed to parse ({:?}) saw error {:?}", source_text, result.errors[0]);
	}
	result.output.unwrap().write_css(&mut writer).unwrap();
	assert_eq!(expected, string);
}


#[cfg(test)]
pub fn test_write<'a, T: Parse<'a> + WriteCss<'a>>(allocator: &'a Allocator, source_text: &'a str, expected: &'a str) {
	test_write_with_options::<T>(allocator, source_text, expected, OutputOption::all());
}

#[cfg(test)]
pub fn test_write_min<'a, T: Parse<'a> + WriteCss<'a>>(allocator: &'a Allocator, source_text: &'a str, expected: &'a str) {
	test_write_with_options::<T>(allocator, source_text, expected, OutputOption::none());
}

#[cfg(test)]
pub fn test_error<'a, T: Parse<'a> + WriteCss<'a>>(allocator: &'a Allocator, source_text: &'a str) {
	let parser = Parser::new(allocator, source_text, Features::default());
	let result = parser.parse_entirely_with::<T>();
	if result.errors.is_empty() {
		panic!("Expected errors but ({:?}) parsed without error.", source_text);
	}
	assert!(!result.errors.is_empty());
}
