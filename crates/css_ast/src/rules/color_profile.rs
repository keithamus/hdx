use super::prelude::*;
use crate::Url;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};

/// Represents the `@color-profile` at-rule, e.g. `@color-profile --swop5c { src: url(swop.icc) }`.
///
/// ```md
/// <color-profile-rule>
///  │├─ "@color-profile" ─╮─ <dashed-ident> ─╭─ <block> ─┤│
///                        ╰─ "device-cmyk" ──╯
/// ```
///
/// <https://drafts.csswg.org/css-color-5/#at-profile>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.at-rules.color-profile")
)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = ColorProfile, property_kinds = Name)]
pub struct ColorProfileRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::ColorProfile)]
	pub name: T![AtKeyword],
	pub prelude: ColorProfilePrelude,
	#[metadata(block)]
	pub block: ColorProfileRuleBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for ColorProfileRule<'a> {
	const NODE_ID: NodeId = NodeId::ColorProfileRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.ident()),
			_ => None,
		}
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorProfilePrelude {
	Name(T![DashedIdent]),
	#[atom(CssAtomSet::DeviceCmyk)]
	DeviceCmyk(T![Ident]),
}

impl ColorProfilePrelude {
	/// Returns a cursor to the profile name, e.g. `--swop5c` or `device-cmyk`.
	pub fn ident(&self) -> Cursor {
		match self {
			Self::Name(c) => (*c).into(),
			Self::DeviceCmyk(c) => (*c).into(),
		}
	}
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorProfileRuleBlock<'a>(pub DeclarationList<'a, ColorProfileRuleStyleValue<'a>, CssMetadata>);

#[node]
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorProfileRuleStyleValue<'a> {
	Src(ColorProfileSrcValue),
	RenderingIntent(RenderingIntentValue),
	Components(ComponentsValue<'a>),
	Unknown(ComponentValues<'a>),
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorProfileSrcValue(pub Url);

#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RenderingIntentValue {
	#[atom(CssAtomSet::RelativeColorimetric)]
	RelativeColorimetric(T![Ident]),
	#[atom(CssAtomSet::AbsoluteColorimetric)]
	AbsoluteColorimetric(T![Ident]),
	#[atom(CssAtomSet::Perceptual)]
	Perceptual(T![Ident]),
	#[atom(CssAtomSet::Saturation)]
	Saturation(T![Ident]),
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ComponentsValue<'a>(pub CommaSeparated<'a, T![Ident], 1>);

impl<'a> DeclarationValue<'a, CssMetadata> for ColorProfileRuleStyleValue<'a> {
	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::Src | CssAtomSet::RenderingIntent | CssAtomSet::Components)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn parse_declaration_value<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::Src => Self::Src(p.parse::<ColorProfileSrcValue>()?),
			CssAtomSet::RenderingIntent => Self::RenderingIntent(p.parse::<RenderingIntentValue>()?),
			CssAtomSet::Components => Self::Components(p.parse::<ComponentsValue<'a>>()?),
			_ => Self::Unknown(p.parse::<ComponentValues<'a>>()?),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorProfileRule, "@color-profile --swop5c{src:url(\"swop.icc\")}");
		assert_parse!(CssAtomSet::ATOMS, ColorProfileRule, "@color-profile device-cmyk{src:url(cmyk.icc)}");
		assert_parse!(
			CssAtomSet::ATOMS,
			ColorProfileRule,
			"@color-profile --unwise{src:url(unwise);components:mi,pi,ni;rendering-intent:perceptual}"
		);
	}
}
