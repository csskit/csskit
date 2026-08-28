use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Shorthand {
	pub(crate) name: String,
	pub(crate) longhands: Vec<String>,
	#[serde(default)]
	pub(crate) resets: Vec<String>,
	#[serde(default)]
	pub(crate) resets_all: bool,
}

#[derive(Debug, Deserialize)]
struct Shorthands {
	shorthand: Vec<Shorthand>,
}

#[derive(Clone, Debug)]
pub(crate) struct ShorthandMetadata {
	pub(crate) longhands: Vec<String>,
	pub(crate) resets: Vec<String>,
	pub(crate) resets_all: bool,
}

pub(crate) struct ShorthandRelationships {
	pub(crate) reset_by: Vec<String>,
}
/// Shorthand dependency graph parsed from the generator registry.
pub(crate) struct ShorthandGraph {
	shorthands: BTreeMap<String, Shorthand>,
}

impl ShorthandGraph {
	fn new(shorthands: Vec<Shorthand>) -> Self {
		let mut by_name = BTreeMap::new();
		for shorthand in shorthands {
			let name = shorthand.name.clone();
			assert!(by_name.insert(name.clone(), shorthand).is_none(), "duplicate shorthand {name}");
		}
		let graph = Self { shorthands: by_name };
		graph.validate();
		graph
	}

	pub(crate) fn shorthand(&self, name: &str) -> Option<ShorthandMetadata> {
		let shorthand = self.shorthands.get(name)?;
		Some(ShorthandMetadata {
			longhands: self.expanded_longhands(&shorthand.longhands),
			resets: shorthand.resets.clone(),
			resets_all: shorthand.resets_all,
		})
	}

	pub(crate) fn shorthand_group(&self, property: &str) -> Option<String> {
		self.shorthands
			.iter()
			.filter(|(_, shorthand)| self.contains_descendant(&shorthand.longhands, property))
			.map(|(name, _)| name)
			.min_by_key(|name| (name.len(), *name))
			.cloned()
	}

	pub(crate) fn relationships(&self, property: &str) -> ShorthandRelationships {
		let mut reset_by = Vec::new();
		for (name, shorthand) in &self.shorthands {
			if shorthand
				.resets
				.iter()
				.any(|reset| reset == property || self.contains_descendant(std::slice::from_ref(reset), property))
			{
				reset_by.push(name.clone());
			}
		}
		ShorthandRelationships { reset_by }
	}
	fn expanded_longhands(&self, properties: &[String]) -> Vec<String> {
		let mut longhands = BTreeSet::new();
		for property in properties {
			self.collect_expanded_longhands(property, &mut BTreeSet::new(), &mut longhands);
		}
		longhands.into_iter().collect()
	}

	fn collect_expanded_longhands(
		&self,
		property: &str,
		visiting: &mut BTreeSet<String>,
		longhands: &mut BTreeSet<String>,
	) {
		assert!(visiting.insert(property.to_string()), "shorthand cycle at {property}");
		longhands.insert(property.to_string());
		if let Some(shorthand) = self.shorthands.get(property) {
			for longhand in &shorthand.longhands {
				self.collect_expanded_longhands(longhand, visiting, longhands);
			}
		}
		visiting.remove(property);
	}

	fn terminal_longhands(&self, properties: &[String]) -> Vec<String> {
		let mut longhands = BTreeSet::new();
		for property in properties {
			self.collect_terminal_longhands(property, &mut BTreeSet::new(), &mut longhands);
		}
		longhands.into_iter().collect()
	}

	fn collect_terminal_longhands(
		&self,
		property: &str,
		visiting: &mut BTreeSet<String>,
		longhands: &mut BTreeSet<String>,
	) {
		let Some(shorthand) = self.shorthands.get(property).filter(|shorthand| !shorthand.longhands.is_empty()) else {
			longhands.insert(property.to_string());
			return;
		};
		assert!(visiting.insert(property.to_string()), "shorthand cycle at {property}");
		for component in &shorthand.longhands {
			self.collect_terminal_longhands(component, visiting, longhands);
		}
		visiting.remove(property);
	}

	fn contains_descendant(&self, properties: &[String], target: &str) -> bool {
		properties.iter().any(|property| {
			property == target
				|| self
					.shorthands
					.get(property)
					.is_some_and(|shorthand| self.contains_descendant(&shorthand.longhands, target))
		})
	}

	fn validate(&self) {
		for (name, shorthand) in &self.shorthands {
			assert!(!(shorthand.resets_all && !shorthand.resets.is_empty()), "{name} cannot set resets and resets_all");
			self.collect_terminal_longhands(name, &mut BTreeSet::new(), &mut BTreeSet::new());
			let expressible = self.terminal_longhands(&shorthand.longhands).into_iter().collect::<BTreeSet<_>>();
			let reset = self.terminal_longhands(&shorthand.resets).into_iter().collect::<BTreeSet<_>>();
			assert!(expressible.is_disjoint(&reset), "{name} resets an expressible longhand");
		}
	}
}

/// Parses the shorthand registry and computes its dependency graph.
pub(crate) fn get_shorthand_graph() -> ShorthandGraph {
	let parsed: Shorthands = toml::from_str(include_str!("../shorthands.toml")).expect("invalid shorthands.toml");
	ShorthandGraph::new(parsed.shorthand)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn registry_contains_expected_shorthands() {
		let graph = get_shorthand_graph();
		assert_eq!(graph.shorthands.len(), 80);
	}

	#[test]
	fn preserves_component_order() {
		let graph = get_shorthand_graph();
		assert_eq!(graph.shorthands["border"].longhands, ["border-width", "border-style", "border-color"]);
	}

	#[test]
	fn expands_transitive_longhands() {
		let graph = get_shorthand_graph();
		let border = graph.shorthand("border").unwrap();
		assert!(border.longhands.contains(&"border-top-width".to_string()));
		assert!(border.longhands.contains(&"border-width".to_string()));
	}

	#[test]
	fn selects_existing_root_shorthand_group() {
		assert_eq!(get_shorthand_graph().shorthand_group("border-left-color").as_deref(), Some("border"));
	}

	#[test]
	fn expands_reset_relationships() {
		let relationships = get_shorthand_graph().relationships("border-image-source");
		assert_eq!(relationships.reset_by, ["border"]);
	}

	#[test]
	fn distinguishes_empty_and_universal_resets() {
		let graph = get_shorthand_graph();
		assert!(graph.shorthand("grid").unwrap().resets.is_empty());
		assert!(graph.shorthand("margin").unwrap().resets.is_empty());
		assert_eq!(graph.shorthand("border").unwrap().resets, ["border-image"]);
		assert!(graph.shorthand("all").unwrap().resets_all);
	}

	#[test]
	#[should_panic(expected = "shorthand cycle")]
	fn rejects_component_cycles() {
		ShorthandGraph::new(vec![
			Shorthand { name: "first".into(), longhands: vec!["second".into()], resets: vec![], resets_all: false },
			Shorthand { name: "second".into(), longhands: vec!["first".into()], resets: vec![], resets_all: false },
		]);
	}
}
