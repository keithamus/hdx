use super::test_helpers::*;
use super::*;

#[test]
fn test_basic() {
	assert_highlight!(
		"basic",
		r#"body {
			appearance: initial;
			color: blue;
			will-ever-exist: foo;
		}"#,
	);
}

#[test]
fn test_tags() {
	assert_highlight!("tags", r#"body, dialog, madeup, marquee, portal, custom-element {}"#);
}

#[test]
fn test_nesting() {
	assert_highlight!("nesting", r#"body{main{dialog{input{}}}}"#);
}

#[test]
fn test_pseudo_classes() {
	assert_highlight!("pseudo_classes", r#"body:focus,dialog:modal{}"#);
}
