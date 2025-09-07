use crate::{TypeIsOption, err};
use itertools::{Itertools, Position};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput};

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive ToSpan on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let members: Vec<_> = fields.members().zip(fields.iter().map(|f| f.ty.is_option())).collect();
			if members.len() == 1 {
				let member = fields.members().next().unwrap();
				quote! { self.#member.to_span() }

			// All members are Option<T>, so we have no choice but to try and add them all to get something useful.
			} else if members.iter().all(|(_, is_option)| *is_option) {
				let members = fields.members();
				quote! { #(self.#members.to_span())+* }
			} else {
				// To get a reliable span we need to find the first member, and the last. However as some members are
				// Optional<T>, and could potentially all be none, we need to find the first non-optional member to guarantee we can get a Span.
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
					let len = variant.fields.len();
					if len == 1 {
						quote! { #ident::#variant_ident(val) => val.to_span(), }
					} else {
						let rest = (2..len).map(|_| quote! { _ }).chain([quote! {last}]);
						quote! {
							#ident::#variant_ident(first, #(#rest),*) => first.to_span() + last.to_span(),
						}
					}
				})
				.collect();
			quote! {
				match self {
					#steps
				}
			}
		}
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::ToSpan for #ident #type_generics #where_clause {
			fn to_span(&self) -> ::css_parse::Span {
				use ::css_parse::{Span, ToSpan};
				#body
			}
		}
	}
}
