use crossbeam_channel::{Receiver, Sender, bounded};
use css_ast::{CssAtomSet, StyleSheet, Visitable};
use css_lexer::Lexer;
use css_parse::{Arena, Diagnostic, Parser, ParserReturn, Severity};
use csskit_highlight::{SemanticKind, SemanticModifier, TokenHighlighter};
use dashmap::DashMap;
use itertools::Itertools;
use lsp_service::{Client, ErrorCode, Handler, PositionEncoding, ResponseError};
use lsp_types::{SemanticToken, Uri, notification::PublishDiagnostics};
use ropey::Rope;
use std::{
	sync::{Arc, OnceLock},
	thread::{Builder, JoinHandle},
};
use strum::VariantNames;
use tracing::{instrument, trace, trace_span, warn};

#[derive(Debug)]
enum FileCall {
	RopeChange(Rope, i32),
	Highlight,
}

#[derive(Debug)]
enum FileReturn {
	Tokens(Vec<SemanticToken>),
}

#[derive(Debug)]
pub struct File {
	pub content: Rope,
	#[allow(dead_code)]
	thread: JoinHandle<()>,
	sender: Sender<FileCall>,
	receiver: Receiver<FileReturn>,
}

impl File {
	fn new(uri: Uri, encoding: PositionEncoding, client: Client) -> Self {
		let (sender, read_receiver) = bounded::<FileCall>(0);
		let (write_sender, receiver) = bounded::<FileReturn>(0);
		Self {
			content: Rope::new(),
			sender,
			receiver,
			thread: Builder::new()
				.name("LspDocumentHandler".into())
				.spawn(move || {
					let mut alloc = Arena::default();
					let mut rope = Rope::new();
					let mut string: String = "".into();
					let lexer = Lexer::new(&CssAtomSet::ATOMS, "");
					let mut result: ParserReturn<'_, StyleSheet<'_>> =
						Parser::new(&alloc, "", lexer).parse_entirely::<StyleSheet>();
					while let Ok(call) = read_receiver.recv() {
						match call {
							FileCall::RopeChange(new_rope, version) => {
								let span = trace_span!("Parsing document");
								let _ = span.enter();
								// TODO! we should be able to optimize this by parsing a subset of the tree and mutating in
								// place. For now though a partial parse request re-parses it all.
								alloc.reset();
								rope = new_rope;
								string = String::from(&rope);
								let lexer = Lexer::new(&CssAtomSet::ATOMS, &string);
								result = Parser::new(&alloc, &string, lexer).parse_entirely::<StyleSheet>();
								// if let Some(stylesheet) = &result.output {
								// 	trace!("Sucessfully parsed stylesheet: {:#?}", &stylesheet);
								// }
								let diagnostics = result
									.errors
									.iter()
									.map(|diagnostic| to_lsp_diagnostic(diagnostic, &string, &rope, encoding))
									.collect();
								let params = lsp_types::PublishDiagnosticsParams {
									uri: uri.clone(),
									diagnostics,
									version: Some(version),
								};
								if let Err(e) = client.notify::<PublishDiagnostics>(params) {
									warn!("Failed to publish diagnostics, connection closed: {e:?}");
								}
							}
							FileCall::Highlight => {
								let span = trace_span!("Highlighting document");
								let _ = span.enter();
								let mut highlighter = TokenHighlighter::new();
								let mut tokens = vec![];
								if let Some(stylesheet) = &result.output {
									let _ = stylesheet.accept(&mut highlighter);
									let mut current_line = 0;
									let mut current_start = 0;
									tokens = highlighter
										.highlights()
										.sorted_by(|a, b| Ord::cmp(&a.span(), &b.span()))
										.map(|h| {
											let span = h.span();
											let start =
												encoding.position(&rope, rope.byte_to_char(span.start().into()));
											let delta_line = start.line - current_line;
											let delta_start = if delta_line == 0 {
												start.character - current_start
											} else {
												start.character
											};
											current_line = start.line;
											current_start = start.character;
											SemanticToken {
												delta_line,
												delta_start,
												length: encoding.len_of(span.str_slice(&string)),
												token_type: h.kind().bits() as u32,
												token_modifiers_bitset: h.modifier().bits() as u32,
											}
										})
										.collect();
								}
								write_sender.send(FileReturn::Tokens(tokens)).ok();
							}
						}
					}
				})
				.expect("Failed to document thread Reader"),
		}
	}

	fn commit(&mut self, rope: Rope, version: i32) {
		self.content = rope;
		self.sender.send(FileCall::RopeChange(self.content.clone(), version)).unwrap();
	}

	#[instrument]
	fn get_tokens(&self) -> Vec<SemanticToken> {
		self.sender.send(FileCall::Highlight).unwrap();
		if let Ok(FileReturn::Tokens(tokens)) = self.receiver.recv() {
			return tokens;
		}
		vec![]
	}
}

fn to_lsp_diagnostic(
	diagnostic: &Diagnostic,
	source: &str,
	rope: &Rope,
	encoding: PositionEncoding,
) -> lsp_types::Diagnostic {
	let span = diagnostic.span();
	let meta = diagnostic.meta(source);
	let position = |offset: usize| encoding.position(rope, rope.byte_to_char(offset.min(source.len())));
	lsp_types::Diagnostic {
		range: lsp_types::Range::new(position(span.start().into()), position(span.end().into())),
		severity: Some(match diagnostic.severity {
			Severity::Error => lsp_types::DiagnosticSeverity::ERROR,
			Severity::Warning => lsp_types::DiagnosticSeverity::WARNING,
			Severity::Advice => lsp_types::DiagnosticSeverity::INFORMATION,
		}),
		code: Some(lsp_types::NumberOrString::String(meta.code.into())),
		source: Some("csskit".into()),
		message: meta.message,
		..Default::default()
	}
}

#[derive(Debug)]
pub struct LSPService {
	version: String,
	client: Client,
	files: Arc<DashMap<Uri, File>>,
	encoding: OnceLock<PositionEncoding>,
}

impl LSPService {
	pub fn new(version: &'static str, client: Client) -> Self {
		Self { version: version.into(), client, files: Arc::new(DashMap::new()), encoding: OnceLock::new() }
	}

	fn encoding(&self) -> PositionEncoding {
		self.encoding.get().copied().unwrap_or_default()
	}
}

impl Handler for LSPService {
	#[instrument]
	fn initialized(&self) -> bool {
		self.encoding.get().is_some()
	}

	#[instrument]
	fn initialize(&self, req: lsp_types::InitializeParams) -> Result<lsp_types::InitializeResult, ResponseError> {
		let encoding = *self.encoding.get_or_init(|| PositionEncoding::negotiate(&req.capabilities));
		Ok(lsp_types::InitializeResult {
			capabilities: lsp_types::ServerCapabilities {
				position_encoding: Some(encoding.kind()),
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
				name: String::from("csskit-lsp"),
				version: Some(self.version.clone()),
			}),
			offset_encoding: None,
		})
	}

	#[instrument]
	fn semantic_tokens_full_request(
		&self,
		req: lsp_types::SemanticTokensParams,
	) -> Result<Option<lsp_types::SemanticTokensResult>, ResponseError> {
		let uri = req.text_document.uri;
		trace!("Asked for SemanticTokens for {:?}", &uri);
		if let Some(document) = self.files.get(&uri) {
			let data = document.get_tokens();
			Ok(Some(lsp_types::SemanticTokensResult::Tokens(lsp_types::SemanticTokens { result_id: None, data })))
		} else {
			Err(ErrorCode::InternalError.into())
		}
	}

	#[instrument]
	fn completion(
		&self,
		req: lsp_types::CompletionParams,
	) -> Result<Option<lsp_types::CompletionResponse>, ResponseError> {
		// let uri = req.text_document_position.text_document.uri;
		// let position = req.text_document_position.position;
		// let context = req.context;
		Ok(None)
	}

	#[instrument]
	fn on_did_open_text_document(&self, req: lsp_types::DidOpenTextDocumentParams) {
		let uri = req.text_document.uri;
		let mut doc = File::new(uri.clone(), self.encoding(), self.client.clone());
		let rope = Rope::from_str(&req.text_document.text);
		trace!("comitting new document {:?} {:?}", &uri, rope);
		doc.commit(rope, req.text_document.version);
		self.files.clone().insert(uri, doc);
	}

	#[instrument]
	fn on_did_change_text_document(&self, req: lsp_types::DidChangeTextDocumentParams) {
		let uri = req.text_document.uri;
		let encoding = self.encoding();
		if let Some(mut file) = self.files.clone().get_mut(&uri) {
			let mut rope = file.content.clone();
			for change in req.content_changes {
				let (start, end) = match change.range {
					Some(range) => (encoding.char_index(&rope, range.start), encoding.char_index(&rope, range.end)),
					None => (0, rope.len_chars()),
				};
				rope.remove(start..end.max(start));
				rope.insert(start, &change.text);
			}
			file.commit(rope, req.text_document.version)
		}
	}
}
