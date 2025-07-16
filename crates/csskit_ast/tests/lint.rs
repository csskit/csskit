mod helpers;

#[test]
fn empty_block() {
	assert_snap_ast!("../css_lint/rules/empty-block.cks");
}
