use hdx_parser::{FromToken, Parse, Parser, Result as ParserResult, Span, Spanned};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{macros::*, Value};

use super::{AlignmentBaseline, BaselineShift, BaselineSource};

// https://drafts.csswg.org/css-box-4/#padding-physical
#[derive(Value, Default, PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
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
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<VerticalAlign>(), 40);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<VerticalAlign>(&allocator, "baseline", "baseline");
		test_write::<VerticalAlign>(&allocator, "first baseline", "first baseline");
		test_write::<VerticalAlign>(&allocator, "first text-top bottom", "first text-top bottom");
	}
}
