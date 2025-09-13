use super::to_deriveinput;
use crate::parse;

macro_rules! assert_parse_snapshot {
	( $data:ident, $name:literal) => {
		let tokens = parse::derive($data);
		dbg!(tokens.to_string());
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
			Solid(Ident),
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

#[test]
fn parse_struct_with_inclusive_range() {
	let data = to_deriveinput! {
		struct Volume {
			#[in_range(0.0f32..=100.0f32)]
			level: Number,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_range");
}

#[test]
fn parse_struct_with_range_from() {
	let data = to_deriveinput! {
		struct PositiveValue {
			#[in_range(1.0f32..)]
			value: CSSInt,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_range_from");
}

#[test]
fn parse_struct_with_range_to_exclusive() {
	let data = to_deriveinput! {
		struct Probability {
			#[in_range(..1.0f32)]
			value: Number,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_range_to");
}

#[test]
fn parse_struct_with_multiple_range_fields() {
	let data = to_deriveinput! {
		struct Color {
			#[in_range(0..=255)]
			red: CSSInt,
			#[in_range(0..=255)]
			green: CSSInt,
			#[in_range(0..=255)]
			blue: CSSInt,
			#[in_range(0..=1)]
			alpha: Number,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_multiple_range_fields");
}

#[test]
fn parse_tuple_struct_with_range() {
	let data = to_deriveinput! {
		struct Scale(#[in_range(0.1..=10.0)] Number);
	};
	assert_parse_snapshot!(data, "parse_tuple_struct_with_range");
}

#[test]
fn parse_enum_with_range_validation() {
	let data = to_deriveinput! {
		enum Value {
			Percentage(#[in_range(0..=100)] Number),
			Scale(#[in_range(0.1..)] Number),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_with_range_validation");
}

#[test]
fn parse_enum_struct_variants_with_ranges() {
	let data = to_deriveinput! {
		enum Transform {
			Scale {
				#[in_range(0..)]
				x: Number,
				#[in_range(0..)]
				y: Number,
			},
			Rotate {
				#[in_range(-360..=360)]
				angle: Number,
			},
			Translate {
				x: Length,
				#[in_range(-100..=100)]
				y: Percentage,
			},
		}
	};
	assert_parse_snapshot!(data, "parse_enum_struct_variants_with_ranges");
}

#[test]
fn parse_struct_with_all_must_occur() {
	let data = to_deriveinput! {
		#[parse(all_must_occur)]
		struct AutoAndLength {
			auto: AutoKeyword,
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_all_must_occur");
}

#[test]
fn parse_struct_with_all_must_occur_and_range() {
	let data = to_deriveinput! {
		#[parse(all_must_occur)]
		struct AutoAndLengthWithRange {
			auto: AutoKeyword,
			#[in_range(0..=100)]
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_all_must_occur_and_range");
}

#[test]
fn parse_struct_with_all_must_occur_and_state() {
	let data = to_deriveinput! {
		#[parse(all_must_occur, state = State::InValue)]
		struct AutoAndLengthInState {
			auto: AutoKeyword,
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_all_must_occur_and_state");
}

#[test]
fn parse_enum_variant_with_all_must_occur() {
	let data = to_deriveinput! {
		enum Value {
			Normal(String),
			#[parse(all_must_occur)]
			Complex {
				auto: AutoKeyword,
				length: Length
			},
		}
	};
	assert_parse_snapshot!(data, "parse_enum_variant_with_all_must_occur");
}

#[test]
fn parse_enum_variant_with_all_must_occur_and_range() {
	let data = to_deriveinput! {
		enum Value {
			#[parse(all_must_occur)]
			WithRange {
				auto: AutoKeyword,
				#[in_range(0..=100)]
				percentage: Number,
			},
			Simple(String),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_variant_with_all_must_occur_and_range");
}

#[test]
fn parse_enum_mixed_variants() {
	let data = to_deriveinput! {
		enum FlexValue {
			Auto(AutoKeyword),
			#[parse(all_must_occur)]
			MinMax {
				min: Length,
				#[in_range(0..)]
				max: Length,
			},
			Length(Length),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_mixed_variants");
}

#[test]
fn parse_struct_with_keyword_pattern_and_range() {
	let data = to_deriveinput! {
		#[parse(all_must_occur)]
		struct KeywordWithRange {
			#[atom(FooKeywords::Auto)]
			auto_value: AutoValue,
			#[in_range(0..=100)]
			percentage: Number,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_keyword_pattern_and_range");
}

#[test]
fn parse_struct_with_different_keyword_variants() {
	let data = to_deriveinput! {
		#[parse(all_must_occur)]
		struct SpecificKeywordTest {
			#[atom(FooKeywords::Auto)]
			auto_field: AutoValue,
			#[atom(FooKeywords::None)]
			none_field: NoneValue,
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_different_keyword_variants");
}

#[test]
fn parse_struct_with_optional_keywords() {
	let data = to_deriveinput! {
		#[parse(one_must_occur)]
		struct KeywordWithRange {
			#[atom(FooKeywords::Auto)]
			auto_value: Option<Ident>,
			#[atom(FooKeywords::None)]
			none_value: Option<Ident>,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_optional_keywords");
}

#[test]
fn parse_struct_regular_with_keyword_pattern() {
	let data = to_deriveinput! {
		struct RegularKeywordTest {
			#[atom(FooKeywords::Auto)]
			auto_value: AutoValue,
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_regular_with_keyword_pattern");
}

#[test]
fn parse_enum_variant_with_keyword_pattern() {
	let data = to_deriveinput! {
		enum TestEnum {
			Normal(String),
			WithKeyword {
				#[atom(FooKeywords::None)]
				none_value: NoneValue,
				other_field: Length,
			},
		}
	};
	assert_parse_snapshot!(data, "parse_enum_variant_with_keyword_pattern");
}

#[test]
fn parse_enum_variant_all_must_occur_with_keyword() {
	let data = to_deriveinput! {
		enum AllMustOccurEnum {
			Simple(String),
			#[parse(all_must_occur)]
			Complex {
				#[atom(FooKeywords::Auto)]
				auto_field: AutoValue,
				#[atom(FooKeywords::None)]
				none_field: NoneValue,
				length: Length,
			},
		}
	};
	assert_parse_snapshot!(data, "parse_enum_variant_all_must_occur_with_keyword");
}

#[test]
fn parse_struct_with_newtype_keyword() {
	let data = to_deriveinput! {
		struct NewtypeKeywordTest {
			#[atom(Auto)]
			auto_value: Auto,
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_with_newtype_keyword");
}

#[test]
fn parse_struct_all_must_occur_with_newtype_keyword() {
	let data = to_deriveinput! {
		#[parse(all_must_occur)]
		struct AllMustOccurNewtypeTest {
			#[atom(Auto)]
			auto_value: Auto,
			#[atom(None)]
			none_value: None,
			length: Length,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_all_must_occur_with_newtype_keyword");
}

#[test]
fn parse_struct_one_must_occur_with_optionals() {
	let data = to_deriveinput! {
		#[parse(one_must_occur)]
		struct OneMustOccurTest {
			foo: Option<Foo>,
			bar: Option<Bar>,
		}
	};
	assert_parse_snapshot!(data, "parse_struct_one_must_occur_with_optionals");
}

#[test]
fn parse_enum_variant_with_keyword_variants() {
	let data = to_deriveinput! {
		enum NewtypeEnum {
			#[atom(Keyword::Foo)]
			Foo(Ident),
			#[atom(Keyword::Bar)]
			Bar(Ident),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_variant_with_keyword_variants");
}

#[test]
fn parse_enum_variant_with_keyword_variants_or_type() {
	let data = to_deriveinput! {
		enum NewtypeEnum {
			Length(Length),
			#[atom(Keyword::Foo)]
			Foo(Ident),
			#[atom(Keyword::Bar)]
			Bar(Ident),
		}
	};
	assert_parse_snapshot!(data, "parse_enum_variant_with_keyword_variants_or_type");
}
