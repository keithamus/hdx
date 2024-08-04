use hdx_proc_macro::*;

/**
 * https://drafts.csswg.org/css-box-3/
 * css-box-3
 *
 */

/*
 * https://drafts.csswg.org/css-box-3/#margin-physical
 * 3.1. Page-relative (Physical) Margin Properties: the margin-top, margin-right, margin-bottom, and margin-left properties
 */

// https://drafts.csswg.org/css-box-3/#propdef-margin-top
#[value(" <length-percentage> | auto ")]
#[initial(0)]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("margin")]
#[animation_type("by computed value type")]
pub enum MarginTop {}

// https://drafts.csswg.org/css-box-3/#propdef-margin-right
#[value(" <length-percentage> | auto ")]
#[initial(0)]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("margin")]
#[animation_type("by computed value type")]
pub enum MarginRight {}

// https://drafts.csswg.org/css-box-3/#propdef-margin-bottom
#[value(" <length-percentage> | auto ")]
#[initial(0)]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("margin")]
#[animation_type("by computed value type")]
pub enum MarginBottom {}

// https://drafts.csswg.org/css-box-3/#propdef-margin-left
#[value(" <length-percentage> | auto ")]
#[initial(0)]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[logical_property_group("margin")]
#[animation_type("by computed value type")]
pub enum MarginLeft {}

/*
 * https://drafts.csswg.org/css-box-3/#margin-shorthand
 * 3.2. Margin Shorthand: the margin property
 */

// https://drafts.csswg.org/css-box-3/#propdef-margin
#[value(" <'margin-top'>{1,4} ")]
#[initial(0)]
#[applies_to("all elements except internal table elements, ruby base containers, and ruby annotation containers")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("by computed value type")]
pub enum Margin {}

/**
 * https://drafts.csswg.org/css-box-4/
 * css-box-4
 *
 */

/*
 * https://drafts.csswg.org/css-box-4/#margin-trim
 * 3.3. Margins at Container Edges: the margin-trim property
 */

// https://drafts.csswg.org/css-box-4/#propdef-margin-trim
#[value(" none | [ block || inline ] | [ block-start || inline-start || block-end || inline-end ] ")]
#[initial(none)]
#[applies_to("block containers, multi-column containers, flex containers, grid containers")]
#[inherited("no")]
#[canonical_order("per grammar")]
#[animation_type("discrete")]
pub enum Margin {}
