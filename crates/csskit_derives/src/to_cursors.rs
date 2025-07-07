use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Index};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			let steps: Vec<TokenStream> = fields
				.unnamed
				.into_iter()
				.enumerate()
				.map(|(i, _)| {
					let index = Index { index: i as u32, span: Span::call_site() };
					quote! {
						::css_parse::ToCursors::to_cursors(&self.#index, s);
					}
				})
				.collect();
			quote! { #(#steps)* }
		}

		Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
			let steps: Vec<TokenStream> = fields
				.named
				.into_iter()
				.map(|f| {
					let ident = f.ident.expect("Named field");
					quote! {
						::css_parse::ToCursors::to_cursors(&self.#ident, s);
					}
				})
				.collect();
			quote! { #(#steps)* }
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive ToCursors on this struct"),

		Data::Union(_) => err(ident.span(), "Cannot derive ToCursors on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps = vec![];
			for var in variants {
				let var_ident = var.ident;
				if var.fields.len() == 1 {
					steps.push(quote! {
						Self::#var_ident(val) => { ::css_parse::ToCursors::to_cursors(val, s); }
					});
				} else {
					let mut idents = vec![];
					let field_steps: Vec<_> = var
						.fields
						.into_iter()
						.enumerate()
						.map(|(i, _)| {
							let ident = format_ident!("v{}", i);
							idents.push(ident.clone());
							quote! { ::css_parse::ToCursors::to_cursors(#ident, s); }
						})
						.collect();
					steps.push(quote! {
						Self::#var_ident(#(#idents),*) => { #(#field_steps)* }
					});
				}
			}
			quote! {
				match self {
					#(#steps)*
				}
			}
		}
	};

	quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::ToCursors for #ident #impl_generics {
			fn to_cursors(&self, s: &mut impl ::css_parse::CursorSink) {
				use ::css_parse::ToCursors;
				#body
			}
		}
	}
}
