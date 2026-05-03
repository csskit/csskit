use crate::{
	FieldsExt, WhereCollector,
	attributes::{Atom, extract_atom},
	ensure_lifetime_a,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Result, Type, parse_quote};

fn generate_field_peek(ty: &Type, atom: &Option<Atom>, where_collector: &mut WhereCollector) -> TokenStream {
	where_collector.add(ty);
	if let Some(atom) = atom {
		let atom_path = atom.path();
		quote! { (<#ty>::peek(p, c) && p.equals_atom(c.into(), &#atom_path)) }
	} else {
		quote! { <#ty>::peek(p, c) }
	}
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = &input.generics;
	let generic_with_a = ensure_lifetime_a(generics);
	let (impl_generics, _, _) = generic_with_a.split_for_impl();
	let (_, type_generics, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive Peek on a Union")),

		Data::Struct(DataStruct { fields, .. }) => {
			let mut checks: Vec<TokenStream> = vec![];
			for (view, syn_field) in fields.views().into_iter().zip(fields.iter()) {
				let atom = extract_atom(&syn_field.attrs)?;
				checks.push(generate_field_peek(view.ty, &atom, &mut where_collector));
				if !view.is_option {
					break;
				}
			}
			quote! { #(#checks)||* }
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let mut type_checks: Vec<TokenStream> = vec![];
			for variant in variants.iter() {
				let views = variant.fields.views();
				let Some((view, syn_field)) = views.first().zip(variant.fields.iter().next()) else {
					continue;
				};
				let atom = match extract_atom(&variant.attrs)? {
					Some(a) => Some(a),
					None => extract_atom(&syn_field.attrs)?,
				};
				// Use raw ty (including Option<T> if present) to preserve original behaviour.
				type_checks.push(generate_field_peek(view.ty, &atom, &mut where_collector));
			}
			quote! { #(#type_checks)||* }
		}
	};

	let generics = input.generics.clone();
	let where_clause = where_collector.extend_where_clause(&generics, parse_quote! { ::css_parse::Peek<'a> });

	Ok(quote! {
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
	})
}
