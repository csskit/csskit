use itertools::{Itertools, Position};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Type};

use crate::err;

trait ToVarsAndTypes {
	fn to_vars_and_types(&self) -> (Vec<Ident>, Vec<Type>);
}

impl ToVarsAndTypes for Fields {
	fn to_vars_and_types(&self) -> (Vec<Ident>, Vec<Type>) {
		self.into_iter()
			.enumerate()
			.map(|(i, field)| (field.ident.clone().unwrap_or_else(|| format_ident!("f{}", i)), field.ty.clone()))
			.collect::<Vec<_>>()
			.into_iter()
			.unzip()
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive Parse on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let members = fields.members();
			let (vars, types) = fields.to_vars_and_types();
			quote! {
				#( let #vars = p.parse::<#types>()?; )*
				Ok(Self { #(#members: #vars),* })
			}
		}

		Data::Enum(DataEnum { variants, .. }) => variants
			.iter()
			.with_position()
			.map(|(position, variant)| {
				let variant_ident = &variant.ident;
				let members = variant.fields.members();
				let (vars, types) = variant.fields.to_vars_and_types();
				let first_type = types.first();
				let step = quote! {
					#( let #vars = p.parse::<#types>()?; )*
					Ok(Self::#variant_ident { #(#members: #vars),* })
				};
				match position {
					Position::First => quote! { if p.peek::<#first_type>() { #step } },
					Position::Last => quote! { else { #step } },
					Position::Only => step,
					Position::Middle => quote! { else if p.peek::<#first_type>() { #step } },
				}
			})
			.collect(),
	};
	quote! {
		#[automatically_derived]
		impl<'a> ::css_parse::Parse<'a> for #ident #impl_generics {
			fn parse(p: &mut css_parse::Parser<'a>) -> css_parse::Result<Self> {
				use css_parse::{Parse};
				#body
			}
		}
	}
}
