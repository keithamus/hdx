use bumpalo::collections::Vec;
use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{diagnostics, Build, CursorSink, Is, Parse, Parser, Result as ParserResult, ToCursors, T};

use crate::css::{Visit, Visitable};

use super::{ForgivingSelector, Nth, RelativeSelector, SelectorList};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum FunctionalPseudoClass<'a> {
	Dir(DirPseudoFunction),
	Has(HasPseudoFunction<'a>),
	Host(HostPseudoFunction<'a>),
	HostContext(HostContextPseudoFunction<'a>),
	Is(IsPseudoFunction<'a>),
	Lang(LangPseudoFunction<'a>),
	Not(NotPseudoFunction<'a>),
	NthChild(NthChildPseudoFunction<'a>),
	NthCol(NthColPseudoFunction<'a>),
	NthLastChild(NthLastChildPseudoFunction<'a>),
	NthLastCol(NthLastColPseudoFunction<'a>),
	NthLastOfType(NthLastOfTypePseudoFunction<'a>),
	NthOfType(NthOfTypePseudoFunction<'a>),
	State(StatePseudoFunction),
	Where(WherePseudoFunction<'a>),
}

impl<'a> Parse<'a> for FunctionalPseudoClass<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colon = p.parse::<T![:]>();
		let function = p.parse::<T![Function]>();
		p.set_skip(skip);
		let colon = colon?;
		let function = function?;
		let c: Cursor = function.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("dir") => {
				let value = p.parse::<DirValue>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::Dir(DirPseudoFunction { colon, function, value, close })
			}
			atom!("has") => {
				let value = p.parse::<RelativeSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::Has(HasPseudoFunction { colon, function, value, close })
			}
			atom!("host") => {
				let value = p.parse::<SelectorList>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::Host(HostPseudoFunction { colon, function, value, close })
			}
			atom!("host-context") => {
				let value = p.parse::<SelectorList>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::HostContext(HostContextPseudoFunction { colon, function, value, close })
			}
			atom!("is") => {
				let value = p.parse::<ForgivingSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::HostContext(HostContextPseudoFunction { colon, function, value, close })
			}
			atom!("lang") => {
				let mut value = Vec::new_in(p.bump());
				loop {
					value.push(p.parse::<LangValue>()?);
					if p.peek::<T![')']>() {
						break;
					}
				}
				let close = p.parse_if_peek::<T![')']>()?;
				Self::Lang(LangPseudoFunction { colon, function, value, close })
			}
			atom!("not") => {
				let value = p.parse::<SelectorList>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::Not(NotPseudoFunction { colon, function, value, close })
			}
			atom!("nth-child") => {
				let value = p.parse::<Nth>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::NthChild(NthChildPseudoFunction { colon, function, value, close })
			}
			atom!("nth-col") => {
				let value = p.parse::<Nth>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::NthCol(NthColPseudoFunction { colon, function, value, close })
			}
			atom!("nth-last-child") => {
				let value = p.parse::<Nth>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::NthLastChild(NthLastChildPseudoFunction { colon, function, value, close })
			}
			atom!("nth-last-col") => {
				let value = p.parse::<Nth>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::NthLastCol(NthLastColPseudoFunction { colon, function, value, close })
			}
			atom!("nth-last-of-type") => {
				let value = p.parse::<Nth>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::NthLastOfType(NthLastOfTypePseudoFunction { colon, function, value, close })
			}
			atom!("nth-of-type") => {
				let value = p.parse::<Nth>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::NthOfType(NthOfTypePseudoFunction { colon, function, value, close })
			}
			atom!("where") => {
				let value = p.parse::<ForgivingSelector>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::Where(WherePseudoFunction { colon, function, value, close })
			}
			atom!("state") => {
				let value = p.parse::<T![Ident]>()?;
				let close = p.parse_if_peek::<T![')']>()?;
				Self::State(StatePseudoFunction { colon, function, value, close })
			}
			ident => Err(diagnostics::UnexpectedFunction(ident, c.into()))?,
		})
	}
}

impl<'a> ToCursors for FunctionalPseudoClass<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Dir(c) => ToCursors::to_cursors(c, s),
			Self::Has(c) => ToCursors::to_cursors(c, s),
			Self::Host(c) => ToCursors::to_cursors(c, s),
			Self::HostContext(c) => ToCursors::to_cursors(c, s),
			Self::Is(c) => ToCursors::to_cursors(c, s),
			Self::Lang(c) => ToCursors::to_cursors(c, s),
			Self::Not(c) => ToCursors::to_cursors(c, s),
			Self::NthChild(c) => ToCursors::to_cursors(c, s),
			Self::NthCol(c) => ToCursors::to_cursors(c, s),
			Self::NthLastChild(c) => ToCursors::to_cursors(c, s),
			Self::NthLastCol(c) => ToCursors::to_cursors(c, s),
			Self::NthLastOfType(c) => ToCursors::to_cursors(c, s),
			Self::NthOfType(c) => ToCursors::to_cursors(c, s),
			Self::State(c) => ToCursors::to_cursors(c, s),
			Self::Where(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl<'a> Visitable<'a> for FunctionalPseudoClass<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		todo!();
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DirPseudoFunction {
	pub colon: T![:],
	pub function: T![Function],
	pub value: DirValue,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for DirPseudoFunction {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		match self.value {
			DirValue::Rtl(c) => s.append(c.into()),
			DirValue::Ltr(c) => s.append(c.into()),
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DirValue {
	Rtl(T![Ident]),
	Ltr(T![Ident]),
}

impl<'a> Is<'a> for DirValue {
	fn is(p: &Parser<'a>, c: hdx_lexer::Cursor) -> bool {
		<T![Ident]>::is(p, c) && matches!(p.parse_atom_lower(c), atom!("rtl") | atom!("ltr"))
	}
}

impl<'a> Build<'a> for DirValue {
	fn build(p: &Parser<'a>, c: hdx_lexer::Cursor) -> Self {
		match p.parse_atom_lower(c) {
			atom!("rtl") => Self::Rtl(<T![Ident]>::build(p, c)),
			atom!("ltr") => Self::Ltr(<T![Ident]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

impl From<DirValue> for Cursor {
	fn from(value: DirValue) -> Cursor {
		match value {
			DirValue::Rtl(c) => c.into(),
			DirValue::Ltr(c) => c.into(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HasPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: RelativeSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for HasPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HostPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: SelectorList<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for HostPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct HostContextPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: SelectorList<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for HostContextPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct IsPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for IsPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LangPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Vec<'a, LangValue>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for LangPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		for value in &self.value {
			ToCursors::to_cursors(value, s);
		}
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LangValue {
	Ident(T![Ident], Option<T![,]>),
	String(T![String], Option<T![,]>),
}

impl<'a> Parse<'a> for LangValue {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() {
			let value = p.parse::<T![Ident]>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			Ok(Self::Ident(value, comma))
		} else {
			let value = p.parse::<T![String]>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			Ok(Self::String(value, comma))
		}
	}
}

impl<'a> ToCursors for LangValue {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Ident(value, comma) => {
				s.append(value.into());
				if let Some(comma) = comma {
					s.append(comma.into());
				}
			}
			Self::String(value, comma) => {
				s.append(value.into());
				if let Some(comma) = comma {
					s.append(comma.into());
				}
			}
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NotPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: SelectorList<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NotPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthChildPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NthChildPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthColPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NthColPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthLastChildPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NthLastChildPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthLastColPseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NthLastColPseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthLastOfTypePseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NthLastOfTypePseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct NthOfTypePseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for NthOfTypePseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct WherePseudoFunction<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub value: ForgivingSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for WherePseudoFunction<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		ToCursors::to_cursors(&self.value, s);
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct StatePseudoFunction {
	pub colon: T![:],
	pub function: T![Function],
	pub value: T![Ident],
	pub close: Option<T![')']>,
}

impl<'a> ToCursors for StatePseudoFunction {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.colon.into());
		s.append(self.function.into());
		s.append(self.value.into());
		if let Some(close) = self.close {
			s.append(close.into());
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(FunctionalPseudoClass, 88);
		assert_size!(DirValue, 16);
	}
}
