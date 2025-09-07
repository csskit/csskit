use super::to_deriveinput;
use crate::to_cursors;

macro_rules! assert_to_cursors_snapshot {
	( $data:ident, $name:literal) => {
		let tokens = to_cursors::derive($data);
		let file = ::syn::parse2::<syn::File>(tokens).unwrap();
		let pretty = ::prettyplease::unparse(&file);
		::insta::assert_snapshot!($name, pretty)
	};
}

#[test]
fn to_cursors_tuple_struct_single_field() {
	let data = to_deriveinput! {
		struct Length(Number);
	};
	assert_to_cursors_snapshot!(data, "to_cursors_tuple_struct_single_field");
}

#[test]
fn to_cursors_tuple_struct_multiple_fields() {
	let data = to_deriveinput! {
		struct Position(Length, Length, Length);
	};
	assert_to_cursors_snapshot!(data, "to_cursors_tuple_struct_multiple_fields");
}

#[test]
fn to_cursors_named_struct() {
	let data = to_deriveinput! {
		struct Color {
			red: CSSInt,
			green: CSSInt,
			blue: CSSInt,
		}
	};
	assert_to_cursors_snapshot!(data, "to_cursors_named_struct");
}

#[test]
fn to_cursors_named_struct_with_lifetime() {
	let data = to_deriveinput! {
		struct Value<'a> {
			content: &'a Ident,
			unit: Unit,
		}
	};
	assert_to_cursors_snapshot!(data, "to_cursors_named_struct_with_lifetime");
}

#[test]
fn to_cursors_enum_single_field_variants() {
	let data = to_deriveinput! {
		enum Display {
			Block(Ident),
			Inline(Ident),
			None(Ident),
		}
	};
	assert_to_cursors_snapshot!(data, "to_cursors_enum_single_field_variants");
}

#[test]
fn to_cursors_enum_multiple_field_variants() {
	let data = to_deriveinput! {
		enum Value {
			Length(Number, Unit),
			Percentage(Number),
			Function(Ident, Vec<Value>),
		}
	};
	assert_to_cursors_snapshot!(data, "to_cursors_enum_multiple_field_variants");
}

#[test]
fn to_cursors_enum_mixed_variants() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dashed { width: Length },
			Dotted { radius: Length, spacing: Length },
		}
	};
	assert_to_cursors_snapshot!(data, "to_cursors_enum_mixed_variants");
}

#[test]
fn to_cursors_enum_with_lifetime() {
	let data = to_deriveinput! {
		enum CssValue<'a> {
			Keyword(&'a Ident),
			Function { name: &'a Ident, args: Vec<String> },
		}
	};
	assert_to_cursors_snapshot!(data, "to_cursors_enum_with_lifetime");
}
