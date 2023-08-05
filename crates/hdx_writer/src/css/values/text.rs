use hdx_ast::css::values::{text::*, Shorthand};

use crate::{CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for WhiteSpaceTrimValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::None => sink.write_str("none"),
			Self::Discard { inner, after, before } => {
				if *inner {
					sink.write_str("discard-inner")?;
					if *before {
						sink.write_char(' ')?;
					}
				}
				if *before {
					sink.write_str("discard-before")?;
					if *after {
						sink.write_char(' ')?;
					}
				}
				if *after {
					sink.write_str("discard-after")?;
				}
				Ok(())
			}
		}
	}
}

impl<'a> WriteCss<'a> for WhiteSpaceShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal")?,
			Self::Pre => sink.write_str("pre")?,
			Self::Nowrap => sink.write_str("nowrap")?,
			Self::PreWrap => sink.write_str("pre-wrap")?,
			Self::PreLine => sink.write_str("pre-line")?,
			Self::Expanded { collapse, trim, wrap } => {
				if let Shorthand::Explicit(value) = collapse {
					value.write_css(sink)?;
					if trim.is_explicit() {
						sink.write_char(' ')?;
					}
				}
				if let Shorthand::Explicit(value) = trim {
					value.write_css(sink)?;
					if wrap.is_explicit() {
						sink.write_char(' ')?;
					}
				}
				if let Shorthand::Explicit(value) = wrap {
					value.write_css(sink)?;
				}
			}
		}
		Ok(())
	}
}
