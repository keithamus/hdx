use std::{fmt::Debug, hash::Hash};

use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{
	CursorStream, Declaration, DeclarationValue, Important, Is, Parse, Parser, Result as ParserResult, State,
	ToCursors, T,
};

use crate::{css::values, syntax::ComponentValues};

// The build.rs generates a list of CSS properties from the value mods
include!(concat!(env!("OUT_DIR"), "/css_apply_properties.rs"));

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Custom<'a>(pub ComponentValues<'a>);

impl<'a> Parse<'a> for Custom<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let state = p.set_state(State::Nested);
		let stop = p.set_stop(KindSet::RIGHT_CURLY_OR_SEMICOLON);
		let value = p.parse::<ComponentValues>();
		p.set_state(state);
		p.set_stop(stop);
		Ok(Self(value?))
	}
}

impl<'a> ToCursors<'a> for Custom<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		ToCursors::to_cursors(&self.0, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> Is<'a> for Computed<'a> {
	fn is(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::is(p, c)
			&& matches!(
				p.parse_atom_lower(c),
				atom!("var")
					| atom!("calc") | atom!("min")
					| atom!("max") | atom!("clamp")
					| atom!("round")
					| atom!("mod") | atom!("rem")
					| atom!("sin") | atom!("cos")
					| atom!("tan") | atom!("asin")
					| atom!("atan") | atom!("atan2")
					| atom!("pow") | atom!("sqrt")
					| atom!("hypot")
					| atom!("log") | atom!("exp")
					| atom!("abs") | atom!("sign")
			)
	}
}

impl<'a> Parse<'a> for Computed<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let state = p.set_state(State::Nested);
		let stop = p.set_stop(KindSet::RIGHT_CURLY_OR_SEMICOLON);
		let values = p.parse::<ComponentValues>();
		p.set_state(state);
		p.set_stop(stop);
		Ok(Self(values?))
	}
}

impl<'a> ToCursors<'a> for Computed<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		ToCursors::to_cursors(&self.0, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Unknown<'a>(pub ComponentValues<'a>);

impl<'a> Parse<'a> for Unknown<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let state = p.set_state(State::Nested);
		let stop = p.set_stop(KindSet::RIGHT_CURLY_OR_SEMICOLON);
		let values = p.parse::<ComponentValues>();
		p.set_state(state);
		p.set_stop(stop);
		Ok(Self(values?))
	}
}

impl<'a> ToCursors<'a> for Unknown<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		ToCursors::to_cursors(&self.0, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
pub struct Property<'a> {
	pub name: T![Ident],
	pub colon: Option<T![:]>,
	pub value: StyleValue<'a>,
	pub important: Option<Important>,
	pub semicolon: Option<T![;]>,
}

impl<'a> Parse<'a> for Property<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important, semicolon) = Self::parse_declaration(p)?;
		Ok(Self { name, colon, value, important, semicolon })
	}
}

impl<'a> Declaration<'a> for Property<'a> {
	type DeclarationValue = StyleValue<'a>;
}

impl<'a> ToCursors<'a> for Property<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		s.append(self.name.into());
		if let Some(colon) = self.colon {
			s.append(colon.into());
		}
		ToCursors::to_cursors(&self.value, s);
		if let Some(important) = &self.important {
			ToCursors::to_cursors(important, s);
		}
		if let Some(semicolon) = self.semicolon {
			s.append(semicolon.into());
		}
	}
}

macro_rules! style_value {
    ( $(
        $name: ident$(<$a: lifetime>)?: $atom: pat,
    )+ ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		pub enum StyleValue<'a> {
			Initial(T![Ident]),
			Inherit(T![Ident]),
			Unset(T![Ident]),
			Revert(T![Ident]),
			RevertLayer(T![Ident]),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Custom(Custom<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Computed(Computed<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Unknown(Unknown<'a>),
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(values::$name$(<$a>)?),
			)+
		}
	}
}

apply_properties!(style_value);

impl<'a> DeclarationValue<'a> for StyleValue<'a> {
	fn parse_declaration_value(name: Cursor, p: &mut Parser<'a>) -> ParserResult<Self> {
		if name.token().is_dashed_ident() {
			return Ok(Self::Custom(p.parse::<Custom>()?));
		}
		if p.peek::<T![Ident]>() {
			let c = p.peek_n(1);
			match p.parse_atom_lower(c) {
				atom!("initial") => return Ok(Self::Initial(p.parse::<T![Ident]>()?)),
				atom!("inherit") => return Ok(Self::Inherit(p.parse::<T![Ident]>()?)),
				atom!("unset") => return Ok(Self::Unset(p.parse::<T![Ident]>()?)),
				atom!("revert") => return Ok(Self::Revert(p.parse::<T![Ident]>()?)),
				atom!("revert-layer") => return Ok(Self::RevertLayer(p.parse::<T![Ident]>()?)),
				_ => {}
			}
		}
		if p.peek::<Computed>() {
			return p.parse::<Computed>().map(Self::Computed);
		}
		let checkpoint = p.checkpoint();
		macro_rules! parse_declaration_value {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match p.parse_atom_lower(name) {
					$(
						$atom => {
							if let Ok(val) = p.parse::<values::$name>() {
								if p.at_end() {
									return Ok(Self::$name(val))
								} else if p.peek_n(1) == KindSet::RIGHT_CURLY_OR_SEMICOLON || p.peek::<T![!]>() {
									return Ok(Self::$name(val))
								}
							}
						},
					)+
					_ => {}
				}
			}
		}
		apply_properties!(parse_declaration_value);
		if p.peek::<Computed>() {
			p.rewind(checkpoint);
			Ok(Self::Computed(p.parse::<Computed>()?))
		} else {
			p.rewind(checkpoint);
			Ok(Self::Unknown(p.parse::<Unknown>()?))
		}
	}
}

impl<'a> ToCursors<'a> for StyleValue<'a> {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		macro_rules! match_value {
			( $(
				$name: ident$(<$a: lifetime>)?: $atom: pat,
			)+ ) => {
				match self {
					Self::Initial(ident) => s.append(ident.into()),
					Self::Inherit(ident) => s.append(ident.into()),
					Self::Unset(ident) => s.append(ident.into()),
					Self::Revert(ident) => s.append(ident.into()),
					Self::RevertLayer(ident) => s.append(ident.into()),
					Self::Custom(custom) => ToCursors::to_cursors(custom, s),
					Self::Computed(computed) => ToCursors::to_cursors(computed, s),
					Self::Unknown(unknown) => ToCursors::to_cursors(unknown, s),
					$( Self::$name(value) => ToCursors::to_cursors(value, s),)+
				}
			}
		}
		apply_properties!(match_value);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(Property, 360);
		assert_size!(StyleValue, 296);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Property, "float:none!important");
		assert_parse!(Property, "width:1px");
		assert_parse!(Property, "width:min(1px, 2px)");
		assert_parse!(Property, "border:1px solid var(--red)");
		// Should still parse unknown properties
		assert_parse!(Property, "dunno:like whatever");
		assert_parse!(Property, "rotate:1.21gw");
	}
}
