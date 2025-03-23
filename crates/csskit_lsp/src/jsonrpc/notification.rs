use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

/// A notification message. A processed notification message must not send a response back. They work like events.
///
/// As defined in [JSON-RPC](https://www.jsonrpc.org/specification#notification) and [LSP](https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#notificationMessage).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Notification {
	pub method: String,
	#[serde(default = "serde_json::Value::default", skip_serializing_if = "serde_json::Value::is_null")]
	pub params: Value,
}

impl Notification {
	pub fn new<T>(params: T::Params) -> Notification
	where
		T: lsp_types::request::Request,
	{
		Notification { method: T::METHOD.into(), params: to_value(params).unwrap() }
	}
}
