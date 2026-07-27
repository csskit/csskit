use proc_macro2::TokenStream;
use quote::quote;

/// The structural shape of a node type, which determines its `#[repr(...)]`.
///
/// This is the single source of truth shared by the `#[syntax]` generator and the
/// `#[node]` attribute macro so generated and hand-written node types cannot drift.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape {
	/// A struct: `#[repr(C)]`.
	Struct,
	/// A fieldless (C-like) enum: `#[repr(uN)]`. `wide` forces `u32` when the enum
	/// carries explicit discriminant values (whose magnitude may exceed the width
	/// implied by the variant count).
	FieldlessEnum { variants: usize, wide: bool },
	/// A data-carrying enum: `#[repr(C, uN)]`.
	DataEnum { variants: usize },
}

/// The tag width for an enum with `variants` variants: `u8` if it fits, else `u16`.
///
/// A `u8` discriminant can hold up to 256 distinct values (`0..=255`).
fn tag_ty(variants: usize) -> TokenStream {
	if variants <= 256 {
		quote! { u8 }
	} else {
		quote! { u16 }
	}
}

/// Maps a node's [`Shape`] to the `#[repr(...)]` attribute it must carry so the
/// generated JS deserialiser reads a stable, declaration-order layout.
pub fn shape_to_repr(shape: Shape) -> TokenStream {
	match shape {
		Shape::Struct => quote! { #[repr(C)] },
		Shape::FieldlessEnum { variants, wide } => {
			let tag = if wide {
				quote! { u32 }
			} else {
				tag_ty(variants)
			};
			quote! { #[repr(#tag)] }
		}
		Shape::DataEnum { variants } => {
			let tag = tag_ty(variants);
			quote! { #[repr(C, #tag)] }
		}
	}
}
