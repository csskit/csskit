//! Derive macro for `stable_type_layout::TypeLayout`.
//!
//! Use it through the [`stable-type-layout`](https://docs.rs/stable-type-layout) crate, which re-exports this macro
//! alongside the trait it implements.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, Index, parse_macro_input};

/// Implements `TypeLayout`, recording the type's size, alignment, field offsets and enum variant discriminants as a
/// `const`.
///
/// Structs and unions record every field's name and byte offset. Enums record every variant's name and declaration
/// index, plus its discriminant when the enum is fieldless. Lifetime, type and const parameters are carried onto the
/// impl, so a generic type describes whichever instantiation is asked for.
///
/// See the `stable_type_layout` crate docs for examples; this crate cannot depend on it to run one here.
#[proc_macro_derive(TypeLayout)]
pub fn derive_type_layout(input: TokenStream) -> TokenStream {
	let ast = parse_macro_input!(input as DeriveInput);
	let ident = &ast.ident;
	let name = ident.to_string();
	let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
	let structure = match &ast.data {
		Data::Struct(data) => {
			let fields = fields(&data.fields);
			quote! { ::stable_type_layout::TypeStructure::Struct { fields: &[#(#fields),*] } }
		}
		Data::Union(data) => {
			let fields = data.fields.named.iter().map(|f| named_field(f.ident.as_ref().unwrap()));
			quote! { ::stable_type_layout::TypeStructure::Union { fields: &[#(#fields),*] } }
		}
		Data::Enum(data) => {
			// A fieldless enum is castable, so its real compiler-assigned tag value can be recorded. A data-carrying variant
			// is not, and only its index is pinned.
			let fieldless = data.variants.iter().all(|v| matches!(v.fields, Fields::Unit));
			let variants = data.variants.iter().enumerate().map(|(index, variant)| {
				let variant_ident = &variant.ident;
				let variant_name = variant_ident.to_string();
				let discriminant = if fieldless {
					quote! { ::core::option::Option::Some(Self::#variant_ident as i64) }
				} else {
					quote! { ::core::option::Option::None }
				};
				quote! {
					::stable_type_layout::Variant {
						name: #variant_name,
						index: #index,
						discriminant: #discriminant,
					}
				}
			});
			quote! { ::stable_type_layout::TypeStructure::Enum { variants: &[#(#variants),*] } }
		}
	};
	quote! {
		impl #impl_generics ::stable_type_layout::TypeLayout for #ident #ty_generics #where_clause {
			const TYPE_LAYOUT: ::stable_type_layout::TypeLayoutInfo = ::stable_type_layout::TypeLayoutInfo {
				name: #name,
				size: ::core::mem::size_of::<Self>(),
				align: ::core::mem::align_of::<Self>(),
				structure: #structure,
			};
		}
	}
	.into()
}

fn fields(fields: &Fields) -> Vec<TokenStream2> {
	match fields {
		Fields::Named(named) => named.named.iter().map(|f| named_field(f.ident.as_ref().unwrap())).collect(),
		Fields::Unnamed(unnamed) => (0..unnamed.unnamed.len())
			.map(|i| {
				let index = Index::from(i);
				let name = i.to_string();
				quote! { ::stable_type_layout::Field { name: #name, offset: ::core::mem::offset_of!(Self, #index) } }
			})
			.collect(),
		Fields::Unit => vec![],
	}
}

fn named_field(ident: &Ident) -> TokenStream2 {
	let name = ident.to_string();
	quote! { ::stable_type_layout::Field { name: #name, offset: ::core::mem::offset_of!(Self, #ident) } }
}
