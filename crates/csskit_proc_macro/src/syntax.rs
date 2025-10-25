use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Meta, Token};

use crate::generate::*;
use css_value_definition_parser::*;

fn has_derive_of(attrs: &[Attribute], name: &'static str) -> bool {
	for attr in attrs {
		if attr.path().is_ident("derive") {
			let nested = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).unwrap_or_default();
			for meta in nested {
				match meta {
					Meta::Path(path)
						if path.is_ident(name) || path.segments.last().map(|s| s.ident == name).unwrap_or(false) =>
					{
						return true;
					}
					_ => {}
				}
			}
		}
	}
	false
}

pub fn generate(defs: Def, ast: DeriveInput) -> TokenStream {
	let has_a_lifetime = ast.generics.lifetimes().any(|l| l.lifetime.ident == "a");
	if !has_a_lifetime && defs.requires_allocator_lifetime() {
		return Error::new(ast.ident.span(), "this object needs the <'a> lifetime but it didn't have it. Add it")
			.into_compile_error();
	}
	let vis = &ast.vis;
	let attrs = &ast.attrs;
	let ident = &ast.ident;
	match &ast.data {
		Data::Enum(DataEnum { variants, .. }) => {
			if !variants.is_empty() {
				return Error::new(ident.span(), "enum must be empty").into_compile_error();
			}
			if !defs.generated_data_type().is_enum() {
				return Error::new(ident.span(), "wrong structure for this syntax, please redefine as a Struct")
					.into_compile_error();
			}
		}
		Data::Struct(DataStruct { fields, .. }) => {
			if !fields.is_empty() {
				return Error::new(ident.span(), "struct must be empty").into_compile_error();
			}
			if !defs.generated_data_type().is_struct() {
				return Error::new(ident.span(), "wrong structure for this syntax, please redefine as an Enum")
					.into_compile_error();
			}
		}
		Data::Union(_) => {
			return Error::new(ident.span(), "cannot create from_syntax on Union").into_compile_error();
		}
	}
	let derives_visitable = has_derive_of(attrs, "Visitable");
	let derives_parse = has_derive_of(attrs, "Parse");
	let additonal_defs = defs.generate_additional_types(vis, ident, &ast.generics);
	let def = defs.generate_definition(vis, ident, &ast.generics, derives_parse, derives_visitable);
	quote! {
		#additonal_defs

		#(#attrs)*
		#def
	}
}
