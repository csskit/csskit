//! Layout descriptors for struct or enum types with an inventory registry.
//!
//! [`TypeLayout`] is derivable and exposes a `const` [`TypeLayoutInfo`] holding a type's name, size, minimum alignment,
//! field offsets and enum variant discriminants.
//!
//! ```
//! use stable_type_layout::{Field, TypeLayout, TypeStructure};
//!
//! #[derive(TypeLayout)]
//! #[repr(C)]
//! struct Foo {
//!     a: u8,
//!     b: u32,
//! }
//!
//! const LAYOUT: stable_type_layout::TypeLayoutInfo = <Foo as TypeLayout>::TYPE_LAYOUT;
//! assert_eq!(LAYOUT.name, "Foo");
//! assert_eq!((LAYOUT.size, LAYOUT.align), (8, 4));
//! assert_eq!(
//!     LAYOUT.structure,
//!     TypeStructure::Struct { fields: &[Field { name: "a", offset: 0 }, Field { name: "b", offset: 4 }] }
//! );
//! ```
//!
//! Types can additionally [`register!`] themselves into a crate-wide [`inventory`] registry, which can be used for
//! snapshot testing, so a field, variant reorder, or a size change can be caught.

extern crate self as stable_type_layout;

pub use inventory;
pub use stable_type_layout_derive::TypeLayout;

/// A field's name and byte offset within its type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Field {
	pub name: &'static str,
	pub offset: usize,
}

/// An enum variant's name, declaration index and discriminant.
///
/// `discriminant` is the compiler-assigned tag value, recorded only for fieldless enums where it can be read on stable
/// via an `as` cast. Data-carrying variants record `None`; their `index` still pins declaration order, which is what a
/// `#[repr(C, uN)]` tag is derived from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Variant {
	pub name: &'static str,
	pub index: usize,
	pub discriminant: Option<i64>,
}

/// Whether a type is a struct, union or enum, and the fields or variants it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeStructure {
	Struct { fields: &'static [Field] },
	Union { fields: &'static [Field] },
	Enum { variants: &'static [Variant] },
}

/// The concrete memory layout of a type: its name, size, minimum alignment and
/// [`TypeStructure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeLayoutInfo {
	pub name: &'static str,
	pub size: usize,
	pub align: usize,
	pub structure: TypeStructure,
}

/// A type whose memory layout is known at compile time.
pub trait TypeLayout: Sized {
	const TYPE_LAYOUT: TypeLayoutInfo;
}

inventory::collect!(TypeLayoutInfo);

/// Registers a type's [`TypeLayoutInfo`] with the registry read by [`all`] and [`render`].
///
/// ```
/// use stable_type_layout::TypeLayout;
///
/// #[derive(TypeLayout)]
/// #[repr(C)]
/// struct Registered(u32);
///
/// stable_type_layout::register!(Registered);
/// assert!(stable_type_layout::all().iter().any(|info| info.name == "Registered"));
/// ```
#[macro_export]
macro_rules! register {
	($ty:ty) => {
		$crate::inventory::submit! { <$ty as $crate::TypeLayout>::TYPE_LAYOUT }
	};
}

/// Every registered [`TypeLayoutInfo`], sorted by type name (deterministic order for snapshotting).
pub fn all() -> Vec<&'static TypeLayoutInfo> {
	let mut infos: Vec<&'static TypeLayoutInfo> = inventory::iter::<TypeLayoutInfo>.into_iter().collect();
	infos.sort_by_key(|info| info.name);
	infos
}

/// Render every registered layout as a stable, human-readable string for snapshot assertions.
pub fn render() -> String {
	let mut out = String::new();
	for info in all() {
		out.push_str(info.name);
		out.push_str(&format!(": size={} align={}", info.size, info.align));
		let fields = match info.structure {
			TypeStructure::Struct { fields } | TypeStructure::Union { fields } => fields,
			TypeStructure::Enum { variants } => {
				for variant in variants {
					match variant.discriminant {
						Some(discriminant) => {
							out.push_str(&format!("\n    {}: {} = {}", variant.index, variant.name, discriminant))
						}
						None => out.push_str(&format!("\n    {}: {}", variant.index, variant.name)),
					}
				}
				&[]
			}
		};
		if fields.is_empty() {
			out.push('\n');
		} else {
			out.push_str(" {\n");
			for field in fields {
				out.push_str(&format!("    {}: {}\n", field.name, field.offset));
			}
			out.push_str("}\n");
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(TypeLayout)]
	#[repr(C)]
	struct SmallFirst {
		small: u8,
		large: u64,
	}

	#[derive(TypeLayout)]
	#[repr(C)]
	struct LargeFirst {
		large: u64,
		small: u8,
	}

	#[derive(TypeLayout)]
	#[repr(u8)]
	enum Fieldless {
		Zero,
		Seven = 7,
		Eight,
	}

	#[allow(dead_code)]
	#[derive(TypeLayout)]
	#[repr(C, u8)]
	enum DataCarrying {
		Unit,
		Payload(u32),
	}

	#[derive(TypeLayout)]
	#[repr(C)]
	struct Tuple<'a>(&'a str, u8);

	#[derive(TypeLayout)]
	#[repr(C)]
	struct Unit;

	register!(Fieldless);
	register!(Tuple<'static>);
	register!(Unit);

	fn fields_of(info: TypeLayoutInfo) -> &'static [Field] {
		match info.structure {
			TypeStructure::Struct { fields } | TypeStructure::Union { fields } => fields,
			TypeStructure::Enum { .. } => panic!("not a struct"),
		}
	}

	#[test]
	fn reorder_changes_field_offsets() {
		let small_first = fields_of(SmallFirst::TYPE_LAYOUT);
		let large_first = fields_of(LargeFirst::TYPE_LAYOUT);
		assert_eq!(small_first, &[Field { name: "small", offset: 0 }, Field { name: "large", offset: 8 }]);
		assert_eq!(large_first, &[Field { name: "large", offset: 0 }, Field { name: "small", offset: 8 }]);
	}

	#[test]
	fn records_explicit_and_implicit_discriminants() {
		assert_eq!(
			Fieldless::TYPE_LAYOUT.structure,
			TypeStructure::Enum {
				variants: &[
					Variant { name: "Zero", index: 0, discriminant: Some(0) },
					Variant { name: "Seven", index: 1, discriminant: Some(7) },
					Variant { name: "Eight", index: 2, discriminant: Some(8) },
				]
			}
		);
	}

	#[test]
	fn data_carrying_variants_pin_index_only() {
		assert_eq!(
			DataCarrying::TYPE_LAYOUT.structure,
			TypeStructure::Enum {
				variants: &[
					Variant { name: "Unit", index: 0, discriminant: None },
					Variant { name: "Payload", index: 1, discriminant: None },
				]
			}
		);
	}

	#[test]
	fn tuple_fields_are_named_by_index() {
		assert_eq!(fields_of(Tuple::TYPE_LAYOUT), &[Field { name: "0", offset: 0 }, Field { name: "1", offset: 16 }]);
	}

	#[test]
	fn renders_every_registered_type() {
		assert_eq!(
			render(),
			"Fieldless: size=1 align=1\n    0: Zero = 0\n    1: Seven = 7\n    2: Eight = 8\nTuple: size=24 align=8 {\n    0: 0\n    1: 16\n}\nUnit: size=0 align=1\n"
		);
	}
}
