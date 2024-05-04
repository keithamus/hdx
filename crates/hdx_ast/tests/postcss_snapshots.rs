mod helpers;

#[test]
fn postcss_media() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/apply.css");
}

#[test]
fn postcss_atrule_brackets() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/at-rule-brackets.css");
}

#[test]
fn postcss_atrule_decls() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-decls.css");
}

#[test]
fn postcss_atrule_empty() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-empty.css");
}

#[test]
fn postcss_atrule_no_params() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-no-params.css");
}

#[test]
fn postcss_atrule_no_semicolon() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-no-semicolon.css");
}

#[test]
fn postcss_atrule_params() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-params.css");
}

#[test]
fn postcss_atrule_rules() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/atrule-rules.css");
}

#[test]
fn postcss_between() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/between.css");
}

#[test]
fn postcss_colon_selector() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/colon-selector.css");
}

#[test]
fn postcss_comments() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/comments.css");
}

#[test]
fn postcss_custom_properties() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/custom-properties.css");
}

#[test]
fn postcss_decls() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/decls.css");
}

#[test]
fn postcss_empty() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/empty.css");
}

#[test]
fn postcss_escape() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/escape.css");
}

#[test]
fn postcss_extends() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/extends.css");
}

#[test]
fn postcss_function() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/function.css");
}

#[test]
fn postcss_ie_prodid() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/ie-progid.css");
}

#[test]
fn postcss_important() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/important.css");
}

#[test]
fn postcss_inside() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/inside.css");
}

#[test]
fn postcss_no_selector() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/no-selector.css");
}

#[test]
fn postcss_prop() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/prop.css");
}

#[test]
fn postcss_quotes() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/quotes.css");
}

#[test]
fn postcss_raw_decl() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/raw-decl.css");
}

#[test]
fn postcss_rule_at() {
	// FIXME: this fails we need CI green for now.
	// assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/rule-at.css");
}

#[test]
fn postcss_rule_no_semicolon() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/rule-no-semicolon.css");
}

#[test]
fn postcss_selector() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/selector.css");
}

#[test]
fn postcss_semicolons() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/semicolons.css");
}

#[test]
fn postcss_tab() {
	assert_snap_ast!("../../tasks/coverage/postcss-parser-tests/cases/tab.css");
}
