use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use super::Percentage;
use crate::{atom, Atom, Atomizable};

macro_rules! expr {
	($expr: expr) => {
		$expr
	};
}
macro_rules! pat {
	($pat: pat) => {
		$pat
	};
}

macro_rules! length_units {
    ( $($unit: ident($($atom: tt)*),)+ ) => {
        $(
            #[derive(Default, Clone, Copy, Debug, PartialEq)]
            #[cfg_attr(feature = "serde", derive(Serialize), serde())]
            pub struct $unit(pub f32);

            impl Hash for $unit {
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.0.to_bits().hash(state);
                    expr!($($atom)*).hash(state);
                }
            }

            impl Atomizable for $unit {
                fn from_atom(atom: Atom) -> Option<Self> {
                    if atom == expr!($($atom)*) {
                        Some(Self(0.0))
                    } else {
                        None
                    }
                }
                fn to_atom(&self) -> Atom {
                    expr!($($atom)*)
                }
            }
        )+

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum Length {
            #[default]
            Zero,
            $(
                $unit($unit),
            )+
        }

        impl Length {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None,
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum LengthOrAuto {
            #[default]
            Zero,
            Auto,
            $(
                $unit($unit),
            )+
        }

        impl LengthOrAuto {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None,
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    Self::Auto => (0.0, atom!("auto")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum PositiveLength {
            #[default]
            Zero,
            $(
                $unit($unit),
            )+
        }

        impl PositiveLength {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                if n < 0.0 {
                    return None;
                }
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum LengthPercentage {
            #[default]
            Zero,
            Percentage(Percentage),
            $(
                $unit($unit),
            )+
        }

        impl LengthPercentage {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    Self::Percentage(n) => (n.0, atom!("%")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum LengthPercentageOrNormal {
            #[default]
            Zero,
            Normal,
            Percentage(Percentage),
            $(
                $unit($unit),
            )+
        }

        impl LengthPercentageOrNormal {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    Self::Normal => (0.0, atom!("normal")),
                    Self::Percentage(n) => (n.0, atom!("%")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum PositiveLengthPercentage {
            #[default]
            Zero,
            Percentage(Percentage),
            $(
                $unit($unit),
            )+
        }

        impl PositiveLengthPercentage {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                if n < 0.0 {
                    return None;
                }
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    Self::Percentage(n) => (n.0, atom!("%")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum PositiveLengthPercentageOrNormal {
            #[default]
            Zero,
            Normal,
            Percentage(Percentage),
            $(
                $unit($unit),
            )+
        }

        impl PositiveLengthPercentageOrNormal {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                if n < 0.0 {
                    return None;
                }
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None,
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    Self::Percentage(n) => (n.0, atom!("%")),
                    Self::Normal => (0.0, atom!("normal")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }

        #[derive(Default, Clone, Copy, Debug, PartialEq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize), serde())]
        pub enum LengthPercentageOrAuto {
            #[default]
            Zero,
            Auto,
            Percentage(Percentage),
            $(
                $unit($unit),
            )+
        }

        impl LengthPercentageOrAuto {
            pub fn from_f32_and_atom(n: f32, atom: Atom) -> Option<Self> {
                match atom {
                    $(
                        pat!($($atom)*) => Some(Self::$unit($unit(n))),
                    )+
                    _ => None,
                }
            }
            pub fn to_f32_and_atom(&self) -> (f32, Atom) {
                match self {
                    Self::Zero => (0.0, atom!("")),
                    Self::Auto => (0.0, atom!("auto")),
                    Self::Percentage(n) => (n.0, atom!("%")),
                    $(
                        Self::$unit(n) => (n.0, n.to_atom()),
                    )+
                }
            }
        }
    };
}

length_units! {
	// Absolute Units
	// https://drafts.csswg.org/css-values/#absolute-lengths
	Cm(atom!("cm")),
	Mm(atom!("mm")),
	Q(atom!("q")),
	In(atom!("in")),
	Pc(atom!("pc")),
	Pt(atom!("pt")),
	Px(atom!("px")),

	// Font Relative Units
	// https://drafts.csswg.org/css-values/#font-relative-lengths
	Em(atom!("em")),
	Rem(atom!("rem")),
	Ex(atom!("ex")),
	Rex(atom!("rex")),
	Cap(atom!("cap")),
	Rcap(atom!("rcap")),
	Ch(atom!("ch")),
	Rch(atom!("rch")),
	Ic(atom!("ic")),
	Ric(atom!("ric")),
	Lh(atom!("lh")),
	Rlh(atom!("rlh")),

	// Viewport Relative Units
	// https://drafts.csswg.org/css-values/#viewport-relative-units
	Vw(atom!("vw")),
	Svw(atom!("svw")),
	Lvw(atom!("lvw")),
	Dvw(atom!("dvw")),
	Vh(atom!("vh")),
	Svh(atom!("svh")),
	Lvh(atom!("lvh")),
	Dvh(atom!("dvh")),
	Vi(atom!("vi")),
	Svi(atom!("svi")),
	Lvi(atom!("lvi")),
	Dvi(atom!("dvi")),
	Vb(atom!("vb")),
	Svb(atom!("svb")),
	Lvb(atom!("lvb")),
	Dvb(atom!("dvb")),
	Vmin(atom!("vmin")),
	Svmin(atom!("svmin")),
	Lvmin(atom!("lvmin")),
	Dvmin(atom!("dvmin")),
	Vmax(atom!("vmax")),
	Svmax(atom!("svmax")),
	Lvmax(atom!("lvmax")),
	Dvmax(atom!("dvmax")),

	// Container Relative Units
	// https://www.w3.org/TR/css-contain-3/#container-lengths
	Cqw(atom!("cqw")),
	Cqh(atom!("cqh")),
	Cqi(atom!("cqi")),
	Cqb(atom!("cqb")),
	Cqmin(atom!("cqmin")),
	Cqmax(atom!("cqmax")),

}

#[cfg(test)]
mod tests {

	use super::super::*;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<Length>(), 8);
		assert_eq!(size_of::<LengthOrAuto>(), 8);
		assert_eq!(size_of::<PositiveLength>(), 8);
		assert_eq!(size_of::<LengthPercentage>(), 8);
		assert_eq!(size_of::<PositiveLengthPercentage>(), 8);
		assert_eq!(size_of::<LengthPercentageOrAuto>(), 8);
	}
}
