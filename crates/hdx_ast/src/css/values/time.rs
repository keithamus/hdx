use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{atom, Atom, Atomizable, Unit};

// https://drafts.csswg.org/css-values/#time-value
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Time {
    value: f32,
    unit: TimeUnit,
}

impl Unit for Time {
    type Unit = TimeUnit;

    fn new(value: f32, unit: Self::Unit) -> Self {
        Self { value, unit }
    }
}

#[derive(Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum TimeUnit {
    #[default]
    None,

    Seconds,
    Milliseconds,
}

impl Atomizable for TimeUnit {
    fn to_atom(&self) -> Atom {
        match self {
            Self::None => atom!(""),
            Self::Seconds => atom!("s"),
            Self::Milliseconds => atom!("ms"),
        }
    }

    fn from_atom(unit: Atom) -> Option<Self> {
        match unit {
            atom!("s") => Some(Self::Seconds),
            atom!("ms") => Some(Self::Milliseconds),
            _ => None,
        }
    }
}

impl Hash for Time {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.to_bits().hash(state);
        self.unit.hash(state);
    }
}
