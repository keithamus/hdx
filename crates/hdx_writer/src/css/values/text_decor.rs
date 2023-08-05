use hdx_ast::css::values::{text_decor::*, Shorthand};

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for TextDecorationLineValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Style { underline, overline, line_through, blink } => {
				if *underline {
					sink.write_str("underline")?;
				}
				if *overline {
					if *underline {
						sink.write_char(' ')?;
					}
					sink.write_str("overline")?;
				}
				if *line_through {
					if *underline || *overline {
						sink.write_char(' ')?;
					}
					sink.write_str("line-through")?;
				}
				if *blink {
					if *underline || *overline || *line_through {
						sink.write_char(' ')?;
					}
					sink.write_str("blink")?;
				}
				Ok(())
			}
		}
	}
}

impl<'a> WriteCss<'a> for TextDecorationShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(color) = &self.color {
			color.write_css(sink)?;
			if self.style.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(line) = &self.line {
			line.write_css(sink)?;
			if self.style.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(style) = &self.style {
			style.write_css(sink)?;
		}
		Ok(())
	}
}
