use heck::ToPascalCase;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Shorthand {
	pub(crate) name: String,
	pub(crate) longhands: Vec<String>,
	#[serde(default)]
	pub(crate) resets: Vec<String>,
	#[serde(default)]
	pub(crate) resets_all: bool,
	/// How this shorthand's value writes the longhands it sets, when the registry states it.
	#[serde(default)]
	pub(crate) writes: Option<WritesDef>,
}

/// The `writes` of a registry entry: a keyword, or the slots stated one by one.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum WritesDef {
	Keyword(WritesKeyword),
	Slots(Vec<SlotDef>),
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum WritesKeyword {
	/// One optional value per longhand, in `longhands` order, as `border`'s
	/// `<'border-width'> || <'border-style'> || <'border-color'>` writes.
	Any,
	/// One to as many values as the shorthand sets longhands, the position of each picking the
	/// longhand in `longhands` order, as `margin`'s `<'margin-top'>{1,4}` writes.
	Repeat,
	/// The one value the grammar takes sets every longhand, as `marker`'s `none | <url>` does.
	Same,
}

/// One value of a shorthand's grammar, and the longhand property that value sets.
// minimal: flat slot list; a grammar this cannot state goes writes-less, like `background`
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SlotDef {
	pub(crate) property: String,
	/// The token the grammar writes before this slot, such as the `/` of `font`'s line height.
	#[serde(default)]
	pub(crate) before: String,
	/// The token the grammar writes after this slot.
	#[serde(default)]
	pub(crate) after: String,
	/// True when the grammar lets the slot be left out, which sets its property to the initial
	/// value.
	#[serde(default)]
	pub(crate) optional: bool,
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

/// How a shorthand's value writes the longhands it sets.
#[derive(Clone, Debug)]
pub(crate) enum Writes {
	/// One value per slot, in the order the grammar writes them.
	Slots(Vec<SlotDef>),
	/// The value takes one to as many values as the shorthand sets longhands, and the position of
	/// each value picks the longhand it sets, as `margin`'s `<'margin-top'>{1,4}` does.
	Repeat,
	/// The value is written once and every longhand takes it.
	Same,
}

/// The atom of a property name, which keeps the leading `-` of a vendor prefix as a `_`.
pub(crate) fn atom_ident(property: &str) -> String {
	let pascal = property.to_pascal_case();
	if property.starts_with('-') { format!("_{pascal}") } else { pascal }
}

/// Shorthand dependency graph parsed from the generator registry.
pub(crate) struct ShorthandGraph {
	shorthands: BTreeMap<String, Shorthand>,
	/// The properties each named property sets which set nothing themselves.
	terminals: HashMap<String, BTreeSet<String>>,
}

impl ShorthandGraph {
	fn new(shorthands: Vec<Shorthand>) -> Self {
		let mut by_name = BTreeMap::new();
		let mut properties = BTreeSet::new();
		for shorthand in shorthands {
			let name = shorthand.name.clone();
			for property in std::iter::once(&name).chain(&shorthand.longhands).chain(&shorthand.resets) {
				properties.insert(property.clone());
			}
			assert!(by_name.insert(name.clone(), shorthand).is_none(), "duplicate shorthand {name}");
		}
		let mut graph = Self { shorthands: by_name, terminals: HashMap::new() };
		graph.terminals = properties
			.iter()
			.map(|property| {
				let mut terminals = BTreeSet::new();
				graph.collect_terminal_longhands(property, &mut BTreeSet::new(), &mut terminals);
				(property.clone(), terminals)
			})
			.collect();
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

	/// Every shorthand which expresses a property, narrowest first.
	pub(crate) fn shorthands(&self, property: &str) -> Vec<String> {
		let mut shorthands = self
			.shorthands
			.iter()
			.filter(|(_, shorthand)| self.contains_descendant(&shorthand.longhands, property))
			.map(|(name, _)| name.clone())
			.collect::<Vec<_>>();
		shorthands.sort_by_cached_key(|name| (self.terminals(name).len(), name.clone()));
		shorthands
	}

	/// How a shorthand's value writes the longhands it sets, when the registry states it.
	pub(crate) fn writes(&self, name: &str) -> Option<Writes> {
		let shorthand = self.shorthands.get(name)?;
		match shorthand.writes.as_ref()? {
			WritesDef::Keyword(WritesKeyword::Any) => Some(Writes::Slots(
				shorthand
					.longhands
					.iter()
					.map(|property| SlotDef {
						property: property.clone(),
						before: String::new(),
						after: String::new(),
						optional: true,
					})
					.collect(),
			)),
			WritesDef::Keyword(WritesKeyword::Repeat) => Some(Writes::Repeat),
			WritesDef::Keyword(WritesKeyword::Same) => Some(Writes::Same),
			WritesDef::Slots(slots) => Some(Writes::Slots(slots.clone())),
		}
	}

	/// Every shorthand which resets a property without expressing it.
	pub(crate) fn reset_by(&self, property: &str) -> Vec<String> {
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
		reset_by
	}

	fn expanded_longhands(&self, properties: &[String]) -> Vec<String> {
		let mut longhands = Vec::new();
		for property in properties {
			self.collect_expanded_longhands(property, &mut BTreeSet::new(), &mut longhands);
		}
		longhands
	}

	fn collect_expanded_longhands(&self, property: &str, visiting: &mut BTreeSet<String>, longhands: &mut Vec<String>) {
		assert!(visiting.insert(property.to_string()), "shorthand cycle at {property}");
		if !longhands.iter().any(|seen| seen == property) {
			longhands.push(property.to_string());
		}
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

	/// The properties a property sets which set nothing themselves.
	fn terminals(&self, property: &str) -> &BTreeSet<String> {
		self.terminals.get(property).unwrap_or_else(|| panic!("{property} is not a property the registry names"))
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
			self.validate_writes(name, shorthand);
		}
	}

	/// A shorthand's `writes` must state a value for every longhand it sets, and only those.
	fn validate_writes(&self, name: &str, shorthand: &Shorthand) {
		let Some(writes) = &shorthand.writes else { return };
		assert!(!shorthand.longhands.is_empty(), "{name} states writes but sets no longhands");
		match writes {
			WritesDef::Keyword(WritesKeyword::Repeat) => {
				for longhand in &shorthand.longhands {
					assert!(
						self.terminals(longhand).len() == 1,
						"{name} repeats its values over {longhand}, which sets longhands of its own"
					);
				}
			}
			WritesDef::Keyword(WritesKeyword::Any | WritesKeyword::Same) => {}
			WritesDef::Slots(slots) => {
				let mut written = BTreeSet::new();
				for slot in slots {
					assert!(
						self.terminals.contains_key(&slot.property),
						"{name} writes {}, which is not a property the registry names",
						slot.property
					);
					written.extend(self.terminals(&slot.property).iter().cloned());
				}
				let longhands = self.terminals(name);
				assert!(
					&written == longhands,
					"{name} states no value for {}",
					longhands.difference(&written).cloned().collect::<Vec<_>>().join(", ")
				);
			}
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
		assert_eq!(graph.shorthands.len(), 84);
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
	fn expands_longhands_in_grammar_order() {
		let graph = get_shorthand_graph();
		assert_eq!(
			graph.shorthand("margin").unwrap().longhands,
			["margin-top", "margin-right", "margin-bottom", "margin-left"]
		);
		assert_eq!(
			graph.shorthand("border").unwrap().longhands,
			[
				"border-width",
				"border-top-width",
				"border-right-width",
				"border-bottom-width",
				"border-left-width",
				"border-style",
				"border-top-style",
				"border-right-style",
				"border-bottom-style",
				"border-left-style",
				"border-color",
				"border-top-color",
				"border-right-color",
				"border-bottom-color",
				"border-left-color",
			]
		);
	}

	#[test]
	fn lists_every_shorthand_of_a_longhand_narrowest_first() {
		assert_eq!(get_shorthand_graph().shorthands("border-left-color"), ["border-left", "border-color", "border"]);
	}

	#[test]
	fn reports_repeat_writes() {
		assert!(matches!(get_shorthand_graph().writes("margin"), Some(Writes::Repeat)));
	}

	#[test]
	fn expands_any_writes_to_one_optional_slot_per_longhand() {
		let Some(Writes::Slots(slots)) = get_shorthand_graph().writes("border") else {
			panic!("border states any");
		};
		assert_eq!(
			slots.iter().map(|slot| slot.property.as_str()).collect::<Vec<_>>(),
			["border-width", "border-style", "border-color"]
		);
		assert!(slots.iter().all(|slot| slot.optional && slot.before.is_empty() && slot.after.is_empty()));
	}

	#[test]
	fn reports_stated_slots() {
		let Some(Writes::Slots(slots)) = get_shorthand_graph().writes("font") else {
			panic!("font states its slots");
		};
		let line_height = slots.iter().find(|slot| slot.property == "line-height").unwrap();
		assert_eq!(line_height.before, "/");
		assert!(line_height.optional);
	}

	#[test]
	fn reports_no_writes_when_the_registry_states_none() {
		assert!(get_shorthand_graph().writes("background").is_none());
	}

	#[test]
	fn expands_reset_relationships() {
		assert_eq!(get_shorthand_graph().reset_by("border-image-source"), ["border"]);
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
			Shorthand {
				name: "first".into(),
				longhands: vec!["second".into()],
				resets: vec![],
				resets_all: false,
				writes: None,
			},
			Shorthand {
				name: "second".into(),
				longhands: vec!["first".into()],
				resets: vec![],
				resets_all: false,
				writes: None,
			},
		]);
	}

	#[test]
	#[should_panic(expected = "states no value for")]
	fn rejects_slots_which_leave_a_longhand_out() {
		ShorthandGraph::new(vec![Shorthand {
			name: "gap".into(),
			longhands: vec!["row-gap".into(), "column-gap".into()],
			resets: vec![],
			resets_all: false,
			writes: Some(WritesDef::Slots(vec![SlotDef {
				property: "row-gap".into(),
				before: String::new(),
				after: String::new(),
				optional: false,
			}])),
		}]);
	}
}
