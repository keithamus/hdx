use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

use super::Id;

/// A request message to describe a request between the client and the server. Every processed request must send a response back to the sender of the request.
///
/// As defined in [JSON-RPC](https://www.jsonrpc.org/specification#request_object) and [LSP](https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#requestMessage).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Request {
	pub id: Id,
	pub method: String,
	#[serde(default = "serde_json::Value::default", skip_serializing_if = "serde_json::Value::is_null")]
	pub params: Value,
}

impl Request {
	pub fn new<T>(id: Id, params: T::Params) -> Request
	where
		T: lsp_types::request::Request,
	{
		Request { id, method: T::METHOD.into(), params: to_value(params).unwrap() }
	}
}

#[cfg(test)]
mod tests {
	use lsp_types::request::{Initialize, Request as RequestTrait};

	use super::*;

	#[test]
	fn test_request_deserialize() {
		assert_eq!(
			serde_json::from_str::<Request>(r#"{"id":0, "method": "initialize", "params": null}"#).unwrap(),
			Request { id: 0.into(), params: Value::Null, method: Initialize::METHOD.into() }
		);
	}
}
