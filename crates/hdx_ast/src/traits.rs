pub trait Value: PartialEq + Default + Sized {
	fn initial() -> Self {
		Self::default()
	}

	fn inherits() -> bool {
		false
	}
}
