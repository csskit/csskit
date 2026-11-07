use crate::jsonrpc::{ErrorCode, Id, Message, Notification, Request, ResponseError};
use lsp_types::{
	notification::{Notification as _, *},
	request::{Request as _, *},
};
use serde_json::{Value, from_value};
use tracing::{debug, trace_span, warn};

/// Answers `id` with what `answer` makes of `params` read as the parameters of `R`. Typing `answer` by `R` holds each
/// [`Handler`] method to the parameters and result lsp-types gives its request.
fn respond<R: lsp_types::request::Request>(
	id: Id,
	params: Value,
	answer: impl FnOnce(R::Params) -> Result<R::Result, ResponseError>,
) -> Message {
	match from_value(params) {
		Ok(params) => Message::from_result(id, answer(params)),
		Err(error) => {
			Message::from_result(id, Err::<(), _>(ResponseError::new(ErrorCode::InvalidParams, error.to_string())))
		}
	}
}

/// Hands `params`, read as the parameters of `N`, to `receive`.
fn notified<N: lsp_types::notification::Notification>(params: Value, receive: impl FnOnce(N::Params)) {
	match from_value(params) {
		Ok(params) => receive(params),
		Err(error) => warn!("Ignoring {} with invalid params: {error}", N::METHOD),
	}
}

fn answer<H: Handler>(handler: &H, id: Id, method: &str, params: Value) -> Message {
	match method {
		ApplyWorkspaceEdit::METHOD => respond::<ApplyWorkspaceEdit>(id, params, |p| handler.apply_workspace_edit(p)),
		CallHierarchyIncomingCalls::METHOD => {
			respond::<CallHierarchyIncomingCalls>(id, params, |p| handler.call_hierarchy_incoming_calls(p))
		}
		CallHierarchyOutgoingCalls::METHOD => {
			respond::<CallHierarchyOutgoingCalls>(id, params, |p| handler.call_hierarchy_outgoing_calls(p))
		}
		CallHierarchyPrepare::METHOD => {
			respond::<CallHierarchyPrepare>(id, params, |p| handler.call_hierarchy_prepare(p))
		}
		CodeActionRequest::METHOD => respond::<CodeActionRequest>(id, params, |p| handler.code_action_request(p)),
		CodeActionResolveRequest::METHOD => {
			respond::<CodeActionResolveRequest>(id, params, |p| handler.code_action_resolve_request(p))
		}
		CodeLensRefresh::METHOD => respond::<CodeLensRefresh>(id, Value::Null, |()| handler.code_lens_refresh()),
		CodeLensRequest::METHOD => respond::<CodeLensRequest>(id, params, |p| handler.code_lens_request(p)),
		CodeLensResolve::METHOD => respond::<CodeLensResolve>(id, params, |p| handler.code_lens_resolve(p)),
		ColorPresentationRequest::METHOD => {
			respond::<ColorPresentationRequest>(id, params, |p| handler.color_presentation_request(p))
		}
		Completion::METHOD => respond::<Completion>(id, params, |p| handler.completion(p)),
		DocumentColor::METHOD => respond::<DocumentColor>(id, params, |p| handler.document_color(p)),
		DocumentDiagnosticRequest::METHOD => {
			respond::<DocumentDiagnosticRequest>(id, params, |p| handler.document_diagnostic_request(p))
		}
		DocumentHighlightRequest::METHOD => {
			respond::<DocumentHighlightRequest>(id, params, |p| handler.document_highlight_request(p))
		}
		DocumentLinkRequest::METHOD => respond::<DocumentLinkRequest>(id, params, |p| handler.document_link_request(p)),
		DocumentLinkResolve::METHOD => respond::<DocumentLinkResolve>(id, params, |p| handler.document_link_resolve(p)),
		DocumentSymbolRequest::METHOD => {
			respond::<DocumentSymbolRequest>(id, params, |p| handler.document_symbol_request(p))
		}
		ExecuteCommand::METHOD => respond::<ExecuteCommand>(id, params, |p| handler.execute_command(p)),
		FoldingRangeRequest::METHOD => respond::<FoldingRangeRequest>(id, params, |p| handler.folding_range_request(p)),
		Formatting::METHOD => respond::<Formatting>(id, params, |p| handler.formatting(p)),
		GotoDeclaration::METHOD => respond::<GotoDeclaration>(id, params, |p| handler.goto_declaration(p)),
		GotoDefinition::METHOD => respond::<GotoDefinition>(id, params, |p| handler.goto_definition(p)),
		GotoImplementation::METHOD => respond::<GotoImplementation>(id, params, |p| handler.goto_implementation(p)),
		GotoTypeDefinition::METHOD => respond::<GotoTypeDefinition>(id, params, |p| handler.goto_type_definition(p)),
		HoverRequest::METHOD => respond::<HoverRequest>(id, params, |p| handler.hover_request(p)),
		Initialize::METHOD => respond::<Initialize>(id, params, |p| handler.initialize(p)),
		InlayHintRefreshRequest::METHOD => {
			respond::<InlayHintRefreshRequest>(id, Value::Null, |()| handler.inlay_hint_refresh_request())
		}
		InlayHintRequest::METHOD => respond::<InlayHintRequest>(id, params, |p| handler.inlay_hint_request(p)),
		InlayHintResolveRequest::METHOD => {
			respond::<InlayHintResolveRequest>(id, params, |p| handler.inlay_hint_resolve_request(p))
		}
		InlineValueRefreshRequest::METHOD => {
			respond::<InlineValueRefreshRequest>(id, Value::Null, |()| handler.inline_value_refresh_request())
		}
		InlineValueRequest::METHOD => respond::<InlineValueRequest>(id, params, |p| handler.inline_value_request(p)),
		LinkedEditingRange::METHOD => respond::<LinkedEditingRange>(id, params, |p| handler.linked_editing_range(p)),
		MonikerRequest::METHOD => respond::<MonikerRequest>(id, params, |p| handler.moniker_request(p)),
		OnTypeFormatting::METHOD => respond::<OnTypeFormatting>(id, params, |p| handler.on_type_formatting(p)),
		PrepareRenameRequest::METHOD => {
			respond::<PrepareRenameRequest>(id, params, |p| handler.prepare_rename_request(p))
		}
		RangeFormatting::METHOD => respond::<RangeFormatting>(id, params, |p| handler.range_formatting(p)),
		References::METHOD => respond::<References>(id, params, |p| handler.references(p)),
		RegisterCapability::METHOD => respond::<RegisterCapability>(id, params, |p| handler.register_capability(p)),
		Rename::METHOD => respond::<Rename>(id, params, |p| handler.rename(p)),
		ResolveCompletionItem::METHOD => {
			respond::<ResolveCompletionItem>(id, params, |p| handler.resolve_completion_item(p))
		}
		SelectionRangeRequest::METHOD => {
			respond::<SelectionRangeRequest>(id, params, |p| handler.selection_range_request(p))
		}
		SemanticTokensFullDeltaRequest::METHOD => {
			respond::<SemanticTokensFullDeltaRequest>(id, params, |p| handler.semantic_tokens_full_delta_request(p))
		}
		SemanticTokensFullRequest::METHOD => {
			respond::<SemanticTokensFullRequest>(id, params, |p| handler.semantic_tokens_full_request(p))
		}
		SemanticTokensRangeRequest::METHOD => {
			respond::<SemanticTokensRangeRequest>(id, params, |p| handler.semantic_tokens_range_request(p))
		}
		SemanticTokensRefresh::METHOD => {
			respond::<SemanticTokensRefresh>(id, Value::Null, |()| handler.semantic_tokens_refresh())
		}
		ShowDocument::METHOD => respond::<ShowDocument>(id, params, |p| handler.show_document(p)),
		ShowMessageRequest::METHOD => respond::<ShowMessageRequest>(id, params, |p| handler.show_message_request(p)),
		Shutdown::METHOD => respond::<Shutdown>(id, Value::Null, |()| handler.shutdown()),
		SignatureHelpRequest::METHOD => {
			respond::<SignatureHelpRequest>(id, params, |p| handler.signature_help_request(p))
		}
		TypeHierarchyPrepare::METHOD => {
			respond::<TypeHierarchyPrepare>(id, params, |p| handler.type_hierarchy_prepare(p))
		}
		TypeHierarchySubtypes::METHOD => {
			respond::<TypeHierarchySubtypes>(id, params, |p| handler.type_hierarchy_subtypes(p))
		}
		TypeHierarchySupertypes::METHOD => {
			respond::<TypeHierarchySupertypes>(id, params, |p| handler.type_hierarchy_supertypes(p))
		}
		UnregisterCapability::METHOD => {
			respond::<UnregisterCapability>(id, params, |p| handler.unregister_capability(p))
		}
		WillCreateFiles::METHOD => respond::<WillCreateFiles>(id, params, |p| handler.will_create_files(p)),
		WillDeleteFiles::METHOD => respond::<WillDeleteFiles>(id, params, |p| handler.will_delete_files(p)),
		WillRenameFiles::METHOD => respond::<WillRenameFiles>(id, params, |p| handler.will_rename_files(p)),
		WillSaveWaitUntil::METHOD => respond::<WillSaveWaitUntil>(id, params, |p| handler.will_save_wait_until(p)),
		WorkDoneProgressCreate::METHOD => {
			respond::<WorkDoneProgressCreate>(id, params, |p| handler.work_done_progress_create(p))
		}
		WorkspaceConfiguration::METHOD => {
			respond::<WorkspaceConfiguration>(id, params, |p| handler.workspace_configuration(p))
		}
		WorkspaceDiagnosticRefresh::METHOD => {
			respond::<WorkspaceDiagnosticRefresh>(id, Value::Null, |()| handler.workspace_diagnostic_refresh())
		}
		WorkspaceDiagnosticRequest::METHOD => {
			respond::<WorkspaceDiagnosticRequest>(id, params, |p| handler.workspace_diagnostic_request(p))
		}
		WorkspaceFoldersRequest::METHOD => {
			respond::<WorkspaceFoldersRequest>(id, Value::Null, |()| handler.workspace_folders_request())
		}
		WorkspaceSymbolRequest::METHOD => {
			respond::<WorkspaceSymbolRequest>(id, params, |p| handler.workspace_symbol_request(p))
		}
		WorkspaceSymbolResolve::METHOD => {
			respond::<WorkspaceSymbolResolve>(id, params, |p| handler.workspace_symbol_resolve(p))
		}
		_ => Message::from_result(id, Err::<(), _>(ErrorCode::MethodNotFound.into())),
	}
}

fn receive<H: Handler>(handler: &H, method: &str, params: Value) {
	match method {
		Cancel::METHOD => notified::<Cancel>(params, |p| handler.on_cancel(p)),
		DidChangeConfiguration::METHOD => {
			notified::<DidChangeConfiguration>(params, |p| handler.on_did_change_configuration(p))
		}
		DidChangeNotebookDocument::METHOD => {
			notified::<DidChangeNotebookDocument>(params, |p| handler.on_did_change_notebook_document(p))
		}
		DidChangeTextDocument::METHOD => {
			notified::<DidChangeTextDocument>(params, |p| handler.on_did_change_text_document(p))
		}
		DidChangeWatchedFiles::METHOD => {
			notified::<DidChangeWatchedFiles>(params, |p| handler.on_did_change_watched_files(p))
		}
		DidChangeWorkspaceFolders::METHOD => {
			notified::<DidChangeWorkspaceFolders>(params, |p| handler.on_did_change_workspace_folders(p))
		}
		DidCloseNotebookDocument::METHOD => {
			notified::<DidCloseNotebookDocument>(params, |p| handler.on_did_close_notebook_document(p))
		}
		DidCloseTextDocument::METHOD => {
			notified::<DidCloseTextDocument>(params, |p| handler.on_did_close_text_document(p))
		}
		DidCreateFiles::METHOD => notified::<DidCreateFiles>(params, |p| handler.on_did_create_files(p)),
		DidDeleteFiles::METHOD => notified::<DidDeleteFiles>(params, |p| handler.on_did_delete_files(p)),
		DidOpenNotebookDocument::METHOD => {
			notified::<DidOpenNotebookDocument>(params, |p| handler.on_did_open_notebook_document(p))
		}
		DidOpenTextDocument::METHOD => {
			notified::<DidOpenTextDocument>(params, |p| handler.on_did_open_text_document(p))
		}
		DidRenameFiles::METHOD => notified::<DidRenameFiles>(params, |p| handler.on_did_rename_files(p)),
		DidSaveNotebookDocument::METHOD => {
			notified::<DidSaveNotebookDocument>(params, |p| handler.on_did_save_notebook_document(p))
		}
		DidSaveTextDocument::METHOD => {
			notified::<DidSaveTextDocument>(params, |p| handler.on_did_save_text_document(p))
		}
		Initialized::METHOD => notified::<Initialized>(params, |p| handler.on_initialized(p)),
		LogMessage::METHOD => notified::<LogMessage>(params, |p| handler.on_log_message(p)),
		LogTrace::METHOD => notified::<LogTrace>(params, |p| handler.on_log_trace(p)),
		Progress::METHOD => notified::<Progress>(params, |p| handler.on_progress(p)),
		PublishDiagnostics::METHOD => notified::<PublishDiagnostics>(params, |p| handler.on_publish_diagnostics(p)),
		SetTrace::METHOD => notified::<SetTrace>(params, |p| handler.on_set_trace(p)),
		ShowMessage::METHOD => notified::<ShowMessage>(params, |p| handler.on_show_message(p)),
		TelemetryEvent::METHOD => notified::<TelemetryEvent>(params, |p| handler.on_telemetry_event(p)),
		WillSaveTextDocument::METHOD => {
			notified::<WillSaveTextDocument>(params, |p| handler.on_will_save_text_document(p))
		}
		WorkDoneProgressCancel::METHOD => {
			notified::<WorkDoneProgressCancel>(params, |p| handler.on_work_done_progress_cance(p))
		}
		_ => debug!("Ignoring unhandled notification {method}"),
	}
}

pub trait Handler: Sized + Send + Sync + 'static {
	fn initialized(&self) -> bool {
		false
	}

	fn handle(&self, message: Message) -> Option<Message> {
		let span = trace_span!("Handling request", "{:#?}", message);
		let _entered = span.enter();
		if message.is_exit_notification() {
			return None;
		}
		if !self.initialized() && !message.is_initialize_request() {
			debug!("Skipping message {:?} before initialization", message);
			return None;
		}
		match message {
			Message::Request(Request { id, method, params }) => Some(answer(self, id, &method, params)),
			Message::Notification(Notification { method, params }) => {
				receive(self, &method, params);
				None
			}
			Message::Response(_) => None,
		}
	}

	// Requests

	fn apply_workspace_edit(
		&self,
		_req: lsp_types::ApplyWorkspaceEditParams,
	) -> Result<lsp_types::ApplyWorkspaceEditResponse, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn call_hierarchy_incoming_calls(
		&self,
		_req: lsp_types::CallHierarchyIncomingCallsParams,
	) -> Result<Option<Vec<lsp_types::CallHierarchyIncomingCall>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn call_hierarchy_outgoing_calls(
		&self,
		_req: lsp_types::CallHierarchyOutgoingCallsParams,
	) -> Result<Option<Vec<lsp_types::CallHierarchyOutgoingCall>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn call_hierarchy_prepare(
		&self,
		_req: lsp_types::CallHierarchyPrepareParams,
	) -> Result<Option<Vec<lsp_types::CallHierarchyItem>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn code_action_request(
		&self,
		_req: lsp_types::CodeActionParams,
	) -> Result<Option<lsp_types::CodeActionResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn code_action_resolve_request(&self, _req: lsp_types::CodeAction) -> Result<lsp_types::CodeAction, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn code_lens_refresh(&self) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn code_lens_request(
		&self,
		_req: lsp_types::CodeLensParams,
	) -> Result<Option<Vec<lsp_types::CodeLens>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn code_lens_resolve(&self, _req: lsp_types::CodeLens) -> Result<lsp_types::CodeLens, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn color_presentation_request(
		&self,
		_req: lsp_types::ColorPresentationParams,
	) -> Result<Vec<lsp_types::ColorPresentation>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn completion(
		&self,
		_req: lsp_types::CompletionParams,
	) -> Result<Option<lsp_types::CompletionResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn document_color(
		&self,
		_req: lsp_types::DocumentColorParams,
	) -> Result<Vec<lsp_types::ColorInformation>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn document_diagnostic_request(
		&self,
		_req: lsp_types::DocumentDiagnosticParams,
	) -> Result<lsp_types::DocumentDiagnosticReportResult, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn document_highlight_request(
		&self,
		_req: lsp_types::DocumentHighlightParams,
	) -> Result<Option<Vec<lsp_types::DocumentHighlight>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn document_link_request(
		&self,
		_req: lsp_types::DocumentLinkParams,
	) -> Result<Option<Vec<lsp_types::DocumentLink>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn document_link_resolve(&self, _req: lsp_types::DocumentLink) -> Result<lsp_types::DocumentLink, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn document_symbol_request(
		&self,
		_req: lsp_types::DocumentSymbolParams,
	) -> Result<Option<lsp_types::DocumentSymbolResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn execute_command(&self, _req: lsp_types::ExecuteCommandParams) -> Result<Option<Value>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn folding_range_request(
		&self,
		_req: lsp_types::FoldingRangeParams,
	) -> Result<Option<Vec<lsp_types::FoldingRange>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn formatting(
		&self,
		_req: lsp_types::DocumentFormattingParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn goto_declaration(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<Option<lsp_types::GotoDefinitionResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn goto_definition(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<Option<lsp_types::GotoDefinitionResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn goto_implementation(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<Option<lsp_types::GotoDefinitionResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn goto_type_definition(
		&self,
		_req: lsp_types::GotoDefinitionParams,
	) -> Result<Option<lsp_types::GotoDefinitionResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn hover_request(&self, _req: lsp_types::HoverParams) -> Result<Option<lsp_types::Hover>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn initialize(&self, _req: lsp_types::InitializeParams) -> Result<lsp_types::InitializeResult, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn inlay_hint_refresh_request(&self) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn inlay_hint_request(
		&self,
		_req: lsp_types::InlayHintParams,
	) -> Result<Option<Vec<lsp_types::InlayHint>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn inlay_hint_resolve_request(&self, _req: lsp_types::InlayHint) -> Result<lsp_types::InlayHint, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn inline_value_refresh_request(&self) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn inline_value_request(
		&self,
		_req: lsp_types::InlineValueParams,
	) -> Result<Option<Vec<lsp_types::InlineValue>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn linked_editing_range(
		&self,
		_req: lsp_types::LinkedEditingRangeParams,
	) -> Result<Option<lsp_types::LinkedEditingRanges>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn moniker_request(
		&self,
		_req: lsp_types::MonikerParams,
	) -> Result<Option<Vec<lsp_types::Moniker>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn on_type_formatting(
		&self,
		_req: lsp_types::DocumentOnTypeFormattingParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn prepare_rename_request(
		&self,
		_req: lsp_types::TextDocumentPositionParams,
	) -> Result<Option<lsp_types::PrepareRenameResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn range_formatting(
		&self,
		_req: lsp_types::DocumentRangeFormattingParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn references(&self, _req: lsp_types::ReferenceParams) -> Result<Option<Vec<lsp_types::Location>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn register_capability(&self, _req: lsp_types::RegistrationParams) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn rename(&self, _req: lsp_types::RenameParams) -> Result<Option<lsp_types::WorkspaceEdit>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn resolve_completion_item(
		&self,
		_req: lsp_types::CompletionItem,
	) -> Result<lsp_types::CompletionItem, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn selection_range_request(
		&self,
		_req: lsp_types::SelectionRangeParams,
	) -> Result<Option<Vec<lsp_types::SelectionRange>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn semantic_tokens_full_delta_request(
		&self,
		_req: lsp_types::SemanticTokensDeltaParams,
	) -> Result<Option<lsp_types::SemanticTokensFullDeltaResult>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn semantic_tokens_full_request(
		&self,
		_req: lsp_types::SemanticTokensParams,
	) -> Result<Option<lsp_types::SemanticTokensResult>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn semantic_tokens_range_request(
		&self,
		_req: lsp_types::SemanticTokensRangeParams,
	) -> Result<Option<lsp_types::SemanticTokensRangeResult>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn semantic_tokens_refresh(&self) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn show_document(
		&self,
		_req: lsp_types::ShowDocumentParams,
	) -> Result<lsp_types::ShowDocumentResult, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn show_message_request(
		&self,
		_req: lsp_types::ShowMessageRequestParams,
	) -> Result<Option<lsp_types::MessageActionItem>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn shutdown(&self) -> Result<(), ResponseError> {
		Ok(())
	}

	fn signature_help_request(
		&self,
		_req: lsp_types::SignatureHelpParams,
	) -> Result<Option<lsp_types::SignatureHelp>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn type_hierarchy_prepare(
		&self,
		_req: lsp_types::TypeHierarchyPrepareParams,
	) -> Result<Option<Vec<lsp_types::TypeHierarchyItem>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn type_hierarchy_subtypes(
		&self,
		_req: lsp_types::TypeHierarchySubtypesParams,
	) -> Result<Option<Vec<lsp_types::TypeHierarchyItem>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn type_hierarchy_supertypes(
		&self,
		_req: lsp_types::TypeHierarchySupertypesParams,
	) -> Result<Option<Vec<lsp_types::TypeHierarchyItem>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn unregister_capability(&self, _req: lsp_types::UnregistrationParams) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn will_create_files(
		&self,
		_req: lsp_types::CreateFilesParams,
	) -> Result<Option<lsp_types::WorkspaceEdit>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn will_delete_files(
		&self,
		_req: lsp_types::DeleteFilesParams,
	) -> Result<Option<lsp_types::WorkspaceEdit>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn will_rename_files(
		&self,
		_req: lsp_types::RenameFilesParams,
	) -> Result<Option<lsp_types::WorkspaceEdit>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn will_save_wait_until(
		&self,
		_req: lsp_types::WillSaveTextDocumentParams,
	) -> Result<Option<Vec<lsp_types::TextEdit>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn work_done_progress_create(&self, _req: lsp_types::WorkDoneProgressCreateParams) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn workspace_configuration(&self, _req: lsp_types::ConfigurationParams) -> Result<Vec<Value>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn workspace_diagnostic_refresh(&self) -> Result<(), ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn workspace_diagnostic_request(
		&self,
		_req: lsp_types::WorkspaceDiagnosticParams,
	) -> Result<lsp_types::WorkspaceDiagnosticReportResult, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn workspace_folders_request(&self) -> Result<Option<Vec<lsp_types::WorkspaceFolder>>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn workspace_symbol_request(
		&self,
		_req: lsp_types::WorkspaceSymbolParams,
	) -> Result<Option<lsp_types::WorkspaceSymbolResponse>, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
	}

	fn workspace_symbol_resolve(
		&self,
		_req: lsp_types::WorkspaceSymbol,
	) -> Result<lsp_types::WorkspaceSymbol, ResponseError> {
		Err(ErrorCode::MethodNotFound.into())
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
