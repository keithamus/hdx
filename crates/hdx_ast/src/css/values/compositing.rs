#[cfg(feature = "serde")]
use serde::Serialize;

// https://drafts.fxtf.org/compositing/#ltblendmodegt
#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDoge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn size_test() {
        use std::mem::size_of;
        assert_eq!(size_of::<BlendMode>(), 1);
    }
}
