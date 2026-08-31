use crate::declaration_metadata::derive;
use crate::test::to_deriveinput;

/// A shorthand states the longhands its value writes, so slots without them name nothing.
#[test]
fn rejects_writes_without_longhands() {
	let input = to_deriveinput! {
		#[declaration_writes(MarginTop, MarginRight?)]
		struct MarginStyleValue<'a>;
	};
	let error = derive(input).expect_err("derive accepted writes without longhands");
	assert_eq!(error.to_string(), "declaration_writes states no longhands for the value to write");
}
