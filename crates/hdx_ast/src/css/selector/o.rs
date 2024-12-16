use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet, Span};
use hdx_parser::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum OPseudoElement {
	InnerSpinButton(T![::], T![Ident]),
	OuterSpinButton(T![::], T![Ident]),
	Placeholder(T![::], T![Ident]),
	Scrollbar(T![::], T![Ident]),
	ScrollbarThumb(T![::], T![Ident]),
	ScrollbarTrack(T![::], T![Ident]),
	ScrollbarTrackPiece(T![::], T![Ident]),
	Selection(T![::], T![Ident]),
}

impl<'a> Parse<'a> for OPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colons = p.parse::<T![::]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colons = colons?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-o-inner-spin-button") => Self::InnerSpinButton(colons, ident),
			atom!("-o-outer-spin-button") => Self::OuterSpinButton(colons, ident),
			atom!("-o-placeholder") => Self::Placeholder(colons, ident),
			atom!("-o-scrollbar") => Self::Scrollbar(colons, ident),
			atom!("-o-scrollbar-thumb") => Self::ScrollbarThumb(colons, ident),
			atom!("-o-scrollbar-track") => Self::ScrollbarTrack(colons, ident),
			atom!("-o-scrollbar-track-piece") => Self::ScrollbarTrackPiece(colons, ident),
			atom!("-o-selection") => Self::Selection(colons, ident),
			atom => Err(diagnostics::UnexpectedPseudoElement(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors for OPseudoElement {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::InnerSpinButton(colons, ident)
			| Self::OuterSpinButton(colons, ident)
			| Self::Placeholder(colons, ident)
			| Self::Scrollbar(colons, ident)
			| Self::ScrollbarThumb(colons, ident)
			| Self::ScrollbarTrack(colons, ident)
			| Self::ScrollbarTrackPiece(colons, ident)
			| Self::Selection(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
		}
	}
}

impl From<&OPseudoElement> for Span {
	fn from(value: &OPseudoElement) -> Self {
		match value {
			OPseudoElement::InnerSpinButton(colons, ident)
			| OPseudoElement::OuterSpinButton(colons, ident)
			| OPseudoElement::Placeholder(colons, ident)
			| OPseudoElement::Scrollbar(colons, ident)
			| OPseudoElement::ScrollbarThumb(colons, ident)
			| OPseudoElement::ScrollbarTrack(colons, ident)
			| OPseudoElement::ScrollbarTrackPiece(colons, ident)
			| OPseudoElement::Selection(colons, ident) => Into::<Span>::into(colons) + ident.into(),
		}
	}
}

impl<'a> Visitable<'a> for OPseudoElement {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_o_pseudo_element(self);
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum OPseudoClass {
	Prefocus(T![:], T![Ident]),
}

impl<'a> Parse<'a> for OPseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colon = p.parse::<T![:]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colon = colon?;
		let ident = ident?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("-o-prefocus") => Self::Prefocus(colon, ident),
			atom => Err(diagnostics::UnexpectedPseudoClass(atom, c.into()))?,
		})
	}
}

impl<'a> ToCursors for OPseudoClass {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Prefocus(colon, ident) => {
				ToCursors::to_cursors(colon, s);
				s.append(ident.into());
			}
		}
	}
}

impl From<&OPseudoClass> for Span {
	fn from(value: &OPseudoClass) -> Self {
		match value {
			OPseudoClass::Prefocus(colon, ident) => Into::<Span>::into(colon) + ident.into(),
		}
	}
}

impl<'a> Visitable<'a> for OPseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_o_pseudo_class(self);
	}
}
