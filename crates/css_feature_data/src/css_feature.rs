use crate::{
	BaselineStatus, BrowserSupport, NamedBrowserVersion,
	data::{CSS_FEATURES, GROUPS, SPECS},
};
#[cfg(feature = "browserslist")]
use browserslist::{Error, Opts, resolve};
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq)]
pub struct CSSFeature {
	/// The ID of the feature.
	pub id: &'static str,

	/// A friendly, human-readable name of the feature.
	pub name: &'static str,

	/// A description for this feature.
	pub description: &'static str,

	/// A URL to the CSS specification that includes this feature.
	pub spec: &'static str,

	/// The groups this feature is part of.
	pub groups: &'static [&'static str],

	/// The CanIUse URLs available for this feature.
	pub caniuse: &'static [&'static str],

	/// The current BaselineStatus of this feature
	pub baseline_status: BaselineStatus,

	/// The browsers that support this feature.
	pub browser_support: BrowserSupport,

	/// The percentage of web pages which use this feature, as reported by Chrome usage data.
	pub popularity: f32,
}

#[derive(Debug, Clone)]
pub struct CompatibilityResult {
	pub is_supported: bool,
	pub unsupported_browsers: Vec<String>,
	pub supported_browsers: Vec<String>,
}

impl CSSFeature {
	pub fn by_feature_name(name: &str) -> Option<&'static CSSFeature> {
		CSS_FEATURES.get(name)
	}

	pub fn by_property_name(name: &str) -> Option<&'static CSSFeature> {
		CSS_FEATURES.get(&format!("css.properties.{name}"))
	}

	/// Check if a feature has Baseline support.
	///
	/// ```rust
	/// use css_feature_data::CSSFeature;
	/// assert_eq!(CSSFeature::by_property_name("word-break").is_some_and(|f| f.has_baseline_support()), true)
	/// ```
	pub fn has_baseline_support(&self) -> bool {
		matches!(self.baseline_status, BaselineStatus::High { .. } | BaselineStatus::Low(_))
	}

	/// Check the earliest date this feature was supported as Baseline.
	///
	/// If BaselineStatus::Low, then that date will be returned.
	/// If BaselineStatus::High, then the low date will be returned.
	///
	/// ```rust
	/// use css_feature_data::CSSFeature;
	/// use chrono::NaiveDate;
	/// assert_eq!(CSSFeature::by_property_name("word-break").map(|f| f.baseline_supported_since()), Some(NaiveDate::from_ymd_opt(2015,07,29)));
	/// ```
	pub fn baseline_supported_since(&self) -> Option<NaiveDate> {
		match self.baseline_status {
			BaselineStatus::High { low_since, .. } | BaselineStatus::Low(low_since) => Some(low_since),
			_ => None,
		}
	}

	/// Get all CSS properties in the same groups as this one
	pub fn group_siblings(&self) -> impl Iterator<Item = &'static CSSFeature> {
		self.groups
			.iter()
			.filter_map(|f| GROUPS.get(f).map(|names| names.iter().filter_map(|name| Self::by_feature_name(name))))
			.flatten()
	}

	/// Get all CSS properties in the same specification as this one
	pub fn spec_siblings(&self) -> impl Iterator<Item = &'static CSSFeature> {
		SPECS
			.get(self.spec)
			.map(|names| names.iter().filter_map(|name| Self::by_feature_name(name)))
			.into_iter()
			.flatten()
	}

	/// Check if a CSS property is supported across browsers specified by a browserslist query
	#[cfg(feature = "browserslist")]
	pub fn supports_browserslist(
		&self,
		browserslist_query: &[&str],
		opts: &Opts,
	) -> Result<CompatibilityResult, Error> {
		let browsers = resolve(browserslist_query, opts)?;
		let mut supported_browsers = Vec::new();
		let mut unsupported_browsers = Vec::new();

		for browser in browsers {
			let str = format!("{} {}", browser.name(), browser.version());
			let named_browser = NamedBrowserVersion::try_from(browser);
			dbg!(&named_browser);
			if named_browser.is_ok_and(|ver| self.browser_support.supports(ver)) {
				supported_browsers.push(str);
			} else {
				unsupported_browsers.push(str);
			}
		}
		Ok(CompatibilityResult {
			is_supported: unsupported_browsers.is_empty(),
			unsupported_browsers,
			supported_browsers,
		})
	}

	pub fn supports(&self, browser: NamedBrowserVersion) -> bool {
		self.browser_support.supports(browser)
	}
}

pub trait ToCSSFeature {
	fn to_css_feature(&self) -> Option<&'static CSSFeature>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_baseline_supported() {
		let flex_wrap = CSSFeature::by_feature_name("css.properties.flex-wrap");
		assert!(flex_wrap.is_some_and(|f| f.has_baseline_support()));
	}

	#[test]
	fn test_group_siblings() {
		let flex_wrap = CSSFeature::by_property_name("speak").unwrap();
		assert_eq!(
			flex_wrap.group_siblings().map(|f| f.id).collect::<Vec<_>>(),
			vec![
				"css.properties.speak",
				"css.properties.speak-as",
				"css.properties.speak-as.digits",
				"css.properties.speak-as.literal-punctuation",
				"css.properties.speak-as.no-punctuation",
				"css.properties.speak-as.normal",
				"css.properties.speak-as.spell-out"
			]
		);
	}

	#[test]
	fn test_spec_siblings() {
		let flex_wrap = CSSFeature::by_property_name("display").unwrap();
		assert_eq!(
			flex_wrap.spec_siblings().map(|f| &f.id).collect::<Vec<_>>(),
			SPECS
				.get("https://drafts.csswg.org/css-display-3/#the-display-properties")
				.unwrap()
				.iter()
				.collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_supports_browserslist_flex_wrap() {
		let compat = CSSFeature::by_property_name("flex-wrap")
			.unwrap()
			.supports_browserslist(&["Chrome >= 30", "Firefox >= 21", "Safari >= 9.1"], &Default::default())
			.unwrap();

		// flex-wrap should be fully supported in these modern browsers
		assert!(compat.is_supported, "flex-wrap should be supported");
		assert!(!compat.supported_browsers.is_empty(), "Should have supported browsers");
		assert!(compat.unsupported_browsers.is_empty(), "Should have no unsupported browsers for this query");
	}

	#[test]
	fn test_supports_browserslist_ranged() {
		let compat = CSSFeature::by_property_name("flex-wrap")
			.unwrap()
			.supports_browserslist(&["> 1%", "last 2 versions", "not dead", "ie 6"], &Default::default())
			.unwrap();

		// flex-wrap should be fully supported in these browsers
		assert!(!compat.is_supported, "flex-wrap should be supported");
		assert!(!compat.supported_browsers.is_empty(), "Should have supported browsers");
		assert!(!compat.unsupported_browsers.is_empty(), "Should have unsupported browsers");
		assert!(compat.unsupported_browsers.iter().any(|b| b == "ie 6"), "Includes IE6 in unsupported_browsers")
	}

	#[test]
	fn test_invalid_browserslist_query() {
		let result = CSSFeature::by_property_name("flex-wrap")
			.unwrap()
			.supports_browserslist(&["invalid browser query !@#$%"], &Default::default());

		assert!(result.is_err());
	}
}
