use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, GenericParam};

/// Expands `#[node]` on a hand-bodied struct/enum: re-emits it unchanged and, in
/// test builds only, registers its `size_of` with the crate's `layout_test`
/// collector so the size tripwire snapshot catches accidental size regressions.
///
/// `#[node]` injects no `#[repr]` and carries no runtime cost: the registration
/// is `#[cfg(test)]`, so non-test builds see the type verbatim.
pub fn generate(ast: DeriveInput) -> TokenStream {
	let name = ast.ident.to_string();
	let registration = match concrete_ty(&ast) {
		Some(ty) => quote! {
			#[cfg(test)]
			::inventory::submit! {
				crate::layout_test::LayoutInfo { name: #name, size: ::core::mem::size_of::<#ty>() }
			}
		},
		None => quote! {},
	};
	quote! {
		#ast
		#registration
	}
}

/// The concrete type reference used in `size_of`, substituting `'static` for
/// every lifetime. Returns `None` when the type is generic over a type or const
/// parameter, since it has no single size to record.
fn concrete_ty(ast: &DeriveInput) -> Option<TokenStream> {
	let ident = &ast.ident;
	let mut lifetimes = 0usize;
	for param in &ast.generics.params {
		match param {
			GenericParam::Lifetime(_) => lifetimes += 1,
			_ => return None,
		}
	}
	if lifetimes == 0 {
		Some(quote! { #ident })
	} else {
		let statics = (0..lifetimes).map(|_| quote! { 'static });
		Some(quote! { #ident<#(#statics),*> })
	}
}
