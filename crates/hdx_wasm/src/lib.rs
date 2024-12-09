use bumpalo::Bump;
use hdx_ast::css::StyleSheet;
use hdx_lexer::{Kind, Lexer};
use hdx_parser::{Features, Parser};
#[cfg(not(feature = "fancy"))]
use miette::JSONReportHandler;
use miette::NamedSource;
#[cfg(feature = "fancy")]
use miette::{GraphicalReportHandler, GraphicalTheme};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn lex(source_text: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
	let mut lex = Lexer::new(source_text.as_str());
	let serializer = serde_wasm_bindgen::Serializer::json_compatible();
	let mut tokens = vec![];
	loop {
		let token = lex.advance();
		tokens.push(token);
		if token.kind() == Kind::Eof {
			break;
		}
	}
	Ok(tokens.serialize(&serializer).unwrap())
}

#[wasm_bindgen]
pub fn parse(source_text: String) -> Result<SerializableParserResult, serde_wasm_bindgen::Error> {
	let allocator = Bump::default();
	let result = Parser::new(&allocator, source_text.as_str(), Features::default()).parse_entirely::<StyleSheet>();
	let serializer = serde_wasm_bindgen::Serializer::json_compatible();
	let diagnostics = result
		.errors
		.iter()
		.flat_map(|error| {
			let Some(labels) = error.labels() else { return vec![] };
			labels
				.map(|label| {
					Diagnostic {
						from: label.offset(),
						to: label.offset() + label.len(),
						code: format!("{}", error.code().unwrap_or(Box::new(""))),
						severity: format!("{:?}", error.severity().unwrap_or_default()).to_ascii_lowercase(),
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
	let allocator = Bump::default();
	let result = Parser::new(&allocator, source_text.as_str(), Features::default()).parse_entirely::<StyleSheet>();
	if !result.errors.is_empty() {
		return Err(serde_wasm_bindgen::Error::new("Parse error"));
	}
	let mut output_string = String::new();
	if let Err(e) = result.write(&allocator, &mut output_string) {
		return Err(serde_wasm_bindgen::Error::new("Write error"));
	}
	Ok(output_string)
}

#[wasm_bindgen]
pub fn parse_error_report(source_text: String) -> String {
	let allocator = Bump::default();
	let result = Parser::new(&allocator, source_text.as_str(), Features::default()).parse_entirely::<StyleSheet>();
	#[cfg(feature = "fancy")]
	let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
	#[cfg(not(feature = "fancy"))]
	let handler = JSONReportHandler::new();
	let mut report = String::new();
	for err in result.errors {
		let err = err.with_source_code(NamedSource::new("", source_text.to_string()));
		handler.render_report(&mut report, err.as_ref()).unwrap();
		report += "\n";
	}
	report.to_string()
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
