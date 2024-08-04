pub trait StyleValue: PartialEq + Default + Sized + Clone {
	fn initial() -> Self {
		Self::default()
	}

	fn inherits() -> bool {
		false
	}
}
