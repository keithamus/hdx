use bumpalo::Bump;
use dashmap::DashMap;
use hdx_ast::css::{StyleSheet, Visitable};
use hdx_highlight::{SemanticKind, SemanticModifier, TokenHighlighter};
use hdx_parser::{Features, Parser};
use itertools::Itertools;
use lsp_types::{
	notification::{DidChangeTextDocument, DidOpenTextDocument},
	request::{Initialize, SemanticTokensFullRequest},
	InitializeResult, SemanticToken, SemanticTokenModifier, SemanticTokenType, SemanticTokens,
	SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions, SemanticTokensResult,
	SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind,
	TextDocumentSyncOptions, TextDocumentSyncSaveOptions, Uri, WorkDoneProgressOptions,
};
use std::{io, sync::Arc};
use strum::VariantNames;
use tracing::trace;

use crate::Server;

pub fn server_with_handlers(version: &'static str) -> Server {
	let files = Arc::new(DashMap::<Uri, String>::new());
	let files_for_semantic_tokens = files.clone();
	let files_for_open_doc = files.clone();
	let files_for_change_doc = files.clone();
	Server::new()
		.handle::<Initialize>(move |_| -> Result<InitializeResult, io::Error> {
			Ok(InitializeResult {
				capabilities: ServerCapabilities {
					// position_encoding: (),
					text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
						open_close: Some(true),
						change: Some(TextDocumentSyncKind::FULL),
						will_save: Some(true),
						will_save_wait_until: Some(false),
						save: Some(TextDocumentSyncSaveOptions::Supported(false)),
					})),
					// notebook_document_sync: (),
					// selection_range_provider: (),
					// hover_provider: (),
					// completion_provider: (),
					// signature_help_provider: (),
					// definition_provider: (),
					// type_definition_provider: (),
					// implementation_provider: (),
					// references_provider: (),
					// document_highlight_provider: (),
					// document_symbol_provider: (),
					// workspace_symbol_provider: (),
					// code_action_provider: (),
					// code_lens_provider: (),
					// document_formatting_provider: (),
					// document_range_formatting_provider: (),
					// document_on_type_formatting_provider: (),
					// rename_provider: (),
					// document_link_provider: (),
					// color_provider: (),
					// folding_range_provider: (),
					// declaration_provider: (),
					// execute_command_provider: (),
					// workspace: (),
					// call_hierarchy_provider: (),
					semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
						SemanticTokensOptions {
							work_done_progress_options: WorkDoneProgressOptions { work_done_progress: Some(false) },
							legend: SemanticTokensLegend {
								token_types: SemanticKind::VARIANTS.iter().map(|v| SemanticTokenType::new(v)).collect(),
								token_modifiers: SemanticModifier::VARIANTS
									.iter()
									.map(|v| SemanticTokenModifier::new(v))
									.collect(),
							},
							range: Some(false),
							full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
						},
					)),
					// moniker_provider: (),
					// linked_editing_range_provider: (),
					// inline_value_provider: (),
					// inlay_hint_provider: (),
					// diagnostic_provider: (),
					// inline_completion_provider: (),
					// experimental: (),
					..Default::default()
				},
				server_info: Some(ServerInfo { name: String::from("hdx-lsp"), version: Some(version.into()) }),
				offset_encoding: None,
			})
		})
		.handle::<SemanticTokensFullRequest>(move |params| -> Result<Option<SemanticTokensResult>, io::Error> {
			let uri = params.text_document.uri;
			let allocator = Bump::default();
			if let Some(source_text) = files_for_semantic_tokens.get(&uri) {
				trace!("Asked for SemanticTokens");
				let result =
					Parser::new(&allocator, source_text.as_str(), Features::default()).parse_entirely::<StyleSheet>();
				if let Some(stylesheet) = result.output {
					trace!("Sucessfully parsed stylesheet: {:#?}", &stylesheet);
					let mut highlighter = TokenHighlighter::new();
					stylesheet.accept(&mut highlighter);
					let mut current_line = 0;
					let mut current_start = 0;
					let data = highlighter
						.highlights()
						.sorted_by(|a, b| Ord::cmp(&a.span(), &b.span()))
						.map(|highlight| {
							trace!("Highlight: {:?}", &highlight);
							let span_contents = highlight.span().span_contents(source_text.as_str());
							let (line, start) = span_contents.line_and_column();
							let delta_line = line - current_line;
							current_line = line;
							let delta_start = if delta_line == 0 { start - current_start } else { start };
							current_start = start;
							SemanticToken {
								token_type: highlight.kind().bits() as u32,
								token_modifiers_bitset: highlight.modifier().bits() as u32,
								delta_line,
								delta_start,
								length: span_contents.size(),
							}
						})
						.collect();
					return Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data })));
				} else if !result.errors.is_empty() {
					trace!("\n\nParse on {:?} failed. Saw error {:?}", &uri, result.errors);
				}
			}
			Ok(None)
		})
		.on::<DidOpenTextDocument>(move |params| -> Result<(), io::Error> {
			let uri = params.text_document.uri;
			let source_text = params.text_document.text;
			files_for_open_doc.insert(uri, source_text);
			Ok(())
		})
		.on::<DidChangeTextDocument>(move |params| -> Result<(), io::Error> {
			let uri = params.text_document.uri;
			let changes = params.content_changes;
			if changes.len() == 1 && changes[0].range.is_none() {
				let source_text = &changes[0].text;
				files_for_change_doc.clone().insert(uri, source_text.into());
			}
			Ok(())
		})
}
