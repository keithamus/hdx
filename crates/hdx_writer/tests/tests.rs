use hdx_parser::{Parser, ParserOptions};
use hdx_writer::{BaseCssWriter, WriteCss};
use oxc_allocator::Allocator;

#[test]
fn smoke_test() {
	let allocator = Allocator::default();
	let css = "
		body {
			padding-right: 1px;
		}
	";
	let result = Parser::new(&allocator, css, ParserOptions::default()).parse();
	let mut output = String::new();
	let mut writer = BaseCssWriter::new(&mut output, true);
	result.output.unwrap().write_css(&mut writer).unwrap();
	assert_eq!(output, "body{padding-right:1px}");
}

#[test]
fn smoke_test_expand() {
	let allocator = Allocator::default();
	let css = "body{padding-right:1px}";
	let result = Parser::new(&allocator, css, ParserOptions::default()).parse();
	let mut output = String::new();
	let mut writer = BaseCssWriter::new(&mut output, false);
	result.output.unwrap().write_css(&mut writer).unwrap();
	let expected = "body {\n\tpadding-right: 1px;\n}\n";
	assert_eq!(output, expected);
}
