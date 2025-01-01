use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	diagnostics, function_set, AtRule, Build, CommaSeparatedPreludeList, CursorSink, Parse, Parser,
	Result as ParserResult, RuleList, ToCursors, T,
};
use hdx_proc_macro::visit;

use crate::{stylesheet::Rule, Visit, Visitable};

// https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct DocumentRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub matchers: DocumentMatcherList<'a>,
	pub block: DocumentRuleBlock<'a>,
}
// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for DocumentRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, matchers, block) = Self::parse_at_rule(p)?;
		if let Some(matchers) = matchers {
			Ok(Self { at_keyword, matchers, block })
		} else {
			let c: Cursor = at_keyword.into();
			Err(diagnostics::MissingAtRulePrelude(c.into()))?
		}
	}
}

impl<'a> AtRule<'a> for DocumentRule<'a> {
	const NAME: Option<&'static str> = Some("document");
	type Prelude = DocumentMatcherList<'a>;
	type Block = DocumentRuleBlock<'a>;
}

impl ToCursors for DocumentRule<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		ToCursors::to_cursors(&self.matchers, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a> Visitable<'a> for DocumentRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_document_rule(self);
		Visitable::accept(&self.matchers, v);
		Visitable::accept(&self.block, v);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DocumentMatcherList<'a>(pub Vec<'a, (DocumentMatcher, Option<T![,]>)>);

impl<'a> CommaSeparatedPreludeList<'a> for DocumentMatcherList<'a> {
	type PreludeItem = DocumentMatcher;
}

impl<'a> Parse<'a> for DocumentMatcherList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl ToCursors for DocumentMatcherList<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for (selector, comma) in &self.0 {
			ToCursors::to_cursors(selector, s);
			if let Some(comma) = comma {
				s.append(comma.into());
			}
		}
	}
}

impl<'a> Visitable<'a> for DocumentMatcherList<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (matcher, _) in &self.0 {
			Visitable::accept(matcher, v);
		}
	}
}

function_set!(DocumentMatcherFunctionKeyword {
	Url: "url",
	UrlPrefix: "url-prefix",
	Domain: "domain",
	MediaDocument: "media-document",
	Regexp: "regexp",
});

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum DocumentMatcher {
	Url(T![Url]),
	UrlFunction(T![Function], T![String], T![')']),
	UrlPrefix(T![Function], T![String], T![')']),
	Domain(T![Function], T![String], T![')']),
	MediaDocument(T![Function], T![String], T![')']),
	Regexp(T![Function], T![String], T![')']),
}

impl<'a> Parse<'a> for DocumentMatcher {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Url]>() {
			Ok(Self::Url(p.parse::<T![Url]>()?))
		} else {
			let keyword = p.parse::<DocumentMatcherFunctionKeyword>()?;
			let c = keyword.into();
			let function = <T![Function]>::build(p, c);
			match keyword {
				DocumentMatcherFunctionKeyword::Url(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlFunction(function, string, close))
				}
				DocumentMatcherFunctionKeyword::UrlPrefix(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::Domain(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::MediaDocument(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::Regexp(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
			}
		}
	}
}

impl<'a> ToCursors for DocumentMatcher {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Url(url) => s.append(url.into()),
			Self::UrlFunction(function, string, close)
			| Self::UrlPrefix(function, string, close)
			| Self::Domain(function, string, close)
			| Self::MediaDocument(function, string, close)
			| Self::Regexp(function, string, close) => {
				s.append(function.into());
				s.append(string.into());
				s.append(close.into());
			}
		}
	}
}

impl<'a> Visitable<'a> for DocumentMatcher {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_document_matcher(self);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct DocumentRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for DocumentRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

impl<'a> RuleList<'a> for DocumentRuleBlock<'a> {
	type Rule = Rule<'a>;
}

impl ToCursors for DocumentRuleBlock<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for rule in &self.rules {
			ToCursors::to_cursors(rule, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

impl<'a> Visitable<'a> for DocumentRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DocumentRule>(), 112);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DocumentRule, r#"@document url("http://www.w3.org"){}"#);
		assert_parse!(DocumentRule, r#"@document domain("mozilla.org"){}"#);
		assert_parse!(DocumentRule, r#"@document url-prefix("http://www.w3.org/Style/"){}"#);
		assert_parse!(DocumentRule, r#"@document media-document("video"){}"#);
		assert_parse!(DocumentRule, r#"@document regexp("https:.*"){}"#);
		assert_parse!(
			DocumentRule,
			r#"@document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){}"#
		);
		assert_parse!(
			DocumentRule,
			r#"@document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
