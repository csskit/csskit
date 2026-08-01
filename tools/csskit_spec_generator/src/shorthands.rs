use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Shorthand {
	name: String,
	longhands: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Shorthands {
	shorthand: Vec<Shorthand>,
}

/// Properties that are known shorthands, that expand into various longhand values.
#[allow(dead_code)]
pub fn get_shorthand_properties() -> HashMap<String, HashSet<String>> {
	let toml_str = include_str!("../shorthands.toml");
	let parsed: Shorthands = toml::from_str(toml_str).expect("Failed to parse shorthands.toml");

	parsed.shorthand.into_iter().map(|sh| (sh.name, sh.longhands.into_iter().collect())).collect()
}
