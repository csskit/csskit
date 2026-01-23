use crate::{
	WhereCollector,
	attributes::{Atom, extract_atom},
	err,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Type, parse_quote};

fn generate_field_peek(ty: &Type, atom: &Option<Atom>, where_collector: &mut WhereCollector) -> TokenStream {
	where_collector.add(ty);
	if let Some(atom) = atom {
		// For atom fields, check both the type AND the atom
		let atom_path = atom.path();
		quote! { (<#ty>::peek(p, c) && p.equals_atom(c.into(), &#atom_path)) }
	} else {
		quote! { <#ty>::peek(p, c) }
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = &input.generics;
	let mut generic_with_alloc = generics.clone();
	let (impl_generics, type_generics, _) = if generics.lifetimes().all(|l| l.lifetime.ident != "a") {
		generic_with_alloc.params.insert(0, parse_quote!('a));
		let (impl_generics, _, where_clause) = generic_with_alloc.split_for_impl();
		let (_, type_generics, _) = generics.split_for_impl();
		(impl_generics, type_generics, where_clause)
	} else {
		// If 'a lifetime already exists, we already added the bounds to generic_with_alloc
		generic_with_alloc.split_for_impl()
	};
	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive Peek on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let field = fields.iter().next().unwrap();
			let ty = match &field.ty {
				Type::Reference(refty) => refty.elem.as_ref(),
				ty => ty,
			};
			let atom = extract_atom(&field.attrs);
			generate_field_peek(ty, &atom, &mut where_collector)
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
					atom = atom.or_else(|| extract_atom(&field.attrs));
					Some(generate_field_peek(ty, &atom, &mut where_collector))
				} else {
					None
				}
			});
			quote! { #(#type_checks)||* }
		}
	};

	let mut generics = input.generics.clone();
	let where_clause = where_collector.extend_where_clause(&mut generics, parse_quote! { ::css_parse::Peek<'a> });

	quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Peek<'a> for #ident #type_generics #where_clause {
			fn peek<I>(p: &::css_parse::Parser<'a, I>, c: ::css_parse::Cursor) -> bool
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
			{
				use ::css_parse::{Peek};
				#body
			}
		}
	}
}
