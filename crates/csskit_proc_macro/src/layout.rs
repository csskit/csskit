use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericParam, Generics, Ident};

/// The number of lifetime parameters a node type carries, or `None` when it also
/// carries a type or const generic parameter.
///
/// A type-generic node has no single layout, so it cannot derive one `TypeLayout` or
/// self-register a single `TypeLayoutInfo`. Such types still receive their `#[repr]`;
/// their concrete instantiations are covered where they are monomorphised.
fn lifetimes(generics: &Generics) -> Option<usize> {
	let mut lifetimes = 0usize;
	for param in &generics.params {
		match param {
			GenericParam::Lifetime(_) => lifetimes += 1,
			GenericParam::Type(_) | GenericParam::Const(_) => return None,
		}
	}
	Some(lifetimes)
}

/// The `TypeLayout` derive a node type carries in test builds, or nothing when the
/// type is generic over a type/const parameter.
pub fn derive_attr(generics: &Generics) -> TokenStream {
	if lifetimes(generics).is_none() {
		return quote! {};
	}
	quote! { #[derive(::stable_type_layout::TypeLayout)] }
}

/// Registers a node type's layout with the crate's `layout_test` collector so its
/// size, field offsets and variant discriminants are snapshot tested. Emits nothing
/// when the type is generic over a type/const parameter.
pub fn registration(ident: &Ident, generics: &Generics) -> TokenStream {
	let Some(lifetimes) = lifetimes(generics) else {
		return quote! {};
	};
	let ty = if lifetimes == 0 {
		quote! { #ident }
	} else {
		let statics = std::iter::repeat_n(quote! { 'static }, lifetimes);
		quote! { #ident<#(#statics),*> }
	};
	quote! {
		#[cfg(test)]
		::stable_type_layout::register!(#ty);
	}
}

/// Parses a token stream of generated items and re-emits it, giving every struct and
/// enum its [`derive_attr`] and [`registration`]. Used by the `#[syntax]` generator,
/// whose bodies (main type + helper types) exist only as generated tokens.
pub fn annotate_items(items: &TokenStream) -> TokenStream {
	let file = match syn::parse2::<syn::File>(items.clone()) {
		Ok(file) => file,
		Err(err) => {
			let message = format!("generated items could not be parsed for layout registration: {err}");
			return quote! { ::core::compile_error!(#message); };
		}
	};
	let mut out = quote! {};
	for item in &file.items {
		let (ident, generics) = match item {
			syn::Item::Struct(s) => (&s.ident, &s.generics),
			syn::Item::Enum(e) => (&e.ident, &e.generics),
			_ => {
				out.extend(quote! { #item });
				continue;
			}
		};
		let attr = derive_attr(generics);
		let registration = registration(ident, generics);
		out.extend(quote! { #attr #item #registration });
	}
	out
}
