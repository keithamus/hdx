use css_lexer::Span;
use css_parse::{diagnostics, Parse, Parser, Result as ParserResult, ToCursors, T};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable};

use super::{moz::MozPseudoClass, ms::MsPseudoClass, o::OPseudoClass, webkit::WebkitPseudoClass};

macro_rules! apply_pseudo_class {
	($macro: ident) => {
		$macro! {
			Active: "active",
			AnyLink: "any-link",
			Autofill: "autofill",
			Blank: "blank",
			Checked: "checked",
			Current: "current",
			Default: "default",
			Defined: "defined",
			Disabled: "disabled",
			Empty: "empty",
			Enabled: "enabled",
			First: "first",
			FirstChild: "first-child",
			FirstOfType: "first-of-type",
			Fullscreen: "fullscreen",
			Future: "future",
			Focus: "focus",
			FocusVisible: "focus-visible",
			FocusWithin: "focus-within",
			Host: "host",
			Hover: "hover",
			Indeterminate: "indeterminate",
			InRange: "in-range",
			Invalid: "invalid",
			LastChild: "last-child",
			LastOfType: "last-of-type",
			Left: "left",
			Link: "link",
			LocalLink: "local-link",
			Modal: "modal",
			OnlyChild: "only-child",
			OnlyOfType: "only-of-type",
			Optional: "optional",
			OutOfRange: "out-of-range",
			Past: "past",
			PictureInPicture: "picture-in-picture",
			PlaceholderShown: "placeholder-shown",
			PopoverOpen: "popover-open",
			Paused: "paused",
			Playing: "playing",
			ReadOnly: "read-only",
			ReadWrite: "read-write",
			Required: "required",
			Right: "right",
			Root: "root",
			Scope: "scope",
			Target: "target",
			TargetWithin: "target-within",
			Valid: "valid",
			Visited: "visited",
		}
	};
}

macro_rules! define_pseudo_class {
	( $($ident: ident: $str: tt $(,)*)+ ) => {
		#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
		#[visit]
		pub enum PseudoClass {
			$($ident(T![:], T![Ident]),)+
			Webkit(WebkitPseudoClass),
			Moz(MozPseudoClass),
			Ms(MsPseudoClass),
			O(OPseudoClass),
		}
	};
}
apply_pseudo_class!(define_pseudo_class);

macro_rules! define_pseudo_class_keyword {
	( $($ident: ident: $str: tt $(,)*)+ ) => {
		mod defined {
			use css_parse::pseudo_class;
			pseudo_class!(PseudoClass {
				$($ident: $str,)+
			});
		}
	}
}
apply_pseudo_class!(define_pseudo_class_keyword);

impl<'a> Parse<'a> for PseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let keyword = p.parse::<defined::PseudoClass>();
		macro_rules! match_keyword {
			( $($ident: ident: $str: tt $(,)*)+ ) => {
				match keyword {
					$(Ok(defined::PseudoClass::$ident(a, b)) => Ok(Self::$ident(a, b)),)+
					Err(_) => {
						p.rewind(checkpoint);
						let c = p.peek_n(2);
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
						Err(diagnostics::UnexpectedPseudoClass(p.parse_str(c).into(), c.into()))?
					}
				}
			};
		}
		apply_pseudo_class!(match_keyword)
	}
}

impl<'a> ToCursors for PseudoClass {
	fn to_cursors(&self, s: &mut impl css_parse::CursorSink) {
		macro_rules! match_keyword {
			( $($ident: ident: $str: tt $(,)*)+ ) => {
				match self {
					$(Self::$ident(colon, ident))|+ => {
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
		apply_pseudo_class!(match_keyword)
	}
}

impl<'a> From<&PseudoClass> for Span {
	fn from(value: &PseudoClass) -> Self {
		macro_rules! match_keyword {
			( $($ident: ident: $str: tt $(,)*)+ ) => {
				match value {
					$(PseudoClass::$ident(colon, ident))|+ => Into::<Span>::into(colon) + ident.into(),
					PseudoClass::Webkit(c) => c.into(),
					PseudoClass::Moz(c) => c.into(),
					PseudoClass::Ms(c) => c.into(),
					PseudoClass::O(c) => c.into(),
				}
			}
		}
		apply_pseudo_class!(match_keyword)
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PseudoClass>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PseudoClass, ":target");
		assert_parse!(PseudoClass, ":scope");
		assert_parse!(PseudoClass, ":valid");
	}
}
