use hdx_atom::atom;
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, AtRule, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::{DocumentBlock, DocumentMatcherList};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MozDocument<'a> {
	pub at_keyword: T![AtKeyword],
	pub matchers: DocumentMatcherList<'a>,
	pub block: DocumentBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for MozDocument<'a> {
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

impl<'a> AtRule<'a> for MozDocument<'a> {
	type Prelude = DocumentMatcherList<'a>;
	type Block = DocumentBlock<'a>;
}

impl<'a> ToCursors<'a> for MozDocument<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.matchers, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(MozDocument, 104);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MozDocument, r#"@-moz-document url(http://www.w3.org){}"#);
		assert_parse!(MozDocument, r#"@-moz-document url(http://www.w3.org),domain("mozilla.org"){}"#);
		assert_parse!(
			MozDocument,
			r#"@-moz-document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
