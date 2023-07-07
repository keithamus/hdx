use std::{
    fs::{read_to_string, write},
    path::{Path, PathBuf},
};

use glob::glob;
use hdx_lexer::Token;
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
        let json_path: PathBuf = (SNAPSHOTS_PATH.to_owned() + name.as_str() + ".json").into();
        let source_text = read_to_string(&source_path).unwrap();
        let desired: Vec<Value> =
            from_str(read_to_string(json_path.clone()).unwrap_or("[]".to_owned()).as_str())
                .expect(format!("maltformed {}", json_path.display()).as_str());
        Self { name, source_path, json_path, source_text, desired }
    }
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
        write(self.json_path.clone(), str).unwrap();
    }

    fn convert_token(&self, token: &Token) -> Value {
        from_str::<Value>(&to_string_pretty(token).unwrap()).unwrap()
    }
}
