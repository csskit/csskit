use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
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
		impl<'a> ::css_parse::Peek<'a> for #ident #impl_generics {
			fn peek(p: &::css_parse::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				use ::css_parse::{Peek};
				#body
			}
		}
	}
}
