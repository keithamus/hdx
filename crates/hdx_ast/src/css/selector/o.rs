use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{diagnostics, CursorSink, Parse, Parser, Result as ParserResult, ToCursors, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
			Self::InnerSpinButton(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::OuterSpinButton(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Placeholder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Scrollbar(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ScrollbarThumb(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ScrollbarTrack(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ScrollbarTrackPiece(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Selection(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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
