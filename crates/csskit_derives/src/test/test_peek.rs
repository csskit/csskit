use super::to_deriveinput;
use crate::peek;

macro_rules! assert_peek_snapshot {
	( $data:ident, $name:literal) => {
		let tokens = peek::derive($data);
		let file = ::syn::parse2::<syn::File>(tokens).unwrap();
		let pretty = ::prettyplease::unparse(&file);
		::insta::assert_snapshot!($name, pretty)
	};
}

#[test]
fn peek_simple_struct() {
	let data = to_deriveinput! {
		struct Length(Number);
	};
	assert_peek_snapshot!(data, "peek_simple_struct");
}

#[test]
fn peek_struct_with_multiple_fields() {
	let data = to_deriveinput! {
		struct Color {
			red: CSSInt,
			green: CSSInt,
			blue: CSSInt,
		}
	};
	assert_peek_snapshot!(data, "peek_struct_with_multiple_fields");
}

#[test]
fn peek_struct_with_lifetime() {
	let data = to_deriveinput! {
		struct Value<'a> {
			content: &'a Ident,
		}
	};
	assert_peek_snapshot!(data, "peek_struct_with_lifetime");
}

#[test]
fn peek_simple_enum() {
	let data = to_deriveinput! {
		enum Display {
			Block(Ident),
			Inline(Ident),
			None(Ident),
		}
	};
	assert_peek_snapshot!(data, "peek_simple_enum");
}

#[test]
fn peek_enum_with_different_types() {
	let data = to_deriveinput! {
		enum Value {
			Keyword(Ident),
			Length(Length),
			Percentage(Percentage),
		}
	};
	assert_peek_snapshot!(data, "peek_enum_with_different_types");
}

#[test]
fn peek_enum_with_duplicate_types() {
	let data = to_deriveinput! {
		enum Color {
			Red(CSSInt),
			Green(CSSInt),
			Blue(CSSInt),
		}
	};
	assert_peek_snapshot!(data, "peek_enum_with_duplicate_types");
}

#[test]
fn peek_enum_with_struct_variants() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dashed { width: Length },
			Dotted { radius: Length },
		}
	};
	assert_peek_snapshot!(data, "peek_enum_with_struct_variants");
}

#[test]
fn peek_enum_with_lifetime() {
	let data = to_deriveinput! {
		enum CssValue<'a> {
			Keyword(&'a Ident),
			Length(Length),
			Function { name: &'a String, args: Vec<String> },
		}
	};
	assert_peek_snapshot!(data, "peek_enum_with_lifetime");
}
