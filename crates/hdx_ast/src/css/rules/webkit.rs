use hdx_atom::atom;
use hdx_lexer::Span;
use hdx_parser::{diagnostics, AtRule, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::{KeyframeBlock, KeyframeName};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct WebkitKeyframes<'a> {
	at_keyword: T![AtKeyword],
	name: KeyframeName,
	rules: KeyframeBlock<'a>,
}

impl<'a> Parse<'a> for WebkitKeyframes<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		match Self::parse_at_rule(p, Some(atom!("-webkit-keyframes")))? {
			(at_keyword, Some(name), Some(rules)) => Ok(Self { at_keyword, name, rules }),
			(_, Some(_), None) => Err(diagnostics::MissingAtRuleBlock(Span::new(start, p.offset())))?,
			(_, None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?,
			(_, None, None) => Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?,
		}
	}
}

impl<'a> AtRule<'a> for WebkitKeyframes<'a> {
	type Prelude = KeyframeName;
	type Block = KeyframeBlock<'a>;
}

impl<'a> ToCursors<'a> for WebkitKeyframes<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		s.append(self.name.into());
		ToCursors::to_cursors(&self.rules, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(WebkitKeyframes, 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WebkitKeyframes, "@-webkit-keyframes foo {}");
		assert_parse!(WebkitKeyframes, "@-webkit-keyframes \"include\" {}");
		assert_parse!(
			WebkitKeyframes,
			"@-webkit-keyframes spin {\n\t0% {\n\t\trotate: 0deg;\n\t}\n\n\t100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
		assert_parse!(
			WebkitKeyframes,
			"@-webkit-keyframes spin {\n\tfrom, 0% {\n\t\trotate: 0deg;\n\t}\n\n\tto, 100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
	}
}
