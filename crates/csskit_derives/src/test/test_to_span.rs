use super::{assert_derive_snapshot, to_deriveinput};
use crate::to_span;

macro_rules! assert_to_span_snapshot {
	( $data:ident, $name:literal) => {
		assert_derive_snapshot!(to_span::derive, $data, $name)
	};
}

#[test]
fn to_span_tuple_struct_single_field() {
	let data = to_deriveinput! {
		struct Length(Number);
	};
	assert_to_span_snapshot!(data, "to_span_tuple_struct_single_field");
}

#[test]
fn to_span_tuple_struct_multiple_fields() {
	let data = to_deriveinput! {
		struct Range(Number, Number);
	};
	assert_to_span_snapshot!(data, "to_span_tuple_struct_multiple_fields");
}

#[test]
fn to_span_enum_single_field_variants() {
	let data = to_deriveinput! {
		enum Display {
			Block(Ident),
			Inline(Ident),
			None(Ident),
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_single_field_variants");
}

#[test]
fn to_span_enum_with_named_struct_variant_single_field() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dashed { width: Length },
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_with_named_struct_variant_single_field");
}

#[test]
fn to_span_enum_with_named_struct_variant_multiple_fields() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dotted { radius: Length, spacing: Length },
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_with_named_struct_variant_multiple_fields");
}

#[test]
fn to_span_enum_mixed_variants() {
	let data = to_deriveinput! {
		enum FlexWrap {
			Nowrap(Ident),
			Wrap { wrap: Option<Ident>, balance: Option<Ident> },
			WrapReverse { wrap_reverse: Option<Ident>, balance: Option<Ident> },
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_mixed_variants");
}
