mod helpers;

#[test]
fn basic_media() {
	assert_snap_ast!("../../tasks/coverage/basic/media.css");
}

#[test]
fn basic_nth() {
	assert_snap_ast!("../../tasks/coverage/basic/nth.css");
}

#[test]
fn basic_rule() {
	assert_snap_ast!("../../tasks/coverage/basic/rule.css");
}

#[test]
fn basic_vars() {
	assert_snap_ast!("../../tasks/coverage/basic/vars.css");
}
