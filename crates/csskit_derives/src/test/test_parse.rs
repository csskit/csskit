use super::to_deriveinput;
use crate::parse;

macro_rules! assert_parse_snapshot {
	( $data:ident, $name:literal) => {
		let tokens = parse::derive($data);
		let file = ::syn::parse2::<syn::File>(tokens).unwrap();
		let pretty = ::prettyplease::unparse(&file);
		::insta::assert_snapshot!($name, pretty)
	};
}

#[test]
fn parse_simple_struct() {
	let data = to_deriveinput! { struct Foo; };
	assert_parse_snapshot!(data, "parse_simple_struct");
}

#[test]
fn parse_struct_with_fields() {
	let data = to_deriveinput! {
		struct Color {
			red: CSSInt,
			green: CSSInt,
			blue: CSSInt,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_fields");
}

#[test]
fn parse_struct_with_lifetime() {
	let data = to_deriveinput! {
		struct Value<'a> {
			content: &'a Ident,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_lifetime");
}

#[test]
fn parse_tuple_struct() {
	let data = to_deriveinput! {
		struct Length(Number, Unit);
	};
	assert_parse_snapshot!(data, "parse_tuple_struct");
}

#[test]
fn parse_simple_enum() {
	let data = to_deriveinput! {
		enum Display {
			Block(Ident),
			Inline(Ident),
			None(Ident),
		}
	};
	assert_parse_snapshot!(data, "parse_simple_enum");
}

#[test]
fn parse_enum_with_fields() {
	let data = to_deriveinput! {
		enum Value {
			Keyword(String),
			Length(Length),
			Percentage(Percentage),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_with_fields");
}

#[test]
fn parse_enum_with_struct_variants() {
	let data = to_deriveinput! {
		enum BorderStyle {
			Solid,
			Dashed { width: Length },
			Dotted { radius: Length },
		}
	};
	assert_parse_snapshot!(data, "parse_enum_with_struct_variants");
}

#[test]
fn parse_with_state_attribute() {
	let data = to_deriveinput! {
		#[parse(state = State::InValue)]
		struct ValueInState {
			content: String,
		}
	};
	assert_parse_snapshot!(data, "parse_with_state_attribute");
}

#[test]
fn parse_with_stop_kind_attribute() {
	let data = to_deriveinput! {
		#[parse(stop = Kind::Semicolon)]
		struct StopOnSemicolon {
			items: Vec<String>,
		}
	};
	assert_parse_snapshot!(data, "parse_with_stop_kind_attribute");
}

#[test]
fn parse_with_stop_kindset_attribute() {
	let data = to_deriveinput! {
		#[parse(stop = KindSet::BlockEnd)]
		struct StopOnBlockEnd {
			declarations: Vec<String>,
		}
	};
	assert_parse_snapshot!(data, "parse_with_stop_kindset_attribute");
}

#[test]
fn parse_with_both_state_and_stop() {
	let data = to_deriveinput! {
		#[parse(state = State::InRule, stop = Kind::RightBrace)]
		struct RuleContent {
			declarations: Vec<String>,
		}
	};
	assert_parse_snapshot!(data, "parse_with_both_state_and_stop");
}

#[test]
fn parse_enum_with_lifetime() {
	let data = to_deriveinput! {
		enum CssValue<'a> {
			Keyword(&'a String),
			Length(Length),
			Function { name: &'a String, args: Vec<String> },
		}
	};
	assert_parse_snapshot!(data, "parse_enum_with_lifetime");
}

#[test]
fn parse_unit_struct() {
	let data = to_deriveinput! { struct Auto; };
	assert_parse_snapshot!(data, "parse_unit_struct");
}

#[test]
fn parse_single_field_tuple_struct() {
	let data = to_deriveinput! { struct Opacity(Number); };
	assert_parse_snapshot!(data, "parse_single_field_tuple_struct");
}

#[test]
fn parse_struct_with_many_fields() {
	let data = to_deriveinput! {
		struct Transform {
			rotate_x: Angle,
			rotate_y: Angle,
			rotate_z: Angle,
			translate_x: Length,
			translate_y: Length,
			translate_z: Length,
			scale_x: Number,
			scale_y: Number,
			scale_z: Number,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_many_fields");
}

#[test]
fn parse_struct_existing_lifetime() {
	let data = to_deriveinput! {
		struct Token<'a> {
			span: &'a str,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_existing_lifetime");
}

#[test]
fn parse_enum_struct_and_tuple_mixed() {
	let data = to_deriveinput! {
		enum FlexValue {
			Auto(Ident),
			Length(Length),
			Percentage(Percentage),
			Calc { expr: String, unit: Unit },
			MinMax { min: Length, max: Length },
		}
	};
	assert_parse_snapshot!(data, "parse_enum_struct_and_tuple_mixed");
}

#[test]
fn parse_enum_nested_generics() {
	let data = to_deriveinput! {
		enum Container<T> {
			Single(T),
			Multiple(Vec<T>),
			Optional(Option<T>),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_nested_generics");
}

#[test]
fn parse_with_multiple_state_stop_combinations() {
	let data = to_deriveinput! {
		#[parse(state = State::InDeclaration, stop = KindSet::DeclarationEnd)]
		struct Declaration {
			property: Ident,
			value: CSSValue,
		}
	};
	assert_parse_snapshot!(data, "parse_with_multiple_state_stop_combinations");
}
