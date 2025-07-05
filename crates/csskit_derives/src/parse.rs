use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsUnnamed, spanned::Spanned};

use crate::err;

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Struct(DataStruct { fields: Fields::Unnamed(fields), .. }) => {
			if fields.unnamed.len() != 1 {
				err(ident.span(), "Cannot derive Parse on a struct with multiple unnamed fields")
			} else {
				let ty = &fields.unnamed.first().unwrap().ty;
				quote! { p.parse::<#ty>().map(Self) }
			}
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive Parse on a struct with named fields"),

		Data::Union(_) => err(ident.span(), "Cannot derive Parse on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps = vec![];
			let mut first = true;
			for var in variants {
				let var_ident = var.ident;
				match var.fields {
					Fields::Unit => {
						steps.push(err(ident.span(), "Cannot derive Parse on a Field Unit Variant"));
					}
					Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
						if unnamed.len() != 1 {
							steps
								.push(err(ident.span(), "Cannot derive Parse on a struct with multiple unnamed fields"))
						} else {
							let field = unnamed.first().unwrap();
							let field_ty = &field.ty;
							if first {
								steps.push(quote! {
									p.parse_if_peek::<#field_ty>().ok().flatten().map(Self::#var_ident)
								});
							} else {
								steps.push(quote! {
									.or_else(|| p.parse_if_peek::<#field_ty>().ok().flatten().map(Self::#var_ident))
								});
							}
						}
					}
					Fields::Named(_) => {
						steps.push(err(var.fields.span(), "Cannot derive on Parse on a Named Field Variant"));
					}
				}
				first = false;
			}
			quote! {
				#(#steps)*
					.ok_or_else(|| {
						let c = p.next();
						::css_parse::diagnostics::Unexpected(c.into(), c.into()).into()
					})
			}
		}
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
