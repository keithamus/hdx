use crate::{def::*, value::generate};
use hdx_atom::{atom, Atom};
use quote::quote;

macro_rules! to_valuedef {
	( $lit:literal ) => {
		::syn::parse2::<StrWrapped<Def>>(::quote::quote!{ $lit }).unwrap().0
	};
	( $($tt:tt)+ ) => {
		::syn::parse2::<Def>(::quote::quote!{ $($tt)+ }).unwrap()
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
		Def::Type(DefType::Custom(
			DefIdent(Atom::from("SomePropStyleValue")),
			DefIdent(Atom::from("SomePropStyleValue"))
		))
	)
}

#[test]
fn test_def_builds_type_with_multiplier_oneormore() {
	assert_eq!(
		to_valuedef!( <integer>+ ),
		Def::Multiplier(Box::new(Def::Type(DefType::Integer(DefRange::None))), DefMultiplierStyle::OneOrMore)
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
		::syn::parse2::<StrWrapped<Def>>(quote! { "<'animation-delay'>{1,3}" }).unwrap().0,
		Def::Multiplier(
			Box::new(Def::Type(DefType::Custom(
				DefIdent(atom!("AnimationDelayStyleValue")),
				DefIdent(atom!("AnimationDelayStyleValue"))
			))),
			DefMultiplierStyle::Range(DefRange::Range(1.0..3.0))
		)
	)
}

#[test]
fn def_builds_combinator_of_keywords() {
	assert_eq!(
		to_valuedef! { none | auto },
		Def::Combinator(
			vec![Def::Ident(DefIdent(atom!("none"))), Def::Ident(DefIdent(atom!("auto")))],
			DefCombinatorStyle::Alternatives,
		)
	)
}

#[test]
fn def_builds_ordered_combinator_of_keywords() {
	assert_eq!(
		to_valuedef! { none auto },
		Def::Combinator(
			vec![Def::Ident(DefIdent(atom!("none"))), Def::Ident(DefIdent(atom!("auto")))],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn test_def_builds_dashed_idents() {
	assert_eq!(
		to_valuedef!( length-percentage preserve-3d  ),
		Def::Combinator(
			vec![Def::Ident(DefIdent(atom!("length-percentage"))), Def::Ident(DefIdent(atom!("preserve-3d")))],
			DefCombinatorStyle::Ordered,
		)
	)
}

#[test]
fn def_builds_group_with_brackets() {
	assert_eq!(
		to_valuedef! { [ block || inline ] | none },
		Def::Combinator(
			vec![
				Def::Group(
					Box::new(Def::Combinator(
						vec![Def::Ident(DefIdent(atom!("block"))), Def::Ident(DefIdent(atom!("inline")))],
						DefCombinatorStyle::Options,
					)),
					DefGroupStyle::None,
				),
				Def::Ident(DefIdent(atom!("none"))),
			],
			DefCombinatorStyle::Alternatives,
		)
	);
}

#[test]
fn def_builds_combinator_with_correct_precedence() {
	assert_eq!(
		to_valuedef! { none | underline || overline },
		Def::Combinator(
			vec![
				Def::Ident(DefIdent(atom!("none"))),
				Def::Combinator(
					vec![Def::Ident(DefIdent(atom!("underline"))), Def::Ident(DefIdent(atom!("overline")))],
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
		to_valuedef! { underline || overline | none },
		Def::Combinator(
			vec![
				Def::Combinator(
					vec![Def::Ident(DefIdent(atom!("underline"))), Def::Ident(DefIdent(atom!("overline")))],
					DefCombinatorStyle::Options,
				),
				Def::Ident(DefIdent(atom!("none"))),
			],
			DefCombinatorStyle::Alternatives,
		)
	);
}

#[test]
fn def_builds_combinator_with_correct_precedence3() {
	assert_eq!(
		to_valuedef! { auto none | underline || overline && block inline },
		Def::Combinator(
			vec![
				Def::Combinator(
					vec![Def::Ident(DefIdent(atom!("auto"))), Def::Ident(DefIdent(atom!("none")))],
					DefCombinatorStyle::Ordered,
				),
				Def::Combinator(
					vec![
						Def::Ident(DefIdent(atom!("underline"))),
						Def::Combinator(
							vec![
								Def::Ident(DefIdent(atom!("overline"))),
								Def::Combinator(
									vec![Def::Ident(DefIdent(atom!("block"))), Def::Ident(DefIdent(atom!("inline")))],
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
		to_valuedef! { <length [1,]> | auto },
		Def::Combinator(
			vec![Def::Type(DefType::Length(DefRange::RangeFrom(1f32..))), Def::Ident(DefIdent(atom!("auto")))],
			DefCombinatorStyle::Alternatives,
		)
	)
}

#[test]
fn def_builds_multiplier_of_types() {
	assert_eq!(
		to_valuedef! { <length># },
		Def::Multiplier(
			Box::new(Def::Type(DefType::Length(DefRange::None))),
			DefMultiplierStyle::OneOrMoreCommaSeparated(DefRange::None)
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
			DefMultiplierStyle::OneOrMoreCommaSeparated(DefRange::Range(range))
		)
	)
}

#[test]
fn def_builds_multiplier_of_type_fixed_range() {
	assert_eq!(
		to_valuedef! { <length>{5} },
		Def::Multiplier(
			Box::new(Def::Type(DefType::Length(DefRange::None))),
			DefMultiplierStyle::Range(DefRange::Fixed(5f32))
		)
	)
}

#[test]
fn def_builds_complex_combination_1() {
	assert_eq!(
		to_valuedef! { [ inset? && <length>{2,4} && <color>? ]# | none },
		Def::Combinator(
			vec![
				Def::Group(
					Box::new(Def::Combinator(
						vec![
							Def::Optional(Box::new(Def::Ident(DefIdent(atom!("inset"))))),
							Def::Multiplier(
								Box::new(Def::Type(DefType::Length(DefRange::None))),
								DefMultiplierStyle::Range(DefRange::Range(2f32..4f32)),
							),
							Def::Optional(Box::new(Def::Type(DefType::Color))),
						],
						DefCombinatorStyle::AllMustOccur,
					)),
					DefGroupStyle::OneOrMore,
				),
				Def::Ident(DefIdent(atom!("none"))),
			],
			DefCombinatorStyle::Alternatives,
		)
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
fn keyword_or_type() {
	let syntax = to_valuedef!( none | <custom-ident> );
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "keyword_or_type");
}

#[test]
fn custom_type_with_checks() {
	let syntax = to_valuedef!(" none | <length-percentage [0,∞]> ");
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "custom_type_with_checks");
}

#[test]
fn custom_function_type() {
	let syntax = to_valuedef!(" none | <calc-size()> ");
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "custom_function_type");
}

#[test]
fn custom_function_variant_with_args() {
	let syntax = to_valuedef!(" fit-content | fit-content(<length-percentage [0,∞]>) ");
	let data = to_deriveinput! { enum Foo {} };
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
fn enum_with_variable_count_type() {
	let syntax = to_valuedef!(" auto | <animateable-feature># ");
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
fn value_fixed_range_color2_optimized() {
	let syntax = to_valuedef! { <color>{2} };
	let data = to_deriveinput! { struct Foo {} };
	assert_snapshot!(syntax, data, "value_fixed_range_color2_optimized");
}

#[test]
fn value_fixed_range_auto_color2_optimized() {
	let syntax = to_valuedef! { auto | <color>{2} };
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "value_fixed_range_auto_color2_optimized");
}
