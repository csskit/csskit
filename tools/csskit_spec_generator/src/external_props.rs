use crate::spec_parser::PropertyDefinition;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ExternalPropsFile {
	#[serde(rename = "module")]
	modules: Vec<ExternalModule>,
}

#[derive(Debug, Deserialize)]
struct ExternalModule {
	name: String,
	url: String,
	title: String,
	#[serde(default)]
	properties: Vec<ExternalProperty>,
}

#[derive(Debug, Deserialize)]
struct ExternalProperty {
	name: String,
	value: String,
	#[serde(default)]
	initial: String,
	#[serde(default)]
	applies_to: String,
	#[serde(default)]
	inherited: String,
	#[serde(default)]
	percentages: String,
	animation_type: Option<String>,
	computed_value: Option<String>,
	canonical_order: Option<String>,
	logical_property_group: Option<String>,
}

impl From<ExternalProperty> for PropertyDefinition {
	fn from(p: ExternalProperty) -> Self {
		PropertyDefinition {
			name: p.name,
			value: p.value,
			initial: p.initial,
			applies_to: p.applies_to,
			inherited: p.inherited,
			percentages: p.percentages,
			animation_type: p.animation_type,
			computed_value: p.computed_value,
			canonical_order: p.canonical_order,
			logical_property_group: p.logical_property_group,
		}
	}
}

/// Properties defined outside of w3c/csswg-drafts, grouped by output module.
///
/// Returns `(module_name, module_url, module_title, properties)` for each module in external_props.toml.
pub fn get_external_props() -> Vec<(String, String, String, Vec<PropertyDefinition>)> {
	let toml_str = include_str!("../external_props.toml");
	let parsed: ExternalPropsFile = toml::from_str(toml_str).expect("Failed to parse external_props.toml");
	parsed
		.modules
		.into_iter()
		.map(|m| (m.name, m.url, m.title, m.properties.into_iter().map(Into::into).collect()))
		.collect()
}
