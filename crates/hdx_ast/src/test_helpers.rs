use hdx_parser::{Features, Parse, Parser};
use hdx_writer::{BaseCssWriter, WriteCss};
use oxc_allocator::Allocator;

pub fn test_write<'a, T: Parse<'a> + WriteCss<'a>>(allocator: &'a Allocator, source_text: &'a str, expected: &'a str) {
	let mut string = String::new();
	let mut writer = BaseCssWriter::new(&mut string, true);
	let parser = Parser::new(&allocator, source_text, Features::default());
	let result = parser.parse_entirely_with::<T>();
	if !result.errors.is_empty() {
		panic!("{:?}", result.errors[0]);
	}
	result.output.unwrap().write_css(&mut writer).unwrap();
	assert_eq!(string, expected);
}
