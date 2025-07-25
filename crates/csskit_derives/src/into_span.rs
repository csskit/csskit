use itertools::{Itertools, Position};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Type, TypePath};

use crate::err;

trait TypeIsOption {
	fn is_option(&self) -> bool;
}

impl TypeIsOption for Type {
	fn is_option(&self) -> bool {
		match self {
			Self::Path(TypePath { path, .. }) => path.segments.last().map_or(false, |s| s.ident == "Option"),
			_ => false,
		}
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive Into<Span> on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let members: Vec<_> = fields.members().zip(fields.iter().map(|f| f.ty.is_option())).collect();
			if members.len() == 1 {
				let member = fields.members().next().unwrap();
				quote! { (&value.#member).into() }

			// All members are Option<T>, so we have no choice but to try and add them all to get something useful.
			} else if members.iter().all(|(_, is_option)| *is_option) {
				let members = fields.members();
				quote! { #(Into::<Span>::into(&value.#members))+* }
			} else {
				// To get a reliable span we need to find the first member, and the last. However as some members are
				// Optional<T>, and could potentially all be none, we need to find the first non-optional member to guarantee we can get a Span.
				members
					.iter()
					.take_while_inclusive(|(_, is_option)| *is_option)
					.with_position()
					.map(|(position, (member, _))| match position {
						Position::Only => quote! { let first = Into::<Span>::into(&value.#member); },
						Position::First => quote! {
							let first = if let Some(value) = value.#member {
								Into::<Span>::into(&value)
							}
						},
						Position::Middle => quote! {
							else if let Some(value) = value.#member {
								Into::<Span>::into(&value)
							}
						},
						Position::Last => quote! {
							else {
								Into::<Span>::into(&value.#member)
							};
						},
					})
					.chain(members.iter().rev().take_while_inclusive(|(_, is_option)| *is_option).with_position().map(
						|(position, (member, _))| match position {
							Position::Only => quote! { first + (&value.#member).into() },
							Position::First => quote! {
								let last = if let Some(ref value) = value.#member {
									value.into()
								}
							},
							Position::Middle => quote! {
								else if let Some(ref value) = value.#member {
									value.into()
								}
							},
							Position::Last => quote! {
								else {
									(&value.#member).into()
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
						quote! { #ident::#variant_ident(val) => val.into(), }
					} else {
						let rest = (2..len).map(|_| quote! { _ }).chain([quote! {last}]);
						quote! {
							#ident::#variant_ident(first, #(#rest),*) => Into::<Span>::into(first) + last.into(),
						}
					}
				})
				.collect();
			quote! {
				match value {
					#steps
				}
			}
		}
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics From<&#ident #impl_generics> for ::css_lexer::Span {
			fn from(value: &#ident) -> ::css_lexer::Span {
				use ::css_lexer::Span;
				#body
			}
		}
	}
}
