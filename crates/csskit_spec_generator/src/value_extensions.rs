use std::collections::HashMap;

/// Returns a map of spec names to property value extensions
///
/// Each entry maps a property name to a string that will be appended to its grammar.
/// For example, the sizing spec adds " | stretch | fit-content | contain" to width.
pub fn get_value_extensions() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
	let mut extensions = HashMap::new();

	// https://drafts.csswg.org/css-sizing-4/#sizing-values
	let mut sizing_extensions = HashMap::new();
	sizing_extensions.insert("width", " | stretch | fit-content | contain");
	sizing_extensions.insert("max-width", " | stretch | fit-content | contain");
	sizing_extensions.insert("min-width", " | stretch | fit-content | contain");
	sizing_extensions.insert("height", " | stretch | fit-content | contain");
	sizing_extensions.insert("max-height", " | stretch | fit-content | contain");
	sizing_extensions.insert("min-height", " | stretch | fit-content | contain");
	extensions.insert("sizing", sizing_extensions);

	extensions
}
