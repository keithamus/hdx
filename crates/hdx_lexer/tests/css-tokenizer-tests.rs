use bumpalo::Bump;
use console::Style;
use glob::glob;
use hdx_atom::atom;
use hdx_lexer::{Include, Lexer, Token};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use similar::{ChangeTag, TextDiff};
use std::{fs::read_to_string, path::PathBuf};

const FIXTURES_GLOB: &str = "../../tasks/coverage/css-tokenizer-tests/tests/**/source.css";

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
pub struct CSSTokenizerTestToken {
	#[serde(rename(deserialize = "type"))]
	kind: CSSTokenizerTestKind,
	raw: String,
	start_index: u32,
	end_index: u32,
	structured: Option<Structured>,
}

#[derive(PartialEq, Serialize, Deserialize)]
enum CSSTokenizerTestKind {
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

fn get_tests() -> Vec<CSSTokenizerTestCase> {
	let mut files = vec![];
	for path in glob(FIXTURES_GLOB).unwrap().flatten() {
		files.push(CSSTokenizerTestCase::new(path));
	}
	files
}

pub struct CSSTokenizerTestCase {
	name: String,
	source_text: String,
	desired: Vec<CSSTokenizerTestToken>,
}

impl CSSTokenizerTestCase {
	fn new(source_path: PathBuf) -> Self {
		let json_path = source_path.as_path().parent().unwrap().join("tokens.json").to_path_buf();
		let path = source_path.parent().unwrap();
		let name = format!(
			"{}-{}",
			&path.parent().unwrap().file_name().unwrap().to_str().unwrap(),
			&path.file_name().unwrap().to_str().unwrap()
		);
		let source_text = read_to_string(&source_path).unwrap();
		let desired: Vec<CSSTokenizerTestToken> = from_str(read_to_string(json_path.clone()).unwrap().as_str())
			.unwrap_or_else(|e| panic!("{} {}", json_path.display(), e));
		Self { name, source_text, desired }
	}
}

fn convert_token(source_text: &str, start: usize, end: usize, token: &Token) -> CSSTokenizerTestToken {
	let raw = source_text[start..end].to_string();
	let structured = match &token {
		Token::Number(value, numtype) => Some(Structured::Number(NumberStructured {
			value: *value,
			kind: Some(String::from(if numtype.is_int() { "integer" } else { "number" })),
		})),
		Token::Dimension(value, unit, numtype) => {
			if unit == &atom!("%") {
				Some(Structured::Number(NumberStructured { value: *value, kind: None }))
			} else {
				Some(Structured::Dimension(DimensionStructured {
					value: *value,
					unit: (**unit).into(),
					kind: String::from(if numtype.is_int() { "integer" } else { "number" }),
				}))
			}
		}
		Token::Ident(value)
		| Token::String(value, _)
		| Token::AtKeyword(value)
		| Token::Function(value)
		| Token::Url(value, _)
		| Token::Hash(value)
		| Token::HashId(value) => Some(Structured::String(StringStructured { value: (**value).into() })),
		Token::Delim(value) => Some(Structured::String(StringStructured { value: value.to_string() })),
		_ => None,
	};
	// token.start/end count utf8 bytes because rust strings are utf8. The tokenizer tests,
	// however, are JS based and JS strings are utf16. So recalculate the indexes to utf16
	let start_index = source_text[0..start].encode_utf16().count() as u32;
	let end_index = start_index + raw.encode_utf16().count() as u32;
	CSSTokenizerTestToken {
		kind: match token {
			Token::Comment(_) => CSSTokenizerTestKind::Comment,
			Token::Ident(_) => CSSTokenizerTestKind::Ident,
			Token::Function(_) => CSSTokenizerTestKind::Function,
			Token::AtKeyword(_) => CSSTokenizerTestKind::AtKeyword,
			Token::Hash(_) => CSSTokenizerTestKind::Hash,
			Token::HashId(_) => CSSTokenizerTestKind::Hash,
			Token::String(_, _) => CSSTokenizerTestKind::String,
			Token::BadString => CSSTokenizerTestKind::BadString,
			Token::Url(_, _) => CSSTokenizerTestKind::Url,
			Token::BadUrl => CSSTokenizerTestKind::BadUrl,
			Token::Delim(_) => CSSTokenizerTestKind::Delim,
			Token::Number(_, _) => CSSTokenizerTestKind::Number,
			Token::Dimension(_, unit, _) => {
				if unit == &atom!("%") {
					CSSTokenizerTestKind::Percentage
				} else {
					CSSTokenizerTestKind::Dimension
				}
			}
			Token::Whitespace => CSSTokenizerTestKind::Whitespace,
			Token::Cdo => CSSTokenizerTestKind::Cdo,
			Token::Cdc => CSSTokenizerTestKind::Cdc,
			Token::Colon => CSSTokenizerTestKind::Colon,
			Token::Semicolon => CSSTokenizerTestKind::Semicolon,
			Token::Comma => CSSTokenizerTestKind::Comma,
			Token::LeftSquare => CSSTokenizerTestKind::LeftSquare,
			Token::RightSquare => CSSTokenizerTestKind::RightSquare,
			Token::LeftParen => CSSTokenizerTestKind::LeftParen,
			Token::RightParen => CSSTokenizerTestKind::RightParen,
			Token::LeftCurly => CSSTokenizerTestKind::LeftCurly,
			Token::RightCurly => CSSTokenizerTestKind::RightCurly,
			_ => unreachable!(),
		},
		raw,
		start_index,
		end_index,
		structured,
	}
}

fn test_case(case: CSSTokenizerTestCase) -> u8 {
	let allocator = Bump::default();
	let mut lexer = Lexer::new(&allocator, &case.source_text);
	let mut tokens = vec![];
	let mut pos = 0;
	loop {
		let token = lexer.advance_with(Include::all());
		if token == Token::Eof {
			break;
		}
		tokens.push(convert_token(&case.source_text, pos, lexer.pos() as usize, &token));
		pos = lexer.pos() as usize;
	}
	if tokens != *case.desired {
		let left: String = to_string_pretty(&tokens).unwrap_or("".to_string());
		let right: String = to_string_pretty(&case.desired).unwrap_or("".to_string());
		println!("{} {}", Style::new().red().apply_to("✘ FAILED"), case.name);
		println!("{}", Style::new().red().apply_to("- actual"));
		println!("{}", Style::new().green().apply_to("+ expected"));
		let diff = TextDiff::from_lines(&*left, &*right);
		for change in diff.iter_all_changes() {
			let (sign, style) = match change.tag() {
				ChangeTag::Delete => ("-", Style::new().red()),
				ChangeTag::Insert => ("+", Style::new().green()),
				ChangeTag::Equal => (" ", Style::new()),
			};
			print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
		}
		return 1;
	}
	0
}

#[test]
fn full_suite() {
	let cases = get_tests();
	let mut fails = 0;
	for case in cases {
		fails += test_case(case);
	}
	assert_eq!(fails, 0);
}
