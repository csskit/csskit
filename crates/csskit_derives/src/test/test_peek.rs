use super::{assert_derive_snapshot, to_deriveinput};
use crate::peek;

macro_rules! assert_peek_snapshot {
	( $data:ident, $name:literal) => {
		assert_derive_snapshot!(peek::derive, $data, $name)
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

#[test]
fn peek_enum_struct_variant_one_must_occur() {
	let data = to_deriveinput! {
		enum FlexWrap {
			#[atom(FooAtoms::Nowrap)]
			Nowrap(Ident),
			#[parse(one_must_occur)]
			Wrap {
				#[atom(FooAtoms::Wrap)]
				wrap: Option<Ident>,
				#[atom(FooAtoms::Balance)]
				balance: Option<Ident>,
			},
			#[parse(one_must_occur)]
			WrapReverse {
				#[atom(FooAtoms::WrapReverse)]
				wrap_reverse: Option<Ident>,
				#[atom(FooAtoms::Balance)]
				balance: Option<Ident>,
			},
		}
	};
	assert_peek_snapshot!(data, "peek_enum_struct_variant_one_must_occur");
}

#[test]
fn peek_enum_variant_one_must_occur_distinct_types() {
	let data = to_deriveinput! {
		enum TextDecoration {
			#[parse(one_must_occur)]
			Decorated {
				#[atom(FooAtoms::Underline)]
				line: Option<Ident>,
				style: Option<DecorationStyle>,
				color: Option<Color>,
			},
		}
	};
	assert_peek_snapshot!(data, "peek_enum_variant_one_must_occur_distinct_types");
}

#[test]
fn peek_enum_variant_all_must_occur_distinct_types() {
	let data = to_deriveinput! {
		enum TextDecoration {
			#[parse(all_must_occur)]
			Decorated {
				#[atom(FooAtoms::Underline)]
				line: Option<Ident>,
				style: Option<DecorationStyle>,
				color: Option<Color>,
			},
		}
	};
	assert_peek_snapshot!(data, "peek_enum_variant_all_must_occur_distinct_types");
}

#[test]
fn peek_enum_with_multiple_identical_types() {
	let data = to_deriveinput! {
		enum FlowInto {
			#[atom(FooAtoms::None)]
			None(Ident),
			Element(
				CustomIdent,
				#[atom(FooAtoms::Element)]
				Ident,
			),
			Content(
				CustomIdent,
				#[atom(CssAtomSet::Content)]
				Ident,
			),
			CustomIdent(CustomIdent),
		}
	};
	assert_peek_snapshot!(data, "peek_enum_with_multiple_identical_types");
}
