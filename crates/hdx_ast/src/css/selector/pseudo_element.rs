use hdx_atom::{atom, Atom};
use hdx_lexer::{Cursor, KindSet};
use hdx_parser::{diagnostics, CursorStream, Parse, Parser, Result as ParserResult, ToCursors, T};

use super::{moz::MozPseudoElement, ms::MsPseudoElement, o::OPseudoElement, webkit::WebkitPseudoElement};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PseudoElement {
	After(T![::], T![Ident]),
	Backdrop(T![::], T![Ident]),
	Before(T![::], T![Ident]),
	Cue(T![::], T![Ident]),
	CueRegion(T![::], T![Ident]),
	FirstLetter(T![::], T![Ident]),
	FirstLine(T![::], T![Ident]),
	FileSelectorButton(T![::], T![Ident]),
	GrammarError(T![::], T![Ident]),
	Marker(T![::], T![Ident]),
	Placeholder(T![::], T![Ident]),
	Selection(T![::], T![Ident]),
	SpellingError(T![::], T![Ident]),
	TargetText(T![::], T![Ident]),
	ViewTransition(T![::], T![Ident]),
	Webkit(WebkitPseudoElement),
	Moz(MozPseudoElement),
	Ms(MsPseudoElement),
	O(OPseudoElement),
}

impl<'a> Parse<'a> for PseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let skip = p.set_skip(KindSet::NONE);
		let double_colon = p.parse::<T![::]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let double_colon = double_colon?;
		let ident = ident?;
		let c: Cursor = ident.into();
		match p.parse_atom_lower(c) {
			atom!("after") => Ok(Self::After(double_colon, ident)),
			atom!("backdrop") => Ok(Self::Backdrop(double_colon, ident)),
			atom!("before") => Ok(Self::Before(double_colon, ident)),
			atom!("cue") => Ok(Self::Cue(double_colon, ident)),
			atom!("cue-region") => Ok(Self::CueRegion(double_colon, ident)),
			atom!("first-letter") => Ok(Self::FirstLetter(double_colon, ident)),
			atom!("first-line") => Ok(Self::FirstLine(double_colon, ident)),
			atom!("file-selector-button") => Ok(Self::FileSelectorButton(double_colon, ident)),
			atom!("grammar-error") => Ok(Self::GrammarError(double_colon, ident)),
			atom!("marker") => Ok(Self::Marker(double_colon, ident)),
			atom!("placeholder") => Ok(Self::Placeholder(double_colon, ident)),
			atom!("selection") => Ok(Self::Selection(double_colon, ident)),
			atom!("spelling-error") => Ok(Self::SpellingError(double_colon, ident)),
			atom!("target-text") => Ok(Self::TargetText(double_colon, ident)),
			atom!("view-transition") => Ok(Self::ViewTransition(double_colon, ident)),
			atom => {
				p.rewind(checkpoint);
				if let Ok(psuedo) = p.try_parse::<WebkitPseudoElement>() {
					return Ok(Self::Webkit(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<MozPseudoElement>() {
					return Ok(Self::Moz(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<MsPseudoElement>() {
					return Ok(Self::Ms(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<OPseudoElement>() {
					return Ok(Self::O(psuedo));
				}
				Err(diagnostics::UnexpectedPseudoElement(atom, c.into()))?
			}
		}
	}
}

impl<'a> ToCursors<'a> for PseudoElement {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::After(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Backdrop(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Before(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Cue(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::CueRegion(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FirstLetter(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FirstLine(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::FileSelectorButton(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::GrammarError(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Marker(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Placeholder(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Selection(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::SpellingError(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::TargetText(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::ViewTransition(colons, ident) => {
				ToCursors::to_cursors(colons, s);
				s.append(ident.into());
			}
			Self::Webkit(c) => ToCursors::to_cursors(c, s),
			Self::Moz(c) => ToCursors::to_cursors(c, s),
			Self::Ms(c) => ToCursors::to_cursors(c, s),
			Self::O(c) => ToCursors::to_cursors(c, s),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LegacyPseudoElement {
	After(T![:], T![Ident]),
	Before(T![:], T![Ident]),
	FirstLetter(T![:], T![Ident]),
	FirstLine(T![:], T![Ident]),
}

impl LegacyPseudoElement {
	pub fn matches_name(name: &Atom) -> bool {
		matches!(name, &atom!("after") | &atom!("before") | &atom!("first-letter") | &atom!("first-line"))
	}
}

impl<'a> Parse<'a> for LegacyPseudoElement {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skip = p.set_skip(KindSet::NONE);
		let colon = p.parse::<T![:]>();
		let ident = p.parse::<T![Ident]>();
		p.set_skip(skip);
		let colon = colon?;
		let ident = ident?;
		let c: Cursor = ident.into();
		match p.parse_atom_lower(c) {
			atom!("after") => Ok(Self::After(colon, ident)),
			atom!("before") => Ok(Self::Before(colon, ident)),
			atom!("first-letter") => Ok(Self::FirstLetter(colon, ident)),
			atom!("first-line") => Ok(Self::FirstLine(colon, ident)),
			atom => Err(diagnostics::UnexpectedPseudoElement(atom, c.into()))?,
		}
	}
}

impl<'a> ToCursors<'a> for LegacyPseudoElement {
	fn to_cursors(&self, s: &mut CursorStream<'a>) {
		match self {
			Self::After(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Before(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FirstLetter(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::FirstLine(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PseudoElement, 36);
		assert_size!(LegacyPseudoElement, 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PseudoElement, "::after");
		assert_parse!(PseudoElement, "::first-letter");
		assert_parse!(PseudoElement, "::view-transition");
		assert_parse!(LegacyPseudoElement, ":after");
	}
}
