use hdx_ast::css::StyleSheet;
use hdx_lexer::{Token, Lexer};
use hdx_parser::{Parser, Features};
use hdx_writer::{BaseCssWriter, WriteCss};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use oxc_allocator::Allocator;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn lex(source_text: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, source_text.as_str());
	let serializer = serde_wasm_bindgen::Serializer::json_compatible();
	let mut tokens = vec![];
	loop {
		let token = lex.advance();
		tokens.push(token.clone());
		if matches!(token, Token::Eof) {
			break;
		}
	}
	Ok(tokens.serialize(&serializer).unwrap())
}

#[wasm_bindgen]
pub fn parse(source_text: String) -> Result<SerializableParserResult, serde_wasm_bindgen::Error> {
	let allocator = Allocator::default();
	let result = Parser::new(&allocator, source_text.as_str(), Features::default())
		.parse_with::<StyleSheet>();
	let serializer = serde_wasm_bindgen::Serializer::json_compatible();
	let diagnostics = result
		.errors
		.iter()
		.chain(result.warnings.iter())
		.flat_map(|error| {
			let Some(labels) = error.labels() else { return vec![] };
			labels
				.map(|label| {
					Diagnostic {
						from: label.offset(),
						to: label.offset() + label.len(),
						code: format!("{}", error.code().unwrap_or(Box::new(""))),
						severity: format!("{:?}", error.severity().unwrap_or_default())
							.to_ascii_lowercase(),
						message: format!("{error}"),
					}
					.serialize(&serializer)
					.unwrap()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	Ok(SerializableParserResult { ast: result.output.serialize(&serializer).unwrap(), diagnostics })
}

#[wasm_bindgen]
pub fn minify(source_text: String) -> Result<String, serde_wasm_bindgen::Error> {
	let allocator = Allocator::default();
	let result = Parser::new(&allocator, source_text.as_str(), Features::default())
		.parse_with::<StyleSheet>();
	if !result.errors.is_empty() {
		return Err(serde_wasm_bindgen::Error::new("Parse error"));
	}
	let mut string = String::new();
	let mut writer = BaseCssWriter::new(&mut string, true);
	result.output.unwrap().write_css(&mut writer).unwrap();
	Ok(string)
}

#[wasm_bindgen]
pub fn parse_error_report(source_text: String) -> String {
	let allocator = Allocator::default();
	let result = Parser::new(&allocator, source_text.as_str(), Features::default())
		.parse_with::<StyleSheet>();
	let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
	let mut report = String::new();
	for err in result.errors {
		let err = err.with_source_code(NamedSource::new("", source_text.to_string()));
		handler.render_report(&mut report, err.as_ref()).unwrap();
		report += "\n";
	}
	for warn in result.warnings {
		let warn = warn.with_source_code(NamedSource::new("", source_text.to_string()));
		handler.render_report(&mut report, warn.as_ref()).unwrap();
		report += "\n";
	}
	format!("{}", &report)
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SerializableParserResult {
	ast: JsValue,
	diagnostics: Vec<JsValue>,
}

#[derive(Default, Clone, Serialize)]
pub struct Diagnostic {
	pub from: usize,
	pub to: usize,
	pub code: String,
	pub severity: String,
	pub message: String,
}

#[wasm_bindgen]
impl SerializableParserResult {
	#[wasm_bindgen(getter)]
	pub fn ast(&self) -> JsValue {
		self.ast.clone()
	}

	#[wasm_bindgen(getter)]
	pub fn diagnostics(&self) -> Vec<JsValue> {
		self.diagnostics.clone()
	}
}
