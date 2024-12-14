use hdx_atom::atom;
use hdx_lexer::Span;
use hdx_parser::{diagnostics, AtRule, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

use super::{KeyframesBlock, KeyframesName};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct WebkitKeyframesRule<'a> {
	at_keyword: T![AtKeyword],
	name: KeyframesName,
	block: KeyframesBlock<'a>,
}

impl<'a> Parse<'a> for WebkitKeyframesRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, name, block) = Self::parse_at_rule(p, Some(atom!("-webkit-keyframes")))?;
		if let Some(name) = name {
			Ok(Self { at_keyword, name, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for WebkitKeyframesRule<'a> {
	type Prelude = KeyframesName;
	type Block = KeyframesBlock<'a>;
}

impl<'a> ToCursors for WebkitKeyframesRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		s.append(self.name.into());
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for WebkitKeyframesRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		todo!();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WebkitKeyframesRule, 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WebkitKeyframesRule, "@-webkit-keyframes foo{}");
		assert_parse!(WebkitKeyframesRule, "@-webkit-keyframes\"include\"{}");
		assert_parse!(WebkitKeyframesRule, "@-webkit-keyframes spin{to{transform:rotate(360deg)}}");
	}
}
