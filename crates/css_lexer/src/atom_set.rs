/// Object-safe version of AtomSet for use with trait objects. This trait mirrors the functionality of AtomSet but is
/// compatible with `dyn` trait objects.
pub trait DynAtomSet: std::fmt::Debug {
	/// Converts a string keyword to the corresponding atom variant, returning its bit representation.
	fn str_to_bits(&self, keyword: &str) -> u32;

	/// Converts this atom's bit representation back to its string representation.
	fn bits_to_str(&self, bits: u32) -> &'static str;

	/// Get the current bits of this Atom.
	fn bits(&self) -> u32;
}

/// # Usage with `#[derive(AtomSet)]`
///
/// The easiest way to implement this trait is using the `AtomSet` derive macro:
///
/// ```rust
/// use derive_atom_set::AtomSet;
/// use css_lexer::AtomSet;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
/// pub enum Units {
///   // AtomSet must derive default, ideally with an empty atom, or equivalent!
///   #[default]
///   _None,
///
///   // Automatically converts to "px"
///   Px,
///
///   // Automatically converts to "rem"
///   Rem,
///
///   // Custom string mapping
///   #[atom("%")]
///   Percent,
/// }
/// ```
pub trait AtomSet: Default + std::fmt::Debug {
	/// Converts a string keyword to the corresponding atom variant.
	///
	/// This performs case-insensitive matching and returns the `Empty` variant for unrecognized strings.
	///
	/// # Examples
	///
	/// ```rust
	/// # use css_lexer::{AtomSet};
	/// use derive_atom_set::*;
	///
	/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
	/// enum MyAtomSet {
	///   #[default]
	///   _None,
	///   Url
	/// }
	/// assert_eq!(MyAtomSet::from_str("url"), MyAtomSet::Url);
	/// assert_eq!(MyAtomSet::from_str("URL"), MyAtomSet::Url);  // Case insensitive
	/// assert_eq!(MyAtomSet::from_str("unknown"), MyAtomSet::_None);
	/// ```
	fn from_str(keyword: &str) -> Self;

	/// Converts this atom back to its string representation.
	///
	/// Returns a static string slice that represents this atom's canonical form.
	///
	/// The variant marked `#[default]` will always return the empty string.
	///
	/// # Examples
	///
	/// ```rust
	/// # use css_lexer::AtomSet;
	/// use derive_atom_set::*;
	///
	/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
	/// enum MyAtomSet {
	///   #[default]
	///   _None,
	///   Url
	/// }
	/// assert_eq!(MyAtomSet::Url.to_str(), "url");
	/// assert_eq!(MyAtomSet::_None.to_str(), "");
	///
	/// // Round-trip conversion
	/// let atom = MyAtomSet::from_str("url");
	/// assert_eq!(atom.to_str(), "url");
	/// ```
	fn to_str(self) -> &'static str;

	/// Returns the length in characters of this atom's string representation.
	///
	/// This is equivalent to `self.to_str().len()` but may be more efficient depending on the implementation.
	///
	/// # Examples
	///
	/// ```rust
	/// # use css_lexer::AtomSet;
	/// use derive_atom_set::*;
	///
	/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
	/// enum MyAtomSet {
	///   #[default]
	///   _None,
	///   Url
	/// }
	/// assert_eq!(MyAtomSet::Url.len(), 3);
	/// assert_eq!(MyAtomSet::_None.len(), 0);
	/// ```
	fn len(&self) -> u32;

	/// Returns true if the length of this atom is 0.
	///
	/// This is equivalent to `self.to_str().is_empty()` but may be more efficient depending on the implementation.
	///
	/// # Examples
	///
	/// ```rust
	/// # use css_lexer::AtomSet;
	/// use derive_atom_set::*;
	///
	/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
	/// enum MyAtomSet {
	///   #[default]
	///   _None,
	///   Url
	/// }
	/// assert!(!MyAtomSet::Url.is_empty());
	/// assert!(MyAtomSet::_None.is_empty());
	/// ```
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Converts a numeric bit representation back to an atom variant.
	///
	/// This is used internally for efficient storage and retrieval. Returns the `Empty` variant for unrecognized bit
	/// values.
	///
	/// # Examples
	///
	/// ```rust
	/// # use css_lexer::AtomSet;
	/// use derive_atom_set::*;
	///
	/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
	/// enum MyAtomSet {
	///   #[default]
	///   _None,
	///   Url
	/// }
	/// let atom = MyAtomSet::Url;
	/// let bits = atom.as_bits();
	/// let restored = MyAtomSet::from_bits(bits);
	/// assert_eq!(atom, restored);
	/// ```
	fn from_bits(bits: u32) -> Self;

	/// Converts this atom to its numeric bit representation.
	///
	/// This is used internally for efficient storage. The bit value corresponds to the enum discriminant.
	///
	/// # Examples
	///
	/// ```rust
	/// # use css_lexer::AtomSet;
	/// use derive_atom_set::*;
	///
	/// #[derive(Debug, Default, Copy, Clone, PartialEq, AtomSet)]
	/// enum MyAtomSet {
	///   #[default]
	///   _None,
	///   Url
	/// }
	/// let bits = MyAtomSet::Url.as_bits();
	/// assert_eq!(MyAtomSet::from_bits(bits), MyAtomSet::Url);
	/// ```
	fn as_bits(&self) -> u32;
}

/// Blanket implementation so any AtomSet can be used as a DynAtomSet
impl<T: AtomSet + Clone + 'static> DynAtomSet for T {
	fn str_to_bits(&self, keyword: &str) -> u32 {
		T::from_str(keyword).as_bits()
	}

	fn bits_to_str(&self, bits: u32) -> &'static str {
		T::from_bits(bits).to_str()
	}

	fn bits(&self) -> u32 {
		self.clone().as_bits()
	}
}
