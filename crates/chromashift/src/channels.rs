/// Trait for extracting the alpha channel of a color.
pub trait ToAlpha: Sized {
	/// Returns a number between 0.0 (fully transparent) to 100.0 (fully opaque).
	fn to_alpha(&self) -> f32;

	/// Returns true if the alpha of this colour is 100.0
	fn fully_opaque(&self) -> bool {
		self.to_alpha() == 100.0
	}

	/// Returns true if the alpha of this colour is 0.0
	fn fully_transparent(&self) -> bool {
		self.to_alpha() == 0.0
	}
}
