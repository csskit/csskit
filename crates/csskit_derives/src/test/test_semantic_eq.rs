use super::to_deriveinput;
use crate::semantic_eq;

macro_rules! assert_semantic_eq_snapshot {
	( $data:ident, $name:literal) => {
		let tokens = semantic_eq::derive($data);
		let file = ::syn::parse2::<syn::File>(tokens).unwrap();
		let pretty = ::prettyplease::unparse(&file);
		::insta::assert_snapshot!($name, pretty)
	};
}

#[test]
fn semantic_eq_tuple_struct_single_field() {
	let data = to_deriveinput! {
		struct Length(Number);
	};
	assert_semantic_eq_snapshot!(data, "semantic_eq_tuple_struct_single_field");
}

#[test]
fn semantic_eq_enum_single_field_variants() {
	let data = to_deriveinput! {
		enum Display {
			Block(Ident),
			Inline(Ident),
			None(Ident),
		}
	};
	assert_semantic_eq_snapshot!(data, "semantic_eq_enum_single_field_variants");
}

#[test]
fn semantic_eq_enum_with_named_struct_variant_single_field() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dashed { width: Length },
		}
	};
	assert_semantic_eq_snapshot!(data, "semantic_eq_enum_with_named_struct_variant_single_field");
}

#[test]
fn semantic_eq_enum_with_named_struct_variant_multiple_fields() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dotted { radius: Length, spacing: Length },
		}
	};
	assert_semantic_eq_snapshot!(data, "semantic_eq_enum_with_named_struct_variant_multiple_fields");
}

#[test]
fn semantic_eq_enum_mixed_variants() {
	let data = to_deriveinput! {
		enum FlexWrap {
			Nowrap(Ident),
			Wrap { wrap: Option<Ident>, balance: Option<Ident> },
			WrapReverse { wrap_reverse: Option<Ident>, balance: Option<Ident> },
		}
	};
	assert_semantic_eq_snapshot!(data, "semantic_eq_enum_mixed_variants");
}
