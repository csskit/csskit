use std::collections::HashMap;

/// Returns a map of spec names to property value replacements
///
/// Each entry maps a property name to a string that will replace its grammar.
pub fn get_value_replacements() -> HashMap<String, HashMap<String, String>> {
	let toml_str = include_str!("../value_replacements.toml");
	let parsed: HashMap<String, HashMap<String, String>> =
		toml::from_str(toml_str).expect("Failed to parse value_replacements.toml");

	parsed
}
