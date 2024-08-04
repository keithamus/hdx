use crate::{def::*, value::generate};
use hdx_atom::atom;
use quote::quote;

macro_rules! to_valuedef {
	( $($tt:tt)+ ) => {
		::syn::parse2::<Def>(::quote::quote!{ $($tt)+ }).unwrap()
	}
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
		::syn::parse2::<StrWrapped<Def>>(quote! { "<'integer'>" }).unwrap().0,
		Def::Type(DefType::Integer(DefRange::None))
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
			Box::new(Def::Type(DefType::Custom(DefIdent(atom!("AnimationDelay")), FunctionNotation::No))),
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
fn from_syntax_lone_type() {
	let syntax = to_valuedef! { <integer> };
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "from_syntax_lone_type");
}

#[test]
fn from_syntax_group_type_keyword() {
	let syntax = to_valuedef!( <length [1,]> | line-through );
	let data = to_deriveinput! { enum Foo {} };
	assert_snapshot!(syntax, data, "from_syntax_group_type_keyword");
}

#[test]
fn from_syntax_with_multiplier_range() {
	let syntax = to_valuedef!( <length>{2,4});
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "from_syntax_with_multiplier_range");
}

#[test]
fn from_syntax_with_punctuated_optional_group() {
	let syntax = to_valuedef!( <color> [/ <color>]?);
	let data = to_deriveinput! { struct Foo; };
	assert_snapshot!(syntax, data, "from_syntax_with_punctuated_optional_group");
}
