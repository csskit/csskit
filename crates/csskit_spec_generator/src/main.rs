#![deny(warnings)]
mod codegen;
mod excluded_specs;
mod fetch_cached;
mod ignore_properties;
mod manual_parse_properties;
mod shorthands;
mod spec_parser;
mod todo_properties;
mod value_extensions;
mod web_features_data;

use anyhow::Result;
use clap::{Parser, Subcommand};
use heck::ToSnakeCase;
use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;

use crate::codegen::{generate_feature_data, generate_property_atoms, generate_spec_module};
use crate::fetch_cached::{
	default_http_client, get_css_popularity, get_csswg_commit_sha, get_spec, get_spec_versions, get_web_features_data,
};
use crate::spec_parser::{PropertyDefinition, parse_spec_properties};

/// Preview line count when verbose mode is enabled
const PREVIEW_LINE_COUNT: usize = 30;

fn strip_css_prefix(name: &str) -> &str {
	name.strip_prefix("css-").unwrap_or(name)
}

/// Type alias for property descriptions map
type PropertyDescriptions = HashMap<String, String>;

#[derive(Parser, Debug)]
#[command(name = "csskit_spec_generator")]
#[command(about = "Generate CSS spec definitions and feature data", long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// List all available CSS specs
	List,
	/// Generate CSS feature data from web-features
	GenerateFeatureData,
	/// Generate Rust code for a specific CSS spec
	GenerateSpec {
		/// Name of the spec (e.g., "align", "backgrounds", "borders")
		name: String,
		/// Show verbose output including generated code preview
		#[arg(short, long)]
		verbose: bool,
	},
	/// Generate all CSS specs
	GenerateAll,
	/// Generate property_atoms.rs from CSS specs
	GeneratePropertyAtoms,
}

/// Generates code for a single spec by processing all its versions
///
/// Accumulates properties from all spec versions, with later versions overwriting
/// earlier ones for properties with the same name, then generates a single Rust
/// module containing all the property definitions.
async fn generate_single_spec(
	client: &reqwest::Client,
	name: &str,
	versions: &[usize],
	verbose: bool,
	property_descriptions: &PropertyDescriptions,
	csswg_sha: Option<&str>,
) -> Result<()> {
	let (properties, latest_version) = collect_properties_from_versions(client, name, versions).await?;

	if properties.is_empty() {
		println!("  No properties found across all versions");
		return Ok(());
	}

	println!("  Total unique properties across all versions: {}", properties.len());

	let code = generate_spec_module(name, latest_version, &properties, None, property_descriptions, csswg_sha);
	let line_count = code.lines().count();

	if verbose {
		show_verbose_output(&properties, &code, line_count);
	}

	write_spec_module(name, &code)?;

	Ok(())
}

/// Collects and merges properties from all versions of a spec
///
/// Returns the unique properties and the latest version number
async fn collect_properties_from_versions(
	client: &reqwest::Client,
	name: &str,
	versions: &[usize],
) -> Result<(Vec<PropertyDefinition>, usize)> {
	let mut all_properties: HashMap<String, PropertyDefinition> = HashMap::new();
	let mut latest_version = 0;

	for &version in versions {
		println!("    Processing {}-{}/...", name, version);
		latest_version = version;

		match process_single_version(client, name, version, &mut all_properties).await {
			Ok(count) => {
				if count == 0 {
					println!("    No properties found");
				} else {
					println!("    Found {} properties", count);
				}
			}
			Err(e) => {
				eprintln!("    Error processing version: {}", e);
			}
		}
	}

	let properties: Vec<PropertyDefinition> = all_properties.into_values().collect();
	Ok((properties, latest_version))
}

/// Processes a single spec version and adds its properties to the accumulator
///
/// Returns the number of properties found
async fn process_single_version(
	client: &reqwest::Client,
	name: &str,
	version: usize,
	accumulator: &mut HashMap<String, PropertyDefinition>,
) -> Result<usize> {
	let html = get_spec(client, name, version).await?;
	let properties = parse_spec_properties(&html)?;

	let count = properties.len();
	for prop in properties {
		accumulator.insert(prop.name.clone(), prop);
	}

	Ok(count)
}

/// Displays verbose output including property names and code preview
fn show_verbose_output(properties: &[PropertyDefinition], code: &str, line_count: usize) {
	println!("  Properties:");
	for prop in properties {
		println!("    • {}", prop.name);
	}

	println!("  Code preview (first {} lines):", PREVIEW_LINE_COUNT);
	for (i, line) in code.lines().take(PREVIEW_LINE_COUNT).enumerate() {
		println!("    {:3} | {}", i + 1, line);
	}
	if line_count > PREVIEW_LINE_COUNT {
		println!("    ... ({} more lines)", line_count - PREVIEW_LINE_COUNT);
	}
}

/// Writes the generated spec module to the appropriate file
fn write_spec_module(name: &str, code: &str) -> Result<()> {
	let workspace_root = find_workspace_root()?;
	let spec_snake = strip_css_prefix(name).to_snake_case();
	let output_dir = workspace_root.join("crates").join("css_ast").join("src").join("values").join(&spec_snake);

	create_dir_all(&output_dir)?;

	let file_path = output_dir.join("mod.rs");
	write(&file_path, code)?;
	println!("  Wrote to {}", file_path.display());

	Ok(())
}

/// Finds the workspace root by searching for Cargo.toml upward from the current directory
fn find_workspace_root() -> Result<PathBuf> {
	let mut workspace_root = std::env::current_dir()?;
	while !workspace_root.join("Cargo.toml").exists() {
		workspace_root =
			workspace_root.parent().ok_or_else(|| anyhow::anyhow!("Could not find workspace root"))?.to_path_buf();
	}
	Ok(workspace_root)
}

/// Extracts property descriptions from web features data
///
/// Maps CSS property names (e.g., "align-content") to their descriptions by
/// searching for compat keys that match "css.properties.{name}" pattern.
fn extract_property_descriptions(web_features: &crate::web_features_data::WebFeaturesData) -> PropertyDescriptions {
	let mut descriptions = HashMap::new();

	for feature in web_features.features.values() {
		if let Some(description) = extract_description_from_feature(feature) {
			descriptions.extend(description);
		}
	}

	descriptions
}

/// Extracts descriptions from a single feature's compat keys
fn extract_description_from_feature(feature: &crate::web_features_data::FeatureData) -> Option<PropertyDescriptions> {
	const CSS_PROPERTIES_PREFIX: &str = "css.properties.";

	let status = feature.status.as_ref()?;
	let by_compat_key = status.by_compat_key.as_ref()?;
	let description = feature.description.as_ref()?;

	let mut descriptions = HashMap::new();
	for id in by_compat_key.keys() {
		if let Some(prop_name) = id.strip_prefix(CSS_PROPERTIES_PREFIX) {
			descriptions.insert(prop_name.to_string(), description.clone());
		}
	}

	Some(descriptions)
}

#[tokio::main]
async fn main() -> Result<()> {
	let client = default_http_client()?;
	let cli = Cli::parse();

	// Fetch web features data for property descriptions
	let web_features = get_web_features_data(&client).await?;
	let property_descriptions = extract_property_descriptions(&web_features);

	match cli.command {
		Commands::List => {
			println!("Available CSS spec modules:");
			let specs = get_spec_versions(&client).await?;
			let mut spec_names: Vec<_> = specs.keys().collect();
			spec_names.sort();
			for name in spec_names {
				let versions = &specs[name];
				println!(
					"  {} (versions: {})",
					name,
					versions.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
				);
			}
			println!("Total: {} spec modules", specs.len());
		}
		Commands::GenerateFeatureData => {
			println!("Generating CSS feature data...");

			println!("  Fetching web features data...");
			let features = get_web_features_data(&client).await?;
			println!("  Fetched {} features", features.features.len());

			println!("  Fetching CSS popularity data...");
			let popularity = get_css_popularity(&client).await?;
			println!("  Fetched popularity data for {} properties", popularity.len());

			println!("  Generating Rust code...");
			let code = generate_feature_data(&features, &popularity);

			println!("  Writing to file...");
			let workspace_root = find_workspace_root()?;
			let output_path = workspace_root.join("crates").join("css_feature_data").join("src").join("data.rs");
			write(&output_path, code)?;

			println!("Generated feature data at {}", output_path.display());
		}
		Commands::GenerateSpec { name, verbose } => {
			println!("Generating spec: {}", name);
			let specs = get_spec_versions(&client).await?;
			match specs.get(&name) {
				Some(versions) => {
					println!("Found {} version(s) for spec '{}'", versions.len(), name);
					generate_single_spec(&client, &name, versions, verbose, &property_descriptions, None).await?;
					println!("  Spec generation complete");
				}
				None => {
					eprintln!("  Error: Spec '{}' not found", name);
					eprintln!("  Run 'list' command to see available specs");
					std::process::exit(1);
				}
			}
		}
		Commands::GenerateAll => {
			println!("Generating all CSS specs...");
			let specs = get_spec_versions(&client).await?;
			let csswg_sha = get_csswg_commit_sha(&client).await?;
			println!("Found {} spec modules to generate", specs.len());
			println!("Using w3c/csswg-drafts commit: {}", csswg_sha);

			let mut spec_names: Vec<_> = specs.keys().cloned().collect();
			spec_names.sort();

			let mut successful = 0;
			let mut failed = 0;
			let mut skipped = 0;
			let mut all_property_names = std::collections::HashSet::new();

			for name in &spec_names {
				// Skip excluded specs (easter eggs, superseded specs, etc.)
				if excluded_specs::is_excluded_spec(name) {
					skipped += 1;
					println!("  Skipping excluded spec: {}", name);
					continue;
				}

				if let Some(versions) = specs.get(name) {
					println!("  Generating spec: {}", name);
					match collect_properties_from_versions(&client, name, versions).await {
						Ok((properties, _)) => {
							if properties.is_empty() {
								continue;
							}
							// Collect property names for property_atoms.rs
							for prop in &properties {
								all_property_names.insert(prop.name.clone());
							}

							// Generate the spec module
							let code = generate_spec_module(
								name,
								versions.iter().max().copied().unwrap_or(1),
								&properties,
								None,
								&property_descriptions,
								Some(&csswg_sha),
							);
							match write_spec_module(name, &code) {
								Ok(_) => {
									successful += 1;
									println!("  Completed: {}", name);
								}
								Err(e) => {
									failed += 1;
									eprintln!("  Failed to write {}: {}", name, e);
								}
							}
						}
						Err(e) => {
							failed += 1;
							eprintln!("  Failed: {} - {}", name, e);
						}
					}
				}
			}

			println!("═══════════════════════════════════");
			println!("✓ Spec generation complete!");
			println!("  Successful: {}", successful);
			if skipped > 0 {
				println!("  Skipped: {}", skipped);
			}
			if failed > 0 {
				println!("  Failed: {}", failed);
			}
			println!("═══════════════════════════════════");

			println!("\nGenerating CSS feature data...");

			println!("  Fetching web features data...");
			let features = get_web_features_data(&client).await?;
			println!("  Fetched {} features", features.features.len());

			println!("  Fetching CSS popularity data...");
			let popularity = get_css_popularity(&client).await?;
			println!("  Fetched popularity data for {} properties", popularity.len());

			println!("  Generating Rust code...");
			let code = generate_feature_data(&features, &popularity);

			println!("  Writing to file...");
			let workspace_root = find_workspace_root()?;
			let output_path = workspace_root.join("crates").join("css_feature_data").join("src").join("data.rs");
			write(&output_path, code)?;

			println!("Generated feature data at {}", output_path.display());

			// Generate property_atoms.rs
			println!("\nGenerating property_atoms.rs...");
			println!("  Collected {} unique property names from specs", all_property_names.len());
			let property_atoms_code = generate_property_atoms(&all_property_names);
			let property_atoms_path =
				workspace_root.join("crates").join("css_ast").join("src").join("property_atoms.rs");
			write(&property_atoms_path, property_atoms_code)?;
			println!("Generated property atoms at {}", property_atoms_path.display());

			// Save the csswg-drafts commit SHA for tracking
			println!("\nSaving generation metadata...");
			let metadata_path = workspace_root.join(".csswg-drafts-sha");
			write(&metadata_path, &csswg_sha)?;
			println!("Saved csswg-drafts commit SHA to {}", metadata_path.display());
		}
		Commands::GeneratePropertyAtoms => {
			println!("Generating property_atoms.rs from CSS specs...");
			let specs = get_spec_versions(&client).await?;
			println!("Found {} spec modules", specs.len());

			let mut spec_names: Vec<_> = specs.keys().cloned().collect();
			spec_names.sort();

			let mut all_property_names = std::collections::HashSet::new();

			for name in &spec_names {
				// Skip excluded specs
				if excluded_specs::is_excluded_spec(name) {
					continue;
				}

				if let Some(versions) = specs.get(name) {
					print!("  Processing spec: {}...", name);
					match collect_properties_from_versions(&client, name, versions).await {
						Ok((properties, _)) => {
							for prop in &properties {
								all_property_names.insert(prop.name.clone());
							}
							println!(" {} properties", properties.len());
						}
						Err(e) => {
							println!(" error: {}", e);
						}
					}
				}
			}

			println!("\nCollected {} unique property names from specs", all_property_names.len());

			let workspace_root = find_workspace_root()?;
			let property_atoms_code = generate_property_atoms(&all_property_names);
			let property_atoms_path =
				workspace_root.join("crates").join("css_ast").join("src").join("property_atoms.rs");
			write(&property_atoms_path, property_atoms_code)?;
			println!("Generated property atoms at {}", property_atoms_path.display());
		}
	}

	Ok(())
}
