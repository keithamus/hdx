use css_lexer::{Cursor, Token};
use css_parse::{Build, Parser, Peek, T};

use super::Flex;

// const PX_CM: f32 = PX_IN / 2.54;
// const PX_MM: f32 = PX_IN / 25.4;
// const PX_Q: f32 = PX_MM / 4.0;
// const PX_IN: f32 = 96.0;
// const PX_PC: f32 = PX_IN / 6.0;
// const PX_PT: f32 = PX_IN / 72.0;

macro_rules! apply_lengths {
	($ident: ident) => {
		$ident! {
			// https://drafts.csswg.org/css-values/#font-relative-lengths
			Em,
			Rem,
			Ex,
			Rex,
			Cap,
			Rcap,
			Ch,
			Rch,
			Ic,
			Ric,
			Lh,
			Rlh,

			// https://drafts.csswg.org/css-values/#viewport-relative-units
			Vw,
			Svw,
			Lvw,
			Dvw,
			Vh,
			Svh,
			Lvh,
			Dvh,
			Vi,
			Svi,
			Lvi,
			Dvi,
			Vb,
			Svb,
			Lvb,
			Dvb,
			Vmin,
			Svmin,
			Lvmin,
			Dvmin,
			Vmax,
			Svmax,
			Lvmax,
			Dvmax,

			// https://drafts.csswg.org/css-values/#absolute-lengths
			Cm,
			Mm,
			Q,
			In,
			Pc,
			Pt,
			Px,

			// https://www.w3.org/TR/css-contain-3/#container-lengths
			Cqw,
			Cqh,
			Cqi,
			Cqb,
			Cqmin,
			Cqmax,
		}
	};
}

macro_rules! define_length {
	( $($name: ident),+ $(,)* ) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		pub enum Length {
			Zero(T![Number]),
			$($name(T![Dimension::$name]),)+
		}
	}
}
apply_lengths!(define_length);

impl From<Length> for f32 {
	fn from(val: Length) -> Self {
		macro_rules! match_length {
			( $($name: ident),+ $(,)* ) => {
				match val {
					Length::Zero(_) => 0.0,
					$(Length::$name(f) => f.into()),+
				}
			}
		}
		apply_lengths!(match_length)
	}
}

impl From<Length> for Token {
	fn from(value: Length) -> Self {
		macro_rules! match_length {
				( $($name: ident),+ $(,)* ) => {
					match value {
						Length::Zero(l) => l.into(),
						$(Length::$name(l) => l.into(),)+
					}
				}
			}
		apply_lengths!(match_length)
	}
}

impl From<&Length> for Token {
	fn from(value: &Length) -> Self {
		macro_rules! match_length {
				( $($name: ident),+ $(,)* ) => {
					match value {
						Length::Zero(l) => l.into(),
						$(Length::$name(l) => l.into(),)+
					}
				}
			}
		apply_lengths!(match_length)
	}
}

impl<'a> Peek<'a> for Length {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		macro_rules! is_checks {
			( $($name: ident),+ $(,)* ) => {
				(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
					$(|| <T![Dimension::$name]>::peek(p, c))+
			}
		}
		apply_lengths!(is_checks)
	}
}

impl<'a> Build<'a> for Length {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		macro_rules! build_steps {
			( $($name: ident),+ $(,)* ) => {
				$(if <T![Dimension::$name]>::peek(p, c) {
					Self::$name(<T![Dimension::$name]>::build(p, c))
				} else )+ {
					Self::Zero(<T![Number]>::build(p, c))
				}
			}
		}
		apply_lengths!(build_steps)
	}
}

impl From<Length> for Cursor {
	fn from(value: Length) -> Self {
		macro_rules! from_steps {
			( $($name: ident),+ $(,)* ) => {
				match value {
					$(Length::$name(t) => t.into(),)+
					Length::Zero(t) => t.into(),
				}
			}
		}
		apply_lengths!(from_steps)
	}
}

impl From<&Length> for Cursor {
	fn from(value: &Length) -> Self {
		(*value).into()
	}
}

macro_rules! define_length_percentage {
	( $($name: ident),+ $(,)* ) => {
		#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value", rename_all = "kebab-case"))]
		pub enum LengthPercentage {
			Zero(T![Number]),
			$($name(T![Dimension::$name]),)+
			Percent(T![Dimension::%]),
		}
	}
}
apply_lengths!(define_length_percentage);

impl From<LengthPercentage> for f32 {
	fn from(val: LengthPercentage) -> Self {
		macro_rules! match_length {
			( $($name: ident),+ $(,)* ) => {
				match val {
					LengthPercentage::Zero(_) => 0.0,
					LengthPercentage::Percent(f) => f.into(),
					$(LengthPercentage::$name(f) => f.into()),+
				}
			}
		}
		apply_lengths!(match_length)
	}
}

impl From<LengthPercentage> for Token {
	fn from(value: LengthPercentage) -> Self {
		macro_rules! match_length {
				( $($name: ident),+ $(,)* ) => {
					match value {
						LengthPercentage::Zero(l) => l.into(),
						LengthPercentage::Percent(l) => l.into(),
						$(LengthPercentage::$name(l) => l.into(),)+
					}
				}
			}
		apply_lengths!(match_length)
	}
}

impl From<&LengthPercentage> for Token {
	fn from(value: &LengthPercentage) -> Self {
		macro_rules! match_length {
				( $($name: ident),+ $(,)* ) => {
					match value {
						LengthPercentage::Zero(l) => l.into(),
						LengthPercentage::Percent(l) => l.into(),
						$(LengthPercentage::$name(l) => l.into(),)+
					}
				}
			}
		apply_lengths!(match_length)
	}
}

impl From<LengthPercentage> for Cursor {
	fn from(value: LengthPercentage) -> Self {
		macro_rules! from_steps {
			( $($name: ident),+ $(,)* ) => {
				match value {
					$(LengthPercentage::$name(t) => t.into(),)+
					LengthPercentage::Percent(t) => t.into(),
					LengthPercentage::Zero(t) => t.into(),
				}
			}
		}
		apply_lengths!(from_steps)
	}
}

impl From<&LengthPercentage> for Cursor {
	fn from(value: &LengthPercentage) -> Self {
		(*value).into()
	}
}

impl<'a> Peek<'a> for LengthPercentage {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		macro_rules! is_checks {
			( $($name: ident),+ $(,)* ) => {
				(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
				|| <T![Dimension::%]>::peek(p, c)
					$(|| <T![Dimension::$name]>::peek(p, c))+
			}
		}
		apply_lengths!(is_checks)
	}
}

impl<'a> Build<'a> for LengthPercentage {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		macro_rules! build_steps {
			( $($name: ident),+ $(,)* ) => {
				$(if <T![Dimension::$name]>::peek(p, c) {
					Self::$name(<T![Dimension::$name]>::build(p, c))
				} else )+ if <T![Dimension::%]>::peek(p, c) {
					Self::Percent(<T![Dimension::%]>::build(p, c))
				} else {
					Self::Zero(<T![Number]>::build(p, c))
				}
			}
		}
		apply_lengths!(build_steps)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LengthPercentageOrAuto {
	Auto(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl<'a> Peek<'a> for LengthPercentageOrAuto {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		LengthPercentage::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "auto"))
	}
}

impl<'a> Build<'a> for LengthPercentageOrAuto {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if LengthPercentage::peek(p, c) {
			Self::LengthPercentage(LengthPercentage::build(p, c))
		} else {
			Self::Auto(<T![Ident]>::build(p, c))
		}
	}
}

impl From<LengthPercentageOrAuto> for Token {
	fn from(value: LengthPercentageOrAuto) -> Self {
		match value {
			LengthPercentageOrAuto::Auto(l) => l.into(),
			LengthPercentageOrAuto::LengthPercentage(l) => l.into(),
		}
	}
}

impl From<&LengthPercentageOrAuto> for Token {
	fn from(value: &LengthPercentageOrAuto) -> Self {
		match value {
			LengthPercentageOrAuto::Auto(l) => l.into(),
			LengthPercentageOrAuto::LengthPercentage(l) => l.into(),
		}
	}
}

impl From<LengthPercentageOrAuto> for Cursor {
	fn from(value: LengthPercentageOrAuto) -> Self {
		match value {
			LengthPercentageOrAuto::Auto(t) => t.into(),
			LengthPercentageOrAuto::LengthPercentage(t) => t.into(),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum LengthPercentageOrFlex {
	Flex(Flex),
	LengthPercentage(LengthPercentage),
}

impl<'a> Peek<'a> for LengthPercentageOrFlex {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Flex::peek(p, c) || LengthPercentage::peek(p, c)
	}
}

impl<'a> Build<'a> for LengthPercentageOrFlex {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if Flex::peek(p, c) {
			Self::Flex(Flex::build(p, c))
		} else {
			Self::LengthPercentage(LengthPercentage::build(p, c))
		}
	}
}

impl From<LengthPercentageOrFlex> for Token {
	fn from(value: LengthPercentageOrFlex) -> Self {
		match value {
			LengthPercentageOrFlex::Flex(l) => l.into(),
			LengthPercentageOrFlex::LengthPercentage(l) => l.into(),
		}
	}
}

impl From<&LengthPercentageOrFlex> for Token {
	fn from(value: &LengthPercentageOrFlex) -> Self {
		(*value).into()
	}
}

impl From<LengthPercentageOrFlex> for Cursor {
	fn from(value: LengthPercentageOrFlex) -> Self {
		match value {
			LengthPercentageOrFlex::Flex(l) => l.into(),
			LengthPercentageOrFlex::LengthPercentage(l) => l.into(),
		}
	}
}

impl From<&LengthPercentageOrFlex> for Cursor {
	fn from(value: &LengthPercentageOrFlex) -> Self {
		(*value).into()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Length>(), 16);
		assert_eq!(std::mem::size_of::<LengthPercentage>(), 16);
		assert_eq!(std::mem::size_of::<LengthPercentageOrAuto>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Length, "10px");
		// Truncates to 7dp
		assert_parse!(Length, "1.2345679px");
		// Removes redundant dp
		assert_parse!(Length, "-1px");
		// Percent
		assert_parse!(LengthPercentage, "1%");
		assert_parse!(LengthPercentageOrAuto, "auto");
	}
}
