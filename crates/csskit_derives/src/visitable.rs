use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Ident, Meta, parse::Parse, token::SelfValue};

use crate::err;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum VisitStyle {
	All,
	Skip,
	OnlySelf,
	#[default]
	OnlyChildren,
}

impl VisitStyle {
	pub fn visit_self(&self) -> bool {
		matches!(self, Self::All | Self::OnlySelf)
	}
	pub fn visit_children(&self) -> bool {
		matches!(self, Self::All | Self::OnlyChildren)
	}
}

impl Parse for VisitStyle {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		if input.parse::<SelfValue>().is_ok() {
			return Ok(Self::OnlySelf);
		}
		match input.parse::<Ident>()? {
			i if i == "all" => Ok(Self::All),
			i if i == "skip" => Ok(Self::Skip),
			i if i == "children" => Ok(Self::OnlyChildren),
			ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {ident:?}")))?,
		}
	}
}

impl From<&Vec<Attribute>> for VisitStyle {
	fn from(attrs: &Vec<Attribute>) -> Self {
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("visit")) {
			match meta {
				// #[visit(keyword)]
				Meta::List(meta) => meta.parse_args::<VisitStyle>().unwrap(),
				// #[visit]
				_ => Self::All,
			}
		} else {
			// No attribute present
			Self::default()
		}
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let style: VisitStyle = (&input.attrs).into();
	let visit = if style.visit_self() {
		let method = format_ident!("visit_{}", ident.to_string().to_snake_case());
		quote! { v.#method(self); }
	} else {
		quote! {}
	};
	let [body_mut, body] = if style.visit_children() {
		[format_ident!("accept_mut"), format_ident!("accept")].map(|accept| match &input.data {
			Data::Union(_) => err(ident.span(), "Cannot derive Into<Span> on a Union"),

			Data::Struct(DataStruct { fields, .. }) => {
				let members = fields.members().zip(fields).filter_map(|(member, field)| {
					if Into::<VisitStyle>::into(&field.attrs) == VisitStyle::Skip { None } else { Some(member) }
				});
				quote! { #(self.#members.#accept(v);)* }
			}

			Data::Enum(DataEnum { variants, .. }) => {
				let steps: TokenStream = variants
					.iter()
					.map(|variant| {
						let variant_ident = &variant.ident;
						if Into::<VisitStyle>::into(&variant.attrs) == VisitStyle::Skip {
							return quote! {
								Self::#variant_ident(_) => {},
							};
						}
						let (members, steps): (Vec<_>, Vec<_>) = variant
							.fields
							.iter()
							.enumerate()
							.map(|(i, field)| {
								if Into::<VisitStyle>::into(&field.attrs) == VisitStyle::Skip {
									(format_ident!("_"), quote! {})
								} else {
									let ident = format_ident!("v{}", i);
									(ident.clone(), quote! { #ident.#accept(v) })
								}
							})
							.collect::<Vec<_>>()
							.into_iter()
							.unzip();
						quote! {
							Self::#variant_ident(#(#members),*) => { #(#steps;)* },
						}
					})
					.collect();
				quote! { match self { #steps } }
			}
		})
	} else {
		[quote! {}, quote! {}]
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics crate::VisitableMut for #ident #impl_generics {
			fn accept_mut<V: crate::VisitMut>(&mut self, v: &mut V) {
				use crate::VisitableMut;
				#visit
				#body_mut
			}
		}

		#[automatically_derived]
		impl #impl_generics crate::Visitable for #ident #impl_generics {
			fn accept<V: crate::Visit>(&self, v: &mut V) {
				use crate::Visitable;
				#visit
				#body
			}
		}
	}
}
