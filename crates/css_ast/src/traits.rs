pub trait StyleValue: PartialEq + Sized + Clone {
	// fn initial() -> Self {
	// 	Self::default()
	// }

	#[allow(dead_code)]
	fn inherits() -> bool {
		false
	}
}
