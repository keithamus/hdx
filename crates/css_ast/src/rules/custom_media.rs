use super::prelude::*;
use crate::MediaQueryList;

/// <https://drafts.csswg.org/mediaqueries-5/#at-ruledef-custom-media>
///
/// ```text,ignore
/// @custom-media <extension-name> [ <media-query-list> | true | false ] ;
/// ```
#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.at-rules.custom-media")
)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = CustomMedia)]
pub struct CustomMediaRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::CustomMedia)]
	pub name: T![AtKeyword],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub extension: T![DashedIdent],
	pub value: CustomMediaValue<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub semicolon: Option<T![;]>,
}

#[node]
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CustomMediaValue<'a> {
	#[atom(CssAtomSet::True)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	True(T![Ident]),
	#[atom(CssAtomSet::False)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	False(T![Ident]),
	MediaQueryList(MediaQueryList<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::StyleSheet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media --modern (hover),(pointer:fine);");
		assert_parse!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media --narrow screen and (max-width:30em);");
		assert_parse!(
			CssAtomSet::ATOMS,
			StyleSheet,
			"@custom-media --viewportRange-narrow (max-width: calc(48rem - 0.02px));",
			|sheet| assert!(matches!(sheet.rules[..], [Rule::CustomMedia(_)]))
		);
		assert_parse!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media --on true;", |rule| {
			assert!(matches!(rule.value, CustomMediaValue::True(_)))
		});
		assert_parse!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media --off false;", |rule| {
			assert!(matches!(rule.value, CustomMediaValue::False(_)))
		});
		assert_parse!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media --print print", |rule| {
			assert!(matches!(rule.value, CustomMediaValue::MediaQueryList(_)))
		});
		assert_parse!(
			CssAtomSet::ATOMS,
			StyleSheet,
			"@custom-media --modern (hover);@media(--modern){a{color:red}}",
			|sheet| assert!(matches!(sheet.rules[..], [Rule::CustomMedia(_), Rule::Media(_)]))
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media modern (color);");
		assert_parse_error!(CssAtomSet::ATOMS, CustomMediaRule, "@custom-media --modern;");
	}
}
