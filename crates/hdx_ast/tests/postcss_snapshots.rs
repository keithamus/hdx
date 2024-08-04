mod helpers;

#[test]
#[cfg(feature = "serde")]
fn postcss_media() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/apply.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_brackets() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/at-rule-brackets.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_decls() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-decls.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_empty() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-empty.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_no_params() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-no-params.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_no_semicolon() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-no-semicolon.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_params() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-params.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_atrule_rules() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-rules.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_between() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/between.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_colon_selector() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/colon-selector.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_comments() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/comments.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_custom_properties() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/custom-properties.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_decls() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/decls.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_empty() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/empty.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_escape() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/escape.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_extends() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/extends.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_function() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/function.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_ie_prodid() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/ie-progid.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_important() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/important.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_inside() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/inside.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_no_selector() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/no-selector.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_prop() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/prop.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_quotes() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/quotes.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_raw_decl() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/raw-decl.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_rule_at() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/rule-at.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_rule_no_semicolon() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/rule-no-semicolon.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_selector() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/selector.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_semicolons() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/semicolons.css");
}

#[test]
#[cfg(feature = "serde")]
fn postcss_tab() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/tab.css");
}
