//! A library for handling LSP servers
//!
//! This implements the core parts of the [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
//! which is a [JSON-RPC 2.0](https://www.jsonrpc.org/specification) based protocol.
//!
//! A [`Server`] can be instantiated via [`Server::listen_stdio`] (or [`Server::raw_channels`] can be used for testing)
//!

use crossbeam_channel::{bounded, Receiver, Sender};
use lsp_types::SetTraceParams;
use serde_json::from_value;
use std::{
	io,
	sync::{Arc, RwLock},
	thread::{Builder, JoinHandle},
};
use tracing::{level_filters::LevelFilter, trace, warn};

use crate::{Notification, TracingLayer};

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
	request_handler: JoinHandle<Result<(), io::Error>>,
	read_receiver: Receiver<Message>,
	trace_level: Arc<RwLock<LevelFilter>>,
}

impl Server {
	pub fn new<T: Handler>(handler: T) -> Self {
		let (write_sender, write_receiver) = bounded::<Message>(0);
		let (read_sender, read_receiver) = bounded::<Message>(0);

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
					if let Some(response) = response {
						if let Err(e) = handler_sender.send(response) {
							warn!("Handler failed to send response {:?}", &e);
							return Err(io::Error::new(io::ErrorKind::Other, e));
						}
					}
				}
				warn!("LspMessageHandler closing, channel closed");
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
		let reader = Builder::new().name("LspReader".into()).spawn(move || {
			let mut stdin = io::stdin().lock();
			while let Some(message) = Message::read(&mut stdin)? {
				if let Err(e) = read_sender.send(message) {
					return Err(io::Error::new(io::ErrorKind::Other, e));
				}
			}
			Ok(())
		})?;
		Ok(ThreadConnection { sender: reader, receiver: writer })
	}

	#[cfg(test)]
	pub fn raw_channels(&self) -> (Sender<Message>, Receiver<Message>) {
		(self.read_sender.clone(), self.write_receiver.clone())
	}
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicBool, Ordering};

	use crate::{ErrorCode, Notification, Request, Response};

	use super::*;
	use lsp_types::{
		request::{GotoDeclaration, Initialize, Request as RequestTrait},
		InitializeParams, InitializeResult,
	};
	use serde_json::{json, to_value, Value};
	use tracing::level_filters::LevelFilter;
	use tracing_subscriber::{fmt, layer::SubscriberExt, registry, util::SubscriberInitExt, Layer};

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
			fn initialize(&self, _req: InitializeParams) -> Result<InitializeResult, ErrorCode> {
				self.initialized.swap(true, Ordering::SeqCst);
				Ok(InitializeResult { ..Default::default() })
			}
		}

		let server = Server::new(TestHandler { initialized: AtomicBool::new(false) });
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
		assert_eq!(
			receiver.recv(),
			Ok(Message::Response(Response::Err(1.into(), ErrorCode::MethodNotFound, "".into(), Value::Null)))
		);
		sender.send(Message::Notification(Notification { method: "exit".into(), params: Value::Null })).unwrap();
	}
}
