use hdx_derive::Value;
use hdx_parser::{Parse, Parser, Result as ParserResult, Span, Spanned};
use hdx_writer::{CssWriter, OutputOption, Result as WriterResult, WriteCss};

use super::{AlignmentBaseline, BaselineShift, BaselineSource};

// https://drafts.csswg.org/css-box-4/#padding-physical
#[derive(Value, Default, PartialEq, Debug, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct VerticalAlign(pub Spanned<BaselineSource>, pub Spanned<AlignmentBaseline>, pub Spanned<BaselineShift>);

impl<'a> Parse<'a> for VerticalAlign {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let span = parser.span();
		let checkpoint = parser.checkpoint();
		let baseline_source = if let Ok(baseline_source) = BaselineSource::try_parse(parser) {
			// "auto" keyword is not allowed in VerticalAlign shorthand
			if baseline_source != BaselineSource::default() {
				Spanned { node: baseline_source, span }
			} else {
				parser.rewind(checkpoint);
				Spanned { node: BaselineSource::default(), span: Span::dummy() }
			}
		} else {
			Spanned { node: BaselineSource::default(), span: Span::dummy() }
		};
		let alignment_baseline = if let Ok(alignment_baseline) = AlignmentBaseline::parse_spanned(parser) {
			alignment_baseline
		} else {
			Spanned { node: AlignmentBaseline::default(), span: Span::dummy() }
		};
		let baseline_shift = if let Ok(baseline_shift) = BaselineShift::parse_spanned(parser) {
			baseline_shift
		} else {
			Spanned { node: BaselineShift::default(), span: Span::dummy() }
		};
		Ok(Self(baseline_source, alignment_baseline, baseline_shift))
	}
}

impl<'a> WriteCss<'a> for VerticalAlign {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut wrote = false;
		if self.0 != <Spanned<BaselineSource>>::default() {
			self.0.write_css(sink)?;
			wrote = true
		}
		if self.1 != <Spanned<AlignmentBaseline>>::default() || sink.can_output(OutputOption::RedundantDefaultValues) {
			if wrote {
				sink.write_char(' ')?;
			}
			self.1.write_css(sink)?;
			wrote = true
		}
		if self.2 != <Spanned<BaselineShift>>::default() {
			if wrote {
				sink.write_char(' ')?;
			}
			self.2.write_css(sink)?;
			wrote = true
		}
		if !wrote {
			self.0.write_css(sink)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(VerticalAlign, 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(VerticalAlign, "baseline");
		assert_parse!(VerticalAlign, "first baseline");
		assert_parse!(VerticalAlign, "first text-top bottom");
	}
}
