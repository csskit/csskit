use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, parse_quote};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &input.generics;
	let mut generic_with_alloc = generics.clone();
	let (impl_generics, type_generics, where_clause) = if generics.lifetimes().all(|l| l.lifetime.ident != "a") {
		generic_with_alloc.params.insert(0, parse_quote!('a));
		let (impl_generics, _, _) = generic_with_alloc.split_for_impl();
		let (_, type_generics, where_clause) = generics.split_for_impl();
		(impl_generics, type_generics, where_clause)
	} else {
		generics.split_for_impl()
	};
	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive Peek on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let ty = &fields.iter().next().unwrap().ty;
			quote! { <#ty>::peek(p, c) }
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let ty: Vec<_> = variants.iter().map(|variant| variant.fields.iter().next()).dedup().collect();
			quote! { #(<#ty>::peek(p, c))||* }
		}
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Peek<'a> for #ident #type_generics #where_clause {
			fn peek(p: &::css_parse::Parser<'a>, c: ::css_parse::Cursor) -> bool {
				use ::css_parse::{Peek};
				#body
			}
		}
	}
}
