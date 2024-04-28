pub trait Value: PartialEq + Default + Sized + Clone {
	fn initial() -> Self {
		Self::default()
	}

	fn inherits() -> bool {
		false
	}
}
