use hdx_ast::css::values::shorthand::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a, T: WriteCss<'a>> WriteCss<'a> for BoxShorthand<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match &self.top {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => val.write_css(sink)?,
		}
		match &self.right {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		match &self.bottom {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		match &self.left {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		Ok(())
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for XYShorthand<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match &self.x {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => val.write_css(sink)?,
		}
		match &self.y {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		Ok(())
	}
}

impl<'a, T: WriteCss<'a>> WriteCss<'a> for DoubleShorthand<'a, T> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match &self.0 {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => val.write_css(sink)?,
		}
		match &self.1 {
			Shorthand::Implicit => {
				return Ok(());
			}
			Shorthand::Explicit(val) => {
				sink.write_char(' ')?;
				val.write_css(sink)?;
			}
		}
		Ok(())
	}
}
