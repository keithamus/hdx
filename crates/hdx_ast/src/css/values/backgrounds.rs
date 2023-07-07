#[cfg(feature = "serde")]
use serde::Serialize;

use super::Length;
use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-backgrounds-3/#typedef-line-width
#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LineWidth {
    Thin,
    #[default]
    Medium,
    Thick,
    Length(Length),
}

// impl Atomizable for LineWidth {
//     fn from_atom(atom: Atom) -> Option<Self> {
//         match atom {
//             atom!("thin") => Some(Self::Thin),
//             atom!("medium") => Some(Self::Medium),
//             atom!("thick") => Some(Self::Thick),
//             _ => {
//                 if let Some(length) = Length::from_atom(atom) {
//                     return Some(Self::Length(length));
//                 }
//                 None
//             }
//         }
//     }
//
//     fn to_atom(&self) -> Atom {
//         match self {
//             Self::Thin => atom!("thin"),
//             Self::Medium => atom!("medium"),
//             Self::Thick => atom!("thick"),
//             Self::Length(l) => l.unit.to_atom(),
//         }
//     }
// }

// https://drafts.csswg.org/css-backgrounds-3/#typedef-line-style
#[derive(Atomizable, Debug, Default, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum LineStyle {
    #[default]
    None, // atom!("none")
    Hidden, // atom!("hidden")
    Dotted, // atom!("dotted")
    Dashed, // atom!("dashed")
    Solid,  // atom!("solid")
    Double, // atom!("double")
    Groove, // atom!("groove")
    Ridge,  // atom!("ridge")
    Inset,  // atom!("inset")
    Outset, // atom!("outset")
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_test() {
        use std::mem::size_of;
        assert_eq!(size_of::<LineWidth>(), 8);
        assert_eq!(size_of::<LineStyle>(), 1);
    }
}
