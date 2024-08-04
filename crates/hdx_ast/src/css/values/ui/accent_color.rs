use hdx_derive::{Value, from_syntax};
use hdx_lexer::{Kind, Token};
use hdx_parser::{Parse, Parser, Result as ParserResult};

use crate::css::types::Color;

// https://drafts.csswg.org/css-ui/#widget-accent
#[from_syntax(auto | <color>)]
#[derive(Value, Writable, Default, PartialEq, Debug, Clone, Hash)]
#[value(Inherits)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AccentColor {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(AccentColor, 36);
	}
}
