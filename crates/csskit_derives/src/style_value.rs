use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Ident, LitStr, Meta, Result, Token, parse::Parse,
};

use crate::err;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct StyleValueAttr {
	initial: Option<LitStr>,
	applies_to: Option<LitStr>,
	inherited: Option<LitStr>,
	percentages: Option<LitStr>,
	canonical_order: Option<LitStr>,
	animation_type: Option<LitStr>,
}

impl StyleValueAttr {}

impl Parse for StyleValueAttr {
	fn parse(input: syn::parse::ParseStream) -> Result<Self> {
		let mut args = StyleValueAttr::default();
		while !input.is_empty() {
			match input.parse::<Ident>()? {
				i if i == "initial" => {
					input.parse::<Token![=]>()?;
					args.initial = Some(input.parse::<LitStr>()?);
				}
				i if i == "applies_to" => {
					input.parse::<Token![=]>()?;
					args.applies_to = Some(input.parse::<LitStr>()?);
				}
				i if i == "inherited" => {
					input.parse::<Token![=]>()?;
					args.inherited = Some(input.parse::<LitStr>()?);
				}
				i if i == "percentages" => {
					input.parse::<Token![=]>()?;
					args.percentages = Some(input.parse::<LitStr>()?);
				}
				i if i == "canonical_order" => {
					input.parse::<Token![=]>()?;
					args.canonical_order = Some(input.parse::<LitStr>()?);
				}
				i if i == "animation_type" => {
					input.parse::<Token![=]>()?;
					args.animation_type = Some(input.parse::<LitStr>()?);
				}
				ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {ident:?}")))?,
			}

			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(args)
	}
}

impl TryFrom<&Vec<Attribute>> for StyleValueAttr {
	type Error = Error;
	fn try_from(attrs: &Vec<Attribute>) -> Result<Self> {
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("style_value")) {
			match meta {
				// #[style_value(...)]
				Meta::List(meta) => meta.parse_args::<Self>(),
				// #[visit]
				_ => Err(Error::new(Span::call_site(), "`style_value` attribute has no value")),
			}
		} else {
			// No attribute present
			Err(Error::new(Span::call_site(), "Missing `style_value` attribute"))
		}
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let _style_value: StyleValueAttr = (&input.attrs).try_into().unwrap();
	let steps = match &input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			if fields.is_empty() {
				err(ident.span(), "Cannot derive on empty Struct")
			} else {
				quote! {}
			}
		}

		Data::Enum(DataEnum { variants, .. }) => {
			if variants.is_empty() {
				err(ident.span(), "Cannot derive on empty Enum")
			} else {
				quote! {}
			}
		}
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics #ident #impl_generics {
			fn placeholder() {
				#steps
			}
		}
	}
}
