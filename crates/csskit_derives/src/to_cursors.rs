use crate::{FieldsExt, WhereCollector, field_view::Arm};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result, parse_quote};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	if let Data::Struct(s) = &input.data
		&& matches!(s.fields, Fields::Unit)
	{
		return Err(Error::new(input.ident.span(), "Cannot derive ToCursors on this struct"));
	}
	let arms = Arm::all(&input)?;

	let mut wc = WhereCollector::new();
	let arms: Vec<TokenStream> = arms
		.iter()
		.map(|arm| {
			for field in arm.fields.iter() {
				wc.add(&field.ty);
			}
			let pattern = arm.pattern(|_, view| Some(view.binding.clone()));
			let bindings = arm.fields.views().into_iter().map(|view| view.binding);
			quote! { #pattern => { #(::css_parse::ToCursors::to_cursors(#bindings, s);)* } }
		})
		.collect();
	let body = quote! { match self { #(#arms)* } };

	let where_clause = wc.extend_where_clause(&input.generics, parse_quote! { ::css_parse::ToCursors });
	let ident = &input.ident;
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::ToCursors for #ident #type_generics #where_clause {
			fn to_cursors(&self, s: &mut impl ::css_parse::CursorSink) {
				#body
			}
		}
	})
}
