use crate::*;
use quote::quote;

macro_rules! to_valuedef {
	( $lit:literal ) => {
		::syn::parse2::<StrWrapped<Def>>(::quote::quote!{ $lit }).unwrap().0.optimize()
	};
	( $($tt:tt)+ ) => {
		::syn::parse2::<Def>(::quote::quote!{ $($tt)+ }).unwrap().optimize()
	};
}

#[test]
fn test_def_builds_quoted_type() {
	assert_eq!(
		::syn::parse2::<StrWrapped<Def>>(quote! { "<'some-prop'>" }).unwrap().0,
		Def::StyleValue(DefType::new("SomeProp", DefRange::None))
	)
}

#[test]
fn test_def_builds_type_with_multiplier_oneormore() {
	assert_eq!(
		to_valuedef!( <integer>+ ),
		Def::Multiplier(
			Box::new(Def::Type(DefType::new("Integer", DefRange::None))),
			DefMultiplierSeparator::None,
			DefRange::RangeFrom(1.)
		)
	)
}

#[test]
fn def_builds_type_with_checks() {
	assert_eq!(to_valuedef! { <integer [1,3]> }, Def::Type(DefType::new("Integer", DefRange::Range(1f32..3f32))))
}

#[test]
fn test_def_builds_optional() {
	assert_eq!(to_valuedef!( <integer>? ), Def::Optional(Box::new(Def::Type(DefType::new("Integer", DefRange::None)))))
}

#[test]
fn test_def_builds_quoted_custom_type_with_count() {
	assert_eq!(
		::syn::parse2::<StrWrapped<Def>>(quote! { "<'animation-delay'>{1,}" }).unwrap().0,
		Def::Multiplier(
			Box::new(Def::StyleValue(DefType::new("AnimationDelay", DefRange::None))),
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
			vec![Def::Type(DefType::new("Length", DefRange::RangeFrom(1.))), Def::Ident(DefIdent("foo".into()))],
			DefCombinatorStyle::Alternatives,
		)
	)
}

#[test]
fn def_optimizes_length_or_auto_to_lengthorauto_type() {
	assert_eq!(
		to_valuedef! { auto | <length> },
		Def::AutoOr(Box::new(Def::Type(DefType::new("Length", DefRange::None))))
	);
	assert_eq!(
		to_valuedef! { <length [1,]> | auto },
		Def::AutoOr(Box::new(Def::Type(DefType::new("Length", DefRange::RangeFrom(1.)))))
	);
}

#[test]
fn def_optimizes_lengthpercentage_or_flex_to_lengthpercentageorflex_type() {
	assert_eq!(
		to_valuedef! { <flex> | <length-percentage> },
		Def::Type(DefType::new("LengthPercentageOrFlex", DefRange::None))
	);
	assert_eq!(
		to_valuedef! { <length-percentage [1,]> | <flex> },
		Def::Type(DefType::new("LengthPercentageOrFlex", DefRange::RangeFrom(1.)))
	);
}

#[test]
fn def_optimizes_length_or_auto_range_to_ordered_combinator_lengthorauto_type() {
	assert_eq!(
		to_valuedef! { [ auto | <length-percentage> ]{1,4} },
		Def::Combinator(
			vec![
				Def::AutoOr(Box::new(Def::Type(DefType::new("LengthPercentage", DefRange::None)))),
				Def::Optional(Box::new(Def::AutoOr(Box::new(Def::Type(DefType::new(
					"LengthPercentage",
					DefRange::None
				)))))),
				Def::Optional(Box::new(Def::AutoOr(Box::new(Def::Type(DefType::new(
					"LengthPercentage",
					DefRange::None
				)))))),
				Def::Optional(Box::new(Def::AutoOr(Box::new(Def::Type(DefType::new(
					"LengthPercentage",
					DefRange::None
				)))))),
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
			Box::new(Def::Type(DefType::new("Length", DefRange::None))),
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
			Box::new(Def::Type(DefType::new("Length", DefRange::None))),
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
			vec![
				Def::Type(DefType::new("Color", DefRange::None)),
				Def::Punct('/'),
				Def::Type(DefType::new("Color", DefRange::None))
			],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn def_builds_with_quoted_literal_chars() {
	assert_eq!(
		to_valuedef! { <color> '/' <color> },
		Def::Combinator(
			vec![
				Def::Type(DefType::new("Color", DefRange::None)),
				Def::Punct('/'),
				Def::Type(DefType::new("Color", DefRange::None))
			],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn def_builds_with_double_quoted_literal_chars() {
	assert_eq!(
		to_valuedef! { <color> "/" <color> },
		Def::Combinator(
			vec![
				Def::Type(DefType::new("Color", DefRange::None)),
				Def::Punct('/'),
				Def::Type(DefType::new("Color", DefRange::None))
			],
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
			Box::new(Def::Type(DefType::new("Length", DefRange::None))),
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
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
			],
			DefCombinatorStyle::Ordered
		)
	)
}

#[test]
fn def_builds_group_multiplier_of_type_fixed_range_as_ordered_combinator() {
	assert_eq!(
		to_valuedef! { [ <length> ]{5} },
		Def::Combinator(
			vec![
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
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
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Optional(Box::new(Def::Type(DefType::new("Length", DefRange::None)))),
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
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Type(DefType::new("Length", DefRange::None)),
				Def::Optional(Box::new(Def::Type(DefType::new("Length", DefRange::None)))),
				Def::Optional(Box::new(Def::Type(DefType::new("Length", DefRange::None)))),
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
				Def::Optional(Box::new(Def::Type(DefType::new("Length", DefRange::None)))),
				Def::Optional(Box::new(Def::Type(DefType::new("Length", DefRange::None)))),
				Def::Optional(Box::new(Def::Type(DefType::new("Length", DefRange::None)))),
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
			vec![Def::Ident(DefIdent("foo".into())), Def::Type(DefType::new("Length", DefRange::None)),],
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
						Def::Type(DefType::new("Length", DefRange::None)),
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
					Box::new(Def::Type(DefType::new("Length", DefRange::None))),
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
								Box::new(Def::Type(DefType::new("Length", DefRange::None))),
								DefMultiplierSeparator::None,
								DefRange::RangeFrom(2.),
							),
							Def::Optional(Box::new(Def::Type(DefType::new("Color", DefRange::None))))
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
		Def::NoneOr(Box::new(Def::Combinator(
			vec![Def::Type(DefType::new("Foo", DefRange::None)), Def::Type(DefType::new("Bar", DefRange::None)),],
			DefCombinatorStyle::Ordered
		)))
	)
}

#[test]
fn def_returns_true_for_maybe_unsized() {
	assert!(to_valuedef! { <bar># }.maybe_unsized());
	assert!(to_valuedef! { <foo> <bar># }.maybe_unsized());
	assert!(to_valuedef! { <foo> <bar()># }.maybe_unsized());
	assert!(to_valuedef!(" <'bar'># ").maybe_unsized());
	assert!(!to_valuedef!(" <'bar'> ").maybe_unsized());
}
