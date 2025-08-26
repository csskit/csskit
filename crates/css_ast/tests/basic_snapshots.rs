mod helpers;

#[test]
fn basic_media() {
	assert_snap_ast!("../../coverage/basic/media.css");
}

#[test]
fn basic_nth() {
	assert_snap_ast!("../../coverage/basic/nth.css");
}

#[test]
fn basic_rule() {
	assert_snap_ast!("../../coverage/basic/rule.css");
}

#[test]
fn basic_vars() {
	assert_snap_ast!("../../coverage/basic/vars.css");
}
