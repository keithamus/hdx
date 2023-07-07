use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable};

// https://drafts.csswg.org/css-values/#frequency-value
#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Frequency {
    value: f32,
    unit: FrequencyUnit,
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum FrequencyUnit {
    #[default]
    Hertz,
    KiloHertz,
}

impl Atomizable for FrequencyUnit {
    fn to_atom(&self) -> Atom {
        match self {
            Self::Hertz => atom!("hz"),
            Self::KiloHertz => atom!("khz"),
        }
    }

    fn from_atom(unit: Atom) -> Option<Self> {
        match unit {
            atom!("hz") => Some(Self::Hertz),
            atom!("khz") => Some(Self::KiloHertz),
            _ => None,
        }
    }
}

impl Hash for Frequency {
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
        assert_eq!(size_of::<Frequency>(), 8);
    }
}
