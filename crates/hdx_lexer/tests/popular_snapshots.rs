mod helpers;

#[test]
#[cfg(feature = "serde")]
fn popular_960() {
	assert_snap_tokens!("../../tasks/coverage/popular/960.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_animate() {
	assert_snap_tokens!("../../tasks/coverage/popular/animate.4.1.1.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_blueprint() {
	assert_snap_tokens!("../../tasks/coverage/popular/blueprint.1.0.1.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_bootstrap() {
	assert_snap_tokens!("../../tasks/coverage/popular/bootstrap.5.3.0.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_foundation() {
	assert_snap_tokens!("../../tasks/coverage/popular/foundation.6.7.5.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_inuitcss() {
	assert_snap_tokens!("../../tasks/coverage/popular/inuitcss.6.0.0.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_mini() {
	assert_snap_tokens!("../../tasks/coverage/popular/mini.css.3.0.1.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_open_props() {
	assert_snap_tokens!("../../tasks/coverage/popular/open-props.1.5.10.min.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_primer() {
	assert_snap_tokens!("../../tasks/coverage/popular/primer.21.5.1.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_pure() {
	assert_snap_tokens!("../../tasks/coverage/popular/pure.2.0.3.css");
}

#[test]
#[cfg(feature = "serde")]
fn popular_reset() {
	assert_snap_tokens!("../../tasks/coverage/popular/reset.2.0.css");
}
