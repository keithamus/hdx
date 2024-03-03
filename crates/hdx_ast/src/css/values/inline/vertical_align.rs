use hdx_parser::{FromToken, Parse, Parser, Result as ParserResult, Span, Spanned};

use crate::{macros::*, Value};

use super::{AlignmentBaseline, BaselineShift, BaselineSource};

// https://drafts.csswg.org/css-box-4/#padding-physical
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct VerticalAlign(pub Spanned<BaselineSource>, pub Spanned<AlignmentBaseline>, pub Spanned<BaselineShift>);

impl<'a> Parse<'a> for VerticalAlign {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let span = parser.span();
		let baseline_source = if let Some(baseline_source) = BaselineSource::from_token(parser.cur()) {
			// "auto" keyword is not allowed in VerticalAlign shorthand
			if baseline_source != BaselineSource::default() {
				parser.advance();
				Spanned { node: baseline_source, span }
			} else {
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

write_simple_shorthand!(VerticalAlign, Spanned<BaselineSource>, Spanned<AlignmentBaseline>, Spanned<BaselineShift>);

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
