use hdx_atom::atom;
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{diagnostics, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

impl<'a> ToCursors<'a> for MsPseudoElement {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::Backdrop(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Browse(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Check(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Clear(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Expand(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Fill(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FillUpper(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FillLower(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::InputPlaceholder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Placeholder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Reveal(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Selection(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Thumb(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TicksAfter(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TicksBefore(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Tooltip(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Track(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Value(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

impl<'a> ToCursors<'a> for MsPseudoClass {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
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
