use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-values/#resolution-value
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Resolution {
    value: f32,
    unit: ResolutionUnit,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum ResolutionUnit {
    Dpi,
    Dpcm,
    Dppx,
    X,
}
impl Atomizable for ResolutionUnit {
    fn to_atom(&self) -> Atom {
        match self {
            Self::Dpi => atom!("dpi"),
            Self::Dpcm => atom!("dpcm"),
            Self::Dppx => atom!("dppx"),
            Self::X => atom!("x"),
        }
    }

    fn from_atom(unit: Atom) -> Option<Self> {
        match unit {
            atom!("dpi") => Some(Self::Dpi),
            atom!("dpcm") => Some(Self::Dpcm),
            atom!("dppx") => Some(Self::Dppx),
            atom!("x") => Some(Self::X),
            _ => None,
        }
    }
}

impl Hash for Resolution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
        self.unit.hash(state);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_test() {
        use std::mem::size_of;
        assert_eq!(size_of::<Resolution>(), 8);
    }
}
