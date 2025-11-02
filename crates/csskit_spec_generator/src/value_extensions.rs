use std::collections::HashMap;

/// Returns a map of spec names to property value extensions
///
/// Each entry maps a property name to a string that will be appended to its grammar.
/// For example, the sizing spec adds " | stretch | fit-content | contain" to width.
pub fn get_value_extensions() -> HashMap<String, HashMap<String, String>> {
	let toml_str = include_str!("../value_extensions.toml");
	let parsed: HashMap<String, HashMap<String, String>> =
		toml::from_str(toml_str).expect("Failed to parse value_extensions.toml");

	parsed
}
