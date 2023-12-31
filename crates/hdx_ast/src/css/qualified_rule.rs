use hdx_lexer::Span;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "camelCase"))]
pub struct QualifiedRule<'a> {
	pub str: &'a str,
}
