//! A library for handling LSP servers
//!
//! This implements the core parts of the [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
//! which is a [JSON-RPC 2.0](https://www.jsonrpc.org/specification) based protocol.
//!
//! A [`Server`] can be instantiated via [`Server::listen_stdio`] (or [`Server::raw_channels`] can be used for testing)
//!

use crossbeam_channel::{Receiver, Sender, bounded};
use lsp_types::SetTraceParams;
use serde_json::from_value;
use std::{
	io,
	sync::{Arc, RwLock},
	thread::{Builder, JoinHandle},
};
use tracing::{debug, level_filters::LevelFilter, trace, warn};

use crate::{Client, Notification, ParseError, TracingLayer};

use super::Message;

mod handler;

pub use handler::Handler;

pub struct ThreadConnection {
	pub sender: JoinHandle<io::Result<()>>,
	pub receiver: JoinHandle<io::Result<()>>,
}

impl ThreadConnection {
	pub fn is_finished(&self) -> bool {
		self.sender.is_finished() || self.receiver.is_finished()
	}
}

/// A set of Sender/Receiver objects for passing [`Message`s](Message) around.
pub struct Server {
	write_sender: Sender<Message>,
	write_receiver: Receiver<Message>,
	read_sender: Sender<Message>,
	#[allow(dead_code)]
	request_handler: JoinHandle<Result<(), io::Error>>,
	#[allow(dead_code)]
	read_receiver: Receiver<Message>,
	trace_level: Arc<RwLock<LevelFilter>>,
}

impl Server {
	/// Spawns the message handling thread. `make_handler` receives a [`Client`] for sending server-to-client messages.
	pub fn new<T: Handler>(make_handler: impl FnOnce(Client) -> T) -> Self {
		let (write_sender, write_receiver) = bounded::<Message>(0);
		let (read_sender, read_receiver) = bounded::<Message>(0);
		let handler = make_handler(Client::new(write_sender.clone()));

		let handler_receiver = read_receiver.clone();
		let handler_sender = write_sender.clone();
		let trace_level = Arc::new(RwLock::new(LevelFilter::OFF));
		let level_set = trace_level.clone();
		let request_handler = Builder::new()
			.name("LspMessageHandler".into())
			.spawn(move || {
				while let Ok(message) = handler_receiver.recv() {
					trace!("LspMessageHandler -> {:#?}", &message);
					if let Message::Notification(Notification { method, params }) = &message {
						if method == "exit" {
							break;
						}
						if method == "$/setTrace" {
							let level = from_value::<SetTraceParams>(params.clone())
								.map(|p| match p.value {
									lsp_types::TraceValue::Off => LevelFilter::OFF,
									lsp_types::TraceValue::Messages => LevelFilter::WARN,
									lsp_types::TraceValue::Verbose => LevelFilter::TRACE,
								})
								.unwrap_or(LevelFilter::OFF);
							trace!("Changing level to {:?}", level);
							let mut level_set = level_set.write().unwrap();
							*level_set = level;
						}
					}
					let response = handler.handle(message);
					if let Some(response) = response
						&& let Err(e) = handler_sender.send(response)
					{
						warn!("Handler failed to send response {:?}", &e);
						return Err(io::Error::other(e));
					}
				}
				debug!("LspMessageHandler closing");
				Ok(())
			})
			.expect("Failed to create Reader");
		Server { write_sender, write_receiver, read_sender, read_receiver, request_handler, trace_level }
	}

	pub fn tracer(&self) -> TracingLayer {
		TracingLayer::new(self.trace_level.clone(), self.write_sender.clone())
	}

	pub fn listen_stdio(&self) -> Result<ThreadConnection, io::Error> {
		let write_receiver = self.write_receiver.clone();
		let writer = Builder::new().name("LspWriter".into()).spawn(move || {
			let mut stdout = io::stdout().lock();
			while let Ok(message) = write_receiver.recv() {
				trace!("{:#?}", message);
				message.write(&mut stdout)?;
			}
			Ok(())
		})?;
		let read_sender = self.read_sender.clone();
		let reader = Builder::new()
			.name("LspReader".into())
			.spawn(move || read_messages(&mut io::stdin().lock(), &read_sender))?;
		Ok(ThreadConnection { sender: reader, receiver: writer })
	}

	/// Returns the channels a transport would use: send client messages into the first, receive server messages from
	/// the second. Drives the server in-process without stdio.
	pub fn raw_channels(&self) -> (Sender<Message>, Receiver<Message>) {
		(self.read_sender.clone(), self.write_receiver.clone())
	}
}

/// Forwards each message read from `reader` to `sender` until the input ends or the client sends `exit`. A message
/// whose body is not a JSON-RPC message is skipped, as its framing still says where the next one starts.
fn read_messages(reader: &mut impl io::BufRead, sender: &Sender<Message>) -> io::Result<()> {
	loop {
		match Message::read(reader) {
			Ok(Some(message)) => {
				let exit = message.is_exit_notification();
				sender.send(message).map_err(io::Error::other)?;
				if exit {
					return Ok(());
				}
			}
			Ok(None) => return Ok(()),
			Err(ParseError::Body(error)) => warn!("Skipping unparseable message: {error}"),
			Err(error) => return Err(error.into()),
		}
	}
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicBool, Ordering};

	use crate::{ErrorCode, Notification, Request, Response, ResponseError};

	use super::*;
	use lsp_types::{
		InitializeParams, InitializeResult,
		request::{GotoDeclaration, Initialize, Request as RequestTrait},
	};
	use serde_json::{Value, json, to_value};
	use tracing::level_filters::LevelFilter;
	use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

	#[test]
	fn smoke_test() {
		let stderr_log = fmt::layer().with_writer(io::stderr).with_filter(LevelFilter::TRACE);
		struct TestHandler {
			initialized: AtomicBool,
		}
		impl Handler for TestHandler {
			fn initialized(&self) -> bool {
				self.initialized.load(Ordering::SeqCst)
			}
			fn initialize(&self, _req: InitializeParams) -> Result<InitializeResult, ResponseError> {
				self.initialized.swap(true, Ordering::SeqCst);
				Ok(InitializeResult { ..Default::default() })
			}
		}

		let server = Server::new(|_| TestHandler { initialized: AtomicBool::new(false) });
		registry().with(stderr_log).with(server.tracer()).init();
		let (sender, receiver) = server.raw_channels();
		sender
			.send(Message::Request(Request {
				id: 1.into(),
				method: Initialize::METHOD.into(),
				params: to_value(InitializeParams { ..Default::default() }).unwrap(),
			}))
			.unwrap();
		assert_eq!(receiver.recv(), Ok(Message::Response(Response::Ok(1.into(), json!({"capabilities": {}})))));
		sender
			.send(Message::Request(Request {
				id: 1.into(),
				method: GotoDeclaration::METHOD.into(),
				params: json!({
					"textDocument": {
						"uri": "foo/bar",
					},
					"position": {
						"line": 1,
						"character": 1
					}
				}),
			}))
			.unwrap();
		assert!(matches!(
			receiver.recv(),
			Ok(Message::Response(Response::Err(id, ErrorCode::MethodNotFound, _, Value::Null))) if id == 1.into()
		));
		sender.send(Message::Notification(Notification { method: "exit".into(), params: Value::Null })).unwrap();
	}

	#[test]
	fn reader_skips_unparseable_bodies_and_stops_at_exit() {
		let input = concat!(
			"Content-Length: 11\r\n\r\n{\"foo\":\"x\"}",
			"Content-Length: 29\r\n\r\n{\"method\":\"exit\",\"params\":[]}",
			"Content-Length: 29\r\n\r\n{\"method\":\"next\",\"params\":[]}",
		);
		let (sender, receiver) = crossbeam_channel::unbounded();
		read_messages(&mut input.as_bytes(), &sender).unwrap();
		assert_eq!(
			receiver.try_iter().collect::<Vec<_>>(),
			vec![Message::Notification(Notification { method: "exit".into(), params: json!([]) })]
		);
	}
}
