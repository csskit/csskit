use crate::{ReduceColors, ReduceLengths, transformer};
use bitmask_enum::bitmask;
use css_ast::{CssMetadata, Visitable};

transformer!(
	/// Runtime feature flags for the CSS minifier, enabling individual transforms.
	pub enum CssMinifierFeature[CssMetadata, Visitable] {
		/// Enables the [ReduceColors] transformer.
		ReduceColors,
		/// Enables the [ReduceLengths] transformer.
		ReduceLengths,
	}
);

impl Default for CssMinifierFeature {
	fn default() -> Self {
		Self::none()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Transformer;
	use bumpalo::Bump;
	use css_ast::{CssAtomSet, StyleSheet};
	use css_lexer::Lexer;
	use css_parse::{CursorCompactWriteSink, CursorOverlaySink, Parser, ToCursors};

	fn minify(source_text: &str, features: CssMinifierFeature) -> (String, bool) {
		let bump = Bump::default();
		let mut transformer = Transformer::new_in(&bump, features, &CssAtomSet::ATOMS, source_text);
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&bump, source_text, lexer);
		let mut result = parser.parse_entirely::<StyleSheet>();
		let mut output = String::new();
		if let Some(ref mut node) = result.output {
			transformer.transform(node);
			let overlays = transformer.overlays();
			let changed = transformer.has_changed();
			let mut overlay_stream =
				CursorOverlaySink::new(source_text, &*overlays, CursorCompactWriteSink::new(source_text, &mut output));
			result.output.to_cursors(&mut overlay_stream);
			(output.clone(), changed)
		} else {
			panic!("Could not transform output");
		}
	}

	#[test]
	fn test_reduce_lengths_feature() {
		let input = "body { width: 0px; }";
		let (output, changed) = minify(input, CssMinifierFeature::ReduceLengths);
		assert!(changed);
		assert!(output.contains("width:0"), "Should apply length reduction, got: {}", output);
		assert!(!output.contains("0px"), "Should not contain 0px, got: {}", output);
	}

	#[test]
	fn test_no_features() {
		let input = "body { width: 0px; }";
		let (output, changed) = minify(input, CssMinifierFeature::none());
		assert!(!changed, "Should not make changes with no features enabled");
		assert!(output.contains("width:0px"));
	}

	#[test]
	fn test_changed_flag_accuracy() {
		let input = "body { width: 10px; }";
		let (_, changed) = minify(input, CssMinifierFeature::all_bits());
		assert!(!changed, "Should report no changes when no optimizations apply");
	}
}
