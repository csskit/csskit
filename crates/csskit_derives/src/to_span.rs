use crate::{FieldsExt, WhereCollector};
use itertools::{Itertools, Position};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Result, parse_quote};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = input.generics.clone();
	let (impl_generics, type_generics, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive ToSpan on a Union")),

		Data::Struct(DataStruct { fields, .. }) => {
			for syn_field in fields.iter() {
				where_collector.add(&syn_field.ty);
			}
			let members: Vec<_> = fields.views().into_iter().map(|v| (v.member, v.is_option)).collect();

			if members.len() == 1 || members.iter().all(|(_, is_option)| *is_option) {
				let members = members.iter().map(|(m, _)| m);
				quote! { #(self.#members.to_span())+* }
			} else {
				members
					.iter()
					.take_while_inclusive(|(_, is_option)| *is_option)
					.with_position()
					.map(|(position, (member, _))| match position {
						Position::Only => quote! { let first = self.#member.to_span(); },
						Position::First => quote! {
							let first = if let Some(ref value) = self.#member {
								value.to_span()
							}
						},
						Position::Middle => quote! {
							else if let Some(ref value) = self.#member {
								value.to_span()
							}
						},
						Position::Last => quote! {
							else {
								self.#member.to_span()
							};
						},
					})
					.chain(members.iter().rev().take_while_inclusive(|(_, is_option)| *is_option).with_position().map(
						|(position, (member, _))| match position {
							Position::Only => quote! { first + self.#member.to_span() },
							Position::First => quote! {
								let last = if let Some(ref value) = self.#member {
									value.to_span()
								}
							},
							Position::Middle => quote! {
								else if let Some(ref value) = self.#member {
									value.to_span()
								}
							},
							Position::Last => quote! {
								else {
									self.#member.to_span()
								};
								first + last
							},
						},
					))
					.collect()
			}
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let steps: TokenStream = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					let views = variant.fields.views();
					let len = views.len();
					for syn_field in variant.fields.iter() {
						where_collector.add(&syn_field.ty);
					}
					match &variant.fields {
						Fields::Named(_) => {
							if len == 1 {
								let m = &views[0].member;
								quote! { #ident::#variant_ident { #m: val } => val.to_span(), }
							} else {
								let first_m = &views[0].member;
								let last_m = &views[len - 1].member;
								let rest_pats = views[1..len - 1].iter().map(|v| {
									let m = &v.member;
									quote! { #m: _ }
								});
								quote! {
									#ident::#variant_ident { #first_m: first, #(#rest_pats,)* #last_m: last }
										=> first.to_span() + last.to_span(),
								}
							}
						}
						_ => {
							if len == 1 {
								quote! { #ident::#variant_ident(val) => val.to_span(), }
							} else {
								let rest = (2..len).map(|_| quote! { _ }).chain([quote! { last }]);
								quote! {
									#ident::#variant_ident(first, #(#rest),*) => first.to_span() + last.to_span(),
								}
							}
						}
					}
				})
				.collect();
			quote! { match self { #steps } }
		}
	};

	let where_clause = where_collector.extend_where_clause(&generics, parse_quote! { ::css_parse::ToSpan });

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::ToSpan for #ident #type_generics #where_clause {
			fn to_span(&self) -> ::css_parse::Span {
				use ::css_parse::{Span, ToSpan};
				#body
			}
		}
	})
}
