use serde::{Deserialize, Serialize};

/// An ErrorCode representing either a [JSON-RPC Error Code](https://www.jsonrpc.org/specification#error_object) or [LSP
/// defined error
/// code](https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#errorCodes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(into = "i32", from = "i32")]
pub enum ErrorCode {
	// JSON RPC Generic
	/// Invalid JSON was received by the server. An error occurred on the server while parsing the JSON text.
	/// [JSON-RPC defined](https://www.jsonrpc.org/specification#error_object). `-32700`.
	ParseError,

	/// The JSON sent is not a valid Request object.
	/// [JSON-RPC defined](https://www.jsonrpc.org/specification#error_object). `-32600`.
	InvalidRequest,

	/// The method does not exist / is not available.
	/// [JSON-RPC defined](https://www.jsonrpc.org/specification#error_object). `-32601`.
	MethodNotFound,

	/// Invalid method parameter(s).
	/// [JSON-RPC defined](https://www.jsonrpc.org/specification#error_object). `-32602`.
	InvalidParams,

	/// Internal JSON-RPC error.
	/// [JSON-RPC defined](https://www.jsonrpc.org/specification#error_object). `-32603`.
	InternalError,

	/// [JSON RPC Reserved Range Start](https://www.jsonrpc.org/specification#error_object).
	/// Reserved for implementation-defined server-errors. (`-32000` to `-32099`).
	ReservedErrorStart,

	/// Error code indicating that a server received a notification or
	/// request before the server has received the `initialize` request.
	/// [LSP defined](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#errorCodes). `-32002`.
	ServerNotInitialized,

	/// [LSP defined](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#errorCodes). `-32001`.
	UnknownErrorCode,

	/// [JSON RPC Reserved Range End](https://www.jsonrpc.org/specification#error_object).
	/// Reserved for implementation-defined server-errors. (`-32000` to `-32099`).
	ReservedErrorEnd,

	/// [LSP Reserved Range Start](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#errorCodes).
	/// This is the start range of LSP reserved error codes. It doesn't denote a real error code. (`-32899` to `-32800`)
	LspReservedErrorStart,

	/// The server detected that the content of a document got modified outside normal conditions. A server should NOT
	/// send this error code if it detects a content change in its unprocessed messages. The result even computed on
	/// an older state might still be useful for the client.
	///
	/// If a client decides that a result is not of any use anymore the client should cancel the request.
	///
	/// [LSP defined](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#errorCodes). `-32801`.
	ContentModified,

	/// The client has canceled a request and a server has detected the cancel.
	///
	/// [LSP defined](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#errorCodes). `-32800`.
	RequestCancelled,

	/// [LSP Reserved Range Start](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.18/specification/#errorCodes).
	/// This is the start range of LSP reserved error codes. It doesn't denote a real error code. (`-32899` to `-32800`)
	LspReservedErrorEnd,

	/// The error code is not known.
	Unknown(i32),
}

// https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#responseMessage
impl From<ErrorCode> for i32 {
	fn from(value: ErrorCode) -> Self {
		match value {
			ErrorCode::ParseError => -32700,
			ErrorCode::InvalidRequest => -32600,
			ErrorCode::MethodNotFound => -32601,
			ErrorCode::InvalidParams => -32602,
			ErrorCode::InternalError => -32603,

			// JSON RPC Reserved Range
			ErrorCode::ReservedErrorStart => -32099,
			ErrorCode::ServerNotInitialized => -32002,
			ErrorCode::UnknownErrorCode => -32001,
			ErrorCode::ReservedErrorEnd => -32000,

			// LSP Reserved Range
			ErrorCode::LspReservedErrorStart => -32899,
			ErrorCode::ContentModified => -32801,
			ErrorCode::RequestCancelled => -32800,
			ErrorCode::LspReservedErrorEnd => -32800,

			ErrorCode::Unknown(code) => code,
		}
	}
}

impl From<i32> for ErrorCode {
	fn from(value: i32) -> Self {
		match value {
			-32700 => ErrorCode::ParseError,
			-32600 => ErrorCode::InvalidRequest,
			-32601 => ErrorCode::MethodNotFound,
			-32602 => ErrorCode::InvalidParams,
			-32603 => ErrorCode::InternalError,
			// JSON RPC Reserved Range
			-32099 => ErrorCode::ReservedErrorStart,
			-32002 => ErrorCode::ServerNotInitialized,
			-32001 => ErrorCode::UnknownErrorCode,
			-32000 => ErrorCode::ReservedErrorEnd,

			// LSP Reserved Range
			-32899 => ErrorCode::LspReservedErrorStart,
			-32801 => ErrorCode::ContentModified,
			-32800 => ErrorCode::RequestCancelled,

			code => ErrorCode::Unknown(code),
		}
	}
}
