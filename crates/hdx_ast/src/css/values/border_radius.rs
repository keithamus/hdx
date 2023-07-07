use std::hash::Hash;

#[cfg(feature = "serde")]
use serde::Serialize;

use super::LengthPercentage;
use crate::Box;

// https://drafts.csswg.org/css-values/#angles
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct BorderRadiusValue<'a> {
    pub top_left: Box<'a, (LengthPercentage, LengthPercentage)>,
    pub top_right: Box<'a, (LengthPercentage, LengthPercentage)>,
    pub bottom_left: Box<'a, (LengthPercentage, LengthPercentage)>,
    pub bottom_right: Box<'a, (LengthPercentage, LengthPercentage)>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_test() {
        use std::mem::size_of;
        assert_eq!(size_of::<BorderRadiusValue>(), 32);
    }
}
