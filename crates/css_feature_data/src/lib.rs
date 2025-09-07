#![deny(warnings)]
//! CSS Feature Data
//!
//! This crate provides comprehensive data about CSS features and their browser support, based on the
//! [web-features](https://github.com/web-platform-dx/web-features) project.
//!
//! ## Example
//!
//! ```rust
//! use css_feature_data::CSSFeature;
//!
//! if let Some(data) = CSSFeature::by_property_name("flex-wrap") {
//!     println!("flex-wrap status: {:?}", data.baseline_status);
//!     println!("Chrome support: {:?}", data.browser_support.chrome);
//! }
//! ```

mod baseline_status;
mod browser_support;
mod browser_version;
mod css_feature;
mod data;

pub use baseline_status::*;
pub use browser_support::*;
pub use browser_version::*;
pub use css_feature::*;
