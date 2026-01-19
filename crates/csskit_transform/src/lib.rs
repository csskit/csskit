#![deny(warnings)]
mod transform;
mod transformer;

pub use transform::*;
pub use transformer::*;

pub(crate) mod prelude {
	pub(crate) use crate::{CssMinifierFeature, Transform, Transformer};
	pub(crate) use css_ast::{CssMetadata, Visit};
	pub(crate) use css_lexer::ToSpan;
	pub(crate) use css_parse::NodeWithMetadata;
}

mod css_minifier;
mod reduce_colors;
mod reduce_lengths;
mod reduce_time_units;

pub use css_minifier::*;
pub use reduce_colors::*;
pub use reduce_lengths::*;
pub use reduce_time_units::*;

#[cfg(test)]
pub mod test_helpers;
