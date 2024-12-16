use httparse::{parse_headers, EMPTY_HEADER};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::io;

use crate::{Notification, Request, Response};

/// JSON RPC Message
/// This represents a single message coming in or going out, that is
/// compliant with the [JSON-RPC 2.0 spec](https://www.jsonrpc.org/specification).
/// It wraps the [`Request`], [`Response`] and [`Notification`] structs.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Message {
	/// Wraps the [`Request`] object.
	Request(Request),
	/// Wraps the [`Response`] object.
	Response(Response),
	/// Wraps the [`Notification`] object.
	Notification(Notification),
}

impl Message {
	#[doc(hidden)]
	#[inline]
	pub fn is_exit_notification(&self) -> bool {
		if let Message::Notification(notification) = self {
			matches!(notification.method.as_str(), "exit")
		} else {
			false
		}
	}
}

impl Message {
	pub fn read(r: &mut impl io::BufRead) -> Result<Option<Message>, ParseError> {
		let mut buf = String::new();
		// Consume all headers - either end of stream or "\r\n\r\n"
		loop {
			// No more content, therefore no message
			if r.read_line(&mut buf)? == 0 {
				return Ok(None);
			}
			if buf.ends_with("\r\n\r\n") {
				break;
			}
		}
		let mut headers = [EMPTY_HEADER; 2];
		if let httparse::Status::Complete((size, _)) = parse_headers(buf.as_bytes(), &mut headers)? {
			if size != buf.len() {
				Err(ParseError::HeaderDecodeMismatch(size, buf.len()))?
			}
		}
		let mut content_length = 0;
		for header in &headers {
			if header.name.eq_ignore_ascii_case("content-length") {
				content_length = std::str::from_utf8(header.value)?.parse::<usize>()?;
			} else if header.name.eq_ignore_ascii_case("content-type") {
				// ¯\_(ツ)_/¯
			} else if header != &EMPTY_HEADER {
				Err(ParseError::InvalidHeader(header.name.to_owned()))?
			}
		}
		if content_length == 0 {
			Err(ParseError::NoLength)?
		}
		buf.clear();
		let mut buf = buf.into_bytes();
		buf.resize(content_length, 0);
		r.read_exact(&mut buf)?;
		let message: Message = serde_json::from_slice(buf.as_slice())?;
		Ok(Some(message))
	}

	pub fn write(self, w: &mut impl io::Write) -> io::Result<()> {
		#[derive(Serialize)]
		struct JSONRPCMessage {
			jsonrpc: &'static str,
			#[serde(flatten)]
			message: Message,
		}
		let msg = to_string(&JSONRPCMessage { jsonrpc: "2.0", message: self })?;
		write!(w, "Content-Length: {}\r\n\r\n", msg.len())?;
		w.write_all(msg.as_bytes())?;
		w.flush()?;
		Ok(())
	}
}

#[derive(Debug)]
pub enum ParseError {
	NoLength,
	CouldNotDecodeHeader,
	HeaderDecodeMismatch(usize, usize),
	InvalidHeader(String),
	Encode(io::Error),
	Utf8(std::str::Utf8Error),
	InvalidContentLength(std::num::ParseIntError),
	Headers(httparse::Error),
	Body(serde_json::Error),
}

impl From<ParseError> for io::Error {
	fn from(error: ParseError) -> Self {
		match error {
			ParseError::NoLength => io::Error::new(io::ErrorKind::InvalidData, "could not read content-length header"),
			ParseError::CouldNotDecodeHeader => io::Error::new(io::ErrorKind::InvalidData, "could not decode headers"),
			ParseError::HeaderDecodeMismatch(expected, actual) => io::Error::new(
				io::ErrorKind::InvalidData,
				format!("failed to fully parse headers, expected {expected} but parsing ended at {actual} bytes"),
			),
			ParseError::InvalidHeader(string) => {
				io::Error::new(io::ErrorKind::InvalidData, format!("saw invalid header {string}"))
			}
			ParseError::Encode(e) => e,
			ParseError::Utf8(e) => io::Error::new(io::ErrorKind::InvalidData, format!("Utf8 decode error: {e}")),
			ParseError::InvalidContentLength(e) => {
				io::Error::new(io::ErrorKind::InvalidData, format!("invalid content-length: {e}"))
			}
			ParseError::Headers(e) => io::Error::new(io::ErrorKind::InvalidData, format!("invalid headers: {e}")),
			ParseError::Body(e) => io::Error::new(io::ErrorKind::InvalidData, format!("invalid body: {e}")),
		}
	}
}

impl From<io::Error> for ParseError {
	fn from(error: io::Error) -> Self {
		ParseError::Encode(error)
	}
}

impl From<httparse::Error> for ParseError {
	fn from(error: httparse::Error) -> Self {
		ParseError::Headers(error)
	}
}

impl From<serde_json::Error> for ParseError {
	fn from(error: serde_json::Error) -> Self {
		ParseError::Body(error)
	}
}

impl From<std::num::ParseIntError> for ParseError {
	fn from(error: std::num::ParseIntError) -> Self {
		ParseError::InvalidContentLength(error)
	}
}

impl From<std::str::Utf8Error> for ParseError {
	fn from(error: std::str::Utf8Error) -> Self {
		ParseError::Utf8(error)
	}
}

impl From<Request> for Message {
	fn from(request: Request) -> Message {
		Message::Request(request)
	}
}

impl From<Response> for Message {
	fn from(response: Response) -> Message {
		Message::Response(response)
	}
}

impl From<Notification> for Message {
	fn from(notification: Notification) -> Message {
		Message::Notification(notification)
	}
}

pub enum MessageError {
	MethodNotFound,
	MethodMistmatch(String, String),
	JsonError(serde_json::Error),
}

#[cfg(test)]
mod tests {
	use std::str::from_utf8;

	use super::*;
	use lsp_types::request::{Initialize, Request as RequestTrait};
	use serde_json::{from_str, json};

	pub fn into_http_bytes(str: &str) -> String {
		format!("Content-Length: {}\r\n\r\n{}", str.len(), str)
	}

	#[test]
	fn test_message_deserialize() {
		assert_eq!(
			from_str::<Message>(r#"{"jsonrpc": "2.0","method": "initialize", "params": null, "id": 1}"#).unwrap(),
			Message::Request(Request { id: 1.into(), method: Initialize::METHOD.into(), params: json!(null) })
		);
		assert_eq!(
			from_str::<Message>(r#"{"jsonrpc": "2.0","method": "initialize", "params": [1,2], "id": "a"}"#).unwrap(),
			Message::Request(Request { id: "a".into(), method: Initialize::METHOD.into(), params: json!([1, 2]) })
		);
		assert_eq!(
			from_str::<Message>(r#"{"jsonrpc": "2.0","result": "foo","id":8}"#).unwrap(),
			Message::Response(Response::Ok(8.into(), json!("foo")))
		);
		assert_eq!(
			from_str::<Message>(r#"{"jsonrpc": "2.0","method": "exit"}"#).unwrap(),
			Message::Notification(Notification { method: "exit".into(), params: json!(null) })
		);
	}

	#[test]
	fn test_message_read_from_bufreader() {
		let r = into_http_bytes(r#"{"jsonrpc": "2.0","method": "initialize", "params": null, "id": 1}"#);
		assert_eq!(
			Message::read(&mut r.as_bytes()).unwrap(),
			Some(Message::Request(Request { id: 1.into(), method: "initialize".into(), params: json!(null) }))
		);
	}

	#[test]
	fn test_message_write_to_bufreader() {
		let mut bytes = vec![];
		Message::Request(Request { id: 1.into(), method: "initialize".into(), params: json!(null) })
			.write(&mut bytes)
			.unwrap();
		assert_eq!(from_utf8(&bytes).unwrap(), into_http_bytes(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#));
	}
}
