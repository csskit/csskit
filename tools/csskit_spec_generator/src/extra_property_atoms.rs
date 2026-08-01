use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExtraPropertyAtoms {
	properties: Vec<String>,
}

/// CSS properties not covered by any W3C spec module the generator fetches.
pub fn get_extra_property_atoms() -> Vec<String> {
	let toml_str = include_str!("../extra_property_atoms.toml");
	let parsed: ExtraPropertyAtoms = toml::from_str(toml_str).expect("Failed to parse extra_property_atoms.toml");
	parsed.properties
}
