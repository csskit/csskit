use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SpecProperties {
	properties: Vec<String>,
}

/// Properties that require manual Parse implementations instead of generated ones.
///
/// Some properties have complex parsing rules that make them difficult or impractical
/// to express using the derive(Parse) macro. For these properties, we exclude Parse
/// from the generated derive list, allowing developers to provide custom implementations.
pub fn get_manual_parse_properties() -> HashMap<String, HashSet<String>> {
	let toml_str = include_str!("../manual_parse_properties.toml");
	let parsed: HashMap<String, SpecProperties> =
		toml::from_str(toml_str).expect("Failed to parse manual_parse_properties.toml");

	parsed.into_iter().map(|(spec, spec_props)| (spec, spec_props.properties.into_iter().collect())).collect()
}
