use hdx_ast::css::values::size_adjust::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for TextSizeAdjustValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Auto => sink.write_str("auto"),
			Self::Percentage(val) => sink.write_str(&format!("{}%", val)),
		}
	}
}
