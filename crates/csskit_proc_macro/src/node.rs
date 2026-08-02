use crate::repr::{Shape, shape_to_repr};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Expands `#[node]` on a hand-bodied struct/enum, adding the appropriate `#[repr(...)]` tag and registering its
/// layout with the crate's `layout_test` collector so the sizes are snapshot tested.
pub fn generate(ast: DeriveInput) -> TokenStream {
	let shape = match &ast.data {
		Data::Struct(_) | Data::Union(_) => Shape::Struct,
		Data::Enum(data) => {
			let variants = data.variants.len();
			if data.variants.iter().all(|v| matches!(v.fields, Fields::Unit)) {
				let wide = data.variants.iter().any(|v| v.discriminant.is_some());
				Shape::FieldlessEnum { variants, wide }
			} else {
				Shape::DataEnum { variants }
			}
		}
	};
	let repr = shape_to_repr(shape);
	let derive_attr = crate::layout::derive_attr(&ast.generics);
	let registration = crate::layout::registration(&ast.ident, &ast.generics);
	quote! {
		#repr
		#derive_attr
		#ast
		#registration
	}
}
