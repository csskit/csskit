mod helpers;

#[cfg(feature = "serde")]
#[test]
fn basic_media() {
	assert_snap_tokens!("../../coverage/basic/media.css");
}

#[cfg(feature = "serde")]
#[test]
fn basic_nth() {
	assert_snap_tokens!("../../coverage/basic/nth.css");
}

#[cfg(feature = "serde")]
#[test]
fn basic_rule() {
	assert_snap_tokens!("../../coverage/basic/rule.css");
}

#[cfg(feature = "serde")]
#[test]
fn basic_vars() {
	assert_snap_tokens!("../../coverage/basic/vars.css");
}
