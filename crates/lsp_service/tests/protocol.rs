use crossbeam_channel::{Receiver, Sender};
use lsp_service::{
	Client, ErrorCode, Handler, Id, Message, Notification, PositionEncoding, Request, Response, ResponseError, Server,
};
use lsp_types::{
	ClientCapabilities, Diagnostic, DidChangeTextDocumentParams, DidOpenTextDocumentParams, GeneralClientCapabilities,
	Hover, HoverContents, HoverParams, InitializeParams, InitializeResult, InitializedParams, MarkedString, Position,
	PositionEncodingKind, PublishDiagnosticsParams, Range, RegistrationParams, RenameParams,
	TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem, TextDocumentPositionParams, Uri,
	VersionedTextDocumentIdentifier, WorkspaceEdit,
	notification::{
		DidChangeTextDocument, DidOpenTextDocument, Exit, Initialized, Notification as _, PublishDiagnostics,
	},
	request::{GotoDefinition, HoverRequest, Initialize, RegisterCapability, Rename, Request as _, Shutdown},
};
use ropey::Rope;
use serde_json::{Value, from_value, json, to_value};
use std::sync::{
	Mutex, OnceLock,
	atomic::{AtomicBool, Ordering},
};

struct EchoService {
	client: Client,
	encoding: OnceLock<PositionEncoding>,
	document: Mutex<Rope>,
}

impl EchoService {
	fn publish(&self, uri: Uri, version: i32, rope: &Rope) {
		let encoding = self.encoding.get().copied().unwrap_or_default();
		let range = Range::new(encoding.position(rope, 0), encoding.position(rope, rope.len_chars()));
		let diagnostic = Diagnostic { range, message: rope.to_string(), ..Default::default() };
		self.client
			.notify::<PublishDiagnostics>(PublishDiagnosticsParams {
				uri,
				diagnostics: vec![diagnostic],
				version: Some(version),
			})
			.unwrap();
	}
}

impl Handler for EchoService {
	fn initialized(&self) -> bool {
		self.encoding.get().is_some()
	}

	fn initialize(&self, req: InitializeParams) -> Result<InitializeResult, ResponseError> {
		let encoding = *self.encoding.get_or_init(|| PositionEncoding::negotiate(&req.capabilities));
		Ok(InitializeResult {
			capabilities: lsp_types::ServerCapabilities {
				position_encoding: Some(encoding.kind()),
				..Default::default()
			},
			server_info: None,
			offset_encoding: None,
		})
	}

	fn hover_request(&self, _req: HoverParams) -> Result<Option<Hover>, ResponseError> {
		let text = self.document.lock().unwrap().to_string();
		Ok(Some(Hover { contents: HoverContents::Scalar(MarkedString::String(text)), range: None }))
	}

	fn rename(&self, req: RenameParams) -> Result<Option<WorkspaceEdit>, ResponseError> {
		Err(ResponseError::new(ErrorCode::InvalidParams, format!("cannot rename to `{}`", req.new_name)))
	}

	fn on_did_open_text_document(&self, req: DidOpenTextDocumentParams) {
		let rope = Rope::from_str(&req.text_document.text);
		self.publish(req.text_document.uri, req.text_document.version, &rope);
		*self.document.lock().unwrap() = rope;
	}

	fn on_did_change_text_document(&self, req: DidChangeTextDocumentParams) {
		let encoding = self.encoding.get().copied().unwrap_or_default();
		let mut rope = self.document.lock().unwrap();
		for change in req.content_changes {
			let range = change.range.unwrap();
			let start = encoding.char_index(&rope, range.start);
			let end = encoding.char_index(&rope, range.end);
			rope.remove(start..end);
			rope.insert(start, &change.text);
		}
		self.publish(req.text_document.uri, req.text_document.version, &rope);
	}
}

struct Session {
	sender: Sender<Message>,
	receiver: Receiver<Message>,
	_server: Server,
}

impl Session {
	fn start(encodings: Option<Vec<PositionEncodingKind>>) -> (Self, Value) {
		let server =
			Server::new(|client| EchoService { client, encoding: OnceLock::new(), document: Mutex::new(Rope::new()) });
		let (sender, receiver) = server.raw_channels();
		let session = Self { sender, receiver, _server: server };
		let capabilities = ClientCapabilities {
			general: Some(GeneralClientCapabilities { position_encodings: encodings, ..Default::default() }),
			..Default::default()
		};
		let result = session.request(
			1,
			Initialize::METHOD,
			to_value(InitializeParams { capabilities, ..Default::default() }).unwrap(),
		);
		session.notify(Initialized::METHOD, json!({}));
		(session, result)
	}

	fn request(&self, id: i32, method: &str, params: Value) -> Value {
		self.sender.send(Message::Request(Request { id: id.into(), method: method.into(), params })).unwrap();
		match self.receiver.recv().unwrap() {
			Message::Response(Response::Ok(response_id, value)) if response_id == id.into() => value,
			other => panic!("expected response {id}, got {other:?}"),
		}
	}

	fn notify(&self, method: &str, params: Value) {
		self.sender.send(Message::Notification(Notification { method: method.into(), params })).unwrap();
	}

	fn next_diagnostics(&self) -> PublishDiagnosticsParams {
		match self.receiver.recv().unwrap() {
			Message::Notification(Notification { method, params }) if method == PublishDiagnostics::METHOD => {
				from_value(params).unwrap()
			}
			other => panic!("expected publishDiagnostics, got {other:?}"),
		}
	}

	fn open(&self, uri: &Uri, text: &str) -> PublishDiagnosticsParams {
		let item = TextDocumentItem::new(uri.clone(), "text".into(), 1, text.into());
		self.notify(DidOpenTextDocument::METHOD, to_value(DidOpenTextDocumentParams { text_document: item }).unwrap());
		self.next_diagnostics()
	}

	fn change(&self, uri: &Uri, version: i32, range: Range, text: &str) -> PublishDiagnosticsParams {
		let params = DidChangeTextDocumentParams {
			text_document: VersionedTextDocumentIdentifier::new(uri.clone(), version),
			content_changes: vec![TextDocumentContentChangeEvent {
				range: Some(range),
				range_length: None,
				text: text.into(),
			}],
		};
		self.notify(DidChangeTextDocument::METHOD, to_value(params).unwrap());
		self.next_diagnostics()
	}

	fn hover_text(&self, id: i32, uri: &Uri) -> String {
		let params = HoverParams {
			text_document_position_params: TextDocumentPositionParams::new(
				TextDocumentIdentifier::new(uri.clone()),
				Position::new(0, 0),
			),
			work_done_progress_params: Default::default(),
		};
		let hover: Hover = from_value(self.request(id, HoverRequest::METHOD, to_value(params).unwrap())).unwrap();
		match hover.contents {
			HoverContents::Scalar(MarkedString::String(text)) => text,
			other => panic!("unexpected hover {other:?}"),
		}
	}
}

impl Drop for Session {
	fn drop(&mut self) {
		self.notify(Exit::METHOD, Value::Null);
	}
}

fn uri() -> Uri {
	"file:///test.txt".parse().unwrap()
}

fn range(start: (u32, u32), end: (u32, u32)) -> Range {
	Range::new(Position::new(start.0, start.1), Position::new(end.0, end.1))
}

#[test]
fn utf16_session_edits_after_astral_characters() {
	let (session, result) = Session::start(None);
	assert_eq!(result["capabilities"]["positionEncoding"], "utf-16");
	let uri = uri();

	let opened = session.open(&uri, "a🎨b\nx");
	assert_eq!(opened.version, Some(1));
	assert_eq!(opened.diagnostics[0].range, range((0, 0), (1, 1)));

	let changed = session.change(&uri, 2, range((0, 3), (0, 4)), "c😀");
	assert_eq!(changed.version, Some(2));
	assert_eq!(changed.diagnostics[0].message, "a🎨c😀\nx");

	assert_eq!(session.hover_text(2, &uri), "a🎨c😀\nx");
}

#[test]
fn utf8_session_edits_after_astral_characters() {
	let (session, result) = Session::start(Some(vec![PositionEncodingKind::UTF16, PositionEncodingKind::UTF8]));
	assert_eq!(result["capabilities"]["positionEncoding"], "utf-8");
	let uri = uri();

	let opened = session.open(&uri, "a🎨b");
	assert_eq!(opened.diagnostics[0].range, range((0, 0), (0, 6)));

	let changed = session.change(&uri, 2, range((0, 5), (0, 6)), "c");
	assert_eq!(changed.diagnostics[0].message, "a🎨c");
	assert_eq!(changed.diagnostics[0].range, range((0, 0), (0, 6)));
}

#[test]
fn ignores_messages_before_initialize_and_rejects_unknown_requests() {
	let server =
		Server::new(|client| EchoService { client, encoding: OnceLock::new(), document: Mutex::new(Rope::new()) });
	let (sender, receiver) = server.raw_channels();
	sender
		.send(Message::Request(Request { id: 1.into(), method: HoverRequest::METHOD.into(), params: Value::Null }))
		.unwrap();
	let session = Session { sender, receiver, _server: server };
	let params = to_value(InitializeParams::default()).unwrap();
	session.request(2, Initialize::METHOD, params);
	session
		.sender
		.send(Message::Request(Request { id: 3.into(), method: GotoDefinition::METHOD.into(), params: Value::Null }))
		.unwrap();
	assert!(matches!(
		session.receiver.recv().unwrap(),
		Message::Response(Response::Err(id, ErrorCode::InvalidParams, _, _)) if id == 3.into()
	));
	session
		.sender
		.send(Message::Request(Request { id: 4.into(), method: "csskit/unknown".into(), params: Value::Null }))
		.unwrap();
	assert!(matches!(
		session.receiver.recv().unwrap(),
		Message::Response(Response::Err(id, ErrorCode::MethodNotFound, _, _)) if id == 4.into()
	));
}

struct Registrar {
	client: Client,
	initialized: AtomicBool,
}

impl Handler for Registrar {
	fn initialized(&self) -> bool {
		self.initialized.load(Ordering::SeqCst)
	}

	fn initialize(&self, _req: InitializeParams) -> Result<InitializeResult, ResponseError> {
		self.initialized.store(true, Ordering::SeqCst);
		Ok(InitializeResult::default())
	}

	fn on_initialized(&self, _req: InitializedParams) {
		for _ in 0..2 {
			self.client.request::<RegisterCapability>(RegistrationParams { registrations: vec![] }).unwrap();
		}
	}

	fn hover_request(&self, _req: HoverParams) -> Result<Option<Hover>, ResponseError> {
		Ok(None)
	}
}

#[test]
fn client_requests_get_distinct_ids_and_their_replies_are_ignored() {
	let server = Server::new(|client| Registrar { client, initialized: AtomicBool::new(false) });
	let (sender, receiver) = server.raw_channels();
	let session = Session { sender, receiver, _server: server };
	session.request(1, Initialize::METHOD, to_value(InitializeParams::default()).unwrap());
	session.notify(Initialized::METHOD, json!({}));
	let ids: Vec<Id> = (0..2)
		.map(|_| match session.receiver.recv().unwrap() {
			Message::Request(Request { id, method, .. }) if method == RegisterCapability::METHOD => id,
			other => panic!("expected registerCapability, got {other:?}"),
		})
		.collect();
	assert_ne!(ids[0], ids[1]);
	for id in ids {
		session.sender.send(Message::Response(Response::Ok(id, Value::Null))).unwrap();
	}
	let params = HoverParams {
		text_document_position_params: TextDocumentPositionParams::new(
			TextDocumentIdentifier::new(uri()),
			Position::new(0, 0),
		),
		work_done_progress_params: Default::default(),
	};
	assert_eq!(session.request(2, HoverRequest::METHOD, to_value(params).unwrap()), Value::Null);
}

#[test]
fn shutdown_succeeds_without_an_override() {
	let (session, _) = Session::start(None);
	assert_eq!(session.request(2, Shutdown::METHOD, Value::Null), Value::Null);
}

#[test]
fn a_refused_request_reaches_the_client_with_its_message() {
	let (session, _) = Session::start(None);
	let params = RenameParams {
		text_document_position: TextDocumentPositionParams::new(
			TextDocumentIdentifier::new(uri()),
			Position::new(0, 0),
		),
		new_name: "taken".into(),
		work_done_progress_params: Default::default(),
	};
	session.sender.send(Message::Request(Request::new::<Rename>(2.into(), params))).unwrap();
	assert_eq!(
		session.receiver.recv().unwrap(),
		Message::Response(Response::Err(
			2.into(),
			ErrorCode::InvalidParams,
			"cannot rename to `taken`".into(),
			Value::Null
		))
	);
}
