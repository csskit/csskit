use std::collections::{HashMap, HashSet};

/// Properties that require manual Parse implementations instead of generated ones.
///
/// Some properties have complex parsing rules that make them difficult or impractical
/// to express using the derive(Parse) macro. For these properties, we exclude Parse
/// from the generated derive list, allowing developers to provide custom implementations.
///
/// Examples include:
/// - `glyph-orientation-vertical`: Requires special handling for literal integers and dimensions
pub fn get_manual_parse_properties() -> HashMap<&'static str, HashSet<&'static str>> {
	let mut map = HashMap::new();

	// Properties with hand-written Parse implementations
	map.insert("writing-modes", HashSet::from(["glyph-orientation-vertical"]));

	map
}
