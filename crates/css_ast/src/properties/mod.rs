use crate::values;
use css_lexer::{Cursor, Kind, KindSet};
use css_parse::{
	keyword_set, syntax::BangImportant, syntax::ComponentValues, Build, CursorSink, Declaration, DeclarationValue,
	Parse, Parser, Peek, Result as ParserResult, State, ToCursors, T,
};
use csskit_proc_macro::visit;
use std::{fmt::Debug, hash::Hash};

use super::{Visit, Visitable};

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

impl<'a> ToCursors for Custom<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> Peek<'a> for Computed<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c)
			&& matches!(
				p.parse_str_lower(c),
				"var"
					| "calc" | "min"
					| "max" | "clamp"
					| "round" | "mod"
					| "rem" | "sin" | "cos"
					| "tan" | "asin"
					| "atan" | "atan2"
					| "pow" | "sqrt"
					| "hypot" | "log"
					| "exp" | "abs" | "sign"
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

impl<'a> ToCursors for Computed<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
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

impl<'a> ToCursors for Unknown<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
#[visit]
pub struct Property<'a> {
	pub name: T![Ident],
	pub colon: T![:],
	pub value: StyleValue<'a>,
	pub important: Option<BangImportant>,
}

impl<'a> Peek<'a> for Property<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) && p.peek_n(2) == Kind::Colon
	}
}

impl<'a> Parse<'a> for Property<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (name, colon, value, important) = Self::parse_declaration(p)?;
		Ok(Self { name, colon, value, important })
	}
}

impl<'a> Declaration<'a> for Property<'a> {
	type DeclarationValue = StyleValue<'a>;
}

impl ToCursors for Property<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.name.into());
		s.append(self.colon.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(important) = &self.important {
			ToCursors::to_cursors(important, s);
		}
	}
}

impl<'a> Visitable<'a> for Property<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_property(self);
		Visitable::accept(&self.value, v);
	}
}

macro_rules! style_value {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		#[visit]
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
				$name(values::$ty$(<$a>)?),
			)+
		}
	}
}

apply_properties!(style_value);

keyword_set!(CSSWideKeyword {
	Initial: "initial",
	Inherit: "inherit",
	Unset: "unset",
	Revert: "revert",
	RevertLayer: "revert-layer",
});

macro_rules! define_property_id {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		keyword_set!(PropertyId {
			$($name: $str,)+
		});
	}
}
apply_properties!(define_property_id);

impl<'a> DeclarationValue<'a> for StyleValue<'a> {
	fn parse_declaration_value(p: &mut Parser<'a>, name: Cursor) -> ParserResult<Self> {
		if name.token().is_dashed_ident() {
			return Ok(Self::Custom(p.parse::<Custom>()?));
		}
		if let Some(kw) = p.parse_if_peek::<CSSWideKeyword>()? {
			match kw {
				CSSWideKeyword::Initial(c) => return Ok(Self::Initial(<T![Ident]>::build(p, c))),
				CSSWideKeyword::Inherit(c) => return Ok(Self::Inherit(<T![Ident]>::build(p, c))),
				CSSWideKeyword::Unset(c) => return Ok(Self::Unset(<T![Ident]>::build(p, c))),
				CSSWideKeyword::Revert(c) => return Ok(Self::Revert(<T![Ident]>::build(p, c))),
				CSSWideKeyword::RevertLayer(c) => return Ok(Self::RevertLayer(<T![Ident]>::build(p, c))),
			}
		}
		if p.peek::<Computed>() {
			return p.parse::<Computed>().map(Self::Computed);
		}
		let checkpoint = p.checkpoint();
		macro_rules! parse_declaration_value {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				if PropertyId::peek(p, name) {
					match PropertyId::build(p, name) {
						$(
							PropertyId::$name(_) => {
								if let Ok(val) = p.parse::<values::$ty>() {
									if p.at_end() || p.peek_n(1) == KindSet::RIGHT_CURLY_OR_SEMICOLON || p.peek::<T![!]>() {
										return Ok(Self::$name(val))
									}
								}
							},
						)+
					}
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

impl<'a> ToCursors for StyleValue<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		macro_rules! match_value {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match self {
					Self::Initial(ident) => s.append(ident.into()),
					Self::Inherit(ident) => s.append(ident.into()),
					Self::Unset(ident) => s.append(ident.into()),
					Self::Revert(ident) => s.append(ident.into()),
					Self::RevertLayer(ident) => s.append(ident.into()),
					Self::Custom(custom) => ToCursors::to_cursors(custom, s),
					Self::Computed(computed) => ToCursors::to_cursors(computed, s),
					Self::Unknown(unknown) => ToCursors::to_cursors(unknown, s),
					$( Self::$name(value) => ToCursors::to_cursors(value, s), )+
				}
			}
		}
		apply_properties!(match_value);
	}
}

impl<'a> Visitable<'a> for StyleValue<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_style_value(self);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Property>(), 384);
		assert_eq!(std::mem::size_of::<StyleValue>(), 328);
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
		assert_parse!(Property, "_background:black");
		assert_parse!(Property, "--custom:{foo:{bar};baz:(bing);}");
	}
}
