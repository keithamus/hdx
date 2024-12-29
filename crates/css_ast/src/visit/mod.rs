include!(concat!(env!("OUT_DIR"), "/css_node_kind.rs"));
include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));

use crate::*;

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

pub trait VisitableMut<'a> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V);
}

pub trait Visitable<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V);
}

impl<'a, T: VisitableMut<'a>> VisitableMut<'a> for bumpalo::collections::Vec<'a, T> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) {
		for node in self {
			node.accept_mut(v)
		}
	}
}

impl<'a, T: Visitable<'a>> Visitable<'a> for bumpalo::collections::Vec<'a, T> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for node in self {
			node.accept(v)
		}
	}
}
