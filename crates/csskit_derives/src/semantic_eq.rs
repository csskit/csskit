use crate::WhereCollector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Fields, Result, parse_quote};
use synstructure::{AddBounds, BindStyle, Structure};

fn structure_with_prefix<'a>(input: &'a DeriveInput, prefix: &'static str) -> Result<Structure<'a>> {
	let mut s = Structure::try_new(input)?;
	s.add_bounds(AddBounds::None);
	s.bind_with(|_| BindStyle::Move);
	s.binding_name(move |field, i| match &field.ident {
		Some(name) => format_ident!("{prefix}_{name}"),
		None => format_ident!("{prefix}{i}"),
	});
	Ok(s)
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	if let Data::Struct(ref s) = input.data
		&& matches!(s.fields, Fields::Unit)
	{
		return Err(Error::new(input.ident.span(), "Cannot derive SemanticEq on this struct"));
	}
	if matches!(input.data, Data::Union(_)) {
		return Err(Error::new(input.ident.span(), "Cannot derive SemanticEq on a Union"));
	}

	let s_a = structure_with_prefix(&input, "a")?;
	let s_b = structure_with_prefix(&input, "b")?;

	let mut wc = WhereCollector::new();
	for variant in s_a.variants() {
		for bi in variant.bindings() {
			wc.add(&bi.ast().ty);
		}
	}

	let ident = &input.ident;
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();

	let body = if matches!(input.data, Data::Struct(_)) {
		let steps: Vec<TokenStream> = s_a.variants()[0]
			.bindings()
			.iter()
			.zip(s_b.variants()[0].bindings().iter())
			.map(|(a, b)| {
				let a_name = &a.binding;
				let b_name = &b.binding;
				quote! { #a_name.semantic_eq(&#b_name) }
			})
			.collect();
		let a_pat = s_a.variants()[0].pat();
		let b_pat = s_b.variants()[0].pat();
		let body = steps.into_iter().reduce(|acc, item| quote! { #acc && #item }).unwrap_or_default();
		quote! {
			let #a_pat = self;
			let #b_pat = other;
			#body
		}
	} else {
		let arms: TokenStream = s_a
			.variants()
			.iter()
			.zip(s_b.variants().iter())
			.map(|(va, vb)| {
				let a_pat = va.pat();
				let b_pat = vb.pat();
				let steps: Vec<TokenStream> = va
					.bindings()
					.iter()
					.zip(vb.bindings().iter())
					.map(|(a, b)| {
						let a_name = &a.binding;
						let b_name = &b.binding;
						quote! { #a_name.semantic_eq(&#b_name) }
					})
					.collect();
				let body = steps.into_iter().reduce(|acc, item| quote! { #acc && #item }).unwrap_or(quote! { true });
				quote! { (#a_pat, #b_pat) => { #body } }
			})
			.collect();
		quote! {
			match (self, other) {
				#arms
				_ => false,
			}
		}
	};

	let where_clause = wc.extend_where_clause(&input.generics, parse_quote! { ::css_parse::SemanticEq });

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::SemanticEq for #ident #type_generics #where_clause {
			fn semantic_eq(&self, other: &Self) -> bool {
				use ::css_parse::SemanticEq;
				#body
			}
		}
	})
}
