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

/// Returns the canonical (unit-normalised) numeric value for range validation.
///
/// Unlike `ToNumberValue` which returns the raw token value, this returns the value in a unit suitable for range
/// comparison. For example, `Angle` returns degrees regardless of whether the token was `deg`, `rad`, `grad`, or
/// `turn`.
///
/// For plain numeric types (integers, numbers, lengths, percentages) the value is identical to the raw token value.
pub trait ToNormalisedValue {
	fn to_normalised_value(&self) -> Option<f32>;
}

impl<T: ToNormalisedValue> ToNormalisedValue for Option<T> {
	fn to_normalised_value(&self) -> Option<f32> {
		self.as_ref().and_then(|t| t.to_normalised_value())
	}
}
