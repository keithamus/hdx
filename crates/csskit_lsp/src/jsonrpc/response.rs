use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::{json, to_value, Value};

use super::{ErrorCode, Id};

/// A Response Message sent as a result of a request
///
/// As defined in [JSON-RPC](https://www.jsonrpc.org/specification#response_object) and [LSP](https://microsoft.github.io/language-server-protocol/specifications/specification-3-16/#responseMessage).
///
/// Can either be [`Ok`] (the response had a `result`, and did not have attached `error` information),
/// or as [`Err`] (the response had no `result`, instead having attached `error` information).
///
/// Both [`Ok`] and [`Err`] have an [`Id`] which matches the [`Id`] of the [`super::Request`] object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Response {
	/// [`Ok`]s also have an optional payload (serialised as a serde [`Value`]) which may contain more data about the
	/// response, such as computed values.
	Ok(Id, Value),
	/// [`Err`]s also have an [`ErrorCode`] to determine which error occurred, an informatinal [`String`], and may contain
	/// additional data (serialised as serde [`Value`]).
	Err(Id, ErrorCode, String, Value),
}

impl Serialize for Response {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		#[derive(Serialize)]
		struct Res<'a> {
			id: &'a Id,
			result: Option<&'a Value>,
			error: Option<Value>,
		}
		if let Response::Ok(id, value) = self {
			Res { id, result: Some(value), error: None }.serialize(serializer)
		} else if let Response::Err(id, code, message, value) = self {
			Res {
				id,
				result: None,
				error: Some(json!({
					"code": code, "message": message, "data": value
				})),
			}
			.serialize(serializer)
		} else {
			unreachable!()
		}
	}
}

impl<'de> Deserialize<'de> for Response {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		struct Err {
			code: i32,
			message: String,
			#[serde(default = "serde_json::Value::default")]
			data: Value,
		}

		#[derive(Deserialize)]
		struct Res {
			id: Id,
			result: Option<Value>,
			error: Option<Err>,
		}

		let res = Res::deserialize(deserializer)?;
		if let Some(result) = res.result {
			if res.error.is_some() {
				Err(Error::duplicate_field("error"))?
			}
			Ok(Response::Ok(res.id, result))
		} else if let Some(error) = res.error {
			if res.result.is_some() {
				Err(Error::duplicate_field("result"))?
			}
			Ok(Response::Err(res.id, error.code.into(), error.message, error.data))
		} else {
			Err(Error::missing_field("result"))?
		}
	}
}

impl Response {
	pub fn new_ok<T>(id: Id, result: T) -> Response
	where
		T: Serialize,
	{
		Response::Ok(id, to_value(result).unwrap())
	}

	pub fn new_err<T>(id: Id, error_code: ErrorCode, message: String, value: T) -> Response
	where
		T: Serialize,
	{
		Response::Err(id, error_code, message, to_value(value).unwrap())
	}
}

#[cfg(test)]
mod tests {
	use serde_json::{from_str, json};

	use super::*;

	#[test]
	fn test_response_deserialize() {
		assert_eq!(from_str::<Response>(r#"{"id":3, "result":7}"#).unwrap(), Response::Ok(3.into(), json!(7)));
		assert_eq!(
			from_str::<Response>(r#"{"id":4, "result":["a", "b"]}"#).unwrap(),
			Response::Ok(4.into(), json!(["a", "b"]))
		);
		assert_eq!(
			from_str::<Response>(r#"{"id":"a", "error":{"code": -32700, "message": "Parse error"}}"#).unwrap(),
			Response::Err("a".into(), ErrorCode::ParseError, "Parse error".into(), Value::Null)
		);
		assert_eq!(
			from_str::<Response>(
				r#"{"id":"foo", "error":{"code": -32600, "message": "Invalid Request", "data": ["foo"]}}"#
			)
			.unwrap(),
			Response::Err("foo".into(), ErrorCode::InvalidRequest, "Invalid Request".into(), json!(["foo"]))
		);
	}

	#[test]
	fn test_response_deserialize_error() {
		// Missing result/error
		assert!(matches!(from_str::<Response>(r#"{"id":3}"#), Err(_)));

		// Missing error Code/Message
		assert!(matches!(from_str::<Response>(r#"{"id":3, "error":{}}"#), Err(_)));

		// Missing error Message
		assert!(matches!(from_str::<Response>(r#"{"id":3, "error":{"code":0}}"#), Err(_)));

		// Missing error Code
		assert!(matches!(from_str::<Response>(r#"{"id":3, "error":{"message":""}}"#), Err(_)));

		// Both error/result present
		assert!(matches!(from_str::<Response>(r#"{"id":3, "error":{"code":0, "message": ""}, "result":7}"#), Err(_)));
		assert!(matches!(from_str::<Response>(r#"{"id":3, "result":7, "error":{"code":0, "message": ""}}"#), Err(_)));
	}
}
