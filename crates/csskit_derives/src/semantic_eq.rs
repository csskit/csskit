use crate::{WhereCollector, err};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Index, parse_quote};

pub fn derive(input: DeriveInput) -> TokenStream {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = input.generics.clone();
	let (impl_generics, type_generics, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			let steps: Vec<TokenStream> = fields
				.unnamed
				.into_iter()
				.enumerate()
				.map(|(i, field)| {
					let index = Index { index: i as u32, span: Span::call_site() };
					where_collector.add(&field.ty);
					quote! {
						self.#index.semantic_eq(&other.#index)
					}
				})
				.collect();
			quote! { #(#steps)&&* }
		}

		Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
			let steps: Vec<TokenStream> = fields
				.named
				.into_iter()
				.map(|f| {
					let ident = f.ident.expect("Named field");
					where_collector.add(&f.ty);
					quote! {
						self.#ident.semantic_eq(&other.#ident)
					}
				})
				.collect();
			quote! { #(#steps)&&* }
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive SemanticEq on this struct"),

		Data::Union(_) => err(ident.span(), "Cannot derive SemanticEq on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps = vec![];
			for var in variants {
				let var_ident = var.ident;
				match var.fields {
					Fields::Named(fields) => {
						let a_idents: Vec<_> =
							fields.named.iter().map(|f| format_ident!("a_{}", f.ident.as_ref().unwrap())).collect();
						let b_idents: Vec<_> =
							fields.named.iter().map(|f| format_ident!("b_{}", f.ident.as_ref().unwrap())).collect();
						let field_names: Vec<_> =
							fields.named.iter().map(|f| f.ident.as_ref().unwrap().clone()).collect();
						let field_steps: Vec<_> = fields
							.named
							.iter()
							.zip(a_idents.iter().zip(b_idents.iter()))
							.map(|(field, (a_ident, b_ident))| {
								where_collector.add(&field.ty);
								quote! { #a_ident.semantic_eq(&#b_ident) }
							})
							.collect();
						let a_pats =
							field_names.iter().zip(a_idents.iter()).map(|(fname, aname)| quote! { #fname: #aname });
						let b_pats =
							field_names.iter().zip(b_idents.iter()).map(|(fname, bname)| quote! { #fname: #bname });
						let body = if field_steps.is_empty() {
							quote! { true }
						} else {
							quote! { #(#field_steps)&&* }
						};
						steps.push(quote! {
							(Self::#var_ident { #(#a_pats),* }, Self::#var_ident { #(#b_pats),* }) => { #body }
						});
					}
					_ => {
						let mut a_idents = vec![];
						let mut b_idents = vec![];
						let field_steps: Vec<_> = var
							.fields
							.into_iter()
							.enumerate()
							.map(|(i, field)| {
								where_collector.add(&field.ty);
								let a_ident = format_ident!("a{}", i);
								a_idents.push(a_ident.clone());
								let b_ident = format_ident!("b{}", i);
								b_idents.push(b_ident.clone());
								quote! { #a_ident.semantic_eq(&#b_ident) }
							})
							.collect();
						steps.push(quote! {
							(Self::#var_ident(#(#a_idents),*), Self::#var_ident(#(#b_idents),*)) => { #(#field_steps)&&* }
						});
					}
				}
			}
			quote! {
				match (self, other) {
					#(#steps),*
					_ => false,
				}
			}
		}
	};

	let mut generics = input.generics.clone();
	let where_clause = where_collector.extend_where_clause(&mut generics, parse_quote! { ::css_parse::SemanticEq });

	quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::SemanticEq for #ident #type_generics #where_clause {
			fn semantic_eq(&self, other: &Self) -> bool {
				use ::css_parse::SemanticEq;
				#body
			}
		}
	}
}
