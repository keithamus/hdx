use std::{
	fs::{read_to_string, write},
	path::{Path, PathBuf},
};

use glob::glob;
use hdx_parser::{Spanned, Stylesheet};
use serde_json::{from_str, to_string_pretty, Value};

use crate::{
	parser_suite::{ParserCase, ParserSuite},
	AppArgs,
};

const FIXTURES_GLOB: &str = "tasks/coverage/popular/*.css";
const SNAPSHOTS_PATH: &str = "tasks/coverage/snapshots/popular_parser/";

pub struct PopularParserTestSuite;

impl ParserSuite<PopularParserTestCase> for PopularParserTestSuite {
	fn new(_: &AppArgs) -> PopularParserTestSuite {
		PopularParserTestSuite {}
	}

	fn get_tests(&mut self, _: &AppArgs) -> Vec<PopularParserTestCase> {
		let mut files = vec![];
		for path in glob(FIXTURES_GLOB).unwrap().flatten() {
			files.push(PopularParserTestCase::new(path));
		}
		files
	}
}

pub struct PopularParserTestCase {
	name: String,
	source_path: PathBuf,
	json_path: PathBuf,
	warnings_path: PathBuf,
	source_text: String,
	desired: Value,
	warnings: String,
}

impl PopularParserTestCase {
	fn new(source_path: PathBuf) -> Self {
		let name = source_path.file_stem().unwrap().to_str().unwrap().to_owned();
		let json_path: PathBuf = (SNAPSHOTS_PATH.to_owned() + name.as_str() + ".json").into();
		let warnings_path: PathBuf = (SNAPSHOTS_PATH.to_owned() + name.as_str() + ".txt").into();
		let source_text = read_to_string(&source_path).unwrap();
		let desired: Value =
			from_str(read_to_string(json_path.clone()).unwrap_or("{}".to_owned()).as_str())
				.unwrap_or_else(|e| panic!("{} {}", json_path.display(), e));
		let warnings: String =
			read_to_string(warnings_path.clone()).unwrap_or("".to_owned()).as_str().to_owned();
		Self { name, source_path, json_path, warnings_path, source_text, desired, warnings }
	}
}

impl ParserCase for PopularParserTestCase {
	type AST = Value;

	fn name(&self) -> &str {
		&self.name
	}

	fn source_text(&self) -> &str {
		&self.source_text
	}

	fn path(&self) -> &Path {
		&self.source_path
	}

	fn desired(&self) -> &Value {
		&self.desired
	}

	// Comes with fixtures, no need to update
	fn update_desired(&self, parsed: &Value) {
		let str = to_string_pretty(parsed).unwrap();
		write(self.json_path.clone(), str).unwrap();
	}

	fn desired_warnings(&self) -> String {
		self.warnings.to_owned()
	}

	fn update_warnings(&self, warnings: String) {
		write(self.warnings_path.clone(), warnings).unwrap();
	}

	fn convert_ast(&self, ast: &Spanned<Stylesheet>) -> Value {
		from_str::<Value>(&to_string_pretty(ast).unwrap()).unwrap()
	}
}
