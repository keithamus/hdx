use css_parse::keyword_set;

pub(crate) use crate::units::*;

// https://drafts.csswg.org/css-transitions-2/#typedef-transition-behavior-value
// <transition-behavior-value> = normal | allow-discrete
keyword_set!(TransitionBehaviorValue { Normal: "normal", AllowDiscrete: "allow-discrete" });
