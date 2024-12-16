use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet, Span};
use hdx_parser::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum MsPseudoElement {
	Backdrop(T![::], T![Ident]),
	Browse(T![::], T![Ident]),
	Check(T![::], T![Ident]),
	Clear(T![::], T![Ident]),
	Expand(T![::], T![Ident]),
	Fill(T![::], T![Ident]),
	FillUpper(T![::], T![Ident]),
	FillLower(T![::], T![Ident]),
	InputPlaceholder(T![::], T![Ident]),
	Placeholder(T![::], T![Ident]),
	Reveal(T![::], T![Ident]),
	Selection(T![::], T![Ident]),
	Thumb(T![::], T![Ident]),
	TicksAfter(T![::], T![Ident]),
	TicksBefore(T![::], T![Ident]),
	Tooltip(T![::], T![Ident]),
	Track(T![::], T![Ident]),
	Value(T![::], T![Ident]),
}

impl<'a> Parse<'a> for MsPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colons = p.parse::<T![::]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colons = colons?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-ms-backdrop") => Self::Backdrop(colons, ident),
			atom!("-ms-browse") => Self::Browse(colons, ident),
			atom!("-ms-check") => Self::Check(colons, ident),
			atom!("-ms-clear") => Self::Clear(colons, ident),
			atom!("-ms-expand") => Self::Expand(colons, ident),
			atom!("-ms-fill") => Self::Fill(colons, ident),
			atom!("-ms-fill-upper") => Self::FillUpper(colons, ident),
			atom!("-ms-fill-lower") => Self::FillLower(colons, ident),
			atom!("-ms-input-placeholder") => Self::InputPlaceholder(colons, ident),
			atom!("-ms-placeholder") => Self::Placeholder(colons, ident),
			atom!("-ms-reveal") => Self::Reveal(colons, ident),
			atom!("-ms-selection") => Self::Selection(colons, ident),
			atom!("-ms-thumb") => Self::Thumb(colons, ident),
			atom!("-ms-ticks-after") => Self::TicksAfter(colons, ident),
			atom!("-ms-ticks-before") => Self::TicksBefore(colons, ident),
			atom!("-ms-tooltip") => Self::Tooltip(colons, ident),
			atom!("-ms-track") => Self::Track(colons, ident),
			atom!("-ms-value") => Self::Value(colons, ident),
			atom => Err(diagnostics::UnexpectedPseudoElement(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors for MsPseudoElement {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Backdrop(colons, ident)
			| Self::Browse(colons, ident)
			| Self::Check(colons, ident)
			| Self::Clear(colons, ident)
			| Self::Expand(colons, ident)
			| Self::Fill(colons, ident)
			| Self::FillUpper(colons, ident)
			| Self::FillLower(colons, ident)
			| Self::InputPlaceholder(colons, ident)
			| Self::Placeholder(colons, ident)
			| Self::Reveal(colons, ident)
			| Self::Selection(colons, ident)
			| Self::Thumb(colons, ident)
			| Self::TicksAfter(colons, ident)
			| Self::TicksBefore(colons, ident)
			| Self::Tooltip(colons, ident)
			| Self::Track(colons, ident)
			| Self::Value(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
		}
	}
}

impl From<&MsPseudoElement> for Span {
	fn from(value: &MsPseudoElement) -> Self {
		match value {
			MsPseudoElement::Backdrop(colons, ident)
			| MsPseudoElement::Browse(colons, ident)
			| MsPseudoElement::Check(colons, ident)
			| MsPseudoElement::Clear(colons, ident)
			| MsPseudoElement::Expand(colons, ident)
			| MsPseudoElement::Fill(colons, ident)
			| MsPseudoElement::FillUpper(colons, ident)
			| MsPseudoElement::FillLower(colons, ident)
			| MsPseudoElement::InputPlaceholder(colons, ident)
			| MsPseudoElement::Placeholder(colons, ident)
			| MsPseudoElement::Reveal(colons, ident)
			| MsPseudoElement::Selection(colons, ident)
			| MsPseudoElement::Thumb(colons, ident)
			| MsPseudoElement::TicksAfter(colons, ident)
			| MsPseudoElement::TicksBefore(colons, ident)
			| MsPseudoElement::Tooltip(colons, ident)
			| MsPseudoElement::Track(colons, ident)
			| MsPseudoElement::Value(colons, ident) => Into::<Span>::into(colons) + ident.into(),
		}
	}
}

impl<'a> Visitable<'a> for MsPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_ms_pseudo_element(self);
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum MsPseudoClass {
	Fullscreen(T![:], T![Ident]),
	InputPlaceholder(T![:], T![Ident]),
}

impl<'a> Parse<'a> for MsPseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colon = p.parse::<T![:]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colon = colon?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-ms-fullscreen") => Self::Fullscreen(colon, ident),
			atom!("-ms-input-placeholder") => Self::InputPlaceholder(colon, ident),
			atom => Err(diagnostics::UnexpectedPseudoClass(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors for MsPseudoClass {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Fullscreen(colon, ident) => {
				ToCursors::to_cursors(colon, s);
				s.append(ident.into());
			}
			Self::InputPlaceholder(colon, ident) => {
				ToCursors::to_cursors(colon, s);
				s.append(ident.into());
			}
		}
	}
}

impl From<&MsPseudoClass> for Span {
	fn from(value: &MsPseudoClass) -> Self {
		match value {
			MsPseudoClass::Fullscreen(colon, ident) | MsPseudoClass::InputPlaceholder(colon, ident) => {
				Into::<Span>::into(colon) + ident.into()
			}
		}
	}
}

impl<'a> Visitable<'a> for MsPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_ms_pseudo_class(self);
	}
}
