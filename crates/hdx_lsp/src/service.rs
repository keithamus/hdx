use bumpalo::Bump;
use dashmap::DashMap;
use hdx_ast::css::{StyleSheet, Visitable};
use hdx_highlight::{SemanticKind, SemanticModifier, TokenHighlighter};
use hdx_parser::{Features, Parser};
use itertools::Itertools;
use lsp_types::Uri;
use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};
use strum::VariantNames;
use tracing::trace;

use crate::{ErrorCode, Handler};

pub struct LSPService {
	version: String,
	files: Arc<DashMap<Uri, String>>,
	initialized: AtomicBool,
}

impl LSPService {
	pub fn new(version: &'static str) -> Self {
		Self { version: version.into(), files: Arc::new(DashMap::new()), initialized: AtomicBool::new(false) }
	}
}

impl Handler for LSPService {
	fn initialized(&self) -> bool {
		self.initialized.load(Ordering::SeqCst)
	}

	fn initialize(&self, req: lsp_types::InitializeParams) -> Result<lsp_types::InitializeResult, ErrorCode> {
		self.initialized.swap(true, Ordering::SeqCst);
		Ok(lsp_types::InitializeResult {
			capabilities: lsp_types::ServerCapabilities {
				// position_encoding: (),
				text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Options(
					lsp_types::TextDocumentSyncOptions {
						open_close: Some(true),
						change: Some(lsp_types::TextDocumentSyncKind::FULL),
						will_save: Some(true),
						will_save_wait_until: Some(false),
						save: Some(lsp_types::TextDocumentSyncSaveOptions::Supported(false)),
					},
				)),
				// notebook_document_sync: (),
				// selection_range_provider: (),
				// hover_provider: (),
				completion_provider: Some(lsp_types::CompletionOptions {
					resolve_provider: None,
					trigger_characters: Some(vec![".".into(), ":".into(), "@".into(), "#".into(), "-".into()]),
					all_commit_characters: None,
					work_done_progress_options: lsp_types::WorkDoneProgressOptions { work_done_progress: None },
					completion_item: None,
				}),
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
				semantic_tokens_provider: Some(lsp_types::SemanticTokensServerCapabilities::SemanticTokensOptions(
					lsp_types::SemanticTokensOptions {
						work_done_progress_options: lsp_types::WorkDoneProgressOptions {
							work_done_progress: Some(false),
						},
						legend: lsp_types::SemanticTokensLegend {
							token_types: SemanticKind::VARIANTS
								.iter()
								.map(|v| lsp_types::SemanticTokenType::new(v))
								.collect(),
							token_modifiers: SemanticModifier::VARIANTS
								.iter()
								.map(|v| lsp_types::SemanticTokenModifier::new(v))
								.collect(),
						},
						range: Some(false),
						full: Some(lsp_types::SemanticTokensFullOptions::Delta { delta: Some(true) }),
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
			server_info: Some(lsp_types::ServerInfo {
				name: String::from("hdx-lsp"),
				version: Some(self.version.clone()),
			}),
			offset_encoding: None,
		})
	}

	fn semantic_tokens_full_request(
		&self,
		req: lsp_types::SemanticTokensParams,
	) -> Result<Option<lsp_types::SemanticTokensResult>, ErrorCode> {
		let uri = req.text_document.uri;
		let allocator = Bump::default();
		if let Some(source_text) = self.files.get(&uri) {
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
						let span_contents = highlight.span().span_contents(source_text.as_str());
						let (line, start) = span_contents.line_and_column();
						let delta_line = line - current_line;
						current_line = line;
						let delta_start = if delta_line == 0 { start - current_start } else { start };
						current_start = start;
						lsp_types::SemanticToken {
							token_type: highlight.kind().bits() as u32,
							token_modifiers_bitset: highlight.modifier().bits() as u32,
							delta_line,
							delta_start,
							length: span_contents.size(),
						}
					})
					.collect();
				return Ok(Some(lsp_types::SemanticTokensResult::Tokens(lsp_types::SemanticTokens {
					result_id: None,
					data,
				})));
			} else if !result.errors.is_empty() {
				trace!("\n\nParse on {:?} failed. Saw error {:?}", &uri, result.errors);
			}
		}
		Err(ErrorCode::InternalError)
	}

	fn completion(&self, req: lsp_types::CompletionParams) -> Result<Option<lsp_types::CompletionResponse>, ErrorCode> {
		// let uri = req.text_document.uri;
		// let position = req.text_document_position;
		// let context = req.context;
		Err(ErrorCode::UnknownErrorCode)
	}

	fn on_did_open_text_document(&self, req: lsp_types::DidOpenTextDocumentParams) {
		let uri = req.text_document.uri;
		let source_text = req.text_document.text;
		self.files.clone().insert(uri, source_text);
	}

	fn on_did_change_text_document(&self, req: lsp_types::DidChangeTextDocumentParams) {
		let uri = req.text_document.uri;
		let changes = req.content_changes;
		if changes.len() == 1 && changes[0].range.is_none() {
			let source_text = &changes[0].text;
			self.files.clone().insert(uri, source_text.into());
		}
	}
}
