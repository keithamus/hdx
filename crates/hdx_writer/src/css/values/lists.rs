use hdx_ast::css::values::{lists::*, Shorthand};

use crate::{CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for ListStyleShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(position) = &self.position {
			position.write_css(sink)?;
			if self.image.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(image) = &self.image {
			image.write_css(sink)?;
			if self.marker.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(marker) = &self.marker {
			marker.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for ListStyleImageValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Image(image) => image.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for ListStyleTypeValue<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::CounterStyle(c) => c.write_css(sink),
			Self::String(s) => {
				sink.write_char('"')?;
				sink.write_str(s.as_ref())?;
				sink.write_char('"')
			}
		}
	}
}
