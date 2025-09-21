use crate::{
	attributes::{Atom, extract_atom, extract_in_range},
	err,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, ExprRange, Type, parse_quote};

fn generate_range_check(range_expr: &ExprRange) -> TokenStream {
	let start = &range_expr.start;
	let end = &range_expr.end;
	match (start, end) {
		// 1..=10 (inclusive end)
		(Some(start), Some(end)) => {
			quote! { (#start..=#end).contains(&c.token().value()) }
		}
		(Some(start), None) => {
			quote! { #start <= c.token().value() }
		}
		(None, Some(end)) => {
			quote! { c.token().value() <= #end }
		}
		// .. (full range) - no validation needed
		(None, None) => quote! { true },
	}
}

fn generate_field_peek(ty: &Type, in_range: &Option<ExprRange>, atom: &Option<Atom>) -> TokenStream {
	let base_check = if let Some(atom) = atom {
		// For atom fields, check both the type AND the atom
		let atom_path = atom.path();
		quote! { (<#ty>::peek(p, c) && p.equals_atom(c.into(), &#atom_path)) }
	} else {
		quote! { <#ty>::peek(p, c) }
	};

	if let Some(range) = in_range {
		let range_check = generate_range_check(range);
		quote! { (#base_check && #range_check) }
	} else {
		base_check
	}
}

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
			let field = fields.iter().next().unwrap();
			let ty = match &field.ty {
				Type::Reference(refty) => refty.elem.as_ref(),
				ty => ty,
			};
			let in_range = extract_in_range(&field.attrs);
			let atom = extract_atom(&field.attrs);
			generate_field_peek(ty, &in_range, &atom)
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let type_checks = variants.iter().filter_map(|variant| {
				// Skip variants that have atoms - they'll be handled by atom matching
				let mut atom = extract_atom(&variant.attrs);
				if let Some(field) = variant.fields.iter().next() {
					let ty = match &field.ty {
						Type::Reference(refty) => refty.elem.as_ref(),
						ty => ty,
					};
					let in_range = extract_in_range(&field.attrs);
					atom = atom.or_else(|| extract_atom(&field.attrs));
					Some(generate_field_peek(ty, &in_range, &atom))
				} else {
					None
				}
			});
			quote! { #(#type_checks)||* }
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
