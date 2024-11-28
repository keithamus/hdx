pub trait StyleValue: PartialEq + Sized + Clone {
	// fn initial() -> Self {
	// 	Self::default()
	// }

	fn inherits() -> bool {
		false
	}
}
