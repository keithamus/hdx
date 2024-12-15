//! A library for handling LSP servers
//!
//! This implements the core parts of the [LSP Specification](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/)
//! which is a [JSON-RPC 2.0](https://www.jsonrpc.org/specification) based protocol.
//!
//! A [`Server`] can be instantiated via [`Server::listen_stdio`] (or [`Server::raw_channels`] can be used for testing)
//!

use crossbeam_channel::{bounded, Receiver, Sender};
use dashmap::DashMap;
use lsp_types::request::{Initialize, Request as RequestTrait};
use serde_json::{from_value, to_value, Value};
use std::{
	io,
	sync::Arc,
	thread::{Builder, JoinHandle},
};
use tracing::{debug, trace, warn};

use crate::{ErrorCode, TracingLayer};

use super::{Message, Notification, Request, Response};

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
	handler: JoinHandle<io::Result<()>>,
	read_receiver: Receiver<Message>,
	request_handlers: Arc<DashMap<&'static str, RequestHandler>>,
	notification_handlers: Arc<DashMap<&'static str, NotificationHandler>>,
}

struct RequestHandler {
	handle: Box<dyn Fn(&Request) -> Response + Send + Sync>,
}

impl RequestHandler {
	fn new<T, F>(handle: F) -> Self
	where
		T: lsp_types::request::Request,
		F: Fn(T::Params) -> Result<T::Result, io::Error> + Sized + Send + Sync + 'static,
	{
		Self {
			handle: Box::new(move |request| {
				trace!("Deserializing params ({:#?}) into value", &request.params);
				let params = from_value(request.params.clone());
				if let Ok(params) = params {
					trace!("Parsed params successfully, calling handler");
					let result = handle(params);
					if let Ok(result) = result {
						trace!("Handler returned good result, turning into generic value");
						return if let Ok(value) = to_value(result) {
							trace!("Handler Responding {:?}, {:#?}", &request.id, &value);
							Response::Ok(request.id.clone(), value)
						} else {
							trace!("Result failed to_value encode");
							Response::Err(
								request.id.clone(),
								ErrorCode::InternalError,
								"failed to encode response".into(),
								Value::Null,
							)
						};
					}
				}
				trace!("Failed to deserialize params");
				Response::Err(
					request.id.clone(),
					ErrorCode::InvalidParams,
					"could not parse params".into(),
					Value::Null,
				)
			}),
		}
	}
}

struct NotificationHandler {
	handle: Box<dyn Fn(&Notification) -> Result<(), io::Error> + Send + Sync>,
}

impl NotificationHandler {
	fn new<T, F>(handle: F) -> Self
	where
		T: lsp_types::notification::Notification,
		F: Fn(T::Params) -> Result<(), io::Error> + Sized + Send + Sync + 'static,
	{
		Self {
			handle: Box::new(move |request| {
				trace!("Deserializing params ({:#?}) into value", &request.params);
				let params = from_value(request.params.clone());
				if let Ok(params) = params {
					trace!("Parsed params successfully, calling handler");
					let result = handle(params);
					trace!("Handler was: {:?}", &result);
					result
				} else {
					Ok(())
				}
			}),
		}
	}
}

impl Default for Server {
	fn default() -> Self {
		let (write_sender, write_receiver) = bounded::<Message>(0);
		let (read_sender, read_receiver) = bounded::<Message>(0);
		let request_handlers: Arc<DashMap<&'static str, RequestHandler>> = Arc::new(DashMap::new());
		let notification_handlers: Arc<DashMap<&'static str, NotificationHandler>> = Arc::new(DashMap::new());

		let handler_request_handlers = request_handlers.clone();
		let handler_notification_handlers = notification_handlers.clone();
		let handler_receiver = read_receiver.clone();
		let handler_sender = write_sender.clone();
		let mut initialized = false;
		let handler = Builder::new()
			.name("LspMessageHandler".into())
			.spawn(move || {
				while let Ok(message) = handler_receiver.recv() {
					debug!("LspMessageHandler -> {:#?}", &message);
					let exit = message.is_exit_notification();
					if exit {
						break;
					}
					if let Message::Request(request) = &message {
						if let Some(handler) = handler_request_handlers.get(request.method.as_str()) {
							if !initialized && request.method.as_str() != Initialize::METHOD {
								debug!("Skipping method {:?} before initialization", request.method.as_str());
								continue;
							}
							debug!("Found RequestHandler for {:?}", request.method.as_str());
							let response = (handler.handle)(request);
							debug!("RequestHandler <- {:#?}", &response);
							if let Err(e) = handler_sender.send(Message::Response(response)) {
								warn!("Handler failed to send response {:?}", &e);
								return Err(io::Error::new(io::ErrorKind::Other, e));
							}
							if !initialized && request.method.as_str() == Initialize::METHOD {
								debug!("Intialized successfully");
								initialized = true;
							}
						} else {
							warn!("Could not find handler for request {:?}", request.method.as_str());
							let response = Response::Err(
								request.id.clone(),
								ErrorCode::MethodNotFound,
								format!("MethodNotFound: {:?}", request.method.as_str()),
								Value::Null,
							);
							if let Err(e) = handler_sender.send(Message::Response(response)) {
								warn!("Handler failed to send response {:?}", &e);
								return Err(io::Error::new(io::ErrorKind::Other, e));
							}
						}
					} else if let Message::Notification(notification) = &message {
						if let Some(handler) = handler_notification_handlers.get(notification.method.as_str()) {
							debug!("Found NotificationHandler for {:?}", notification.method.as_str());
							let response = (handler.handle)(notification);
							debug!("NotificationHandler <- {:#?}", &response);
						} else {
							warn!("Could not find handler for notification {:?}", notification.method.as_str());
						}
					}
				}
				warn!("LspMessageHandler closing, channel closed");
				Ok(())
			})
			.expect("Failed to create Reader");
		Server {
			write_sender,
			write_receiver,
			read_sender,
			read_receiver,
			request_handlers,
			notification_handlers,
			handler,
		}
	}
}

impl Server {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn listen_stdio(&self) -> Result<ThreadConnection, io::Error> {
		let write_receiver = self.write_receiver.clone();
		let writer = Builder::new().name("LspWriter".into()).spawn(move || {
			trace!("LSPWRITER");
			let mut stdout = io::stdout().lock();
			while let Ok(message) = write_receiver.recv() {
				trace!("{:?}", message);
				message.write(&mut stdout)?;
			}
			Ok(())
		})?;
		let read_sender = self.read_sender.clone();
		let reader = Builder::new().name("LspReader".into()).spawn(move || {
			trace!("LSPREADER");
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

	pub fn tracer(&self) -> TracingLayer {
		TracingLayer::new(self.write_sender.clone())
	}

	#[cfg(test)]
	pub fn raw_channels(&self) -> (Sender<Message>, Receiver<Message>) {
		(self.read_sender.clone(), self.write_receiver.clone())
	}

	pub fn handle<T: lsp_types::request::Request>(
		self,
		handler: impl Fn(T::Params) -> Result<T::Result, io::Error> + Send + Sync + 'static,
	) -> Self {
		self.request_handlers.insert(T::METHOD, RequestHandler::new::<T, _>(handler));
		self
	}

	pub fn on<T: lsp_types::notification::Notification>(
		self,
		handler: impl Fn(T::Params) -> Result<(), io::Error> + Send + Sync + 'static,
	) -> Self {
		self.notification_handlers.insert(T::METHOD, NotificationHandler::new::<T, _>(handler));
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use lsp_types::{
		request::{GotoDeclaration, GotoDeclarationResponse, Request as RequestTrait},
		InitializeParams, InitializeResult,
	};
	use serde_json::json;
	use tracing::level_filters::LevelFilter;
	use tracing_subscriber::{fmt, registry, Layer};

	#[test]
	fn smoke_test() {
		let stderr_log = fmt::layer().with_writer(io::stderr).with_filter(LevelFilter::TRACE);
		let server = Server::new()
			.handle::<Initialize>(move |_| -> Result<InitializeResult, io::Error> {
				Ok(InitializeResult { ..Default::default() })
			})
			.handle::<GotoDeclaration>(move |_| -> Result<Option<GotoDeclarationResponse>, io::Error> { Ok(None) });
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
		assert_eq!(receiver.recv(), Ok(Message::Response(Response::Ok(1.into(), Value::Null))));
	}
}
