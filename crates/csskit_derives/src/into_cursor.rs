use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive Into<Cursor> on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			if fields.len() != 1 {
				return err(ident.span(), "Cannot derive Into<Cursor> for a struct with many fields");
			} else {
				let member = fields.members().next().unwrap();
				quote! { value.#member.into() }
			}
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let steps: TokenStream = variants
				.iter()
				.map(|variant| {
					if variant.fields.len() != 1 {
						err(ident.span(), "Cannot derive Into<Cursor> for an enum variant with none or many fields")
					} else {
						let variant = &variant.ident;
						quote! { #ident::#variant(c) => c.into(), }
					}
				})
				.collect();
			quote! {
				match value {
					#steps
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
