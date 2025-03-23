use crate::jsonrpc::{ErrorCode, Id, Message, Response};
use lsp_types::{notification::*, request::*};
use serde_json::Value;
use tracing::{debug, trace_span};

pub trait Handler: Sized + Send + Sync + 'static {
	fn invalid_params(&self, id: Id) -> Message {
		Message::Response(Response::Err(id, ErrorCode::InvalidParams, "InvalidParams".into(), Value::Null))
	}

	fn method_not_found(&self, id: Id) -> Message {
		Message::Response(Response::Err(id, ErrorCode::MethodNotFound, "MethodNotFound".into(), Value::Null))
	}

	fn internal_error(&self, id: Id) -> Message {
		Message::Response(Response::Err(id, ErrorCode::InternalError, "InternalError".into(), Value::Null))
	}

	fn initialized(&self) -> bool {
		false
	}

	fn handle(&self, message: Message) -> Option<Message> {
		let span = trace_span!("Handling request", "{:#?}", message);
		let _ = span.enter();
		let id = message.id().unwrap_or_default();
		if message.is_exit_notification() {
			return None;
		}
		let initialize_request = message.is_initialize_request();
		if !self.initialized() && !initialize_request {
			debug!("Skipping message {:?} before initialization", message);
			return None;
		}
		match message.method().unwrap_or_default() {
			ApplyWorkspaceEdit::METHOD => {
				Some(message.from_value::<lsp_types::ApplyWorkspaceEditParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.apply_workspace_edit(p)),
				))
			}
			CallHierarchyIncomingCalls::METHOD => {
				Some(message.from_value::<lsp_types::CallHierarchyIncomingCallsParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.call_hierarchy_incoming_calls(p)),
				))
			}
			CallHierarchyOutgoingCalls::METHOD => {
				Some(message.from_value::<lsp_types::CallHierarchyOutgoingCallsParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.call_hierarchy_outgoing_calls(p)),
				))
			}
			CallHierarchyPrepare::METHOD => {
				Some(message.from_value::<lsp_types::CallHierarchyPrepareParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.call_hierarchy_prepare(p)),
				))
			}
			CodeActionRequest::METHOD => Some(message.from_value::<lsp_types::CodeActionParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.code_action_request(p)),
			)),
			CodeActionResolveRequest::METHOD => Some(message.from_value::<lsp_types::CodeAction>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.code_action_resolve_request(p)),
			)),
			CodeLensRefresh::METHOD => Some(Message::from_result(id.clone(), self.code_lens_refresh())),
			CodeLensRequest::METHOD => Some(message.from_value::<lsp_types::CodeLensParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.code_lens_request(p)),
			)),
			CodeLensResolve::METHOD => Some(message.from_value::<lsp_types::CodeLens>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.code_lens_resolve(p)),
			)),
			ColorPresentationRequest::METHOD => {
				Some(message.from_value::<lsp_types::ColorPresentationParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.color_presentation_request(p)),
				))
			}
			Completion::METHOD => Some(message.from_value::<lsp_types::CompletionParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.completion(p)),
			)),
			DocumentColor::METHOD => Some(message.from_value::<lsp_types::DocumentColorParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.document_color(p)),
			)),
			DocumentDiagnosticRequest::METHOD => {
				Some(message.from_value::<lsp_types::DocumentDiagnosticParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.document_diagnostic_request(p)),
				))
			}
			DocumentHighlightRequest::METHOD => {
				Some(message.from_value::<lsp_types::DocumentHighlightParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.document_highlight_request(p)),
				))
			}
			DocumentLinkRequest::METHOD => Some(message.from_value::<lsp_types::DocumentLinkParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.document_link_request(p)),
			)),
			DocumentLinkResolve::METHOD => Some(message.from_value::<lsp_types::DocumentLink>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.document_link_resolve(p)),
			)),
			DocumentSymbolRequest::METHOD => Some(message.from_value::<lsp_types::DocumentSymbolParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.document_symbol_request(p)),
			)),
			ExecuteCommand::METHOD => Some(message.from_value::<lsp_types::ExecuteCommandParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.execute_command(p)),
			)),
			FoldingRangeRequest::METHOD => Some(message.from_value::<lsp_types::FoldingRangeParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.folding_range_request(p)),
			)),
			Formatting::METHOD => Some(message.from_value::<lsp_types::DocumentFormattingParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.formatting(p)),
			)),
			GotoDeclaration::METHOD => Some(message.from_value::<lsp_types::GotoDefinitionParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.goto_declaration(p)),
			)),
			GotoDefinition::METHOD => Some(message.from_value::<lsp_types::GotoDefinitionParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.goto_definition(p)),
			)),
			GotoImplementation::METHOD => Some(message.from_value::<lsp_types::GotoDefinitionParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.goto_implementation(p)),
			)),
			GotoTypeDefinition::METHOD => Some(message.from_value::<lsp_types::GotoDefinitionParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.goto_type_definition(p)),
			)),
			HoverRequest::METHOD => Some(message.from_value::<lsp_types::HoverParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.hover_request(p)),
			)),
			Initialize::METHOD => Some(message.from_value::<lsp_types::InitializeParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.initialize(p)),
			)),
			InlayHintRefreshRequest::METHOD => {
				Some(Message::from_result(id.clone(), self.inlay_hint_refresh_request()))
			}
			InlayHintRequest::METHOD => Some(message.from_value::<lsp_types::InlayHintParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.inlay_hint_request(p)),
			)),
			InlayHintResolveRequest::METHOD => Some(message.from_value::<lsp_types::InlayHint>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.inlay_hint_resolve_request(p)),
			)),
			InlineValueRefreshRequest::METHOD => {
				Some(Message::from_result(id.clone(), self.inline_value_refresh_request()))
			}
			InlineValueRequest::METHOD => Some(message.from_value::<lsp_types::InlineValueParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.inline_value_request(p)),
			)),
			LinkedEditingRange::METHOD => {
				Some(message.from_value::<lsp_types::LinkedEditingRangeParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.linked_editing_range(p)),
				))
			}
			MonikerRequest::METHOD => Some(message.from_value::<lsp_types::MonikerParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.moniker_request(p)),
			)),
			OnTypeFormatting::METHOD => {
				Some(message.from_value::<lsp_types::DocumentOnTypeFormattingParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.on_type_formatting(p)),
				))
			}
			PrepareRenameRequest::METHOD => {
				Some(message.from_value::<lsp_types::TextDocumentPositionParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.prepare_rename_request(p)),
				))
			}
			RangeFormatting::METHOD => {
				Some(message.from_value::<lsp_types::DocumentRangeFormattingParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.range_formatting(p)),
				))
			}
			References::METHOD => Some(message.from_value::<lsp_types::ReferenceParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.references(p)),
			)),
			RegisterCapability::METHOD => Some(message.from_value::<lsp_types::RegistrationParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.register_capability(p)),
			)),
			Rename::METHOD => Some(message.from_value::<lsp_types::RenameParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.rename(p)),
			)),
			ResolveCompletionItem::METHOD => Some(message.from_value::<lsp_types::CompletionItem>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.resolve_completion_item(p)),
			)),
			SelectionRangeRequest::METHOD => Some(message.from_value::<lsp_types::SelectionRangeParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.selection_range_request(p)),
			)),
			SemanticTokensFullDeltaRequest::METHOD => {
				Some(message.from_value::<lsp_types::SemanticTokensDeltaParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.semantic_tokens_full_delta_request(p)),
				))
			}
			SemanticTokensFullRequest::METHOD => {
				Some(message.from_value::<lsp_types::SemanticTokensParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.semantic_tokens_full_request(p)),
				))
			}
			SemanticTokensRangeRequest::METHOD => {
				Some(message.from_value::<lsp_types::SemanticTokensRangeParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.semantic_tokens_range_request(p)),
				))
			}
			SemanticTokensRefresh::METHOD => Some(Message::from_result(id.clone(), self.semantic_tokens_refresh())),
			ShowDocument::METHOD => Some(message.from_value::<lsp_types::ShowDocumentParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.show_document(p)),
			)),
			ShowMessageRequest::METHOD => Some(message.from_value::<lsp_types::ShowMessageParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.show_message_request(p)),
			)),
			Shutdown::METHOD => Some(Message::from_result(id.clone(), self.shutdown())),
			SignatureHelpRequest::METHOD => Some(message.from_value::<lsp_types::SignatureHelpParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.signature_help_request(p)),
			)),
			TypeHierarchyPrepare::METHOD => {
				Some(message.from_value::<lsp_types::TypeHierarchyPrepareParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.type_hierarchy_prepare(p)),
				))
			}
			TypeHierarchySubtypes::METHOD => {
				Some(message.from_value::<lsp_types::TypeHierarchySubtypesParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.type_hierarchy_subtypes(p)),
				))
			}
			TypeHierarchySupertypes::METHOD => {
				Some(message.from_value::<lsp_types::TypeHierarchySupertypesParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.type_hierarchy_supertypes(p)),
				))
			}
			UnregisterCapability::METHOD => Some(message.from_value::<lsp_types::UnregistrationParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.unregister_capability(p)),
			)),
			WillCreateFiles::METHOD => Some(message.from_value::<lsp_types::CreateFilesParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.will_create_files(p)),
			)),
			WillDeleteFiles::METHOD => Some(message.from_value::<lsp_types::DeleteFilesParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.will_delete_files(p)),
			)),
			WillRenameFiles::METHOD => Some(message.from_value::<lsp_types::RenameFilesParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.will_rename_files(p)),
			)),
			WillSaveWaitUntil::METHOD => {
				Some(message.from_value::<lsp_types::WillSaveTextDocumentParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.will_save_wait_until(p)),
				))
			}
			WorkDoneProgressCreate::METHOD => {
				Some(message.from_value::<lsp_types::WorkDoneProgressCreateParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.work_done_progress_create(p)),
				))
			}
			WorkspaceConfiguration::METHOD => Some(message.from_value::<lsp_types::ConfigurationParams>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.workspace_configuration(p)),
			)),
			WorkspaceDiagnosticRefresh::METHOD => {
				Some(Message::from_result(id.clone(), self.workspace_diagnostic_refresh()))
			}
			WorkspaceDiagnosticRequest::METHOD => {
				Some(message.from_value::<lsp_types::WorkspaceDiagnosticParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.workspace_diagnostic_request(p)),
				))
			}
			WorkspaceFoldersRequest::METHOD => Some(Message::from_result(id.clone(), self.workspace_folders_request())),
			WorkspaceSymbolRequest::METHOD => {
				Some(message.from_value::<lsp_types::WorkspaceSymbolParams>().map_or_else(
					|_| self.invalid_params(id.clone()),
					|p| Message::from_result(id.clone(), self.workspace_symbol_request(p)),
				))
			}
			WorkspaceSymbolResolve::METHOD => Some(message.from_value::<lsp_types::WorkspaceSymbol>().map_or_else(
				|_| self.invalid_params(id.clone()),
				|p| Message::from_result(id.clone(), self.workspace_symbol_resolve(p)),
			)),

			// Notifications
			Cancel::METHOD => {
				message.from_value::<lsp_types::CancelParams>().map(|p| self.on_cancel(p)).ok();
				None
			}
			DidChangeConfiguration::METHOD => {
				message
					.from_value::<lsp_types::DidChangeConfigurationParams>()
					.map(|p| self.on_did_change_configuration(p))
					.ok();
				None
			}
			DidChangeNotebookDocument::METHOD => {
				message
					.from_value::<lsp_types::DidChangeNotebookDocumentParams>()
					.map(|p| self.on_did_change_notebook_document(p))
					.ok();
				None
			}
			DidChangeTextDocument::METHOD => {
				message
					.from_value::<lsp_types::DidChangeTextDocumentParams>()
					.map(|p| self.on_did_change_text_document(p))
					.ok();
				None
			}
			DidChangeWatchedFiles::METHOD => {
				message
					.from_value::<lsp_types::DidChangeWatchedFilesParams>()
					.map(|p| self.on_did_change_watched_files(p))
					.ok();
				None
			}
			DidChangeWorkspaceFolders::METHOD => {
				message
					.from_value::<lsp_types::DidChangeWorkspaceFoldersParams>()
					.map(|p| self.on_did_change_workspace_folders(p))
					.ok();
				None
			}
			DidCloseNotebookDocument::METHOD => {
				message
					.from_value::<lsp_types::DidCloseNotebookDocumentParams>()
					.map(|p| self.on_did_close_notebook_document(p))
					.ok();
				None
			}
			DidCloseTextDocument::METHOD => {
				message
					.from_value::<lsp_types::DidCloseTextDocumentParams>()
					.map(|p| self.on_did_close_text_document(p))
					.ok();
				None
			}
			DidCreateFiles::METHOD => {
				message.from_value::<lsp_types::CreateFilesParams>().map(|p| self.on_did_create_files(p)).ok();
				None
			}
			DidDeleteFiles::METHOD => {
				message.from_value::<lsp_types::DeleteFilesParams>().map(|p| self.on_did_delete_files(p)).ok();
				None
			}
			DidOpenNotebookDocument::METHOD => {
				message
					.from_value::<lsp_types::DidOpenNotebookDocumentParams>()
					.map(|p| self.on_did_open_notebook_document(p))
					.ok();
				None
			}
			DidOpenTextDocument::METHOD => {
				message
					.from_value::<lsp_types::DidOpenTextDocumentParams>()
					.map(|p| self.on_did_open_text_document(p))
					.ok();
				None
			}
			DidRenameFiles::METHOD => {
				message.from_value::<lsp_types::RenameFilesParams>().map(|p| self.on_did_rename_files(p)).ok();
				None
			}
			DidSaveNotebookDocument::METHOD => {
				message
					.from_value::<lsp_types::DidSaveNotebookDocumentParams>()
					.map(|p| self.on_did_save_notebook_document(p))
					.ok();
				None
			}
			DidSaveTextDocument::METHOD => {
				message
					.from_value::<lsp_types::DidSaveTextDocumentParams>()
					.map(|p| self.on_did_save_text_document(p))
					.ok();
				None
			}
			Initialized::METHOD => {
				message.from_value::<lsp_types::InitializedParams>().map(|p| self.on_initialized(p)).ok();
				None
			}
			LogMessage::METHOD => {
				message.from_value::<lsp_types::LogMessageParams>().map(|p| self.on_log_message(p)).ok();
				None
			}
			LogTrace::METHOD => {
				message.from_value::<lsp_types::LogTraceParams>().map(|p| self.on_log_trace(p)).ok();
				None
			}
			Progress::METHOD => {
				message.from_value::<lsp_types::ProgressParams>().map(|p| self.on_progress(p)).ok();
				None
			}
			PublishDiagnostics::METHOD => {
				message
					.from_value::<lsp_types::PublishDiagnosticsParams>()
					.map(|p| self.on_publish_diagnostics(p))
					.ok();
				None
			}
			SetTrace::METHOD => {
				message.from_value::<lsp_types::SetTraceParams>().map(|p| self.on_set_trace(p)).ok();
				None
			}
			ShowMessage::METHOD => {
				message.from_value::<lsp_types::ShowMessageParams>().map(|p| self.on_show_message(p)).ok();
				None
			}
			TelemetryEvent::METHOD => {
				message
					.from_value::<lsp_types::OneOf<lsp_types::LSPObject, lsp_types::LSPArray>>()
					.map(|p| self.on_telemetry_event(p))
					.ok();
				None
			}
			WillSaveTextDocument::METHOD => {
				message
					.from_value::<lsp_types::WillSaveTextDocumentParams>()
					.map(|p| self.on_will_save_text_document(p))
					.ok();
				None
			}
			WorkDoneProgressCancel::METHOD => {
				message
					.from_value::<lsp_types::WorkDoneProgressCancelParams>()
					.map(|p| self.on_work_done_progress_cance(p))
					.ok();
				None
			}
			_ => {
				if let Message::Request(crate::jsonrpc::Request { id, .. }) = message {
					Some(self.method_not_found(id.clone()))
				} else {
					None
				}
			}
		}
	}

	// Requests

	fn apply_workspace_edit(
		&self,
		_req: lsp_types::ApplyWorkspaceEditParams,
	) -> Result<lsp_types::ApplyWorkspaceEditResponse, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn call_hierarchy_incoming_calls(
		&self,
		_req: lsp_types::CallHierarchyIncomingCallsParams,
	) -> Result<Option<Vec<lsp_types::CallHierarchyIncomingCall>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn call_hierarchy_outgoing_calls(
		&self,
		_req: lsp_types::CallHierarchyOutgoingCallsParams,
	) -> Result<Option<Vec<lsp_types::CallHierarchyOutgoingCall>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn call_hierarchy_prepare(
		&self,
		_req: lsp_types::CallHierarchyPrepareParams,
	) -> Result<Option<Vec<lsp_types::CallHierarchyItem>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn code_action_request(
		&self,
		_req: lsp_types::CodeActionParams,
	) -> Result<Option<lsp_types::CodeActionResponse>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn code_action_resolve_request(&self, _req: lsp_types::CodeAction) -> Result<lsp_types::CodeAction, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn code_lens_refresh(&self) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn code_lens_request(
		&self,
		_req: lsp_types::CodeLensParams,
	) -> Result<Option<Vec<lsp_types::CodeLens>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn code_lens_resolve(&self, _req: lsp_types::CodeLens) -> Result<lsp_types::CodeLens, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn color_presentation_request(
		&self,
		_req: lsp_types::ColorPresentationParams,
	) -> Result<Vec<lsp_types::ColorPresentation>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn completion(
		&self,
		_req: lsp_types::CompletionParams,
	) -> Result<Option<lsp_types::CompletionResponse>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn document_color(
		&self,
		_req: lsp_types::DocumentColorParams,
	) -> Result<Vec<lsp_types::ColorInformation>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn document_diagnostic_request(
		&self,
		_req: lsp_types::DocumentDiagnosticParams,
	) -> Result<lsp_types::DocumentDiagnosticReportResult, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn document_highlight_request(
		&self,
		_req: lsp_types::DocumentHighlightParams,
	) -> Result<Option<Vec<lsp_types::DocumentHighlight>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn document_link_request(
		&self,
		_req: lsp_types::DocumentLinkParams,
	) -> Result<Option<Vec<lsp_types::DocumentLink>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn document_link_resolve(&self, _req: lsp_types::DocumentLink) -> Result<lsp_types::DocumentLink, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn document_symbol_request(
		&self,
		_req: lsp_types::DocumentSymbolParams,
	) -> Result<Option<lsp_types::DocumentSymbolResponse>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn execute_command(&self, _req: lsp_types::ExecuteCommandParams) -> Result<Option<Value>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn folding_range_request(
		&self,
		_req: lsp_types::FoldingRangeParams,
	) -> Result<Option<Vec<lsp_types::FoldingRange>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn formatting(
		&self,
		_req: lsp_types::DocumentFormattingParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn goto_declaration(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<lsp_types::GotoDefinitionResponse, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn goto_definition(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<lsp_types::GotoDefinitionResponse, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn goto_implementation(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<lsp_types::GotoDefinitionResponse, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn goto_type_definition(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<lsp_types::GotoDefinitionResponse, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn hover_request(&self, _req: lsp_types::HoverParams) -> Result<Option<lsp_types::Hover>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn initialize(&self, _req: lsp_types::InitializeParams) -> Result<lsp_types::InitializeResult, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn inlay_hint_refresh_request(&self) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn inlay_hint_request(
		&self,
		_req: lsp_types::InlayHintParams,
	) -> Result<Option<Vec<lsp_types::InlayHint>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn inlay_hint_resolve_request(&self, _req: lsp_types::InlayHint) -> Result<lsp_types::InlayHint, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn inline_value_refresh_request(&self) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn inline_value_request(
		&self,
		_req: lsp_types::InlineValueParams,
	) -> Result<Option<Vec<lsp_types::InlineValue>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn linked_editing_range(
		&self,
		_req: lsp_types::LinkedEditingRangeParams,
	) -> Result<Option<lsp_types::LinkedEditingRanges>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn moniker_request(&self, _req: lsp_types::MonikerParams) -> Result<Option<Vec<lsp_types::Moniker>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn on_type_formatting(
		&self,
		_req: lsp_types::DocumentOnTypeFormattingParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn prepare_rename_request(
		&self,
		_req: lsp_types::TextDocumentPositionParams,
	) -> Result<Option<lsp_types::PrepareRenameResponse>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn range_formatting(
		&self,
		_req: lsp_types::DocumentRangeFormattingParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn references(&self, _req: lsp_types::ReferenceParams) -> Result<Option<Vec<lsp_types::Location>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn register_capability(&self, _req: lsp_types::RegistrationParams) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn rename(&self, _req: lsp_types::RenameParams) -> Result<Option<lsp_types::WorkspaceEdit>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn resolve_completion_item(&self, _req: lsp_types::CompletionItem) -> Result<lsp_types::CompletionItem, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn selection_range_request(
		&self,
		_req: lsp_types::SelectionRangeParams,
	) -> Result<Option<Vec<lsp_types::SelectionRange>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn semantic_tokens_full_delta_request(
		&self,
		_req: lsp_types::SemanticTokensDeltaParams,
	) -> Result<Option<lsp_types::SemanticTokensFullDeltaResult>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn semantic_tokens_full_request(
		&self,
		_req: lsp_types::SemanticTokensParams,
	) -> Result<Option<lsp_types::SemanticTokensResult>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn semantic_tokens_range_request(
		&self,
		_req: lsp_types::SemanticTokensRangeParams,
	) -> Result<Option<lsp_types::SemanticTokensRangeResult>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn semantic_tokens_refresh(&self) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn show_document(&self, _req: lsp_types::ShowDocumentParams) -> Result<lsp_types::ShowDocumentResult, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn show_message_request(
		&self,
		_req: lsp_types::ShowMessageParams,
	) -> Result<Option<lsp_types::MessageActionItem>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn shutdown(&self) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn signature_help_request(
		&self,
		_req: lsp_types::SignatureHelpParams,
	) -> Result<Option<lsp_types::SignatureHelp>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn type_hierarchy_prepare(
		&self,
		_req: lsp_types::TypeHierarchyPrepareParams,
	) -> Result<Option<Vec<lsp_types::TypeHierarchyItem>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn type_hierarchy_subtypes(
		&self,
		_req: lsp_types::TypeHierarchySubtypesParams,
	) -> Result<Option<Vec<lsp_types::TypeHierarchyItem>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn type_hierarchy_supertypes(
		&self,
		_req: lsp_types::TypeHierarchySupertypesParams,
	) -> Result<Option<Vec<lsp_types::TypeHierarchyItem>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn unregister_capability(&self, _req: lsp_types::UnregistrationParams) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn will_create_files(
		&self,
		_req: lsp_types::CreateFilesParams,
	) -> Result<Option<lsp_types::WorkspaceEdit>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn will_delete_files(
		&self,
		_req: lsp_types::DeleteFilesParams,
	) -> Result<Option<lsp_types::WorkspaceEdit>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn will_rename_files(
		&self,
		_req: lsp_types::RenameFilesParams,
	) -> Result<Option<lsp_types::WorkspaceEdit>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn will_save_wait_until(
		&self,
		_req: lsp_types::WillSaveTextDocumentParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn work_done_progress_create(&self, _req: lsp_types::WorkDoneProgressCreateParams) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn workspace_configuration(&self, _req: lsp_types::ConfigurationParams) -> Result<Vec<Value>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn workspace_diagnostic_refresh(&self) -> Result<(), ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn workspace_diagnostic_request(
		&self,
		_req: lsp_types::WorkspaceDiagnosticParams,
	) -> Result<lsp_types::WorkspaceDiagnosticReportResult, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn workspace_folders_request(&self) -> Result<Option<Vec<lsp_types::WorkspaceFolder>>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn workspace_symbol_request(
		&self,
		_req: lsp_types::WorkspaceSymbolParams,
	) -> Result<Option<lsp_types::WorkspaceSymbolResponse>, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	fn workspace_symbol_resolve(
		&self,
		_req: lsp_types::WorkspaceSymbol,
	) -> Result<lsp_types::WorkspaceSymbol, ErrorCode> {
		Err(ErrorCode::MethodNotFound)
	}

	// Notifications

	fn on_cancel(&self, _req: lsp_types::CancelParams) {}
	fn on_did_change_configuration(&self, _req: lsp_types::DidChangeConfigurationParams) {}
	fn on_did_change_notebook_document(&self, _req: lsp_types::DidChangeNotebookDocumentParams) {}
	fn on_did_change_text_document(&self, _req: lsp_types::DidChangeTextDocumentParams) {}
	fn on_did_change_watched_files(&self, _req: lsp_types::DidChangeWatchedFilesParams) {}
	fn on_did_change_workspace_folders(&self, _req: lsp_types::DidChangeWorkspaceFoldersParams) {}
	fn on_did_close_notebook_document(&self, _req: lsp_types::DidCloseNotebookDocumentParams) {}
	fn on_did_close_text_document(&self, _req: lsp_types::DidCloseTextDocumentParams) {}
	fn on_did_create_files(&self, _req: lsp_types::CreateFilesParams) {}
	fn on_did_delete_files(&self, _req: lsp_types::DeleteFilesParams) {}
	fn on_did_open_notebook_document(&self, _req: lsp_types::DidOpenNotebookDocumentParams) {}
	fn on_did_open_text_document(&self, _req: lsp_types::DidOpenTextDocumentParams) {}
	fn on_did_rename_files(&self, _req: lsp_types::RenameFilesParams) {}
	fn on_did_save_notebook_document(&self, _req: lsp_types::DidSaveNotebookDocumentParams) {}
	fn on_did_save_text_document(&self, _req: lsp_types::DidSaveTextDocumentParams) {}
	fn on_initialized(&self, _req: lsp_types::InitializedParams) {}
	fn on_log_message(&self, _req: lsp_types::LogMessageParams) {}
	fn on_log_trace(&self, _req: lsp_types::LogTraceParams) {}
	fn on_progress(&self, _req: lsp_types::ProgressParams) {}
	fn on_publish_diagnostics(&self, _req: lsp_types::PublishDiagnosticsParams) {}
	fn on_set_trace(&self, _req: lsp_types::SetTraceParams) {}
	fn on_show_message(&self, _req: lsp_types::ShowMessageParams) {}
	fn on_telemetry_event(&self, _req: lsp_types::OneOf<lsp_types::LSPObject, lsp_types::LSPArray>) {}
	fn on_will_save_text_document(&self, _req: lsp_types::WillSaveTextDocumentParams) {}
	fn on_work_done_progress_cance(&self, _req: lsp_types::WorkDoneProgressCancelParams) {}
}
