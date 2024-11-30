#[cfg(test)]
macro_rules! assert_transform {
	($transform: ident, $str: literal, $expected: literal) => {
		{
			use bumpalo::Bump;
			use hdx_ast::css::{StyleSheet, visit::VisitableMut};
			use hdx_parser::{Features, Parser};

			let allocator = Bump::default();
			let mut parser = Parser::new(&allocator, $str, Features::default());
			let result = parser.parse_entirely::<StyleSheet>();
			if !result.errors.is_empty() {
				panic!("\n\nParse on {}:{} failed. ({:?}) saw error {:?}", file!(), line!(), $str, result.errors[0]);
			}
			let mut node = result.output.unwrap();

			let mut eparser = Parser::new(&allocator, $expected, Features::default());
			let eresult = eparser.parse_entirely::<StyleSheet>();
			if !eresult.errors.is_empty() {
				panic!("\n\nParse expected on {}:{} failed. ({:?}) saw error {:?}", file!(), line!(), $expected, result.errors[0]);
			}
			let mut enode = eresult.output.unwrap();

			let mut string = String::new();
			node.(&mut writer).unwrap();

			let mut expected = String::new();
			let mut ewriter = BaseCssWriter::new(&mut expected, OutputOption::all_bits());
			enode.write_css(&mut ewriter).unwrap();

			let mut transformed_string = String::new();
			let mut transformed_writer = BaseCssWriter::new(&mut transformed_string, OutputOption::all_bits());
			let mut t = $transform::default();
			node.accept_mut(&mut t);
			node.write_css(&mut transformed_writer).unwrap();

			if $expected != expected {
				panic!("\n\nTransform on {}:{} failed: did not match expected format:\n\n```pre-transformed\n{}```\n```transformed\n{}```\n```expected\n{}\n```", file!(), line!(), string, transformed_string, expected);
			}

			let mut etransformed_string = String::new();
			let mut etransformed_writer = BaseCssWriter::new(&mut etransformed_string, OutputOption::all_bits());
			let mut t = $transform::default();
			enode.accept_mut(&mut t);
			enode.write_css(&mut etransformed_writer).unwrap();

			if expected != etransformed_string {
				panic!("\n\nTransform on {}:{} failed. First transform was corrected but re-running the transform caused a different output, which is not desired:\n\n        transformed: {:?}\n second-transform: {:?}", file!(), line!(), expected, etransformed_string);
			}
		}
	}
}

#[cfg(test)]
pub(crate) use assert_transform;
