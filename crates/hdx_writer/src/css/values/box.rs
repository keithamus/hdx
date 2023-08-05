use hdx_ast::css::values::r#box::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for MarginTrimValue {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}
