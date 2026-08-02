#[test]
fn assert_layouts() {
	use stable_type_layout::render;
	insta::assert_snapshot!("assert_layouts", render());
}
