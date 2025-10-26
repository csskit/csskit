use crate::ignore_properties::get_ignore_properties;
use crate::spec_parser::PropertyDefinition;
use crate::todo_properties::get_todo_properties;
use crate::value_extensions::get_value_extensions;
use crate::web_features_data::{BaselineStatus, FeatureData, StringOrArray, WebFeaturesData};
use css_value_definition_parser::Def;
use heck::ToPascalCase;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{File, parse2};

/// Generate Rust code for a CSS spec module
pub fn generate_spec_module(
	spec_name: &str,
	version: usize,
	properties: &[PropertyDefinition],
	spec_title: Option<&str>,
	property_descriptions: &HashMap<String, String>,
) -> String {
	let url = format!("https://drafts.csswg.org/css-{}-{}/", spec_name, version);

	let module_doc =
		if let Some(title) = spec_title { format!("//! {}\n//! {}\n", title, url) } else { format!("//! {}\n", url) };

	let ignore_properties = get_ignore_properties();
	let should_ignore: HashSet<&str> = ignore_properties.get(spec_name).cloned().unwrap_or_default();

	let mut filtered_properties: Vec<_> =
		properties.iter().filter(|prop| !should_ignore.contains(prop.name.as_str())).collect();
	filtered_properties.sort_by_key(|prop| &prop.name);

	let todo_properties = get_todo_properties();
	let should_comment_out: HashSet<&str> = todo_properties.get(spec_name).cloned().unwrap_or_default();

	let value_extensions = get_value_extensions();
	let spec_extensions = value_extensions.get(spec_name);

	let property_types = filtered_properties.iter().map(|prop| {
		let description = property_descriptions.get(&prop.name);
		let extension = spec_extensions.and_then(|ext| ext.get(prop.name.as_str()).copied());
		generate_property_type(spec_name, version, prop, description, extension)
	});

	let tokens = quote! {
		mod impls;

		use super::prelude::*;
		use impls::*;

		#(#property_types)*
	};

	let file: File = syn::parse2(tokens).expect("generated code should parse");
	let mut code = prettyplease::unparse(&file);

	code = format!("#![allow(warnings)]\n{}\n{}", module_doc, code);

	code = fix_formatting(&code, spec_name, version, &filtered_properties);

	code = add_blank_lines_between_properties(&code);

	comment_out_properties(&code, &filtered_properties, &should_comment_out)
}

fn fix_formatting(code: &str, spec_name: &str, version: usize, properties: &[&PropertyDefinition]) -> String {
	let mut result = String::new();

	for line in code.lines() {
		let mut fixed_line = line.to_string();

		if fixed_line.starts_with("///") && !fixed_line.starts_with("/// ") {
			fixed_line = fixed_line.replacen("///", "/// ", 1);
		}

		if fixed_line.contains(&format!("css-{}-{}", spec_name, version)) {
			for prop in properties {
				let property_id = if prop.name == "--*" { "defining-variables" } else { &prop.name };
				let wrong_url = format!("css-{}-{}#{}", spec_name, version, property_id);
				let correct_url = format!("css-{}-{}/#{}", spec_name, version, property_id);
				fixed_line = fixed_line.replace(&wrong_url, &correct_url);
			}
		}

		result.push_str(&fixed_line);
		result.push('\n');
	}

	result
}

fn add_blank_lines_between_properties(code: &str) -> String {
	let lines: Vec<&str> = code.lines().collect();
	let mut result = Vec::new();
	let mut prev_was_property_end = false;

	for (i, line) in lines.iter().enumerate() {
		let is_property_start = line.starts_with("/// Represents the style value for");

		if is_property_start && prev_was_property_end && i > 0 {
			result.push(String::new());
		}

		result.push(line.to_string());

		let is_enum_end = line.starts_with("pub enum ") && line.contains("StyleValue") && line.ends_with(" {}");
		let is_struct_end = line.starts_with("pub struct ") && line.contains("StyleValue") && line.ends_with(";");
		let is_property_end = is_enum_end || is_struct_end;

		prev_was_property_end = is_property_end;
	}

	result.join("\n")
}

fn comment_out_properties(
	code: &str,
	properties: &[&PropertyDefinition],
	should_comment_out: &HashSet<&str>,
) -> String {
	let lines: Vec<&str> = code.lines().collect();
	let mut result = Vec::new();
	let mut in_commented_property = false;

	for line in lines.iter() {
		if line.starts_with("/// Represents the style value for") {
			in_commented_property = properties.iter().any(|prop| {
				should_comment_out.contains(prop.name.as_str()) && line.contains(&format!("`{}`", prop.name))
			});
		}

		if in_commented_property {
			if line.trim().is_empty() {
				result.push(String::new());
			} else {
				result.push(format!("// {}", line));
			}

			if line.starts_with("pub enum ") || line.starts_with("pub struct ") {
				in_commented_property = false;
			}
		} else {
			result.push(line.to_string());
		}
	}

	result.join("\n")
}

fn generate_property_type(
	spec_name: &str,
	version: usize,
	prop: &PropertyDefinition,
	description: Option<&String>,
	value_extension: Option<&str>,
) -> TokenStream {
	let type_name_base = if prop.name == "--*" { "Custom".to_string() } else { prop.name.to_pascal_case() };

	let property_id = if prop.name == "--*" { "defining-variables" } else { &prop.name };

	let type_name = syn::Ident::new(&format!("{}StyleValue", type_name_base), proc_macro2::Span::call_site());

	let extended_value =
		if let Some(extension) = value_extension { format!("{}{}", prop.value, extension) } else { prop.value.clone() };

	let grammar_cleaned = extended_value.replace("'", "\"").replace("∞", "");
	let (is_enum, needs_lifetime) = match grammar_cleaned.parse::<TokenStream>() {
		Ok(tokens) => match parse2::<Def>(tokens) {
			Ok(def) => {
				let optimized_def = def.optimize();
				let data_type = optimized_def.suggested_data_type();
				let is_enum = data_type.is_enum();
				let needs_lifetime = optimized_def.maybe_unsized();
				(is_enum, needs_lifetime)
			}
			Err(e) => {
				eprintln!("Warning: Failed to parse Def for {}: {} - Error: {}", prop.name, prop.value, e);
				(false, false)
			}
		},
		Err(e) => {
			eprintln!("Warning: Failed to tokenize syntax for {}: {} - Error: {}", prop.name, prop.value, e);
			(false, false)
		}
	};

	let doc_link_url = format!("https://drafts.csswg.org/css-{}-{}/#{}", spec_name, version, property_id);
	let doc_intro = format!(
		" Represents the style value for `{}` as defined in [css-{}-{}]({}).",
		prop.name, spec_name, version, doc_link_url
	);

	let doc_grammar_header = " The grammar is defined as:";
	let grammar = &extended_value;

	let doc_link = format!("https://drafts.csswg.org/css-{}-{}/#{}", spec_name, version, property_id);

	let syntax_value = format!(" {} ", extended_value.replace('\n', " "));

	// Build style_value attributes
	let initial = &prop.initial;
	let applies_to = &prop.applies_to;
	let inherited = prop.inherited.to_lowercase();
	let percentages = prop.percentages.to_lowercase();
	let canonical_order = prop.canonical_order.as_deref().unwrap_or("per grammar").to_lowercase();
	let animation_type = prop.animation_type.as_deref().unwrap_or("not animatable").to_lowercase();

	let css_feature = format!("css.properties.{}", prop.name);

	let (keyword, suffix) = if is_enum { (quote! { enum }, quote! { {} }) } else { (quote! { struct }, quote! { ; }) };

	let lifetime = if needs_lifetime {
		quote! { <'a> }
	} else {
		quote! {}
	};

	let doc_tokens = if let Some(desc) = description {
		let doc_desc = format!(" {}", desc);
		quote! {
			#[doc = #doc_intro]
			#[doc = ""]
			#[doc = #doc_desc]
			#[doc = ""]
			#[doc = #doc_grammar_header]
			#[doc = ""]
			#[doc = "```text,ignore"]
			#[doc = #grammar]
			#[doc = "```"]
			#[doc = ""]
			#[doc = #doc_link]
		}
	} else {
		quote! {
			#[doc = #doc_intro]
			#[doc = ""]
			#[doc = #doc_grammar_header]
			#[doc = ""]
			#[doc = "```text,ignore"]
			#[doc = #grammar]
			#[doc = "```"]
			#[doc = ""]
			#[doc = #doc_link]
		}
	};

	quote! {
		#doc_tokens
		#[syntax(#syntax_value)]
		#[derive(Parse, Peek, ToSpan, ToCursors, StyleValue, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[style_value(
			initial = #initial,
			applies_to = #applies_to,
			inherited = #inherited,
			percentages = #percentages,
			canonical_order = #canonical_order,
			animation_type = #animation_type,
		)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "css_feature_data", derive(ToCSSFeature), css_feature(#css_feature))]
		#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
		pub #keyword #type_name #lifetime #suffix
	}
}

/// Generate CSS feature data as a Rust source file
pub fn generate_feature_data(web_features: &WebFeaturesData, popularity_map: &HashMap<String, f64>) -> String {
	let mut all_groups: HashMap<String, HashSet<String>> = HashMap::new();
	let mut all_specs: HashMap<String, HashSet<String>> = HashMap::new();
	let mut feature_entries: Vec<(String, TokenStream)> = Vec::new();

	// Collect all features
	for (feature_id, feature) in &web_features.features {
		if let Some(status) = &feature.status
			&& let Some(by_compat_key) = &status.by_compat_key
		{
			for (id, compat_status) in by_compat_key {
				if id.starts_with("css.") {
					let entry = generate_feature_entry_tokens(
						id,
						feature_id,
						feature,
						compat_status,
						popularity_map,
						&mut all_groups,
						&mut all_specs,
					);
					feature_entries.push((id.to_string(), entry));
				}
			}
		}
	}

	feature_entries.sort_by(|a, b| a.0.cmp(&b.0));
	let feature_entries: Vec<TokenStream> = feature_entries.into_iter().map(|(_, tokens)| tokens).collect();

	let mut sorted_groups: Vec<_> = all_groups.into_iter().collect();
	sorted_groups.sort_by(|a, b| a.0.cmp(&b.0));
	let group_entries: Vec<TokenStream> = sorted_groups
		.into_iter()
		.filter(|(group, _)| !group.is_empty())
		.map(|(group, members)| {
			let mut sorted_members: Vec<_> = members.into_iter().collect();
			sorted_members.sort();
			let member_strs = sorted_members.iter().map(|m| m.as_str());
			quote! {
				#group => &[#(#member_strs),*]
			}
		})
		.collect();

	let mut sorted_specs: Vec<_> = all_specs.into_iter().collect();
	sorted_specs.sort_by(|a, b| a.0.cmp(&b.0));
	let spec_entries: Vec<TokenStream> = sorted_specs
		.into_iter()
		.filter(|(spec, _)| !spec.is_empty())
		.map(|(spec, members)| {
			let mut sorted_members: Vec<_> = members.into_iter().collect();
			sorted_members.sort();
			let member_strs = sorted_members.iter().map(|m| m.as_str());
			quote! {
				#spec => &[#(#member_strs),*]
			}
		})
		.collect();

	let tokens = quote! {
		use crate::*;
		use phf::{phf_map, Map};
		use chrono::NaiveDate;

		pub static CSS_FEATURES: Map<&'static str, CSSFeature> = phf_map! {
			#(#feature_entries)*
		};

		pub static GROUPS: Map<&'static str, &'static [&'static str]> = phf_map! {
			#(#group_entries),*
		};

		pub static SPECS: Map<&'static str, &'static [&'static str]> = phf_map! {
			#(#spec_entries),*
		};
	};

	let file: File = parse2(tokens).expect("generated code should parse");
	let code = prettyplease::unparse(&file);

	format!("//! Auto-generated CSS features data\n\n{}", code)
}

fn generate_feature_entry_tokens(
	id: &str,
	feature_id: &str,
	feature: &FeatureData,
	compat_status: &crate::web_features_data::CompatKeyStatus,
	popularity_map: &HashMap<String, f64>,
	all_groups: &mut HashMap<String, HashSet<String>>,
	all_specs: &mut HashMap<String, HashSet<String>>,
) -> TokenStream {
	let name = feature.name.as_deref().unwrap_or("");
	let description = feature.description.as_deref().unwrap_or("");

	let spec = if let Some(spec_val) = &feature.spec {
		match spec_val {
			StringOrArray::Single(s) => s.as_str(),
			StringOrArray::Multiple(v) => v.first().map(|s| s.as_str()).unwrap_or(""),
		}
	} else {
		""
	};

	if !spec.is_empty()
		&& let Some(spec_val) = &feature.spec
	{
		match spec_val {
			StringOrArray::Single(s) => {
				all_specs.entry(s.clone()).or_default().insert(id.to_string());
			}
			StringOrArray::Multiple(v) => {
				for s in v {
					all_specs.entry(s.clone()).or_default().insert(id.to_string());
				}
			}
		}
	}

	let groups: Vec<&str> = if let Some(group_val) = &feature.group {
		match group_val {
			StringOrArray::Single(s) => {
				if !s.is_empty() && s != "css" {
					all_groups.entry(s.clone()).or_default().insert(id.to_string());
					vec![s.as_str()]
				} else {
					vec![]
				}
			}
			StringOrArray::Multiple(v) => v
				.iter()
				.filter(|g| !g.is_empty() && g.as_str() != "css")
				.map(|g| {
					all_groups.entry(g.clone()).or_default().insert(id.to_string());
					g.as_str()
				})
				.collect(),
		}
	} else {
		vec![]
	};

	let baseline_status = generate_baseline_status_tokens(
		&compat_status.baseline,
		compat_status.baseline_high_date.as_deref(),
		compat_status.baseline_low_date.as_deref(),
	);

	let browser_support = generate_browser_support_tokens(&compat_status.support);

	let caniuse: Vec<String> = if let Some(caniuse_val) = &feature.caniuse {
		match caniuse_val {
			StringOrArray::Single(s) => {
				if !s.is_empty() {
					vec![format!("https://caniuse.com/{}", s)]
				} else {
					vec![]
				}
			}
			StringOrArray::Multiple(v) => {
				v.iter().filter(|s| !s.is_empty()).map(|s| format!("https://caniuse.com/{}", s)).collect()
			}
		}
	} else {
		vec![]
	};

	let caniuse_strs: Vec<&str> = caniuse.iter().map(|s| s.as_str()).collect();

	let popularity = popularity_map.get(feature_id).copied().unwrap_or(0.0) as f32;

	quote! {
		#id => CSSFeature {
			id: #id,
			name: #name,
			description: #description,
			spec: #spec,
			groups: &[#(#groups),*],
			baseline_status: #baseline_status,
			browser_support: #browser_support,
			caniuse: &[#(#caniuse_strs),*],
			popularity: #popularity,
		},
	}
}

fn generate_baseline_status_tokens(
	baseline: &BaselineStatus,
	high_date: Option<&str>,
	low_date: Option<&str>,
) -> TokenStream {
	match baseline {
		BaselineStatus::String(s) if s == "high" => {
			let since = date_str_to_naive_date_tokens(high_date.unwrap_or(""));
			let low_since = date_str_to_naive_date_tokens(low_date.unwrap_or(""));
			quote! {
				BaselineStatus::High { since: #since, low_since: #low_since }
			}
		}
		BaselineStatus::String(s) if s == "low" => {
			let low_since = date_str_to_naive_date_tokens(low_date.unwrap_or(""));
			quote! {
				BaselineStatus::Low(#low_since)
			}
		}
		BaselineStatus::Bool(false) => quote! { BaselineStatus::False },
		_ => quote! { BaselineStatus::Unknown },
	}
}

fn date_str_to_naive_date_tokens(date: &str) -> TokenStream {
	let parts: Vec<&str> = date.split('-').collect();
	if parts.len() != 3 {
		return quote! { NaiveDate::from_ymd_opt(1970, 1, 1).unwrap() };
	}

	let mut year = parts[0];
	if year.len() > 4 {
		year = &year[year.len() - 4..];
	}

	let year_num: i32 = year.parse().unwrap_or(1970);
	let month_num: u32 = parts[1].parse().unwrap_or(1);
	let day_num: u32 = parts[2].parse().unwrap_or(1);

	quote! { NaiveDate::from_ymd_opt(#year_num, #month_num, #day_num).unwrap() }
}

fn generate_browser_support_tokens(support: &crate::web_features_data::SupportData) -> TokenStream {
	let chrome = browser_version_tokens(support.chrome.as_deref());
	let chrome_android = browser_version_tokens(support.chrome_android.as_deref());
	let edge = browser_version_tokens(support.edge.as_deref());
	let firefox = browser_version_tokens(support.firefox.as_deref());
	let firefox_android = browser_version_tokens(support.firefox_android.as_deref());
	let safari = browser_version_tokens(support.safari.as_deref());
	let safari_ios = browser_version_tokens(support.safari_ios.as_deref());

	quote! {
		BrowserSupport {
			chrome: #chrome,
			chrome_android: #chrome_android,
			edge: #edge,
			firefox: #firefox,
			firefox_android: #firefox_android,
			safari: #safari,
			safari_ios: #safari_ios,
		}
	}
}

fn browser_version_tokens(version: Option<&str>) -> TokenStream {
	if let Some(ver) = version {
		if ver.is_empty() {
			return quote! { BrowserVersion(0, 0) };
		}

		if ver.chars().all(|c| c.is_ascii_digit() || c == '.') {
			let parts: Vec<&str> = ver.split('.').collect();
			let major: u16 = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
			let minor: u16 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
			return quote! { BrowserVersion(#major, #minor) };
		}
	}

	quote! { BrowserVersion(0, 0) }
}
