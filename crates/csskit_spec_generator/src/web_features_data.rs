use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFeaturesData {
	pub browsers: Browsers,
	pub features: HashMap<String, FeatureData>,
	pub groups: HashMap<String, GroupData>,
	pub snapshots: HashMap<String, SnapshotData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Browsers {
	pub chrome: BrowserData,
	pub chrome_android: BrowserData,
	pub edge: BrowserData,
	pub firefox: BrowserData,
	pub firefox_android: BrowserData,
	pub safari: BrowserData,
	pub safari_ios: BrowserData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserData {
	pub name: String,
	pub releases: Vec<Release>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
	pub version: String,
	pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureData {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description_html: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub spec: Option<StringOrArray>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<Status>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub caniuse: Option<StringOrArray>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compat_features: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub group: Option<StringOrArray>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub snapshot: Option<StringOrArray>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discouraged: Option<Discouraged>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrArray {
	Single(String),
	Multiple(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discouraged {
	pub according_to: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub alternatives: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
	pub baseline: BaselineStatus,
	pub support: SupportData,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub baseline_low_date: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub baseline_high_date: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub by_compat_key: Option<HashMap<String, CompatKeyStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BaselineStatus {
	String(String), // matches "high", "low", or any other string value
	Bool(bool),     // matches false (or true if it exists)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatKeyStatus {
	pub baseline: BaselineStatus,
	pub support: SupportData,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub baseline_low_date: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub baseline_high_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportData {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chrome: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chrome_android: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub edge: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub firefox: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub firefox_android: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub safari: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub safari_ios: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupData {
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotData {
	pub name: String,
	pub spec: String,
}
