use hdx_ast::css::values::sizing::*;

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for Sizing {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Auto => sink.write_str("auto"),
			Self::LengthPercentage(val) => val.write_css(sink),
			Self::MinContent => sink.write_str("min-content"),
			Self::MaxContent => sink.write_str("max-content"),
			Self::FitContentFunction(val) => {
				sink.write_str("fit-content(")?;
				val.write_css(sink)?;
				sink.write_char(')')
			}
			Self::Stretch => sink.write_str("stretch"),
			Self::FitContent => sink.write_str("fit-content"),
			Self::Contain => sink.write_str("contain"),
		}
	}
}

impl<'a> WriteCss<'a> for MaxSizing {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::LengthPercentage(val) => val.write_css(sink),
			Self::MinContent => sink.write_str("min-content"),
			Self::MaxContent => sink.write_str("max-content"),
			Self::FitContentFunction(val) => {
				sink.write_str("fit-content(")?;
				val.write_css(sink)?;
				sink.write_char(')')
			}
			Self::Stretch => sink.write_str("stretch"),
			Self::FitContent => sink.write_str("fit-content"),
			Self::Contain => sink.write_str("contain"),
		}
	}
}
