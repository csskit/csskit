use crossbeam_channel::{SendError, Sender};
use std::sync::{
	Arc,
	atomic::{AtomicI32, Ordering},
};

use crate::{Message, Notification, Request};

/// A handle for sending server-to-client messages, such as `textDocument/publishDiagnostics`.
///
/// Cheap to clone; clones can be moved to other threads.
#[derive(Debug, Clone)]
pub struct Client {
	sender: Sender<Message>,
	next_id: Arc<AtomicI32>,
}

impl Client {
	pub(crate) fn new(sender: Sender<Message>) -> Self {
		Self { sender, next_id: Arc::default() }
	}

	/// Sends the notification `N` to the client. Fails only once the connection has closed.
	pub fn notify<N>(&self, params: N::Params) -> Result<(), SendError<Message>>
	where
		N: lsp_types::notification::Notification,
	{
		self.sender.send(Message::Notification(Notification::new::<N>(params)))
	}

	/// Sends the request `R` to the client, such as `client/registerCapability`, without waiting for its reply. The
	/// client's reply reaches the [`Handler`](crate::Handler), which ignores it. Fails only once the connection has
	/// closed.
	pub fn request<R>(&self, params: R::Params) -> Result<(), SendError<Message>>
	where
		R: lsp_types::request::Request,
	{
		let id = self.next_id.fetch_add(1, Ordering::Relaxed).into();
		self.sender.send(Message::Request(Request::new::<R>(id, params)))
	}
}
