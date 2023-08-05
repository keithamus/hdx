use hdx_ast::css::values::images::*;

use crate::{CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for Image<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		todo!()
	}
}
