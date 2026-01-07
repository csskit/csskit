use std::collections::HashMap;
use std::sync::LazyLock;
pub static TYPE_RENAMES: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
	let toml_str = include_str!("../type_renames.toml");
	toml::from_str(toml_str).expect("Failed to parse type_renames.toml")
});

/// Get the renamed type for a given original type name
pub fn get_type_rename(original_name: &str) -> Option<&str> {
	TYPE_RENAMES.get(original_name).map(|s| s.as_str())
}
