macro_rules! keyword_typedef {
	($name: ident { $( $variant: ident: atom!($variant_atom: tt)),+ $(,)* }) => {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
		pub enum $name {
			$($variant),+
		}

		impl hdx_atom::Atomizable for $name {
			fn from_atom(atom: &hdx_atom::Atom) -> Option<Self> {
				match atom.to_ascii_lowercase() {
					$(hdx_atom::atom!($variant_atom) => Some(Self::$variant),)+
					_ => None,
				}
			}

			fn to_atom(&self) -> hdx_atom::Atom {
				match self {
					$(Self::$variant => hdx_atom::atom!($variant_atom)),+
				}
			}
		}

		impl<'a> hdx_parser::Peek<'a> for $name {
			fn peek(p: &hdx_parser::Parser<'a>) -> Option<hdx_lexer::Token> {
				p.peek::<hdx_parser::T![Ident]>().filter(|token| {
					let atom = p.parse_atom_lower(*token);
					matches!(atom, $(hdx_atom::atom!($variant_atom))|+)
				})
			}
		}

		impl<'a> hdx_parser::Parse<'a> for $name {
			fn parse(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				use hdx_atom::Atomizable;
				let token = *p.parse::<hdx_parser::T![Ident]>()?;
				let atom = p.parse_atom_lower(token);
				Self::from_atom(&atom).ok_or_else(|| {
					hdx_parser::diagnostics::UnexpectedIdent(atom, token.span()).into()
				})
			}
		}

		impl<'a> hdx_writer::WriteCss<'a> for $name {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				use hdx_atom::Atomizable;
				self.to_atom().write_css(sink)
			}
		}
	}
}

pub(crate) use keyword_typedef;

macro_rules! discrete_media_feature {
	($feat: tt[atom!($atom: tt)] { $( $name: ident: atom!($name_atom: tt),)+ }) => {
		#[derive(PartialEq, Default, Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum $feat {
			#[default]
			Any,
			$( $name, )+
		}

		impl<'a> hdx_parser::Parse<'a> for $feat {
			fn parse(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				use hdx_parser::DiscreteMediaFeature;
				Self::parse_descrete_media_feature(hdx_atom::atom!($atom), p)
			}
		}

		impl<'a> hdx_parser::DiscreteMediaFeature<'a> for $feat {
			fn parse_media_feature_value(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				let token = *p.parse::<hdx_parser::T![Ident]>()?;
				match p.parse_atom_lower(token) {
					$(
						hdx_atom::atom!($name_atom) => Ok(Self::$name),
					)+
					atom => Err(::hdx_parser::diagnostics::UnexpectedIdent(atom, token.span()))?
				}
			}
		}

		impl<'a> hdx_writer::WriteCss<'a> for $feat {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				hdx_atom::atom!($atom).write_css(sink)?;
				match self {
				$(
					Self::$name => {
						sink.write_char(':')?;
						sink.write_whitespace()?;
						hdx_atom::atom!($name_atom).write_css(sink)
					}
				)+
					Self::Any => Ok(())
				}
			}
		}

	};
}

pub(crate) use discrete_media_feature;

macro_rules! bool_media_feature {
	($feat: tt[atom!($atom: tt)]) => {
		#[derive(PartialEq, Default, Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum $feat {
			#[default]
			Zero,
			One,
		}

		impl<'a> hdx_parser::Parse<'a> for $feat {
			fn parse(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				use hdx_parser::DiscreteMediaFeature;
				Self::parse_descrete_media_feature(hdx_atom::atom!($atom), p)
			}
		}

		impl<'a> hdx_parser::DiscreteMediaFeature<'a> for $feat {
			fn parse_media_feature_value(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				let token = *p.parse::<hdx_parser::T![Number]>()?;
				if token.is_int() {
					let val = p.parse_number(token);
					if val == 1.0 {
						return Ok(Self::One);
					} else if val == 0.0 {
						return Ok(Self::Zero);
					}
				}
				Err(::hdx_parser::diagnostics::Unexpected(token, token.span()))?
			}
		}

		impl<'a> hdx_writer::WriteCss<'a> for $feat {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				if matches!(self, Self::Zero)
					&& !sink.can_output(hdx_writer::OutputOption::RedundantBooleanMediaFeatures)
				{
					return hdx_atom::atom!($atom).write_css(sink);
				}
				hdx_writer::write_css!(sink, hdx_atom::atom!($atom), ':', ());
				match self {
					Self::One => sink.write_char('1'),
					Self::Zero => sink.write_char('0'),
				}
			}
		}
	};
}

pub(crate) use bool_media_feature;

macro_rules! ranged_media_feature {
	($feat: tt[atom!($atom: tt)], $ty: ty) => {
		#[derive(PartialEq, Debug, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
		pub enum $feat {
			Legacy((hdx_parser::Comparison, $ty)),
			Single((hdx_parser::Comparison, $ty)),
			Double((hdx_parser::Comparison, $ty, hdx_parser::Comparison, $ty)),
		}

		impl<'a> hdx_parser::Parse<'a> for $feat {
			fn parse(p: &mut hdx_parser::Parser<'a>) -> hdx_parser::Result<Self> {
				use hdx_parser::RangedMediaFeature;
				Self::parse_ranged_media_feature(hdx_atom::atom!($atom), p)
			}
		}

		impl<'a> hdx_parser::RangedMediaFeature<'a> for $feat {
			type Type = $ty;

			fn new(
				left: (hdx_parser::Comparison, Self::Type),
				right: Option<(hdx_parser::Comparison, Self::Type)>,
				legacy: bool,
			) -> Self {
				if legacy {
					Self::Legacy(left)
				} else if let Some(right) = right {
					Self::Double((left.0, left.1, right.0, right.1))
				} else {
					Self::Single(left)
				}
			}
		}

		impl<'a> hdx_writer::WriteCss<'a> for $feat {
			fn write_css<W: hdx_writer::CssWriter>(&self, sink: &mut W) -> hdx_writer::Result {
				use hdx_atom::atom;
				use hdx_parser::Comparison::*;
				match self {
					Self::Legacy((Equal, u)) => hdx_writer::write_css!(sink, atom!($atom), ':', (), u),
					Self::Legacy((LessThanEqual, u)) => {
						hdx_writer::write_css!(sink, atom!("min-"), atom!($atom), ':', (), u)
					}
					Self::Legacy((GreaterThanEqual, u)) => {
						hdx_writer::write_css!(sink, atom!("max-"), atom!($atom), ':', (), u)
					}
					Self::Legacy(_) => debug_assert!(false, "Legacy media feature syntax does not support gt, or lt"),
					Self::Single((Equal, u)) => hdx_writer::write_css!(sink, atom!($atom), (), '=', (), u),
					Self::Single((LessThan, u)) => hdx_writer::write_css!(sink, atom!($atom), (), '<', (), u),
					Self::Single((LessThanEqual, u)) => hdx_writer::write_css!(sink, atom!($atom), (), '<', '=', (), u),
					Self::Single((GreaterThan, u)) => hdx_writer::write_css!(sink, atom!($atom), (), '>', (), u),
					Self::Single((GreaterThanEqual, u)) => {
						hdx_writer::write_css!(sink, atom!($atom), (), '>', '=', (), u)
					}
					Self::Double((left_cmp, left, right_cmp, right)) => {
						hdx_writer::write_css!(sink, left, ());
						match left_cmp {
							Equal => hdx_writer::write_css!(sink, '='),
							LessThan => hdx_writer::write_css!(sink, '<'),
							LessThanEqual => hdx_writer::write_css!(sink, '<', '='),
							GreaterThan => hdx_writer::write_css!(sink, '>'),
							GreaterThanEqual => hdx_writer::write_css!(sink, '>', '='),
						}
						hdx_writer::write_css!(sink, (), atom!($atom), ());
						match right_cmp {
							Equal => hdx_writer::write_css!(sink, '='),
							LessThan => hdx_writer::write_css!(sink, '<'),
							LessThanEqual => hdx_writer::write_css!(sink, '<', '='),
							GreaterThan => hdx_writer::write_css!(sink, '>'),
							GreaterThanEqual => hdx_writer::write_css!(sink, '>', '='),
						}
						hdx_writer::write_css!(sink, (), right);
					}
				}
				Ok(())
			}
		}
	};
}

pub(crate) use ranged_media_feature;
