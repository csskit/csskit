use anyhow::{Result, anyhow};
use reqwest::{
	Client,
	header::{HeaderMap, HeaderValue, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::{
	env::current_dir,
	fs::{create_dir_all, metadata, read_to_string, write},
	path::PathBuf,
	time::{Duration, SystemTime},
};

use crate::web_features_data::WebFeaturesData;

const CACHE_DURATION_SECS: u64 = 24 * 60 * 60;

const USER_AGENT_STRING: &str = "csskit_spec_generator/0.1";

const CACHE_DIR_NAME: &str = "cache";

const GITHUB_CSSWG_TREE_URL: &str = "https://api.github.com/repos/w3c/csswg-drafts/git/trees/main";

const WEB_FEATURES_DATA_URL: &str =
	"https://github.com/web-platform-dx/web-features/releases/latest/download/data.extended.json";

const CSS_SPEC_PREFIX: &str = "css-";

const CSSWG_DRAFTS_BASE_URL: &str = "https://drafts.csswg.org";

/// Creates an HTTP client configured with appropriate headers and compression
pub fn default_http_client() -> Result<Client> {
	let mut headers = HeaderMap::new();
	headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_STRING));
	let client = Client::builder().default_headers(headers).gzip(true).brotli(true).build()?;
	Ok(client)
}

/// Finds the workspace root and returns the cache directory path
///
/// Searches upward from the current directory until it finds a Cargo.toml file,
/// then returns the target/cache directory within that workspace.
fn workspace_target_cache() -> Result<PathBuf> {
	let workspace_root = find_workspace_root()?;
	let cache_dir = workspace_root.join("target").join(CACHE_DIR_NAME);
	create_dir_all(&cache_dir)?;
	Ok(cache_dir)
}

/// Finds the workspace root by searching for Cargo.toml upward from the current directory
fn find_workspace_root() -> Result<PathBuf> {
	let mut dir = current_dir()?;
	while !dir.join("Cargo.toml").exists() {
		dir = dir.parent().ok_or_else(|| anyhow!("Could not find workspace"))?.to_path_buf();
	}
	Ok(dir)
}

/// Fetches content from a URL, using cached content if it's fresh enough
///
/// If the cache file exists and is less than 24 hours old, returns the cached content.
/// Otherwise, fetches fresh content from the URL and updates the cache.
async fn fetch_cached(client: &Client, url: &str, cache_name: &str) -> Result<String> {
	let cache_path = workspace_target_cache()?.join(cache_name);

	if let Some(cached_content) = try_read_cache(&cache_path)? {
		return Ok(cached_content);
	}

	fetch_and_cache(client, url, &cache_path).await
}

/// Attempts to read cached content if it's still fresh
fn try_read_cache(cache_path: &PathBuf) -> Result<Option<String>> {
	let modified = metadata(cache_path).and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH);
	let age = SystemTime::now().duration_since(modified)?;

	if age < Duration::from_secs(CACHE_DURATION_SECS) { Ok(Some(read_to_string(cache_path)?)) } else { Ok(None) }
}

/// Fetches content from a URL and writes it to the cache
async fn fetch_and_cache(client: &Client, url: &str, cache_path: &PathBuf) -> Result<String> {
	let res = client.get(url).send().await?;
	if !res.status().is_success() {
		anyhow::bail!("HTTP {} for {}", res.status(), url);
	}
	let contents = res.text().await?;
	write(cache_path, &contents)?;
	Ok(contents)
}

/// Fetches web features data from the web-features repository
pub async fn get_web_features_data(client: &Client) -> Result<WebFeaturesData> {
	let text = fetch_cached(client, WEB_FEATURES_DATA_URL, "web-features-data.extended.json").await?;
	Ok(serde_json::from_str(&text)?)
}

/// Gets a map of spec name to list of available versions
///
/// For example: "align" -> [3, 4]
///
/// Parses spec directory names in the format "css-{name}-{version}"
pub async fn get_spec_versions(client: &Client) -> Result<HashMap<String, Vec<usize>>> {
	let text = fetch_cached(client, GITHUB_CSSWG_TREE_URL, "index.json").await?;
	let v: Value = serde_json::from_str(&text)?;
	let paths = extract_tree_directories(&v)?;

	let mut map: HashMap<String, Vec<usize>> = HashMap::new();
	for path in paths {
		if let Some((name, version)) = parse_spec_path(&path) {
			map.entry(name).or_default().push(version);
		}
	}

	for versions in map.values_mut() {
		versions.sort();
	}

	Ok(map)
}

/// Extracts directory paths from the GitHub API tree response
fn extract_tree_directories(tree_value: &Value) -> Result<Vec<String>> {
	Ok(serde_json::from_value::<Vec<Value>>(tree_value["tree"].clone())?
		.into_iter()
		.filter_map(|item| {
			if item["type"] == "tree" { serde_json::from_value::<String>(item["path"].clone()).ok() } else { None }
		})
		.filter(|s| s.starts_with(CSS_SPEC_PREFIX))
		.collect())
}

/// Parses a spec path in the format "css-{name}-{version}" into (name, version)
///
/// Returns None if the path doesn't match the expected format
fn parse_spec_path(path: &str) -> Option<(String, usize)> {
	let parts: Vec<&str> = path.split('-').collect();
	if parts.len() >= 3
		&& parts[0] == "css"
		&& let Ok(version) = parts[parts.len() - 1].parse::<usize>()
	{
		let name = parts[1..parts.len() - 1].join("-");
		return Some((name, version));
	}
	None
}

/// Fetches the HTML content for a specific CSS spec version
pub async fn get_spec(client: &Client, name: &str, ver: usize) -> Result<String> {
	let url = format!("{}/css-{}-{}/", CSSWG_DRAFTS_BASE_URL, name, ver);
	let cache_name = format!("{}-{}.txt", name, ver);
	fetch_cached(client, &url, &cache_name).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopularityEntry {
	pub property_name: String,
	pub day_percentage: f64,
}

/// Fetches CSS popularity data from chromestatus.com
///
/// Returns a map of property names to their usage percentage (0-100)
pub async fn get_css_popularity(client: &Client) -> Result<HashMap<String, f64>> {
	let cache_key = "popularity.json";
	let url = "https://chromestatus.com/data/csspopularity";

	let text = fetch_cached(client, url, cache_key).await?;
	let data: Vec<PopularityEntry> = serde_json::from_str(&text)?;

	let mut popularity_map = HashMap::new();
	for entry in data {
		popularity_map.insert(entry.property_name, entry.day_percentage * 100.0);
	}

	Ok(popularity_map)
}
