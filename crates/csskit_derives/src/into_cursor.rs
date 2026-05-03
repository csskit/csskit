use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
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
			fn from(value: #ident) -> ::css_parse::Cursor {
				#body
			}
		}

		#[automatically_derived]
		impl #impl_generics From<#ident #type_generics> for ::css_parse::Token #where_clause {
			fn from(value: #ident) -> ::css_parse::Token {
				Cursor::from(value).token()
			}
		}

		#[automatically_derived]
		impl #impl_generics ::css_parse::ToSpan for #ident #type_generics #where_clause {
			fn to_span(&self) -> ::css_parse::Span {
				Cursor::from(*self).span()
			}
		}

		#[automatically_derived]
		impl #impl_generics ::css_parse::SemanticEq for #ident #type_generics #where_clause {
			fn semantic_eq(&self, other: &Self) -> bool  {
				Cursor::from(*self).semantic_eq(&Cursor::from(*other))
			}
		}
	})
}
