use crate::{FieldsExt, WhereCollector};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Result, parse_quote};

/// Indexes of the fields which can bound a node's span: the leading run of optional fields up to and
/// including the first field which is always present, and the same run taken from the end.
///
/// Span addition takes the lowest start and the highest end, and absorbs `Span::DUMMY`, which is what
/// an absent field gives. Adding these fields therefore gives the whole node's span without visiting
/// the fields between them.
fn bounding(optional: &[bool]) -> impl Iterator<Item = usize> {
	let head = optional.iter().position(|optional| !optional).map_or(optional.len(), |i| i + 1);
	let tail = optional.iter().rposition(|optional| !optional).unwrap_or(0).max(head);
	(0..head).chain(tail..optional.len())
}

/// Builds the expression for the span a node covers, from the accessors for its bounding fields.
fn span_of(accesses: &[TokenStream]) -> TokenStream {
	if accesses.is_empty() {
		quote! { Span::DUMMY }
	} else {
		quote! { #(ToSpan::to_span(#accesses))+* }
	}
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = input.generics.clone();
	let (impl_generics, type_generics, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive ToSpan on a Union")),

		Data::Struct(DataStruct { fields, .. }) => {
			for syn_field in fields.iter() {
				where_collector.add(&syn_field.ty);
			}
			let views = fields.views();
			let optional: Vec<bool> = views.iter().map(|view| view.is_option).collect();
			let accesses: Vec<_> = bounding(&optional)
				.map(|i| {
					let member = &views[i].member;
					quote! { &self.#member }
				})
				.collect();
			span_of(&accesses)
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let steps: TokenStream = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					for syn_field in variant.fields.iter() {
						where_collector.add(&syn_field.ty);
					}
					let views = variant.fields.views();
					let optional: Vec<bool> = views.iter().map(|view| view.is_option).collect();
					let bound: Vec<usize> = bounding(&optional).collect();
					let accesses: Vec<_> = bound
						.iter()
						.map(|&i| {
							let binding = &views[i].binding;
							quote! { #binding }
						})
						.collect();
					let body = span_of(&accesses);
					match &variant.fields {
						Fields::Unit => quote! { #ident::#variant_ident => #body, },
						Fields::Named(_) => {
							let bindings = bound.iter().map(|&i| &views[i].binding);
							quote! { #ident::#variant_ident { #(#bindings,)* .. } => #body, }
						}
						Fields::Unnamed(_) => {
							let bindings = views.iter().enumerate().map(|(i, view)| {
								let binding = &view.binding;
								if bound.contains(&i) {
									quote! { #binding }
								} else {
									quote! { _ }
								}
							});
							quote! { #ident::#variant_ident(#(#bindings),*) => #body, }
						}
					}
				})
				.collect();
			quote! { match self { #steps } }
		}
	};

	let where_clause = where_collector.extend_where_clause(&generics, parse_quote! { ::css_parse::ToSpan });

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::ToSpan for #ident #type_generics #where_clause {
			fn to_span(&self) -> ::css_parse::Span {
				use ::css_parse::{Span, ToSpan};
				#body
			}
		}
	})
}
