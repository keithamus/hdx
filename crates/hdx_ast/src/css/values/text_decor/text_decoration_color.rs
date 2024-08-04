use hdx_atom::atom;
use hdx_derive::{Value, Writable};
use hdx_lexer::Kind;
use hdx_parser::{Parse, Parser, Result as ParserResult};

use crate::css::types::Color;

// https://drafts.csswg.org/css-text-decor/#text-decoration-color-property
#[value(" <color> ")]
#[initial("currentcolor")]
#[applies_to("all elements")]
#[inherited("no")]
#[computed_value("computed color")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub struct TextDecorationColor;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(TextDecorationColor, 36);
	}
}
