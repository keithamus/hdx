mod helpers;

#[test]
#[cfg(feature = "serde")]
fn basic_media() {
	assert_snap_tokens!("../../tasks/coverage/basic/media.css");
}

#[test]
#[cfg(feature = "serde")]
fn basic_nth() {
	assert_snap_tokens!("../../tasks/coverage/basic/nth.css");
}

#[test]
#[cfg(feature = "serde")]
fn basic_rule() {
	assert_snap_tokens!("../../tasks/coverage/basic/rule.css");
}

#[test]
#[cfg(feature = "serde")]
fn basic_vars() {
	assert_snap_tokens!("../../tasks/coverage/basic/vars.css");
}
