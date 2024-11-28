use hdx_atom::{atom, Atom};
use hdx_lexer::Cursor;
use hdx_parser::{diagnostics, AtRule, Is, Parse, Parser, Result as ParserResult, ToCursors, Vec, T};

use crate::css::properties::Property;

pub mod kw {
	use hdx_parser::custom_keyword;
	custom_keyword!(From, atom!("from"));
	custom_keyword!(To, atom!("to"));
}

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Keyframes<'a> {
	at_keyword: T![AtKeyword],
	name: Option<KeyframeName>,
	rules: Option<KeyframeBlock<'a>>,
}

impl<'a> Parse<'a> for Keyframes<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, name, rules) = Self::parse_at_rule(p, Some(atom!("keyframes")))?;
		Ok(Self { at_keyword, name, rules })
	}
}

impl<'a> AtRule<'a> for Keyframes<'a> {
	type Prelude = KeyframeName;
	type Block = KeyframeBlock<'a>;
}

impl<'a> ToCursors<'a> for Keyframes<'a> {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
		s.append(self.at_keyword.into());
		if let Some(name) = self.name {
			s.append(name.into());
		}
		if let Some(rules) = &self.rules {
			ToCursors::to_cursors(rules, s);
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum KeyframeName {
	Ident(T![Ident]),
	String(T![String]),
}

impl KeyframeName {
	fn valid_ident(atom: &Atom) -> bool {
		!matches!(
			atom.to_ascii_lowercase(),
			atom!("default") | atom!("initial") | atom!("inherit") | atom!("unset") | atom!("none")
		)
	}
}

impl<'a> Is<'a> for KeyframeName {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		<T![Ident]>::is(p, c) || <T![String]>::is(p, c)
	}
}

impl<'a> Parse<'a> for KeyframeName {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Ok(str) = p.parse::<T![String]>() {
			return Ok(Self::String(str));
		}
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let atom = p.parse_atom(c);
		if !KeyframeName::valid_ident(&atom) {
			Err(diagnostics::ReservedKeyframeName(atom, c.into()))?
		}
		Ok(Self::Ident(ident))
	}
}

impl From<KeyframeName> for Cursor {
	fn from(value: KeyframeName) -> Self {
		match value {
			KeyframeName::String(c) => c.into(),
			KeyframeName::Ident(c) => c.into(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeBlock<'a> {
	pub open: T!['{'],
	pub keyframes: Vec<'a, Keyframe<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for KeyframeBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse::<T!['{']>()?;
		let mut keyframes = Vec::new_in(p.bump());
		loop {
			if p.at_end() || p.peek::<T!['{']>() {
				break;
			}
			keyframes.push(p.parse::<Keyframe>()?);
		}
		let close = p.parse_if_peek::<T!['}']>()?;
		Ok(Self { open, keyframes, close })
	}
}

impl<'a> ToCursors<'a> for KeyframeBlock<'a> {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
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
	selectors: Vec<'a, KeyframeSelector>,
	open: T!['{'],
	properties: Vec<'a, Property<'a>>,
	close: Option<T!['}']>,
}

impl<'a> Parse<'a> for Keyframe<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut selectors = Vec::new_in(p.bump());
		loop {
			selectors.push(p.parse::<KeyframeSelector>()?);
			if p.at_end() || p.peek::<T!['{']>() {
				break;
			}
		}
		let open = p.parse::<T!['{']>()?;
		let mut properties = Vec::new_in(p.bump());
		loop {
			if p.at_end() || p.peek::<T!['}']>() {
				break;
			}
			properties.push(p.parse::<Property>()?);
		}
		let close = p.parse_if_peek::<T!['}']>()?;
		Ok(Self { selectors, open, properties, close })
	}
}

impl<'a> ToCursors<'a> for Keyframe<'a> {
	fn to_cursors(&self, s: &mut hdx_parser::CursorStream<'a>) {
		for selector in &self.selectors {
			s.append((*selector).into());
		}
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Keyframes, 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Keyframes, "@keyframes foo {}");
		assert_parse!(Keyframes, "@keyframes \"include\" {}");
		assert_parse!(
			Keyframes,
			"@keyframes spin {\n\t0% {\n\t\trotate: 0deg;\n\t}\n\n\t100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
		assert_parse!(
			Keyframes,
			"@keyframes spin {\n\tfrom, 0% {\n\t\trotate: 0deg;\n\t}\n\n\tto, 100% {\n\t\trotate: 360deg;\n\t}\n}"
		);
		assert_parse!(
			Keyframes,
			"@keyframes spin {to{transform:rotate(360deg)}}",
			"@keyframes spin {\n\tto {\n\t\ttransform: rotate(360deg);\n\t}\n}"
		);
		assert_parse!(
			Keyframes,
			"@keyframes x {\n\tto {\n\t\tanimation-timing-function: cubic-bezier(0, 0, 0.2, 1);\n\t}\n}"
		);
	}
}
