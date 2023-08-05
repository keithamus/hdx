use hdx_ast::css::values::page_floats::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for FloatValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Some(atom) = self.to_atom() {
			sink.write_str(atom.as_ref())?;
		} else if let FloatValue::SnapBlockFunction(first, second) = self {
			first.write_css(sink)?;
			sink.write_char(' ')?;
			sink.write_str(second.to_atom().as_ref())?;
		} else if let FloatValue::SnapInlineFunction(first, second) = self {
			first.write_css(sink)?;
			sink.write_char(' ')?;
			sink.write_str(second.to_atom().as_ref())?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for FloatDeferValue {
	fn write_css<W: CssWriter>(&self, _sink: &mut W) -> Result {
		todo!()
	}
}
