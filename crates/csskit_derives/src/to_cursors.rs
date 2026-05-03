use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result, parse_quote};
use synstructure::{AddBounds, BindStyle, Structure};

use crate::WhereCollector;

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	if let Data::Struct(ref s) = input.data
		&& matches!(s.fields, Fields::Unit)
	{
		return Err(Error::new(input.ident.span(), "Cannot derive ToCursors on this struct"));
	}
	if matches!(input.data, Data::Union(_)) {
		return Err(Error::new(input.ident.span(), "Cannot derive ToCursors on a Union"));
	}

	let mut s = Structure::try_new(&input)?;
	s.add_bounds(AddBounds::None);
	s.bind_with(|_| BindStyle::Ref);

	let body = s.each(|bi| {
		quote! { ::css_parse::ToCursors::to_cursors(#bi, s); }
	});

	let mut wc = WhereCollector::new();
	for variant in s.variants() {
		for binding in variant.bindings() {
			wc.add(&binding.ast().ty);
		}
	}
	let where_clause = wc.extend_where_clause(&input.generics, parse_quote! { ::css_parse::ToCursors });

	let ident = &input.ident;
	let generics = &input.generics;
	let (impl_generics, type_generics, _) = generics.split_for_impl();

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::ToCursors for #ident #type_generics #where_clause {
			fn to_cursors(&self, s: &mut impl ::css_parse::CursorSink) {
				match *self { #body }
			}
		}
	})
}
