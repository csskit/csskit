use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Result, parse_quote};

use crate::WhereCollector;

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let ident = input.ident;
	let generics = &input.generics;
	let (impl_generics, type_generics, _) = generics.split_for_impl();

	let mut wc = WhereCollector::new();
	match &input.data {
		Data::Struct(DataStruct { fields, .. }) => {
			for field in fields {
				wc.add(&field.ty);
			}
		}
		Data::Enum(DataEnum { variants, .. }) => {
			for variant in variants {
				for field in &variant.fields {
					wc.add(&field.ty);
				}
			}
		}
		Data::Union(_) => {}
	}
	let where_clause = wc.extend_where_clause(generics, parse_quote! { Into<::css_parse::Cursor> });

	let body = match input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive Into<Cursor> on a Union")),

		Data::Struct(DataStruct { fields, .. }) => {
			if fields.len() != 1 {
				return Err(Error::new(ident.span(), "Cannot derive Into<Cursor> for a struct with many fields"));
			} else {
				let member = fields.members().next().expect("len checked");
				quote! { value.#member.into() }
			}
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps: Vec<TokenStream> = Vec::new();
			for variant in &variants {
				if variant.fields.len() != 1 {
					return Err(Error::new(
						variant.ident.span(),
						"Cannot derive Into<Cursor> for an enum variant with none or many fields",
					));
				}
				let variant_ident = &variant.ident;
				steps.push(quote! { #ident::#variant_ident(c) => c.into(), });
			}
			quote! {
				match value {
					#(#steps)*
				}
			}
		}
	};

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics From<#ident #type_generics> for ::css_parse::Cursor #where_clause {
			fn from(value: #ident #type_generics) -> ::css_parse::Cursor {
				#body
			}
		}

		#[automatically_derived]
		impl #impl_generics From<#ident #type_generics> for ::css_parse::Token #where_clause {
			fn from(value: #ident #type_generics) -> ::css_parse::Token {
				::css_parse::Cursor::from(value).token()
			}
		}
	})
}
