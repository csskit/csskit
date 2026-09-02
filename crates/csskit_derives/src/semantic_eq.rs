use crate::{FieldsExt, WhereCollector, attributes::extract_semantic_eq_skip, field_view::Arm};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, Result, parse_quote};

/// Two values are semantically equal when they are the same arm and every field which is not
/// skipped is semantically equal to its counterpart.
pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	if let Data::Struct(s) = &input.data
		&& matches!(s.fields, Fields::Unit)
	{
		return Err(Error::new(input.ident.span(), "Cannot derive SemanticEq on this struct"));
	}
	let arms = Arm::all(&input)?;

	let mut wc = WhereCollector::new();
	let matches: Vec<TokenStream> = arms
		.iter()
		.map(|arm| {
			let compared: Vec<bool> = arm.fields.iter().map(|field| !extract_semantic_eq_skip(&field.attrs)).collect();
			for (field, _) in arm.fields.iter().zip(&compared).filter(|(_, compared)| **compared) {
				wc.add(&field.ty);
			}
			let a = arm.pattern(|i, view| compared[i].then(|| format_ident!("a_{}", view.binding)));
			let b = arm.pattern(|i, view| compared[i].then(|| format_ident!("b_{}", view.binding)));
			let (a_bindings, b_bindings): (Vec<_>, Vec<_>) = arm
				.fields
				.views()
				.iter()
				.zip(&compared)
				.filter(|(_, compared)| **compared)
				.map(|(view, _)| (format_ident!("a_{}", view.binding), format_ident!("b_{}", view.binding)))
				.unzip();
			let equal = if a_bindings.is_empty() {
				quote! { true }
			} else {
				quote! { #(#a_bindings.semantic_eq(#b_bindings, source_text))&&* }
			};
			quote! { (#a, #b) => #equal, }
		})
		.collect();
	let fallback = (arms.len() > 1).then(|| quote! { _ => false, });

	let where_clause = wc.extend_where_clause(&input.generics, parse_quote! { ::css_parse::SemanticEq });
	let ident = &input.ident;
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::SemanticEq for #ident #type_generics #where_clause {
			fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
				use ::css_parse::SemanticEq;
				match (self, other) {
					#(#matches)*
					#fallback
				}
			}
		}
	})
}
