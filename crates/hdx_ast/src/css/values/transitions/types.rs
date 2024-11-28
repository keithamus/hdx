use hdx_parser::keyword_typedef;

pub(crate) use crate::css::units::*;

// https://drafts.csswg.org/css-transitions-2/#typedef-transition-behavior-value
// <transition-behavior-value> = normal | allow-discrete
keyword_typedef!(TransitionBehaviorValue { Normal: atom!("normal"), AllowDiscrete: atom!("allow-discrete") });
