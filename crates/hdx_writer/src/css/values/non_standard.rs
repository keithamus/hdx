use hdx_ast::css::values::non_standard::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for ZoomValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal"),
			Self::Reset => sink.write_str("reset"),
			Self::Number(n) => sink.write_str(&n.to_string()),
			Self::Percentage(n) => sink.write_str(&format!("{}%", n)),
		}
	}
}
