use crate::{def::*, syntax::generate};
use quote::quote;

macro_rules! to_valuedef {
	( $lit:literal ) => {
		::syn::parse2::<StrWrapped<Def>>(::quote::quote!{ $lit }).unwrap().0.optimize()
	};
	( $($tt:tt)+ ) => {
		::syn::parse2::<Def>(::quote::quote!{ $($tt)+ }).unwrap().optimize()
	};
}

macro_rules! to_deriveinput {
	( $($tt:tt)+ ) => {
		::syn::parse2::<::syn::DeriveInput>(::quote::quote!{ $($tt)+ }).unwrap()
	}
}

macro_rules! assert_snapshot {
	( $syntax:ident, $data:ident, $name:literal) => {
		let file = ::syn::parse2::<syn::File>(generate($syntax, $data)).unwrap();
		let pretty = ::prettyplease::unparse(&file);
		::insta::assert_snapshot!($name, pretty)
	};
}

#[test]
fn test_def_builds_type() {
	assert_eq!(to_valuedef!( <integer> ), Def::Type(DefType::Integer(DefRange::None)))
}

#[test]
fn test_def_builds_quoted_type() {
	assert_eq!(
		::syn::parse2::<StrWrapped<Def>>(quote! { "<'some-prop'>" }).unwrap().0,
		Def::Type(DefType::Custom(DefIdent("SomePropStyleValue".into())))
	)
}

#[test]
fn test_def_builds_type_with_multiplier_oneormore() {
	assert_eq!(
		to_valuedef!( <integer>+ ),
		Def::Multiplier(
			Box::new(Def::Type(DefType::Integer(DefRange::None))),
			DefMultiplierSeparator::None,
			DefRange::RangeFrom(1.)
		)
	)
}

#[test]
fn def_builds_type_with_checks() {
	assert_eq!(to_valuedef! { <integer [1,3]> }, Def::Type(DefType::Integer(DefRange::Range(1f32..3f32))))
}

#[test]
fn test_def_builds_optional() {
	assert_eq!(to_valuedef!( <integer>? ), Def::Optional(Box::new(Def::Type(DefType::Integer(DefRange::None)))))
}

#[test]
fn test_def_builds_quoted_custom_type_with_count() {
	assert_eq!(
		::syn::parse2::<StrWrapped<Def>>(quote! { "<'animation-delay'>{1,}" }).unwrap().0,
		Def::Multiplier(
			Box::new(Def::Type(DefType::Custom(DefIdent("AnimationDelayStyleValue".into())),)),
			DefMultiplierSeparator::None,
			DefRange::RangeFrom(1.)
		)
	)
}

#[test]
fn def_builds_combinator_of_keywords() {
	assert_eq!(
		to_valuedef! { foo | bar },
		Def::Combinator(
			vec![Def::Ident(DefIdent("foo".into())), Def::Ident(DefIdent("bar".into()))],
			DefCombinatorStyle::Alternatives,
		)
	)
}

#[test]
fn def_builds_ordered_combinator_of_keywords() {
	assert_eq!(
		to_valuedef! { none auto },
		Def::Combinator(
			vec![Def::Ident(DefIdent("none".into())), Def::Ident(DefIdent("auto".into()))],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn test_def_builds_dashed_idents() {
	assert_eq!(
		to_valuedef!( length-percentage preserve-3d  ),
		Def::Combinator(
			vec![Def::Ident(DefIdent("length-percentage".into())), Def::Ident(DefIdent("preserve-3d".into()))],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn def_builds_group_with_brackets() {
	assert_eq!(
		to_valuedef! { [ block || inline ] | foo },
		Def::Combinator(
			vec![
				Def::Combinator(
					vec![Def::Ident(DefIdent("block".into())), Def::Ident(DefIdent("inline".into()))],
					DefCombinatorStyle::Options,
				),
				Def::Ident(DefIdent("foo".into())),
			],
			DefCombinatorStyle::Alternatives,
		)
	);
}

#[test]
fn def_builds_combinator_with_correct_precedence() {
	assert_eq!(
		to_valuedef! { foo | underline || overline },
		Def::Combinator(
			vec![
				Def::Ident(DefIdent("foo".into())),
				Def::Combinator(
					vec![Def::Ident(DefIdent("underline".into())), Def::Ident(DefIdent("overline".into()))],
					DefCombinatorStyle::Options,
				),
			],
			DefCombinatorStyle::Alternatives,
		)
	);
}

#[test]
fn def_builds_combinator_with_correct_precedence2() {
	assert_eq!(
		to_valuedef! { underline || overline | foo },
		Def::Combinator(
			vec![
				Def::Combinator(
					vec![Def::Ident(DefIdent("underline".into())), Def::Ident(DefIdent("overline".into()))],
					DefCombinatorStyle::Options,
				),
				Def::Ident(DefIdent("foo".into())),
			],
			DefCombinatorStyle::Alternatives,
		)
	);
}

#[test]
fn def_builds_combinator_with_correct_precedence3() {
	assert_eq!(
		to_valuedef! { auto foo | underline || overline && block inline },
		Def::Combinator(
			vec![
				Def::Combinator(
					vec![Def::Ident(DefIdent("auto".into())), Def::Ident(DefIdent("foo".into()))],
					DefCombinatorStyle::Ordered,
				),
				Def::Combinator(
					vec![
						Def::Ident(DefIdent("underline".into())),
						Def::Combinator(
							vec![
								Def::Ident(DefIdent("overline".into())),
								Def::Combinator(
									vec![Def::Ident(DefIdent("block".into())), Def::Ident(DefIdent("inline".into()))],
									DefCombinatorStyle::Ordered,
								),
							],
							DefCombinatorStyle::AllMustOccur,
						),
					],
					DefCombinatorStyle::Options,
				),
			],
			DefCombinatorStyle::Alternatives,
		)
	);
}

#[test]
fn def_builds_group_of_types_and_keywords() {
	assert_eq!(
		to_valuedef! { <length [1,]> | foo },
		Def::Combinator(
			vec![Def::Type(DefType::Length(DefRange::RangeFrom(1.))), Def::Ident(DefIdent("foo".into()))],
			DefCombinatorStyle::Alternatives,
		)
	)
}

#[test]
fn def_optimizes_length_or_auto_to_lengthorauto_type() {
	assert_eq!(
		to_valuedef! { auto | <length> },
		Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::Length(DefRange::None)))))
	);
	assert_eq!(
		to_valuedef! { <length [1,]> | auto },
		Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::Length(DefRange::RangeFrom(1.))))))
	);
}

#[test]
fn def_optimizes_lengthpercentage_or_flex_to_lengthpercentageorflex_type() {
	assert_eq!(
		to_valuedef! { <flex> | <length-percentage> },
		Def::Type(DefType::LengthPercentageOrFlex(DefRange::None))
	);
	assert_eq!(
		to_valuedef! { <length-percentage [1,]> | <flex> },
		Def::Type(DefType::LengthPercentageOrFlex(DefRange::RangeFrom(1.)))
	);
}

#[test]
fn def_optimizes_length_or_auto_range_to_ordered_combinator_lengthorauto_type() {
	assert_eq!(
		to_valuedef! { [ auto | <length-percentage> ]{1,4} },
		Def::Combinator(
			vec![
				Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::LengthPercentage(DefRange::None))))),
				Def::Optional(Box::new(Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::LengthPercentage(
					DefRange::None
				))))))),
				Def::Optional(Box::new(Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::LengthPercentage(
					DefRange::None
				))))))),
				Def::Optional(Box::new(Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::LengthPercentage(
					DefRange::None
				))))))),
			],
			DefCombinatorStyle::Ordered
		)
	);
}

#[test]
fn def_builds_multiplier_of_types() {
	assert_eq!(
		to_valuedef! { <length># },
		Def::Multiplier(
			Box::new(Def::Type(DefType::Length(DefRange::None))),
			DefMultiplierSeparator::Commas,
			DefRange::RangeFrom(1.)
		)
	)
}

#[test]
fn def_builds_multiplier_of_types_zero_or_more_comma() {
	assert_eq!(
		to_valuedef! { <length>#? },
		Def::Multiplier(
			Box::new(Def::Type(DefType::Length(DefRange::None))),
			DefMultiplierSeparator::Commas,
			DefRange::RangeFrom(0.)
		)
	)
}

#[test]
fn def_builds_with_literal_chars() {
	assert_eq!(
		to_valuedef! { <color> / <color> },
		Def::Combinator(
			vec![Def::Type(DefType::Color), Def::Punct('/'), Def::Type(DefType::Color)],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn def_builds_multiplier_of_types_with_range() {
	let range = 5f32..12f32;
	assert_eq!(
		to_valuedef! { <length>#{5,12} },
		Def::Multiplier(
			Box::new(Def::Type(DefType::Length(DefRange::None))),
			DefMultiplierSeparator::Commas,
			DefRange::Range(range)
		)
	)
}

#[test]
fn def_builds_multiplier_of_type_fixed_range_as_ordered_combinator() {
	assert_eq!(
		to_valuedef! { <length>{5} },
		Def::Combinator(
			vec![
				Def::Type(DefType::Length(DefRange::None)),
				Def::Type(DefType::Length(DefRange::None)),
				Def::Type(DefType::Length(DefRange::None)),
				Def::Type(DefType::Length(DefRange::None)),
				Def::Type(DefType::Length(DefRange::None)),
			],
			DefCombinatorStyle::Ordered
		)
	)
}

#[test]
fn def_builds_multiplier_of_small_range_as_ordered_combinator1() {
	assert_eq!(
		to_valuedef! { <length>{1,2} },
		Def::Combinator(
			vec![
				Def::Type(DefType::Length(DefRange::None)),
				Def::Optional(Box::new(Def::Type(DefType::Length(DefRange::None)))),
			],
			DefCombinatorStyle::Ordered
		)
	)
}

#[test]
fn def_builds_multiplier_of_small_range_as_ordered_combinator2() {
	assert_eq!(
		to_valuedef! { <length>{2,4} },
		Def::Combinator(
			vec![
				Def::Type(DefType::Length(DefRange::None)),
				Def::Type(DefType::Length(DefRange::None)),
				Def::Optional(Box::new(Def::Type(DefType::Length(DefRange::None)))),
				Def::Optional(Box::new(Def::Type(DefType::Length(DefRange::None)))),
			],
			DefCombinatorStyle::Ordered
		)
	)
}

#[test]
fn def_builds_multiplier_of_small_range_as_ordered_combinator3() {
	assert_eq!(
		to_valuedef! { <length>{0,3} },
		Def::Combinator(
			vec![
				Def::Optional(Box::new(Def::Type(DefType::Length(DefRange::None)))),
				Def::Optional(Box::new(Def::Type(DefType::Length(DefRange::None)))),
				Def::Optional(Box::new(Def::Type(DefType::Length(DefRange::None)))),
			],
			DefCombinatorStyle::Ordered
		)
	)
}

#[test]
fn def_elides_group_over_single_type() {
	assert_eq!(
		to_valuedef! { foo | [ <length> ] },
		Def::Combinator(
			vec![Def::Ident(DefIdent("foo".into())), Def::Type(DefType::Length(DefRange::None)),],
			DefCombinatorStyle::Alternatives
		)
	)
}

#[test]
fn def_elides_group_over_ordered_combinator() {
	assert_eq!(
		to_valuedef! { foo | [ manual? <length> ] },
		Def::Combinator(
			vec![
				Def::Ident(DefIdent("foo".into())),
				Def::Combinator(
					vec![
						Def::Optional(Box::new(Def::Ident(DefIdent("manual".into())))),
						Def::Type(DefType::Length(DefRange::None)),
					],
					DefCombinatorStyle::Ordered
				),
			],
			DefCombinatorStyle::Alternatives
		)
	)
}

#[test]
fn def_elides_group_over_alternatives_combinator() {
	assert_eq!(
		to_valuedef! { manual? [ left | right ] },
		Def::Combinator(
			vec![
				Def::Optional(Box::new(Def::Ident(DefIdent("manual".into())))),
				Def::Combinator(
					vec![Def::Ident(DefIdent("left".into())), Def::Ident(DefIdent("right".into())),],
					DefCombinatorStyle::Alternatives
				),
			],
			DefCombinatorStyle::Ordered
		),
	)
}

#[test]
fn def_converts_group_of_one_or_more_to_multiplier() {
	assert_eq!(
		to_valuedef! { foo | [ <length> ]+ },
		Def::Combinator(
			vec![
				Def::Ident(DefIdent("foo".into())),
				Def::Multiplier(
					Box::new(Def::Type(DefType::Length(DefRange::None))),
					DefMultiplierSeparator::None,
					DefRange::RangeFrom(1.0)
				)
			],
			DefCombinatorStyle::Alternatives
		)
	)
}

#[test]
fn def_builds_complex_combination_1() {
	assert_eq!(
		to_valuedef! { [ inset? && <length>{2,} && <color>? ]# | foo },
		Def::Combinator(
			vec![
				Def::Multiplier(
					Box::new(Def::Combinator(
						vec![
							Def::Optional(Box::new(Def::Ident(DefIdent("inset".into())))),
							Def::Multiplier(
								Box::new(Def::Type(DefType::Length(DefRange::None))),
								DefMultiplierSeparator::None,
								DefRange::RangeFrom(2.),
							),
							Def::Optional(Box::new(Def::Type(DefType::Color))),
						],
						DefCombinatorStyle::AllMustOccur,
					)),
					DefMultiplierSeparator::Commas,
					DefRange::RangeFrom(1.),
				),
				Def::Ident(DefIdent("foo".into())),
			],
			DefCombinatorStyle::Alternatives,
		)
	)
}

#[test]
fn def_ordered_combinator_alt_none() {
	assert_eq!(
		to_valuedef! { <foo> <bar> | none },
		Def::Type(DefType::NoneOr(Box::new(Def::Combinator(
			vec![
				Def::Type(DefType::Custom(DefIdent("Foo".to_string()))),
				Def::Type(DefType::Custom(DefIdent("Bar".to_string()))),
			],
			DefCombinatorStyle::Ordered
		))))
	)
}

#[test]
fn value_lone_type() {
	let syntax = to_valuedef! { <integer> };
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "value_lone_type");
}

#[test]
fn value_lone_type_errors_with_lifetime_when_necessary() {
	let syntax = to_valuedef! { <image> }; // <image> needs lifetime
	let data = to_deriveinput! { struct Foo; }; // Foo has no lifetime
	assert_snapshot!(syntax, data, "value_lone_type_errors_with_lifetime_when_necessary");
}

#[test]
fn value_lone_type_with_lifetime_2() {
	let syntax = to_valuedef! { <image> }; // <image> needs lifetime
	let data = to_deriveinput! { struct Foo<'a>; }; // Foo specifies lifetime
	assert_snapshot!(syntax, data, "value_lone_type_with_lifetime");
}

#[test]
fn value_vec_type_with_lifetime() {
	let syntax = to_valuedef! { <image># }; // <image> needs lifetime
	let data = to_deriveinput! { struct Foo<'a>; }; // Foo specifies lifetime
	assert_snapshot!(syntax, data, "value_vec_type_with_lifetime");
}

#[test]
fn value_lone_custom_type() {
	let syntax = to_valuedef! { <custom-ident> };
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "value_lone_custom_type");
}

#[test]
fn enum_type_with_lifetime() {
	let syntax = to_valuedef! { <color> | <image-1D> }; // <image-1D> needs lifetime
	let data = to_deriveinput! { enum Foo<'a> {} }; // Foo specifies lifetime
	assert_snapshot!(syntax, data, "enum_type_with_lifetime");
}

#[test]
fn multiple_keywords() {
	let syntax = to_valuedef!("black | white | line-through | pink");
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "multiple_keywords");
}

#[test]
fn multiple_keywords_derive_parse() {
	let syntax = to_valuedef!("black | white | line-through | pink");
	let data = to_deriveinput! { #[derive(Parse)] enum Foo {} };
	assert_snapshot!(syntax, data, "multiple_keywords_derive_parse");
}

#[test]
fn value_group_type_keyword() {
	let syntax = to_valuedef!( <length [1,]> | line-through );
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "value_group_type_keyword");
}

#[test]
fn value_with_multiplier_range() {
	let syntax = to_valuedef!( <length>{2,4} );
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "value_with_multiplier_range");
}

#[test]
fn value_with_multiplier_oneormore() {
	let syntax = to_valuedef! { foo | <length>+ };
	let data = to_deriveinput! { enum Foo<'a> {} };
	assert_snapshot!(syntax, data, "value_with_multiplier_oneormore");
}

#[test]
fn keyword_or_type() {
	let syntax = to_valuedef!( foo | <custom-ident> );
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "keyword_or_type");
}

#[test]
fn custom_type_with_checks() {
	let syntax = to_valuedef!(" foo | <length-percentage [0,∞]> ");
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "custom_type_with_checks");
}

#[test]
fn custom_type_with_checks_derive_parse() {
	let syntax = to_valuedef!(" foo | <length-percentage [0,∞]> ");
	let data = to_deriveinput! { #[derive(Parse)] enum Foo {} };
	assert_snapshot!(syntax, data, "custom_type_with_checks_derive_parse");
}

#[test]
fn custom_function_type() {
	let syntax = to_valuedef!(" foo | <calc-size()> ");
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "custom_function_type");
}

#[test]
fn custom_function_variant_with_args() {
	let syntax = to_valuedef!(" fit-content | fit-content(<length-percentage [0,∞]>) ");
	let data = to_deriveinput! { enum Foo<'a> {} };
	assert_snapshot!(syntax, data, "custom_function_variant_with_args");
}

#[test]
fn custom_function_variant_with_multiplier_args() {
	let syntax = to_valuedef!(" normal | styleset(<feature-value-name>#) ");
	let data = to_deriveinput! { enum Foo<'a> {} };
	assert_snapshot!(syntax, data, "custom_function_variant_with_multiplier_args");
}

#[test]
fn custom_function_all_optionals() {
	let syntax = to_valuedef!(" <'caret-color'> || <'caret-animation'> || <'caret-shape'> ");
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "custom_function_all_optionals");
}

#[test]
fn ordered_custom_function_last_option() {
	let syntax = to_valuedef!(" <'caret-color'> <'caret-animation'>? ");
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "ordered_custom_function_last_option");
}

#[test]
fn struct_with_variable_count_type() {
	let syntax = to_valuedef!(" <animateable-feature># ");
	let data = to_deriveinput! { struct Foo<'a>; };
	assert_snapshot!(syntax, data, "struct_with_variable_count_type");
}

#[test]
fn struct_with_zero_or_more_comma() {
	let syntax = to_valuedef!(" <animateable-feature>#? ");
	let data = to_deriveinput! { struct Foo<'a>; };
	assert_snapshot!(syntax, data, "struct_with_one_or_more_commas");
}

#[test]
fn enum_with_variable_count_type() {
	let syntax = to_valuedef!(" foo | <animateable-feature># ");
	let data = to_deriveinput! { enum Foo<'a> {} };
	assert_snapshot!(syntax, data, "enum_with_variable_count_type");
}

#[test]
fn bounded_range_multiplier_is_optimized_to_options() {
	let syntax = to_valuedef!(" <animateable-feature>{1,3} ");
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "bounded_range_multiplier_is_optimized_to_options");
}

#[test]
fn bounded_range_multiplier_is_optimized_to_options_with_lifetimes_when_necessary() {
	let syntax = to_valuedef!(" <'border-top-color'>{1,2} ");
	let data = to_deriveinput! { struct Foo<'a> {} }; // Foo specifies lifetime
	assert_snapshot!(syntax, data, "bounded_range_multiplier_is_optimized_to_options_with_lifetimes_when_necessary");
}

#[test]
fn bound_range_multiplier_with_keyword() {
	let syntax = to_valuedef!(" <length>{1,2} | foo ");
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "bound_range_multiplier_with_keyword");
}

#[test]
fn value_fixed_range_color2_optimized() {
	let syntax = to_valuedef! { <color>{2} };
	let data = to_deriveinput! { struct Foo {} };
	assert_snapshot!(syntax, data, "value_fixed_range_color2_optimized");
}

#[test]
fn value_with_derive_visitable_adds_attributes() {
	let syntax = to_valuedef! { foo | bar };
	let data = to_deriveinput! { #[derive(Visitable)] enum Foo {} };
	assert_snapshot!(syntax, data, "value_with_derive_visitable_adds_attributes");
}

#[test]
fn value_with_derive_parse_skips_impl() {
	let syntax = to_valuedef! { foo | bar };
	let data = to_deriveinput! { #[derive(Parse)] enum Foo {} };
	assert_snapshot!(syntax, data, "value_with_derive_parse_skips_impl");
}

#[test]
fn value_fixed_range_auto_color2_optimized() {
	let syntax = to_valuedef! { foo | <color>{2} };
	let data = to_deriveinput! { #[derive(Visitable)] enum Foo {} };
	assert_snapshot!(syntax, data, "value_fixed_range_auto_color2_optimized");
}

#[test]
fn keyword_int_literal() {
	let syntax = to_valuedef! { keyword | 2 };
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "keyword_int_literal");
}

#[test]
fn keyword_bounded_type() {
	let syntax = to_valuedef! { foo | oblique <angle [-90deg,90deg]>? };
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "keyword_bounded_type");
}

#[test]
fn keyword_int_literal_dimension_literal() {
	let syntax = to_valuedef! { keyword | 1 | 1deg };
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "keyword_int_literal_dimension_literal");
}

#[test]
fn literal_with_derive_parse() {
	let syntax = to_valuedef!(" 0deg | 90deg ");
	let data = to_deriveinput! { #[derive(Parse)] enum Foo {} };
	assert_snapshot!(syntax, data, "literal_with_derive_parse");
}

#[test]
fn combinator_optional_keyword() {
	let syntax = to_valuedef! { foo | <color>? bar };
	let data = to_deriveinput! { #[derive(Visitable)] enum Foo {} };
	assert_snapshot!(syntax, data, "combinator_optional_keyword");
}

#[test]
fn combinator_optional_last_keyword() {
	let syntax = to_valuedef! { foo | bar <color>? };
	let data = to_deriveinput! { #[derive(Visitable)] enum Foo {} };
	assert_snapshot!(syntax, data, "combinator_optional_last_keyword");
}

#[test]
fn combinator_optional2_keyword() {
	let syntax = to_valuedef! { foo | <color>? <color>? bar };
	let data = to_deriveinput! { #[derive(Visitable)] enum Foo {} };
	assert_snapshot!(syntax, data, "combinator_optional2_keyword");
}

#[test]
fn just_optional() {
	let syntax = to_valuedef! { <color>? <color>? };
	let data = to_deriveinput! { struct Foo {} };
	assert_snapshot!(syntax, data, "just_optional");
}

#[test]
fn combinator_optional_all_keywords() {
	let syntax = to_valuedef! { foo || bar || baz };
	let data = to_deriveinput! { #[derive(Visitable)] struct Foo {} };
	assert_snapshot!(syntax, data, "combinator_optional_all_keywords");
}

#[test]
fn combinator_optional_keywords_and_types() {
	let syntax = to_valuedef! { foo || <bar> };
	let data = to_deriveinput! { struct Foo {} };
	assert_snapshot!(syntax, data, "combinator_optional_keywords_and_types");
}

#[test]
fn multiplier_with_just_keywords() {
	let syntax = to_valuedef! { [ outset | inset ]+ };
	let data = to_deriveinput! { struct Foo<'a> {} };
	assert_snapshot!(syntax, data, "multiplier_with_just_keywords");
}

#[test]
fn bounded_multiplier_of_keywords() {
	let syntax = to_valuedef! { [ foo | bar ]{1,2} };
	let data = to_deriveinput! { #[derive(Visitable)] struct Foo<'a> {} };
	assert_snapshot!(syntax, data, "bounded_multiplier_of_keywords");
}

#[test]
fn multiplier_with_comma_separated_keywords() {
	let syntax = to_valuedef! { [ outset | inset ]# };
	let data = to_deriveinput! { struct Foo<'a> {} };
	assert_snapshot!(syntax, data, "multiplier_with_comma_separated_keywords");
}

#[test]
fn multiplier_with_comma_separated_type() {
	let syntax = to_valuedef! { [ foo | <bar> ]# };
	let data = to_deriveinput! { struct Foo<'a> {} };
	assert_snapshot!(syntax, data, "multiplier_with_comma_separated_types");
}

#[test]
fn group_with_optional_leader() {
	let syntax = to_valuedef! { normal | [ <overflow-position>? <self-position> ] };
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "group_with_optional_leader");
}

#[test]
fn none_or_type() {
	let syntax = to_valuedef!( none | <custom-ident> );
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "none_or_type");
}

#[test]
fn auto_or_none() {
	let syntax = to_valuedef!(auto | none);
	let data = to_deriveinput! { #[derive(Visitable)] enum Foo {} };
	assert_snapshot!(syntax, data, "auto_or_none");
}

#[test]
fn auto_or_none_or_type() {
	let syntax = to_valuedef!( auto | none | <length> );
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "auto_or_none_or_type");
}

#[test]
fn auto_or_type_with_checks() {
	let syntax = to_valuedef!( auto | <angle [-90deg,90deg]> );
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "auto_or_type_with_checks");
}

#[test]
fn auto_or_type_with_checks_derive_parse() {
	let syntax = to_valuedef!( auto | <angle [-90deg,90deg]> );
	let data = to_deriveinput! { #[derive(Parse)] struct Foo; };
	assert_snapshot!(syntax, data, "auto_or_type_with_checks_derive_parse");
}

#[test]
fn auto_or_type() {
	let syntax = to_valuedef!( auto | <custom-ident> );
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "auto_or_type");
}

#[test]
fn auto_or_fixed_multiplier() {
	let syntax = to_valuedef! { auto | <color>{2} };
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "auto_or_fixed_multiplier");
}

#[test]
fn bg_image() {
	let syntax = to_valuedef!(" <bg-image> ");
	let data = to_deriveinput! { struct Foo<'a>; };
	assert_snapshot!(syntax, data, "bg_image");
}

#[test]
fn simple_all_must_occur() {
	let syntax = to_valuedef!(" <length> && auto ");
	let data = to_deriveinput! { struct Foo<'a>; };
	assert_snapshot!(syntax, data, "simple_optionals");
}

#[test]
fn auto_and_length_with_range() {
	let syntax = to_valuedef!(" auto && <length [0,100]> ");
	let data = to_deriveinput! { struct Foo<'a>; };
	assert_snapshot!(syntax, data, "auto_and_length_with_range");
}
