use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SpecProperties {
	properties: Vec<String>,
}

/// Properties to ignore from certain specs because they've moved around or are defined
/// in multiple specifications. These properties should not be generated from these specs.
pub fn get_ignore_properties() -> HashMap<String, HashSet<String>> {
	let toml_str = include_str!("../ignore_properties.toml");
	let parsed: HashMap<String, SpecProperties> =
		toml::from_str(toml_str).expect("Failed to parse ignore_properties.toml");

	parsed.into_iter().map(|(spec, spec_props)| (spec, spec_props.properties.into_iter().collect())).collect()
}
