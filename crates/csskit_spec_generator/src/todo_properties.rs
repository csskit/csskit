use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SpecProperties {
	properties: Vec<String>,
}

/// Properties that should be commented out because their parsing rules are tricky.
/// Once we figure out how to parse them in csskit_proc_macro/src/def.rs, we can
/// remove them from the TOML file and the generator will uncomment them.
pub fn get_todo_properties() -> HashMap<String, HashSet<String>> {
	let toml_str = include_str!("../todo_properties.toml");
	let parsed: HashMap<String, SpecProperties> =
		toml::from_str(toml_str).expect("Failed to parse todo_properties.toml");

	parsed.into_iter().map(|(spec, spec_props)| (spec, spec_props.properties.into_iter().collect())).collect()
}
