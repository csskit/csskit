use anyhow::Result;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PropertyDefinition {
	pub name: String,
	pub value: String,
	pub initial: String,
	pub applies_to: String,
	pub inherited: String,
	pub percentages: String,
	pub canonical_order: Option<String>,
	pub animation_type: Option<String>,
}

/// Parse property definitions from a CSS spec HTML
pub fn parse_spec_properties(html: &str) -> Result<Vec<PropertyDefinition>> {
	let document = Html::parse_document(html);
	let table_selector = Selector::parse("table.propdef, table.descdef").unwrap();
	let tr_selector = Selector::parse("tr").unwrap();
	let th_selector = Selector::parse("th").unwrap();
	let td_selector = Selector::parse("td").unwrap();

	let mut properties = Vec::new();

	for table in document.select(&table_selector) {
		let mut prop_data: HashMap<String, String> = HashMap::new();
		for row in table.select(&tr_selector) {
			let headers: Vec<_> = row.select(&th_selector).collect();
			let data: Vec<_> = row.select(&td_selector).collect();

			if headers.len() == 1 && data.len() == 1 {
				let key = headers[0].text().collect::<String>().trim().to_lowercase();
				let key = key.trim_end_matches(':').to_string();
				let value = data[0].text().collect::<String>().trim().to_string();
				prop_data.insert(key, value);
			}
		}

		if let Some(names) = prop_data.get("name") {
			for name in names.split(',') {
				let name = name.trim().to_string();

				// Skip if:
				//  - this has "new values" - it means it's extending an existing property
				//  - this has "for" - it means it's for an @-rule and not a general style value
				//  TODO: We should be smarter about these, but for now we can simply skip
				if prop_data.contains_key("new values") || prop_data.contains_key("for") {
					continue;
				}

				if let Some(value) = prop_data.get("value") {
					properties.push(PropertyDefinition {
						name: name.clone(),
						value: value.clone(),
						initial: prop_data.get("initial").cloned().unwrap_or_default(),
						applies_to: prop_data.get("applies to").cloned().unwrap_or_default(),
						inherited: prop_data.get("inherited").cloned().unwrap_or_default(),
						percentages: prop_data.get("percentages").cloned().unwrap_or_default(),
						canonical_order: prop_data.get("canonical order").cloned(),
						animation_type: prop_data
							.get("animation type")
							.or_else(|| prop_data.get("animatable"))
							.cloned(),
					});
				}
			}
		}
	}

	Ok(properties)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_simple_property() {
		let html = r#"
		<table class="propdef">
			<tr><th>Name:</th><td>align-content</td></tr>
			<tr><th>Value:</th><td>normal | <baseline-position> | <content-distribution></td></tr>
			<tr><th>Initial:</th><td>normal</td></tr>
			<tr><th>Applies to:</th><td>block containers</td></tr>
			<tr><th>Inherited:</th><td>no</td></tr>
			<tr><th>Percentages:</th><td>n/a</td></tr>
			<tr><th>Animation type:</th><td>discrete</td></tr>
		</table>
		"#;

		let props = parse_spec_properties(html).unwrap();
		assert_eq!(props.len(), 1);
		assert_eq!(props[0].name, "align-content");
		assert_eq!(props[0].initial, "normal");
	}

	#[test]
	fn test_skip_property_with_for_field() {
		let html = r#"
		<table class="propdef">
			<tr><th>Name:</th><td>font-display</td></tr>
			<tr><th>For:</th><td>@font-face</td></tr>
			<tr><th>Value:</th><td>auto | block | swap | fallback | optional</td></tr>
			<tr><th>Initial:</th><td>auto</td></tr>
		</table>
		"#;

		let props = parse_spec_properties(html).unwrap();
		assert_eq!(props.len(), 0);
	}
}
