pub trait ToNumberValue {
	fn to_number_value(&self) -> Option<f32>;

	fn to_int_value(&self) -> Option<i32> {
		self.to_number_value().map(|f| f as i32)
	}
}

impl<T: ToNumberValue> ToNumberValue for Option<T> {
	fn to_number_value(&self) -> Option<f32> {
		self.as_ref().and_then(|t| t.to_number_value())
	}
}
