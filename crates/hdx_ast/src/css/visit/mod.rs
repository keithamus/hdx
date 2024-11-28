use crate::{
	css::{properties::Property, stylerule::StyleRule, StyleSheet},
	syntax::{AtRule, QualifiedRule},
};

macro_rules! apply_visit_methods {
	($macro: ident) => {
		$macro! {
			visit_style_sheet(StyleSheet<'a>),
			visit_style_rule(StyleRule<'a>),
			visit_unknown_at_rule(AtRule<'a>),
			visit_unknown_rule(QualifiedRule<'a>),
			visit_property(Property<'a>),
		}
	};
}

macro_rules! visit_mut_trait {
	( $(
		$name: ident($obj: ty),
	)+ ) => {
		pub trait VisitMut<'a>: Sized + Default {
			$(
				fn $name(&mut self, _rule: &mut $obj) {}
			)+
		}
	}
}
apply_visit_methods!(visit_mut_trait);

macro_rules! visit_trait {
	( $(
		$name: ident($obj: ty),
	)+ ) => {
		pub trait Visit<'a>: Sized + Default {
			$(
				fn $name(&mut self, _rule: &$obj) {}
			)+
		}
	}
}
apply_visit_methods!(visit_trait);

pub trait VisitableMut<'a>: Sized {
	fn accept_mut<V: VisitMut<'a>>(&mut self, visitor: &mut V);
}

pub trait Visitable<'a>: Sized {
	fn accept<V: Visit<'a>>(&self, visitor: &mut V);
}

impl<'a, T: VisitableMut<'a>> VisitableMut<'a> for bumpalo::collections::Vec<'a, T> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, visitor: &mut V) {
		for node in self {
			node.accept_mut(visitor)
		}
	}
}

impl<'a, T: Visitable<'a>> Visitable<'a> for bumpalo::collections::Vec<'a, T> {
	fn accept<V: Visit<'a>>(&self, visitor: &mut V) {
		for node in self {
			node.accept(visitor)
		}
	}
}
