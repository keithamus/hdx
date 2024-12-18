use serde::{Deserialize, Serialize};

/// A identifier for tracking [`Request`s](super::Request) and [`Response`s](super::Response).
/// An identifier can be a String or Number if included.
///
/// The [JSON-RPC spec]() defines the number as ["containing no fractional
/// parts"](https://www.jsonrpc.org/specification#id2), and LSP defines Numbers as integers
/// ["in the range of -2^31 to 2^31 - 1"](https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#integer)
/// (so they're [`i32`] in Rust parlance).
///
/// Strings have no constraints on them, but for example could be a UUID.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Id {
	Number(i32),
	String(String),
}

impl Default for Id {
	fn default() -> Self {
		Self::Number(0)
	}
}

impl From<&str> for Id {
	fn from(value: &str) -> Self {
		Self::String(value.into())
	}
}

impl From<String> for Id {
	fn from(value: String) -> Self {
		Self::String(value)
	}
}

impl From<i32> for Id {
	fn from(value: i32) -> Self {
		Self::Number(value)
	}
}
