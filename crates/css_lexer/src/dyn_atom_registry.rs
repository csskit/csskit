use crate::token::ATOM_DYNAMIC_BIT;
use crate::{AtomSet, DynAtomSet};
use fnv::FnvHashMap;
use std::fmt::{Debug, Formatter, Result};
use std::marker::PhantomData;
use std::sync::RwLock;

/// A typed atom that belongs to a specific `DynAtomRegistry<T>`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Atom<T: AtomSet> {
	bits: u32,
	_phantom: PhantomData<T>,
}

impl<T: AtomSet> Atom<T> {
	/// Creates an atom from raw bits.
	#[inline]
	const fn new(bits: u32) -> Self {
		Self { bits, _phantom: PhantomData }
	}

	/// Returns the raw bit representation.
	#[inline]
	pub const fn as_bits(self) -> u32 {
		self.bits
	}

	/// Returns true if this is a dynamic (runtime-interned) atom.
	#[inline]
	pub const fn is_dynamic(self) -> bool {
		self.bits & (1 << ATOM_DYNAMIC_BIT) != 0
	}

	/// Returns true if this is a static atom.
	#[inline]
	pub const fn is_static(self) -> bool {
		!self.is_dynamic()
	}
}

/// Combines static atoms with dynamic (runtime-interned) atoms.
///
/// Each `DynAtomRegistry<T>` should be registered as a singleton using the `register_atom_set!` macro. This ensures
///
/// # Example
///
/// ```rust
/// use css_lexer::{AtomSet, Atom, DynAtomRegistry, RegisteredAtomSet, register_atom_set};
/// use derive_atom_set::AtomSet as DeriveAtomSet;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, DeriveAtomSet)]
/// enum MyAtoms {
///     #[default]
///     _None,
///     Px,
/// }
///
/// // Register singleton
/// register_atom_set!(MyAtoms);
///
/// let atoms = MyAtoms::get_dyn_set();
///
/// // Work with typed atoms
/// let px: Atom<MyAtoms> = atoms.atom_from_str("px");
/// let custom: Atom<MyAtoms> = atoms.atom_from_str("custom-value");
///
/// assert!(px.is_static());
/// assert!(custom.is_dynamic());
/// ```
pub struct DynAtomRegistry<T: AtomSet> {
	static_atoms: T,
	str_to_bits_map: RwLock<FnvHashMap<&'static str, u32>>,
	bits_to_str_vec: RwLock<Vec<&'static str>>,
}

impl<T: AtomSet> DynAtomRegistry<T> {
	#[doc(hidden)]
	pub fn new() -> Self {
		Self {
			static_atoms: T::default(),
			str_to_bits_map: RwLock::new(FnvHashMap::default()),
			bits_to_str_vec: RwLock::new(Vec::new()),
		}
	}

	/// Converts a string to a typed atom.
	pub fn atom_from_str(&self, s: &str) -> Atom<T> {
		Atom::new(self.str_to_bits(s))
	}

	/// Converts raw bits to a typed atom.
	pub fn atom_from_bits(&self, bits: u32) -> Atom<T> {
		Atom::new(bits)
	}

	/// Converts an atom back to its string representation. Returns `None` for invalid atoms.
	pub fn atom_to_str(&self, atom: Atom<T>) -> Option<&'static str> {
		let bits = atom.as_bits();
		if DynAtomRegistry::<T>::is_dynamic(bits) { self.lookup(bits) } else { Some(T::from_bits(bits).to_str()) }
	}

	/// Interns a string, returning its dynamic atom bits.
	fn atomize(&self, s: &str) -> u32 {
		if let Some(&bits) = self.str_to_bits_map.read().unwrap().get(s) {
			return bits;
		}

		let mut str_to_bits = self.str_to_bits_map.write().unwrap();
		let mut bits_to_str = self.bits_to_str_vec.write().unwrap();

		if let Some(&bits) = str_to_bits.get(s) {
			return bits;
		}

		let id = bits_to_str.len() as u32;
		let bits = id | (1 << ATOM_DYNAMIC_BIT); // Set dynamic bit

		// Leak the string to get a 'static reference - this is an intern pool
		// where strings live for the entire program lifetime anyway due to static
		// lifetime of singleton instances.
		let static_str: &'static str = Box::leak(s.into());
		bits_to_str.push(static_str);
		str_to_bits.insert(static_str, bits);

		bits
	}

	/// Looks up a string by its dynamic atom bits.
	fn lookup(&self, bits: u32) -> Option<&'static str> {
		debug_assert!(Self::is_dynamic(bits), "lookup() called on static atom bits");
		let index = (bits & ((1 << ATOM_DYNAMIC_BIT) - 1)) as usize; // Mask off dynamic bit to get index
		self.bits_to_str_vec.read().unwrap().get(index).copied()
	}

	/// Returns true if the given bits represent a dynamic atom.
	#[inline]
	pub const fn is_dynamic(bits: u32) -> bool {
		bits & (1 << ATOM_DYNAMIC_BIT) != 0
	}
}

impl<T: AtomSet> Default for DynAtomRegistry<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: AtomSet + Debug> Debug for DynAtomRegistry<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		f.debug_struct("DynAtomRegistry").field("static_atoms", &self.static_atoms).finish_non_exhaustive()
	}
}

impl<T: AtomSet> DynAtomSet for DynAtomRegistry<T> {
	fn str_to_bits(&self, keyword: &str) -> u32 {
		let static_atom = T::from_str(keyword);
		let default_bits = T::default().as_bits();

		if static_atom.as_bits() != default_bits {
			let bits = static_atom.as_bits();
			debug_assert!(bits & (1 << ATOM_DYNAMIC_BIT) == 0, "Static atoms must have dynamic bit = 0");
			return bits;
		}

		let bits = self.atomize(keyword);
		debug_assert!(bits & (1 << ATOM_DYNAMIC_BIT) != 0, "Dynamic atoms must have dynamic bit = 1");
		bits
	}

	fn bits_to_str(&self, bits: u32) -> &'static str {
		if Self::is_dynamic(bits) {
			return self.lookup(bits).unwrap_or_else(|| T::from_bits(0).to_str());
		}
		T::from_bits(bits).to_str()
	}

	fn bits(&self) -> u32 {
		self.static_atoms.as_bits()
	}
}

/// Trait for atom types that have a registered singleton `DynAtomRegistry`.
pub trait RegisteredAtomSet: AtomSet + 'static {
	/// Returns the singleton instance of the `DynAtomRegistry` for this atom type.
	fn get_dyn_set() -> &'static DynAtomRegistry<Self>;
}

/// Registers a singleton `DynAtomRegistry<T>` for the given atom type.
///
/// This macro creates a static instance and implements the `RegisteredAtomSet` trait.
///
/// # Example
///
/// ```rust
/// use css_lexer::{AtomSet, RegisteredAtomSet, register_atom_set};
/// use derive_atom_set::AtomSet as DeriveAtomSet;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, DeriveAtomSet)]
/// enum MyAtoms {
///     #[default]
///     _None,
///     Foo,
/// }
///
/// register_atom_set!(MyAtoms);
///
/// // Now you can use the singleton
/// let atoms = MyAtoms::get_dyn_set();
/// let atom = atoms.atom_from_str("foo");
/// ```
#[macro_export]
macro_rules! register_atom_set {
	($atom_type:ty) => {
		static __ATOM_SET: ::std::sync::LazyLock<$crate::DynAtomRegistry<$atom_type>> =
			::std::sync::LazyLock::new(|| $crate::DynAtomRegistry::new());

		impl $crate::RegisteredAtomSet for $atom_type {
			#[inline]
			fn get_dyn_set() -> &'static $crate::DynAtomRegistry<Self> {
				&__ATOM_SET
			}
		}
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Default, Copy, Clone, PartialEq)]
	enum TestAtomSet {
		#[default]
		None,
		Px,
		Rem,
	}

	impl AtomSet for TestAtomSet {
		fn from_str(keyword: &str) -> Self {
			match keyword {
				"px" => Self::Px,
				"rem" => Self::Rem,
				_ => Self::None,
			}
		}

		fn to_str(self) -> &'static str {
			match self {
				Self::None => "",
				Self::Px => "px",
				Self::Rem => "rem",
			}
		}

		fn len(&self) -> u32 {
			self.to_str().len() as u32
		}

		fn from_bits(bits: u32) -> Self {
			match bits {
				0 => Self::None,
				1 => Self::Px,
				2 => Self::Rem,
				_ => Self::None,
			}
		}

		fn as_bits(&self) -> u32 {
			*self as u32
		}
	}

	#[test]
	fn test_static_atoms_have_dynamic_bit_zero() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		let px_bits = combined.str_to_bits("px");
		assert_eq!(px_bits & (1 << ATOM_DYNAMIC_BIT), 0, "Static atoms should have dynamic bit = 0");

		let rem_bits = combined.str_to_bits("rem");
		assert_eq!(rem_bits & (1 << ATOM_DYNAMIC_BIT), 0, "Static atoms should have dynamic bit = 0");
	}

	#[test]
	fn test_dynamic_atoms_have_dynamic_bit_one() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		let custom_bits = combined.str_to_bits("custom");
		assert_eq!(
			custom_bits & (1 << ATOM_DYNAMIC_BIT),
			1 << ATOM_DYNAMIC_BIT,
			"Dynamic atoms should have dynamic bit = 1"
		);
	}

	#[test]
	fn test_dynamic_atom_interning() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		let bits1 = combined.str_to_bits("custom");
		let bits2 = combined.str_to_bits("custom");

		assert_eq!(bits1, bits2, "Same string should atomize to same bits");
	}

	#[test]
	fn test_lookup() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		let custom_bits = combined.str_to_bits("my-custom-value");

		assert_eq!(combined.lookup(custom_bits), Some("my-custom-value"));
	}

	#[test]
	fn test_different_dynamic_atoms_get_different_bits() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		let bits1 = combined.str_to_bits("custom1");
		let bits2 = combined.str_to_bits("custom2");

		assert_ne!(bits1, bits2, "Different strings should get different bits");
		assert_eq!(bits1 & (1 << ATOM_DYNAMIC_BIT), 1 << ATOM_DYNAMIC_BIT);
		assert_eq!(bits2 & (1 << ATOM_DYNAMIC_BIT), 1 << ATOM_DYNAMIC_BIT);
	}

	#[test]
	fn test_typed_atoms() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		// Static atom
		let px_atom: Atom<TestAtomSet> = combined.atom_from_str("px");
		assert!(px_atom.is_static());
		assert!(!px_atom.is_dynamic());
		assert_eq!(combined.atom_to_str(px_atom), Some("px"));

		// Dynamic atom
		let custom_atom: Atom<TestAtomSet> = combined.atom_from_str("custom");
		assert!(custom_atom.is_dynamic());
		assert!(!custom_atom.is_static());
		assert_eq!(combined.atom_to_str(custom_atom), Some("custom"));

		// Atoms should not be equal
		assert_ne!(px_atom, custom_atom);

		// Same string should give same atom
		let custom_atom2 = combined.atom_from_str("custom");
		assert_eq!(custom_atom, custom_atom2);
	}

	#[test]
	fn test_atom_from_bits() {
		let combined = DynAtomRegistry::<TestAtomSet>::new();

		let px_atom = combined.atom_from_str("px");
		let px_bits = px_atom.as_bits();

		let px_atom2 = combined.atom_from_bits(px_bits);
		assert_eq!(px_atom, px_atom2);
		assert_eq!(combined.atom_to_str(px_atom2), Some("px"));
	}
}
