use hdx_ast::css::values::{inline::*, Shorthand};

use crate::{Atomizable, CssWriter, Result, WriteCss};

impl<'a> WriteCss<'a> for BaselineShiftValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Sub => sink.write_str("sub"),
			Self::Super => sink.write_str("super"),
			Self::Top => sink.write_str("top"),
			Self::Center => sink.write_str("center"),
			Self::Bottom => sink.write_str("bottom"),
			Self::LengthPercentage(val) => val.write_css(sink),
		}
	}
}

impl<'a> WriteCss<'a> for VerticalAlignShorthand<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		if let Shorthand::Explicit(baseline_source) = &self.baseline_source {
			baseline_source.write_css(sink)?;
			if self.alignment_baseline.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(alignment_baseline) = &self.alignment_baseline {
			alignment_baseline.write_css(sink)?;
			if self.baseline_shift.is_explicit() {
				sink.write_char(' ')?;
			}
		}
		if let Shorthand::Explicit(baseline_shift) = &self.baseline_shift {
			baseline_shift.write_css(sink)?;
		}
		Ok(())
	}
}

impl<'a> WriteCss<'a> for LineHeightValue {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> Result {
		match self {
			Self::Normal => sink.write_str("normal"),
			Self::Number(n) => sink.write_str(n.to_string().as_str()),
			Self::LengthPercentage(n) => n.write_css(sink),
		}
	}
}
