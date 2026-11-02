use csskit_lsp::LSPService;
use lsp_service::{Message, Notification, Request, Response, Server};
use lsp_types::{
	ClientCapabilities, DidChangeTextDocumentParams, DidOpenTextDocumentParams, GeneralClientCapabilities,
	InitializeParams, Position, PositionEncodingKind, PublishDiagnosticsParams, Range, SemanticTokens,
	SemanticTokensParams, TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem, Uri,
	VersionedTextDocumentIdentifier,
	notification::{
		DidChangeTextDocument, DidOpenTextDocument, Exit, Initialized, Notification as _, PublishDiagnostics,
	},
	request::{Initialize, Request as _, SemanticTokensFullRequest},
};
use serde_json::{Value, from_value, json, to_value};

struct Session {
	server: Server,
	uri: Uri,
}

impl Session {
	fn start(encodings: Option<Vec<PositionEncodingKind>>) -> Self {
		let server = Server::new(|client| LSPService::new("test", client));
		let session = Self { server, uri: "file:///test.css".parse().unwrap() };
		let capabilities = ClientCapabilities {
			general: Some(GeneralClientCapabilities { position_encodings: encodings, ..Default::default() }),
			..Default::default()
		};
		session.request(
			1,
			Initialize::METHOD,
			to_value(InitializeParams { capabilities, ..Default::default() }).unwrap(),
		);
		session.notify(Initialized::METHOD, json!({}));
		session
	}

	fn request(&self, id: i32, method: &str, params: Value) -> Value {
		let (sender, receiver) = self.server.raw_channels();
		sender.send(Message::Request(Request { id: id.into(), method: method.into(), params })).unwrap();
		match receiver.recv().unwrap() {
			Message::Response(Response::Ok(response_id, value)) if response_id == id.into() => value,
			other => panic!("expected response {id}, got {other:?}"),
		}
	}

	fn notify(&self, method: &str, params: Value) {
		let (sender, _) = self.server.raw_channels();
		sender.send(Message::Notification(Notification { method: method.into(), params })).unwrap();
	}

	fn next_diagnostics(&self) -> PublishDiagnosticsParams {
		let (_, receiver) = self.server.raw_channels();
		match receiver.recv().unwrap() {
			Message::Notification(Notification { method, params }) if method == PublishDiagnostics::METHOD => {
				from_value(params).unwrap()
			}
			other => panic!("expected publishDiagnostics, got {other:?}"),
		}
	}

	fn open(&self, text: &str) -> PublishDiagnosticsParams {
		let item = TextDocumentItem::new(self.uri.clone(), "css".into(), 1, text.into());
		self.notify(DidOpenTextDocument::METHOD, to_value(DidOpenTextDocumentParams { text_document: item }).unwrap());
		self.next_diagnostics()
	}

	fn change(&self, version: i32, range: Range, text: &str) -> PublishDiagnosticsParams {
		let params = DidChangeTextDocumentParams {
			text_document: VersionedTextDocumentIdentifier::new(self.uri.clone(), version),
			content_changes: vec![TextDocumentContentChangeEvent {
				range: Some(range),
				range_length: None,
				text: text.into(),
			}],
		};
		self.notify(DidChangeTextDocument::METHOD, to_value(params).unwrap());
		self.next_diagnostics()
	}

	fn semantic_tokens(&self, id: i32) -> Vec<(u32, u32, u32)> {
		let params = SemanticTokensParams {
			text_document: TextDocumentIdentifier::new(self.uri.clone()),
			work_done_progress_params: Default::default(),
			partial_result_params: Default::default(),
		};
		let tokens: SemanticTokens =
			from_value(self.request(id, SemanticTokensFullRequest::METHOD, to_value(params).unwrap())).unwrap();
		tokens.data.iter().map(|token| (token.delta_line, token.delta_start, token.length)).collect()
	}
}

impl Drop for Session {
	fn drop(&mut self) {
		self.notify(Exit::METHOD, Value::Null);
	}
}

fn range(start: (u32, u32), end: (u32, u32)) -> Range {
	Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
}

#[test]
fn utf16_positions_count_code_units() {
	let session = Session::start(None);
	assert_eq!(session.open("/*🎨*/é{color:red}").diagnostics, vec![]);
	assert_eq!(session.semantic_tokens(2)[0], (0, 6, 1));
	let changed = session.change(2, range((0, 6), (0, 7)), "}");
	assert_eq!(changed.version, Some(2));
	assert_eq!(changed.diagnostics[0].range, range((0, 6), (0, 7)));
	assert_eq!(changed.diagnostics[0].message, "Unexpected `Kind::RightCurly`");
}

#[test]
fn utf8_positions_count_bytes() {
	let session = Session::start(Some(vec![PositionEncodingKind::UTF16, PositionEncodingKind::UTF8]));
	assert_eq!(session.open("/*🎨*/é{color:red}").diagnostics, vec![]);
	assert_eq!(session.semantic_tokens(2)[0], (0, 8, 2));
	let changed = session.change(2, range((0, 8), (0, 10)), "}");
	assert_eq!(changed.diagnostics[0].range, range((0, 8), (0, 9)));
}
