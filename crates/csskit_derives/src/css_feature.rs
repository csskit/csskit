use heck::ToKebabCase;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use std::fmt::Display;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, LitStr, Meta, Result, parse::Parse};

use crate::err;

#[derive(Clone, Debug, PartialEq, Eq)]
struct CSSFeatureName(pub LitStr);

impl Display for CSSFeatureName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.value())
	}
}

impl ToTokens for CSSFeatureName {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens);
	}
}

impl Parse for CSSFeatureName {
	fn parse(input: syn::parse::ParseStream) -> Result<Self> {
		input.parse::<LitStr>().map(Self)
	}
}

impl TryFrom<&Vec<Attribute>> for CSSFeatureName {
	type Error = Error;
	fn try_from(attrs: &Vec<Attribute>) -> Result<Self> {
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("css_feature")) {
			match meta {
				// #[css_feature("foo")]
				Meta::List(meta) => meta.parse_args::<Self>(),
				// #[visit]
				_ => Err(Error::new(Span::call_site(), "`css_feature` attribute has no value")),
			}
		} else {
			// No attribute present
			Err(Error::new(Span::call_site(), "Missing `css_feature` attribute"))
		}
	}
}

fn by_feature_name(feature: String) -> TokenStream {
	quote! { ::css_feature_data::CSSFeature::by_feature_name(#feature) }
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &mut input.generics.clone();
	let (impl_generics, _, _) = generics.split_for_impl();
	let feature: CSSFeatureName = (&input.attrs).try_into().unwrap();
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
				let variants = variants
					.iter()
					.map(|variant| {
						let variant_ident = &variant.ident;
						let members = variant.fields.members().map(|_| quote! { _ });
						if let Ok(feature) = TryInto::<CSSFeatureName>::try_into(&variant.attrs) {
							let step = by_feature_name(feature.to_string());
							quote! { Self::#variant_ident(#(#members),*) => { #step }, }
						} else {
							// enum candidates are very likely to be candidates for their own feature name, so just try guessing:
							let str = feature.to_string();
							let guessed_step =
								by_feature_name(format!("{}.{}", str, variant_ident.to_string().to_kebab_case()));
							let or_step = by_feature_name(feature.to_string());
							quote! { Self::#variant_ident(#(#members),*) => { #guessed_step.or_else(|| #or_step) }, }
						}
					})
					.collect::<Vec<_>>();
				let by_feature_name = by_feature_name(feature.to_string());
				quote! {
					match self {
						#(#variants)*
						_ => { #by_feature_name },
					}
				}
			}
		}
	};
	let steps = if steps.is_empty() { by_feature_name(feature.to_string()) } else { steps };
	quote! {
		#[automatically_derived]
		impl #impl_generics #ident #impl_generics {
			fn to_css_feature(&self) -> Option<&'static ::css_feature_data::CSSFeature> {
				#steps
			}
		}
	}
}
