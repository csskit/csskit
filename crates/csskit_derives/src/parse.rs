use itertools::{Itertools, Position};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Meta, Token, Type, TypePath, parse::Parse,
	parse_quote,
};

use crate::err;

trait ToVarsAndTypes {
	fn to_vars_and_types(&self) -> (Vec<Ident>, Vec<Type>);
}

impl ToVarsAndTypes for Fields {
	fn to_vars_and_types(&self) -> (Vec<Ident>, Vec<Type>) {
		self.into_iter()
			.enumerate()
			.map(|(i, field)| {
				(
					field.ident.clone().unwrap_or_else(|| format_ident!("f{}", i)),
					match &field.ty {
						Type::Reference(refty) => refty.elem.as_ref(),
						ty => ty,
					}
					.clone(),
				)
			})
			.collect::<Vec<_>>()
			.into_iter()
			.unzip()
	}
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct ParseArg {
	pub state: Option<Ident>,
	pub stop: Option<(Ident, Ident)>,
}

impl Parse for ParseArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let (mut state, mut stop) = (None, None);
		while !input.is_empty() {
			match input.parse::<Ident>()? {
				i if i == "state" => {
					if state.is_some() {
						Err(Error::new(i.span(), "redefinition of 'state'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					let TypePath { path, .. } = input.parse::<TypePath>()?;
					let ident = path.segments.first().map(|s| s.ident.clone()).unwrap();
					if ident != "State" {
						Err(Error::new(ident.span(), format!("state must use the State type, saw {ident:?}")))?;
					}
					let ident = path.segments.last().map(|s| s.ident.clone()).unwrap();
					state = Some(ident);
				}
				i if i == "stop" => {
					if stop.is_some() {
						Err(Error::new(i.span(), "redefinition of 'state'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					let TypePath { path, .. } = input.parse::<TypePath>()?;
					let kind_or_kindset = path.segments.first().map(|s| s.ident.clone()).unwrap();
					if kind_or_kindset != "Kind" && kind_or_kindset != "KindSet" {
						panic!("stop must use the Kind or KindSet type");
					}
					let ident = path.segments.last().map(|s| s.ident.clone()).unwrap();
					stop = Some((kind_or_kindset, ident));
				}
				ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {ident:?}")))?,
			}

			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(Self { state, stop })
	}
}

impl From<&Vec<Attribute>> for ParseArg {
	fn from(attrs: &Vec<Attribute>) -> Self {
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("parse")) {
			match meta {
				Meta::List(meta) => meta.parse_args::<ParseArg>().unwrap(),
				_ => panic!("could not parse meta"),
			}
		} else {
			Self::default()
		}
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let generics = &input.generics;
	let mut generic_with_alloc = generics.clone();
	let (impl_generics, type_generics, where_clause) = if generics.lifetimes().all(|l| l.lifetime.ident != "a") {
		generic_with_alloc.params.insert(0, parse_quote!('a));
		let (impl_generics, _, _) = generic_with_alloc.split_for_impl();
		let (_, type_generics, where_clause) = generics.split_for_impl();
		(impl_generics, type_generics, where_clause)
	} else {
		generics.split_for_impl()
	};
	let mut pre_parse_steps = quote! {};
	let mut post_parse_steps = quote! {};
	let ParseArg { state, stop } = (&input.attrs).into();
	if let Some(ident) = state {
		pre_parse_steps = quote! {
			let state = p.set_state(State::#ident);
			#pre_parse_steps
		};
		post_parse_steps = quote! {
			#post_parse_steps
			p.set_state(state);
		};
	}
	if let Some((kind_or_kindset, ident)) = stop {
		pre_parse_steps = if kind_or_kindset == "Kind" {
			quote! {
				let stop = p.set_stop(KindSet::new(&[Kind::#ident]));
				#pre_parse_steps
			}
		} else {
			quote! {
				let stop = p.set_stop(KindSet::#ident);
				#pre_parse_steps
			}
		};
		post_parse_steps = quote! {
			#post_parse_steps
			p.set_stop(stop);
		};
	}

	let body = match input.data {
		Data::Union(_) => err(ident.span(), "Cannot derive Parse on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let members = fields.members();
			let (vars, types) = fields.to_vars_and_types();
			quote! {
				#( let #vars = p.parse::<#types>()?; )*
				#post_parse_steps
				Ok(Self { #(#members: #vars),* })
			}
		}

		Data::Enum(DataEnum { variants, .. }) => variants
			.iter()
			.with_position()
			.map(|(position, variant)| {
				let variant_ident = &variant.ident;
				let members = variant.fields.members();
				let (vars, types) = variant.fields.to_vars_and_types();
				let first_type = types.first();
				let step = quote! {
					#( let #vars = p.parse::<#types>()?; )*
					#post_parse_steps
					Ok(Self::#variant_ident { #(#members: #vars),* })
				};
				match position {
					Position::First => quote! { if p.peek::<#first_type>() { #step } },
					Position::Last => quote! { else { #step } },
					Position::Only => step,
					Position::Middle => quote! { else if p.peek::<#first_type>() { #step } },
				}
			})
			.collect(),
	};
	quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Parse<'a> for #ident #type_generics #where_clause {
			fn parse(p: &mut css_parse::Parser<'a>) -> css_parse::Result<Self> {
				use css_parse::{Parse};
				#pre_parse_steps
				#body
			}
		}
	}
}
