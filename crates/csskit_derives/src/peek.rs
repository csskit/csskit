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
			let ty = &fields.unnamed.first().unwrap().ty;
			quote! { <#ty>::peek(p, c) }
		}

		Data::Struct(_) => err(ident.span(), "Cannot derive Peek on a struct with named fields"),

		Data::Union(_) => err(ident.span(), "Cannot derive Peek on a Union"),

		Data::Enum(DataEnum { variants, .. }) => {
			let mut steps = vec![];
			let mut first = true;
			for var in variants {
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
									<#field_ty>::peek(p, c)
								});
							} else {
								steps.push(quote! {
									|| <#field_ty>::peek(p, c)
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
			quote! { #(#steps)* }
		}
	};
	quote! {
		#[automatically_derived]
		impl<'a> #impl_generics ::css_parse::Peek<'a> for #ident #impl_generics {
			fn peek(p: &::css_parse::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
				use ::css_parse::{Peek};
				#body
			}
		}
	}
}
