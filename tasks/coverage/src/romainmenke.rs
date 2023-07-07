use std::{
	fs::read_to_string,
	path::{Path, PathBuf},
};

use glob::glob;
use hdx_lexer::{Kind, Token, TokenValue};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::{
	lexer_suite::{LexerCase, LexerSuite},
	AppArgs,
};

const FIXTURES_GLOB: &str = "tasks/coverage/css-tokenizer-tests/tests/**/source.css";

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
enum Structured {
	Dimension(DimensionStructured),
	Number(NumberStructured),
	String(StringStructured),
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NumberStructured {
	value: f32,
	#[serde(rename = "type")]
	kind: Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DimensionStructured {
	value: f32,
	#[serde(rename = "type")]
	kind: String,
	unit: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StringStructured {
	value: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RomainToken {
	#[serde(rename(deserialize = "type"))]
	kind: RomainKind,
	raw: String,
	start_index: u32,
	end_index: u32,
	structured: Option<Structured>,
}

#[derive(PartialEq, Serialize, Deserialize)]
enum RomainKind {
	#[serde(rename(deserialize = "comment"))]
	Comment,
	#[serde(rename(deserialize = "ident-token"))]
	Ident,
	#[serde(rename(deserialize = "function-token"))]
	Function,
	#[serde(rename(deserialize = "at-keyword-token"))]
	AtKeyword,
	#[serde(rename(deserialize = "hash-token"))]
	Hash,
	#[serde(rename(deserialize = "string-token"))]
	String,
	#[serde(rename(deserialize = "bad-string-token"))]
	BadString,
	#[serde(rename(deserialize = "url-token"))]
	Url,
	#[serde(rename(deserialize = "bad-url-token"))]
	BadUrl,
	#[serde(rename(deserialize = "delim-token"))]
	Delim,
	#[serde(rename(deserialize = "number-token"))]
	Number,
	#[serde(rename(deserialize = "percentage-token"))]
	Percentage,
	#[serde(rename(deserialize = "dimension-token"))]
	Dimension,
	#[serde(rename(deserialize = "whitespace-token"))]
	Whitespace,
	#[serde(rename(deserialize = "CDO-token"))]
	Cdo,
	#[serde(rename(deserialize = "CDC-token"))]
	Cdc,
	#[serde(rename(deserialize = "colon-token"))]
	Colon,
	#[serde(rename(deserialize = "semicolon-token"))]
	Semicolon,
	#[serde(rename(deserialize = "comma-token"))]
	Comma,
	#[serde(rename(deserialize = "[-token"))]
	LeftSquare,
	#[serde(rename(deserialize = "]-token"))]
	RightSquare,
	#[serde(rename(deserialize = "(-token"))]
	LeftParen,
	#[serde(rename(deserialize = ")-token"))]
	RightParen,
	#[serde(rename(deserialize = "{-token"))]
	LeftCurly,
	#[serde(rename(deserialize = "}-token"))]
	RightCurly,
}

pub struct CSSTokenizerTestSuite;

impl LexerSuite<CSSTokenizerTestCase> for CSSTokenizerTestSuite {
	fn new(_: &AppArgs) -> CSSTokenizerTestSuite {
		CSSTokenizerTestSuite {}
	}

	fn get_tests(&mut self, _: &AppArgs) -> Vec<CSSTokenizerTestCase> {
		let mut files = vec![];
		for path in glob(FIXTURES_GLOB).unwrap().flatten() {
			files.push(CSSTokenizerTestCase::new(path));
		}
		files
	}
}

pub struct CSSTokenizerTestCase {
	name: String,
	source_path: PathBuf,
	source_text: String,
	desired: Vec<RomainToken>,
}

impl CSSTokenizerTestCase {
	fn new(source_path: PathBuf) -> Self {
		let json_path = source_path.as_path().parent().unwrap().join("tokens.json").to_path_buf();
		let name = format!(
			"{}",
			source_path
				.parent()
				.unwrap()
				.strip_prefix("tasks/coverage/css-tokenizer-tests/tests")
				.unwrap()
				.display()
		);
		let source_text = read_to_string(&source_path).unwrap();
		let desired: Vec<RomainToken> =
			from_str(read_to_string(json_path.clone()).unwrap().as_str())
				.unwrap_or_else(|e| panic!("{} {}", json_path.display(), e));
		Self { name, source_path, source_text, desired }
	}
}

impl LexerCase for CSSTokenizerTestCase {
	type Token = RomainToken;

	fn name(&self) -> &str {
		&self.name
	}

	fn source_text(&self) -> &str {
		&self.source_text
	}

	fn path(&self) -> &Path {
		&self.source_path
	}

	fn desired(&self) -> &Vec<RomainToken> {
		&self.desired
	}

	// Comes with fixtures, no need to update
	fn update_desired(&self, _: &Vec<RomainToken>) {}

	fn convert_token(&self, token: &Token) -> RomainToken {
		let structured = match &token.value {
			TokenValue::Number { int, value, .. } => {
				if token.kind == Kind::Percentage {
					Some(Structured::Number(NumberStructured { value: *value, kind: None }))
				} else {
					Some(Structured::Number(NumberStructured {
						value: *value,
						kind: Some(String::from(if *int { "integer" } else { "number" })),
					}))
				}
			}
			TokenValue::Dimension { int, value, unit, .. } => {
				Some(Structured::Dimension(DimensionStructured {
					value: *value,
					unit: (**unit).into(),
					kind: String::from(if *int { "integer" } else { "number" }),
				}))
			}
			TokenValue::String(value) => {
				Some(Structured::String(StringStructured { value: (**value).into() }))
			}
			TokenValue::Char(value) => {
				Some(Structured::String(StringStructured { value: value.to_string() }))
			}
			TokenValue::Unrestricted(value) => {
				Some(Structured::String(StringStructured { value: (**value).into() }))
			}
			TokenValue::None => None,
		};
		let raw = self.source_text[token.span.start as usize..token.span.end as usize].to_string();
		// token.start/end count utf8 bytes because rust strings are utf8. The tokenizer tests,
		// however, are JS based and JS strings are utf16. So recalculate the indexes to utf16
		let start_index =
			self.source_text[0..token.span.start as usize].encode_utf16().count() as u32;
		let end_index = start_index + raw.encode_utf16().count() as u32;
		RomainToken {
			kind: match token.kind {
				Kind::Comment => RomainKind::Comment,
				Kind::Ident => RomainKind::Ident,
				Kind::Function => RomainKind::Function,
				Kind::AtKeyword => RomainKind::AtKeyword,
				Kind::Hash => RomainKind::Hash,
				Kind::String => RomainKind::String,
				Kind::BadString => RomainKind::BadString,
				Kind::Url => RomainKind::Url,
				Kind::BadUrl => RomainKind::BadUrl,
				Kind::Delim => RomainKind::Delim,
				Kind::Number => RomainKind::Number,
				Kind::Percentage => RomainKind::Percentage,
				Kind::Dimension => RomainKind::Dimension,
				Kind::Whitespace => RomainKind::Whitespace,
				Kind::Cdo => RomainKind::Cdo,
				Kind::Cdc => RomainKind::Cdc,
				Kind::Colon => RomainKind::Colon,
				Kind::Semicolon => RomainKind::Semicolon,
				Kind::Comma => RomainKind::Comma,
				Kind::LeftSquare => RomainKind::LeftSquare,
				Kind::RightSquare => RomainKind::RightSquare,
				Kind::LeftParen => RomainKind::LeftParen,
				Kind::RightParen => RomainKind::RightParen,
				Kind::LeftCurly => RomainKind::LeftCurly,
				Kind::RightCurly => RomainKind::RightCurly,
				_ => unreachable!(),
			},
			raw,
			start_index,
			end_index,
			structured,
		}
	}
}
