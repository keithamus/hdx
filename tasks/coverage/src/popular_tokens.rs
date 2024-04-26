use std::{
	fs::{read_to_string, write, File},
	io::{Read, Write},
	path::{Path, PathBuf},
};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use glob::glob;
use hdx_lexer::Token;
use serde::Serialize;
use serde_json::{from_str, to_string_pretty, Value};

use crate::{
	lexer_suite::{LexerCase, LexerSuite},
	AppArgs,
};

const FIXTURES_GLOB: &str = "tasks/coverage/popular/*.css";
const SNAPSHOTS_PATH: &str = "tasks/coverage/snapshots/popular_tokens/";

pub struct PopularTokensTestSuite;

impl LexerSuite<PopularTokensTestCase> for PopularTokensTestSuite {
	fn new(_: &AppArgs) -> PopularTokensTestSuite {
		PopularTokensTestSuite {}
	}

	fn get_tests(&mut self, _: &AppArgs) -> Vec<PopularTokensTestCase> {
		let mut files = vec![];
		for path in glob(FIXTURES_GLOB).unwrap().flatten() {
			files.push(PopularTokensTestCase::new(path));
		}
		files
	}
}

pub struct PopularTokensTestCase {
	name: String,
	source_path: PathBuf,
	json_path: PathBuf,
	source_text: String,
	desired: Vec<Value>,
}

impl PopularTokensTestCase {
	fn new(source_path: PathBuf) -> Self {
		let name = source_path.file_stem().unwrap().to_str().unwrap().to_owned();
		let json_path: PathBuf = (SNAPSHOTS_PATH.to_owned() + name.as_str() + ".json.gz").into();
		let source_text = read_to_string(&source_path).unwrap();
		let mut s = String::new();
		let mut str = "";
		if let Ok(f) = File::open(json_path.clone()) {
			let mut d = GzDecoder::new(f);
			if d.read_to_string(&mut s).is_ok() {
				str = s.as_str();
			}
		}

		let desired: Vec<Value> = from_str(str).unwrap_or_else(|_| panic!("malformed {}", json_path.display()));
		Self { name, source_path, json_path, source_text, desired }
	}
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
struct TokenWithSpan {
	start: usize,
	end: usize,
	escaped: bool,
	#[serde(flatten)]
	token: Token,
}

impl LexerCase for PopularTokensTestCase {
	type Token = Value;

	fn name(&self) -> &str {
		&self.name
	}

	fn source_text(&self) -> &str {
		&self.source_text
	}

	fn path(&self) -> &Path {
		&self.source_path
	}

	fn desired(&self) -> &Vec<Value> {
		&self.desired
	}

	// Comes with fixtures, no need to update
	fn update_desired(&self, tokens: &Vec<Value>) {
		let str = to_string_pretty(tokens).unwrap();
		let mut e = GzEncoder::new(Vec::new(), Compression::default());
		e.write_all(str.as_bytes()).unwrap();
		write(self.json_path.clone(), e.finish().unwrap()).unwrap();
	}

	fn convert_token(&self, start: usize, end: usize, token: &Token) -> Value {
		let escaped = match token {
			Token::String(a, _) => end - start > a.len() + 2,
			Token::Url(a, _) => end - start > a.len(),
			Token::Function(a) => end - start > a.len() + 1,
			Token::AtKeyword(a) => end - start > a.len() + 1,
			Token::Hash(a) => end - start > a.len() + 1,
			Token::HashId(a) => end - start > a.len() + 1,
			_ => false,
		};
		let converted_token = TokenWithSpan { escaped, start, end, token: token.clone() };
		from_str::<Value>(&to_string_pretty(&converted_token).unwrap()).unwrap()
	}
}
