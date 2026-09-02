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

#[test]
fn to_span_enum_variant_with_optional_middle_field() {
	let data = to_deriveinput! {
		enum VerticalAlign {
			First { first: Option<Ident>, alignment_baseline: Option<Ident>, baseline_shift: Option<Length> },
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_variant_with_optional_middle_field");
}

#[test]
fn to_span_unit_struct() {
	let data = to_deriveinput! {
		struct Nothing;
	};
	assert_to_span_snapshot!(data, "to_span_unit_struct");
}

#[test]
fn to_span_all_optional_struct() {
	let data = to_deriveinput! {
		struct Sides { top: Option<Length>, right: Option<Length>, bottom: Option<Length> }
	};
	assert_to_span_snapshot!(data, "to_span_all_optional_struct");
}

#[test]
fn to_span_struct_with_required_middle_field() {
	let data = to_deriveinput! {
		struct Block { open: Curly, items: Vec<Item>, close: Curly }
	};
	assert_to_span_snapshot!(data, "to_span_struct_with_required_middle_field");
}

#[test]
fn to_span_enum_variant_with_required_middle_field() {
	let data = to_deriveinput! {
		enum Rule {
			Block { open: Curly, items: Vec<Item>, close: Curly },
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_variant_with_required_middle_field");
}

#[test]
fn to_span_enum_variant_with_fields_named_first_and_last() {
	let data = to_deriveinput! {
		enum BaselinePosition {
			Both { last: Ident, spread: Option<Length>, first: Ident },
		}
	};
	assert_to_span_snapshot!(data, "to_span_enum_variant_with_fields_named_first_and_last");
}

#[test]
fn to_span_tuple_struct_optional_first_and_last() {
	let data = to_deriveinput! {
		struct Ratio(Option<Number>, Slash, Option<Number>);
	};
	assert_to_span_snapshot!(data, "to_span_tuple_struct_optional_first_and_last");
}

#[test]
fn to_span_generic_struct() {
	let data = to_deriveinput! {
		struct Pair<T, U> { left: T, right: U }
	};
	assert_to_span_snapshot!(data, "to_span_generic_struct");
}
