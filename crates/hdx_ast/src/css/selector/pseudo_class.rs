use hdx_atom::atom;
use hdx_lexer::{Cursor, Span};
use hdx_parser::{diagnostics, Parse, Parser, Result as ParserResult, ToCursors, T};
use hdx_proc_macro::visit;

use crate::css::{Visit, Visitable};

use super::{moz::MozPseudoClass, ms::MsPseudoClass, o::OPseudoClass, webkit::WebkitPseudoClass};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum PseudoClass {
	Active(T![:], T![Ident]),
	AnyLink(T![:], T![Ident]),
	Autofill(T![:], T![Ident]),
	Blank(T![:], T![Ident]),
	Checked(T![:], T![Ident]),
	Current(T![:], T![Ident]),
	Default(T![:], T![Ident]),
	Defined(T![:], T![Ident]),
	Disabled(T![:], T![Ident]),
	Empty(T![:], T![Ident]),
	Enabled(T![:], T![Ident]),
	First(T![:], T![Ident]),
	FirstChild(T![:], T![Ident]),
	FirstOfType(T![:], T![Ident]),
	Fullscreen(T![:], T![Ident]),
	Future(T![:], T![Ident]),
	Focus(T![:], T![Ident]),
	FocusVisible(T![:], T![Ident]),
	FocusWithin(T![:], T![Ident]),
	Host(T![:], T![Ident]),
	Hover(T![:], T![Ident]),
	Indeterminate(T![:], T![Ident]),
	InRange(T![:], T![Ident]),
	Invalid(T![:], T![Ident]),
	LastChild(T![:], T![Ident]),
	LastOfType(T![:], T![Ident]),
	Left(T![:], T![Ident]),
	Link(T![:], T![Ident]),
	LocalLink(T![:], T![Ident]),
	Modal(T![:], T![Ident]),
	OnlyChild(T![:], T![Ident]),
	OnlyOfType(T![:], T![Ident]),
	Optional(T![:], T![Ident]),
	OutOfRange(T![:], T![Ident]),
	Past(T![:], T![Ident]),
	PictureInPicture(T![:], T![Ident]),
	PlaceholderShown(T![:], T![Ident]),
	PopoverOpen(T![:], T![Ident]),
	Paused(T![:], T![Ident]),
	Playing(T![:], T![Ident]),
	ReadOnly(T![:], T![Ident]),
	ReadWrite(T![:], T![Ident]),
	Required(T![:], T![Ident]),
	Right(T![:], T![Ident]),
	Root(T![:], T![Ident]),
	Scope(T![:], T![Ident]),
	Target(T![:], T![Ident]),
	TargetWithin(T![:], T![Ident]),
	Valid(T![:], T![Ident]),
	Visited(T![:], T![Ident]),
	Webkit(WebkitPseudoClass),
	Moz(MozPseudoClass),
	Ms(MsPseudoClass),
	O(OPseudoClass),
}

impl<'a> Parse<'a> for PseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let colon = p.parse::<T![:]>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		Ok(match p.parse_atom_lower(c) {
			atom!("active") => Self::Active(colon, ident),
			atom!("any-link") => Self::Active(colon, ident),
			atom!("autofill") => Self::Active(colon, ident),
			atom!("blank") => Self::Active(colon, ident),
			atom!("checked") => Self::Active(colon, ident),
			atom!("current") => Self::Active(colon, ident),
			atom!("default") => Self::Active(colon, ident),
			atom!("defined") => Self::Active(colon, ident),
			atom!("disabled") => Self::Active(colon, ident),
			atom!("empty") => Self::Active(colon, ident),
			atom!("enabled") => Self::Active(colon, ident),
			atom!("first") => Self::Active(colon, ident),
			atom!("first-child") => Self::Active(colon, ident),
			atom!("first-of-type") => Self::Active(colon, ident),
			atom!("fullscreen") => Self::Active(colon, ident),
			atom!("future") => Self::Active(colon, ident),
			atom!("focus") => Self::Active(colon, ident),
			atom!("focus-visible") => Self::Active(colon, ident),
			atom!("focus-within") => Self::Active(colon, ident),
			atom!("host") => Self::Active(colon, ident),
			atom!("hover") => Self::Active(colon, ident),
			atom!("indeterminate") => Self::Active(colon, ident),
			atom!("in-range") => Self::Active(colon, ident),
			atom!("invalid") => Self::Active(colon, ident),
			atom!("last-child") => Self::Active(colon, ident),
			atom!("last-of-type") => Self::Active(colon, ident),
			atom!("left") => Self::Active(colon, ident),
			atom!("link") => Self::Active(colon, ident),
			atom!("local-link") => Self::Active(colon, ident),
			atom!("modal") => Self::Active(colon, ident),
			atom!("only-child") => Self::Active(colon, ident),
			atom!("only-of-type") => Self::Active(colon, ident),
			atom!("optional") => Self::Active(colon, ident),
			atom!("out-of-range") => Self::Active(colon, ident),
			atom!("past") => Self::Active(colon, ident),
			atom!("picture-in-picture") => Self::Active(colon, ident),
			atom!("placeholder-shown") => Self::Active(colon, ident),
			atom!("popover-open") => Self::Active(colon, ident),
			atom!("paused") => Self::Active(colon, ident),
			atom!("playing") => Self::Active(colon, ident),
			atom!("read-only") => Self::Active(colon, ident),
			atom!("read-write") => Self::Active(colon, ident),
			atom!("required") => Self::Active(colon, ident),
			atom!("right") => Self::Active(colon, ident),
			atom!("root") => Self::Active(colon, ident),
			atom!("scope") => Self::Active(colon, ident),
			atom!("target") => Self::Active(colon, ident),
			atom!("target-within") => Self::Active(colon, ident),
			atom!("valid") => Self::Active(colon, ident),
			atom!("visited") => Self::Active(colon, ident),
			atom => {
				p.rewind(checkpoint);
				if let Ok(psuedo) = p.try_parse::<WebkitPseudoClass>() {
					return Ok(Self::Webkit(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<MozPseudoClass>() {
					return Ok(Self::Moz(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<MsPseudoClass>() {
					return Ok(Self::Ms(psuedo));
				}
				if let Ok(psuedo) = p.try_parse::<OPseudoClass>() {
					return Ok(Self::O(psuedo));
				}
				Err(diagnostics::UnexpectedPseudoClass(atom, c.into()))?
			}
		})
	}
}

impl<'a> ToCursors for PseudoClass {
	fn to_cursors(&self, s: &mut impl hdx_parser::CursorSink) {
		match self {
			Self::Active(colon, ident)
			| Self::AnyLink(colon, ident)
			| Self::Autofill(colon, ident)
			| Self::Blank(colon, ident)
			| Self::Checked(colon, ident)
			| Self::Current(colon, ident)
			| Self::Default(colon, ident)
			| Self::Defined(colon, ident)
			| Self::Disabled(colon, ident)
			| Self::Empty(colon, ident)
			| Self::Enabled(colon, ident)
			| Self::First(colon, ident)
			| Self::FirstChild(colon, ident)
			| Self::FirstOfType(colon, ident)
			| Self::Fullscreen(colon, ident)
			| Self::Future(colon, ident)
			| Self::Focus(colon, ident)
			| Self::FocusVisible(colon, ident)
			| Self::FocusWithin(colon, ident)
			| Self::Host(colon, ident)
			| Self::Hover(colon, ident)
			| Self::Indeterminate(colon, ident)
			| Self::InRange(colon, ident)
			| Self::Invalid(colon, ident)
			| Self::LastChild(colon, ident)
			| Self::LastOfType(colon, ident)
			| Self::Left(colon, ident)
			| Self::Link(colon, ident)
			| Self::LocalLink(colon, ident)
			| Self::Modal(colon, ident)
			| Self::OnlyChild(colon, ident)
			| Self::OnlyOfType(colon, ident)
			| Self::Optional(colon, ident)
			| Self::OutOfRange(colon, ident)
			| Self::Past(colon, ident)
			| Self::PictureInPicture(colon, ident)
			| Self::PlaceholderShown(colon, ident)
			| Self::PopoverOpen(colon, ident)
			| Self::Paused(colon, ident)
			| Self::Playing(colon, ident)
			| Self::ReadOnly(colon, ident)
			| Self::ReadWrite(colon, ident)
			| Self::Required(colon, ident)
			| Self::Right(colon, ident)
			| Self::Root(colon, ident)
			| Self::Scope(colon, ident)
			| Self::Target(colon, ident)
			| Self::TargetWithin(colon, ident)
			| Self::Valid(colon, ident)
			| Self::Visited(colon, ident) => {
				s.append(colon.into());
				s.append(ident.into());
			}
			Self::Webkit(c) => ToCursors::to_cursors(c, s),
			Self::Moz(c) => ToCursors::to_cursors(c, s),
			Self::Ms(c) => ToCursors::to_cursors(c, s),
			Self::O(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl<'a> From<&PseudoClass> for Span {
	fn from(value: &PseudoClass) -> Self {
		match value {
			PseudoClass::Active(colon, ident)
			| PseudoClass::AnyLink(colon, ident)
			| PseudoClass::Autofill(colon, ident)
			| PseudoClass::Blank(colon, ident)
			| PseudoClass::Checked(colon, ident)
			| PseudoClass::Current(colon, ident)
			| PseudoClass::Default(colon, ident)
			| PseudoClass::Defined(colon, ident)
			| PseudoClass::Disabled(colon, ident)
			| PseudoClass::Empty(colon, ident)
			| PseudoClass::Enabled(colon, ident)
			| PseudoClass::First(colon, ident)
			| PseudoClass::FirstChild(colon, ident)
			| PseudoClass::FirstOfType(colon, ident)
			| PseudoClass::Fullscreen(colon, ident)
			| PseudoClass::Future(colon, ident)
			| PseudoClass::Focus(colon, ident)
			| PseudoClass::FocusVisible(colon, ident)
			| PseudoClass::FocusWithin(colon, ident)
			| PseudoClass::Host(colon, ident)
			| PseudoClass::Hover(colon, ident)
			| PseudoClass::Indeterminate(colon, ident)
			| PseudoClass::InRange(colon, ident)
			| PseudoClass::Invalid(colon, ident)
			| PseudoClass::LastChild(colon, ident)
			| PseudoClass::LastOfType(colon, ident)
			| PseudoClass::Left(colon, ident)
			| PseudoClass::Link(colon, ident)
			| PseudoClass::LocalLink(colon, ident)
			| PseudoClass::Modal(colon, ident)
			| PseudoClass::OnlyChild(colon, ident)
			| PseudoClass::OnlyOfType(colon, ident)
			| PseudoClass::Optional(colon, ident)
			| PseudoClass::OutOfRange(colon, ident)
			| PseudoClass::Past(colon, ident)
			| PseudoClass::PictureInPicture(colon, ident)
			| PseudoClass::PlaceholderShown(colon, ident)
			| PseudoClass::PopoverOpen(colon, ident)
			| PseudoClass::Paused(colon, ident)
			| PseudoClass::Playing(colon, ident)
			| PseudoClass::ReadOnly(colon, ident)
			| PseudoClass::ReadWrite(colon, ident)
			| PseudoClass::Required(colon, ident)
			| PseudoClass::Right(colon, ident)
			| PseudoClass::Root(colon, ident)
			| PseudoClass::Scope(colon, ident)
			| PseudoClass::Target(colon, ident)
			| PseudoClass::TargetWithin(colon, ident)
			| PseudoClass::Valid(colon, ident)
			| PseudoClass::Visited(colon, ident) => Into::<Span>::into(colon) + ident.into(),
			PseudoClass::Webkit(c) => c.into(),
			PseudoClass::Moz(c) => c.into(),
			PseudoClass::Ms(c) => c.into(),
			PseudoClass::O(c) => c.into(),
		}
	}
}

impl<'a> Visitable<'a> for PseudoClass {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_pseudo_class(self);
		match self {
			Self::Webkit(c) => Visitable::accept(c, v),
			Self::Moz(c) => Visitable::accept(c, v),
			Self::Ms(c) => Visitable::accept(c, v),
			Self::O(c) => Visitable::accept(c, v),
			_ => {}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_size!(PseudoClass, 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PseudoClass, ":target");
		assert_parse!(PseudoClass, ":scope");
		assert_parse!(PseudoClass, ":valid");
	}
}
