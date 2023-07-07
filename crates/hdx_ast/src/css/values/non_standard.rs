use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.csswg.org/css-values/#typedef-length-percentage
#[derive(Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum ZoomValue {
    #[default]
    Normal,
    Reset,
    Number(f32),
    Percentage(f32),
}

impl Hash for ZoomValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Normal => 0.hash(state),
            Self::Reset => 1.hash(state),
            Self::Number(f) => {
                2.hash(state);
                f.to_bits().hash(state);
            }
            Self::Percentage(f) => {
                3.hash(state);
                f.to_bits().hash(state);
            }
        }
    }
}
