pub(crate) use crate::units::*;
use css_parse::keyword_set;

// https://drafts.csswg.org/css-display-4/#typedef-display-outside
// <display-outside>  = block | inline | run-in
keyword_set!(DisplayOutside { Block: "block", Inline: "inline", RunIn: "run-in" });

// https://drafts.csswg.org/css-display-4/#typedef-display-inside
// <display-inside>   = flow | flow-root | table | flex | grid | ruby
keyword_set!(DisplayInside {
	Flow: "flow",
	FlowRoot: "flow-root",
	Table: "table",
	Flex: "flex",
	Grid: "grid",
	Ruby: "ruby",
});
