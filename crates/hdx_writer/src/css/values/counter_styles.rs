use hdx_ast::css::values::counter_styles::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for CounterStyle<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		todo!()
	}
}
