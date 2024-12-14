use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, AtRule, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

use super::{DocumentBlock, DocumentMatcherList};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct MozDocumentRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub matchers: DocumentMatcherList<'a>,
	pub block: DocumentBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for MozDocumentRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, matchers, block) = Self::parse_at_rule(p, Some(atom!("-moz-document")))?;
		if let Some(matchers) = matchers {
			Ok(Self { at_keyword, matchers, block })
		} else {
			let c: Cursor = at_keyword.into();
			Err(diagnostics::MissingAtRulePrelude(c.into()))?
		}
	}
}

impl<'a> AtRule<'a> for MozDocumentRule<'a> {
	type Prelude = DocumentMatcherList<'a>;
	type Block = DocumentBlock<'a>;
}

impl<'a> ToCursors for MozDocumentRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.matchers, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for MozDocumentRule<'a> {
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
		assert_size!(MozDocumentRule, 104);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MozDocumentRule, r#"@-moz-document url(http://www.w3.org){}"#);
		assert_parse!(MozDocumentRule, r#"@-moz-document url(http://www.w3.org),domain("mozilla.org"){}"#);
		assert_parse!(
			MozDocumentRule,
			r#"@-moz-document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
