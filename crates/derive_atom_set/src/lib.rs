#![deny(warnings)]
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod atom_set;

/// Derives an efficient `AtomSet` implementation for interned CSS identifiers.
///
/// This proc macro automatically generates optimized string-to-enum matching code.
///
/// ## Variant Attributes
///
/// - `#[default]`: Marks this variant as the empty/fallback value (returns empty string)
/// - `#[atom("custom")]`: Overrides the default string representation
///
/// ## Naming Convention
///
/// If `#[atom("")]` is not provided a string is derived from the variant name:
/// - `Px` → `"px"`
/// - `FontSize` → `"font-size"`
/// - `WebkitTransform` → `"webkit-transform"`
///
/// # Example
///
/// ```rust
/// // Provide this trait definition:
/// trait AtomSet {
///     fn from_str(keyword: &str) -> Self;
///     fn to_str(self) -> &'static str;
///     fn len(&self) -> u32;
///     fn from_bits(bits: u32) -> Self;
///     fn as_bits(&self) -> u32;
/// }
/// use derive_atom_set::AtomSet;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
/// pub enum MyAtomSet {
///     #[default]
///     Unknown, // Must provide an empty default!
///
///     // Absolute units
///     Px, Pt, Pc, In, Cm, Mm, Q,
///
///     // Relative units
///     Em, Ex, Ch, Rem, Lh,
///
///     // Viewport units
///     Vw, Vh, Vi, Vb, Vmin, Vmax,
///
///     // Container query units
///     Cqw, Cqh, Cqi, Cqb, Cqmin, Cqmax,
///
///     // Special case
///     #[atom("%")]
///     Percent,
/// }
///
/// // Usage:
/// assert_eq!(MyAtomSet::from_str("px"), MyAtomSet::Px);
/// assert_eq!(MyAtomSet::from_str("PX"), MyAtomSet::Px);  // Case insensitive matches
/// assert_eq!(MyAtomSet::Px.to_str(), "px");
/// assert_eq!(MyAtomSet::Percent.to_str(), "%");
/// assert_eq!(MyAtomSet::from_str("unknown"), MyAtomSet::Unknown);
/// ```
#[proc_macro_derive(AtomSet, attributes(default, atom))]
pub fn derive_atom_set(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	atom_set::generate(proc_macro2::TokenStream::new(), ast).into()
}
