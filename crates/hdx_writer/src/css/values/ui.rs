use hdx_ast::css::values::ui::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for CursorValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Custom(g) => todo!(),
			_ => sink.write_str(self.to_atom().as_ref()),
		}
	}
}
