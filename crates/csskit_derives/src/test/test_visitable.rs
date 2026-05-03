use super::{assert_derive_snapshot, to_deriveinput};
use crate::visitable;

macro_rules! assert_visitable_snapshot {
	( $data:ident, $name:literal) => {
		assert_derive_snapshot!(visitable::derive, $data, $name)
	};
}

#[test]
fn visitable_tuple_enum() {
	let data = to_deriveinput! {
		enum Display {
			Block(Ident),
			Inline(Ident),
		}
	};
	assert_visitable_snapshot!(data, "visitable_tuple_enum");
}

#[test]
fn visitable_enum_with_named_struct_variant() {
	let data = to_deriveinput! {
		enum FlexWrap {
			Nowrap(Ident),
			Wrap {
				wrap: Option<Ident>,
				balance: Option<Ident>,
			},
			WrapReverse {
				wrap_reverse: Option<Ident>,
				balance: Option<Ident>,
			},
		}
	};
	assert_visitable_snapshot!(data, "visitable_enum_with_named_struct_variant");
}
