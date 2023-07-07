use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

pub mod lengths;

pub use lengths::*;

#[derive(Default, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Percentage(pub f32);

impl Hash for Percentage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_test() {
        use std::mem::size_of;
        assert_eq!(size_of::<Percentage>(), 4);
    }
}
