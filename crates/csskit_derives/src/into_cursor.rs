use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			if fields.unnamed.len() != 1 {
				return err(ident.span(), "Cannot derive Into<Cursor> for a struct with many fields");
			} else {
				quote! { value.0.into() }
			}
		}
		Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => {
			if fields.named.len() != 1 {
				err(ident.span(), "Cannot derive Into<Cursor> for a struct with many fields")
			} else {
				let ident = fields.named.first().unwrap().ident.clone().unwrap();
				quote! { value.#ident.into() }
			}
		}
		Data::Struct(_) => err(ident.span(), "Cannot derive Into<Cursor> on this struct"),
		Data::Union(_) => err(ident.span(), "Cannot derive Into<Cursor> on a Union"),
		Data::Enum(DataEnum { variants, .. }) => {
			let steps: Vec<_> = variants
				.iter()
				.map(|variant| {
					if variant.fields.len() != 1 {
						err(ident.span(), "Cannot derive Into<Cursor> for an enum variant with many fields")
					} else {
						let variant = &variant.ident;
						quote! { #ident::#variant(value) => { value.into() } }
					}
				})
				.collect();
			quote! {
				match value {
					#(#steps)*
				}
			}
		}
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics From<#ident #impl_generics> for ::css_lexer::Cursor {
			fn from(value: #ident) -> ::css_lexer::Cursor {
				#body
			}
		}

		#[automatically_derived]
		impl #impl_generics From<#ident #impl_generics> for ::css_lexer::Token {
			fn from(value: #ident) -> ::css_lexer::Token {
				Into::<::css_lexer::Cursor>::into(value).token()
			}
		}

		#[automatically_derived]
		impl #impl_generics From<&#ident #impl_generics> for ::css_lexer::Span {
			fn from(value: &#ident) -> ::css_lexer::Span {
				Into::<::css_lexer::Cursor>::into(*value).into()
			}
		}

	}
}
