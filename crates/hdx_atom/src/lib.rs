extern crate string_cache;

include!(concat!(env!("OUT_DIR"), "/hdx_atom.rs"));

pub trait Atomizable: Sized {
	fn from_atom(atom: Atom) -> Option<Self>;
	fn to_atom(&self) -> Atom;
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn smoke_test() {
		assert_eq!(atom!("color"), Atom::from("color"));
		assert_eq!(::std::mem::size_of::<Atom>(), 8);
	}
}
