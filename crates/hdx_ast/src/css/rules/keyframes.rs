use hdx_atom::{atom, Atom};
use hdx_lexer::Cursor;
use hdx_parser::{
	diagnostics, AtRule, CursorSink, DeclarationList, Is, Parse, Parser, PreludeCommaList, QualifiedRule,
	QualifiedRuleList, Result as ParserResult, ToCursors, Vec, T,
};
use hdx_proc_macro::visit;

use crate::{
	css::{properties::Property, Visit, Visitable},
	syntax::BadDeclaration,
};

pub mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(From, atom!("from"));
	custom_keyword!(To, atom!("to"));
}

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct KeyframesRule<'a> {
	at_keyword: T![AtKeyword],
	name: Option<KeyframesName>,
	rules: KeyframesBlock<'a>,
}

impl<'a> AtRule<'a> for KeyframesRule<'a> {
	type Prelude = KeyframesName;
	type Block = KeyframesBlock<'a>;
}

impl<'a> Parse<'a> for KeyframesRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, name, rules) = Self::parse_at_rule(p, Some(atom!("keyframes")))?;
		Ok(Self { at_keyword, name, rules })
	}
}

impl<'a> ToCursors for KeyframesRule<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.at_keyword.into());
		if let Some(name) = self.name {
			s.append(name.into());
		}
		ToCursors::to_cursors(&self.rules, s);
	}
}

impl<'a> Visitable<'a> for KeyframesRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		todo!();
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum KeyframesName {
	Ident(T![Ident]),
	String(T![String]),
}

impl KeyframesName {
	fn valid_ident(atom: &Atom) -> bool {
		!matches!(
			atom.to_ascii_lowercase(),
			atom!("default") | atom!("initial") | atom!("inherit") | atom!("unset") | atom!("none")
		)
	}
}

impl<'a> Is<'a> for KeyframesName {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		<T![Ident]>::is(p, c) || <T![String]>::is(p, c)
	}
}

// Must use Parse rather than Build so ReservedKeyframeName errors can be emitted
impl<'a> Parse<'a> for KeyframesName {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![String]>() {
			return Ok(Self::String(p.parse::<T![String]>()?));
		}
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let atom = p.parse_atom(c);
		if !KeyframesName::valid_ident(&atom) {
			Err(diagnostics::ReservedKeyframeName(atom, c.into()))?
		}
		Ok(Self::Ident(ident))
	}
}

impl From<KeyframesName> for Cursor {
	fn from(value: KeyframesName) -> Self {
		match value {
			KeyframesName::String(c) => c.into(),
			KeyframesName::Ident(c) => c.into(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframesBlock<'a> {
	pub open: T!['{'],
	pub keyframes: Vec<'a, Keyframe<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> QualifiedRuleList<'a> for KeyframesBlock<'a> {
	type QualifiedRule = Keyframe<'a>;
}

impl<'a> Parse<'a> for KeyframesBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, keyframes, close) = Self::parse_qualified_rule_list(p)?;
		Ok(Self { open, keyframes, close })
	}
}

impl<'a> ToCursors for KeyframesBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for keyframe in &self.keyframes {
			ToCursors::to_cursors(keyframe, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Keyframe<'a> {
	selectors: KeyframeSelectors<'a>,
	block: KeyframeBlock<'a>,
}

impl<'a> QualifiedRule<'a> for Keyframe<'a> {
	type Block = KeyframeBlock<'a>;
	type Prelude = KeyframeSelectors<'a>;
	type BadDeclaration = BadDeclaration;
}

impl<'a> Parse<'a> for Keyframe<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (selectors, block) = Self::parse_qualified_rule(p)?;
		Ok(Self { selectors, block })
	}
}

impl<'a> ToCursors for Keyframe<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.selectors, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeSelectors<'a>(pub Vec<'a, (KeyframeSelector, Option<T![,]>)>);

impl<'a> PreludeCommaList<'a> for KeyframeSelectors<'a> {
	type PreludeItem = KeyframeSelector;
}

impl<'a> Parse<'a> for KeyframeSelectors<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

impl<'a> ToCursors for KeyframeSelectors<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for (selector, comma) in &self.0 {
			s.append(selector.into());
			if let Some(comma) = comma {
				s.append(comma.into());
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeBlock<'a> {
	open: T!['{'],
	properties: Vec<'a, Property<'a>>,
	close: Option<T!['}']>,
}

impl<'a> DeclarationList<'a> for KeyframeBlock<'a> {
	type Declaration = Property<'a>;
}

impl<'a> Parse<'a> for KeyframeBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_declaration_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> ToCursors for KeyframeBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.open.into());
		for property in &self.properties {
			ToCursors::to_cursors(property, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum KeyframeSelector {
	From(T![Ident]),
	To(T![Ident]),
	Percent(T![Dimension::%]),
}

impl<'a> Is<'a> for KeyframeSelector {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		kw::From::is(p, c) || kw::To::is(p, c) || <T![Dimension::%]>::is(p, c)
	}
}

impl<'a> Parse<'a> for KeyframeSelector {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(ident) = p.parse_if_peek::<T![Ident]>()? {
			let c: Cursor = ident.into();
			return match p.parse_atom_lower(c) {
				atom!("from") => Ok(Self::From(ident)),
				atom!("to") => Ok(Self::To(ident)),
				atom => Err(diagnostics::UnexpectedIdent(atom, c.into()))?,
			};
		}
		let percent = p.parse::<T![Dimension::%]>()?;
		let c: Cursor = percent.into();
		let f: f32 = c.token().value();
		if (0.0..=100.0).contains(&f) {
			Ok(Self::Percent(percent))
		} else {
			Err(diagnostics::NumberOutOfBounds(f, format!("{:?}", 0.0..=100.0), c.into()))?
		}
	}
}

impl From<KeyframeSelector> for Cursor {
	fn from(value: KeyframeSelector) -> Self {
		match value {
			KeyframeSelector::From(c) => c.into(),
			KeyframeSelector::To(c) => c.into(),
			KeyframeSelector::Percent(c) => c.into(),
		}
	}
}

impl From<&KeyframeSelector> for Cursor {
	fn from(value: &KeyframeSelector) -> Self {
		(*value).into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(KeyframesRule, 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(KeyframesRule, "@keyframes foo{}");
		assert_parse!(KeyframesRule, "@keyframes\"include\"{}");
		assert_parse!(KeyframesRule, "@keyframes spin{0%{rotate:0deg}100%{rotate:360deg}}");
		assert_parse!(KeyframesRule, "@keyframes spin{from,0%{rotate:0deg}to,100%{rotate:360deg}}");
		assert_parse!(KeyframesRule, "@keyframes spin{to{transform:rotate(360deg)}}");
		assert_parse!(KeyframesRule, "@keyframes x{to{animation-timing-function:cubic-bezier(0,0,0.2,1)}}");
	}
}
