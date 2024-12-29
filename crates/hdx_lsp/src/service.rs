use bumpalo::Bump;
use crossbeam_channel::{bounded, Receiver, Sender};
use css_ast::{
	completions::{CompletionContext, CompletionItem, CompletionList},
	css::{StyleSheet, Visitable},
	traits::NodeAtSpan,
};
use css_parse::{Cursor, Features, Parser, ParserReturn, SourceOffset, Span, Token};
use dashmap::DashMap;
use hdx_highlight::{Highlight, SemanticKind, SemanticModifier, TokenHighlighter};
use itertools::Itertools;
use lsp_types::Uri;
use ropey::Rope;
use std::{
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc,
	},
	thread::{Builder, JoinHandle},
};
use strum::VariantNames;
use tracing::{instrument, trace};

use crate::{ErrorCode, Handler};

type Line = u32;
type Col = u32;

#[derive(Debug)]
enum FileCall {
	// Re-parse the document based on changes
	RopeChange(Rope),
	// Highlight a document, returning the semantic highlights
	Highlight,
}

#[derive(Debug)]
enum FileReturn {
	Highlights(Vec<(Highlight, Line, Col)>),
}

#[derive(Debug)]
pub struct File {
	pub content: Rope,
	thread: JoinHandle<()>,
	sender: Sender<FileCall>,
	receiver: Receiver<FileReturn>,
}

impl File {
	fn new() -> Self {
		let (sender, read_receiver) = bounded::<FileCall>(0);
		let (write_sender, receiver) = bounded::<FileReturn>(0);
		Self {
			content: Rope::new(),
			sender,
			receiver,
			thread: Builder::new()
				.name("LspDocumentHandler".into())
				.spawn(move || {
					let mut bump = Bump::default();
					let mut string: String = "".into();
					let mut result: ParserReturn<'_, StyleSheet<'_>> =
						Parser::new(&bump, "", Features::default()).parse_entirely::<StyleSheet>();
					while let Ok(call) = read_receiver.recv() {
						match call {
							FileCall::RopeChange(rope) => {
								trace!("Parsing document");
								// TODO! we should be able to optimize this by parsing a subset of the tree and mutating in
								// place. For now though a partial parse request re-parses it all.
								drop(result);
								bump.reset();
								string = rope.clone().into();
								result =
									Parser::new(&bump, &string, Features::default()).parse_entirely::<StyleSheet>();
								if let Some(stylesheet) = &result.output {
									trace!("Sucessfully parsed stylesheet: {:#?}", &stylesheet);
								}
							}
							FileCall::Highlight => {
								trace!("Highlighting document");
								let mut highlighter = TokenHighlighter::new();
								if let Some(stylesheet) = &result.output {
									stylesheet.accept(&mut highlighter);
									let mut current_line = 0;
									let mut current_start = 0;
									let data = highlighter
										.highlights()
										.sorted_by(|a, b| Ord::cmp(&a.span(), &b.span()))
										.map(|h| {
											// TODO: figure out a more efficient way to get line/col
											let span_contents = h.span().span_contents(&string);
											let (line, start) = span_contents.line_and_column();
											let delta_line: Line = line - current_line;
											current_line = line;
											let delta_start: Col =
												if delta_line == 0 { start - current_start } else { start };
											current_start = start;
											(*h, delta_line, delta_start)
										});
									write_sender.send(FileReturn::Highlights(data.collect())).ok();
								}
							}
						}
					}
				})
				.expect("Failed to document thread Reader"),
		}
	}

	fn to_string(&self) -> String {
		self.content.clone().into()
	}

	fn commit(&mut self, rope: Rope) {
		self.content = rope;
		self.sender.send(FileCall::RopeChange(self.content.clone())).unwrap();
	}

	#[instrument]
	fn get_highlights(&self) -> Vec<(Highlight, Line, Col)> {
		self.sender.send(FileCall::Highlight).unwrap();
		while let Ok(ret) = self.receiver.recv() {
			if let FileReturn::Highlights(highlights) = ret {
				return highlights;
			}
		}
		return vec![];
	}
}

#[derive(Debug)]
pub struct LSPService {
	version: String,
	files: Arc<DashMap<Uri, File>>,
	initialized: AtomicBool,
}

impl LSPService {
	pub fn new(version: &'static str) -> Self {
		Self { version: version.into(), files: Arc::new(DashMap::new()), initialized: AtomicBool::new(false) }
	}
}

impl Handler for LSPService {
	#[instrument]
	fn initialized(&self) -> bool {
		self.initialized.load(Ordering::SeqCst)
	}

	#[instrument]
	fn initialize(&self, req: lsp_types::InitializeParams) -> Result<lsp_types::InitializeResult, ErrorCode> {
		self.initialized.swap(true, Ordering::SeqCst);
		Ok(lsp_types::InitializeResult {
			capabilities: lsp_types::ServerCapabilities {
				// position_encoding: (),
				text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Options(
					lsp_types::TextDocumentSyncOptions {
						open_close: Some(true),
						change: Some(lsp_types::TextDocumentSyncKind::INCREMENTAL),
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

	#[instrument]
	fn semantic_tokens_full_request(
		&self,
		req: lsp_types::SemanticTokensParams,
	) -> Result<Option<lsp_types::SemanticTokensResult>, ErrorCode> {
		let uri = req.text_document.uri;
		trace!("Asked for SemanticTokens for {:?}", &uri);
		if let Some(document) = self.files.get(&uri) {
			let data = document
				.get_highlights()
				.into_iter()
				.map(|(highlight, delta_line, delta_start)| lsp_types::SemanticToken {
					token_type: highlight.kind().bits() as u32,
					token_modifiers_bitset: highlight.modifier().bits() as u32,
					delta_line,
					delta_start,
					length: highlight.span().size(),
				})
				.collect();
			Ok(Some(lsp_types::SemanticTokensResult::Tokens(lsp_types::SemanticTokens { result_id: None, data })))
		} else {
			Err(ErrorCode::InternalError)
		}
	}

	#[instrument]
	fn completion(&self, req: lsp_types::CompletionParams) -> Result<Option<lsp_types::CompletionResponse>, ErrorCode> {
		let uri = req.text_document_position.text_document.uri;
		let position = req.text_document_position.position;
		let context = req.context;
		Ok(None)
	}

	#[instrument]
	fn on_did_open_text_document(&self, req: lsp_types::DidOpenTextDocumentParams) {
		let uri = req.text_document.uri;
		let source_text = req.text_document.text;
		let mut doc = File::new();
		let mut rope = doc.content.clone();
		rope.remove(0..);
		rope.insert(0, &source_text);
		trace!("comitting new document {:?} {:?}", &uri, rope);
		doc.commit(rope);
		self.files.clone().insert(uri, doc);
	}

	#[instrument]
	fn on_did_change_text_document(&self, req: lsp_types::DidChangeTextDocumentParams) {
		let uri = req.text_document.uri;
		let changes = req.content_changes;
		if let Some(mut file) = self.files.clone().get_mut(&uri) {
			let mut rope = file.content.clone();
			for change in changes {
				let range = if let Some(range) = change.range {
					rope.try_line_to_char(range.start.line as usize).map_or_else(
						|_| (0, None),
						|start| {
							rope.try_line_to_char(range.end.line as usize).map_or_else(
								|_| (start + range.start.character as usize, None),
								|end| {
									(start + range.start.character as usize, Some(end + range.end.character as usize))
								},
							)
						},
					)
				} else {
					(0, None)
				};
				match range {
					(start, None) => {
						rope.try_remove(start..).ok();
						rope.try_insert(start, &change.text).ok();
					}
					(start, Some(end)) => {
						rope.try_remove(start..end).ok();
						rope.try_insert(start, &change.text).ok();
					}
				}
			}
			file.commit(rope)
		}
	}
}
