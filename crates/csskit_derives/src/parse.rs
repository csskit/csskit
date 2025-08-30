use itertools::{Itertools, Position};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, ExprRange, Fields, Meta, Token, Type, TypePath,
	parse::Parse, parse_quote,
};

use crate::err;

/// Generate range validation code for a field based on the in_range attribute
fn generate_range_validation(field_ident: &Ident, range_expr: &ExprRange) -> TokenStream {
	let start = &range_expr.start;
	let end = &range_expr.end;
	match (start, end) {
		// 1..=10 (inclusive end)
		(Some(start), Some(end)) => {
			quote! {
				if let Ok(i) = std::convert::TryInto::<f32>::try_into(#field_ident) {
					if !(#start..=#end).contains(&i) {
						use ::css_parse::ToSpan;
						Err(::css_parse::diagnostics::NumberOutOfBounds(
							#field_ident.into(),
							format!("{}..={}", #start, #end),
							#field_ident.to_span()
						))?
					}
				}
			}
		}
		(Some(start), None) => {
			quote! {
				if let Ok(i) = std::convert::TryInto::<f32>::try_into(#field_ident) {
					if #start > i {
						use ::css_parse::ToSpan;
						Err(::css_parse::diagnostics::NumberTooSmall(
							#start,
							#field_ident.to_span()
						))?
					}
				}
			}
		}
		(None, Some(end)) => {
			quote! {
				if let Ok(i) = std::convert::TryInto::<f32>::try_into(#field_ident) {
					if #end < i {
						use ::css_parse::ToSpan;
						Err(::css_parse::diagnostics::NumberTooLarge(
							#end,
							#field_ident.to_span()
						))?
					}
				}
			}
		}
		// .. (full range) - no validation needed
		(None, None) => quote! {},
	}
}

trait ToVarsAndTypes {
	fn to_vars_and_types(&self) -> Vec<(Ident, Type, ParseArg)>;
}

impl ToVarsAndTypes for Fields {
	fn to_vars_and_types(&self) -> Vec<(Ident, Type, ParseArg)> {
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
					ParseArg::from(&field.attrs),
				)
			})
			.collect::<Vec<_>>()
	}
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct ParseArg {
	pub state: Option<Ident>,
	pub stop: Option<(Ident, Ident)>,
	pub in_range: Option<ExprRange>,
	pub all_must_occur: bool,
}

impl Parse for ParseArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let (mut state, mut stop, mut in_range, mut all_must_occur) = (None, None, None, false);
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
						Err(Error::new(i.span(), "redefinition of 'stop'".to_string()))?;
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
				i if i == "in_range" => {
					if in_range.is_some() {
						Err(Error::new(i.span(), "redefinition of 'in_range'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					let range = input.parse::<ExprRange>()?;
					in_range = Some(range);
				}
				i if i == "all_must_occur" => {
					if all_must_occur {
						Err(Error::new(i.span(), "redefinition of 'all_must_occur'".to_string()))?;
					}
					all_must_occur = true;
				}
				ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {ident:?}")))?,
			}

			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(Self { state, stop, in_range, all_must_occur })
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
	let ParseArg { state, stop, all_must_occur, .. } = (&input.attrs).into();
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
			let split_fields = fields.to_vars_and_types();
			let vars = split_fields.iter().map(|(var, _, _)| var);

			if all_must_occur {
				let bindings: Vec<TokenStream> = split_fields
					.iter()
					.map(|(var, ty, _)| {
						quote! { let mut #var: Option<#ty> = None; }
					})
					.collect();

				let parse_steps: Vec<TokenStream> = split_fields
					.iter()
					.map(|(var, ty, arg)| {
						let inner = if let Some(r) = &arg.in_range {
							let inner = format_ident!("val");
							let range_check = generate_range_validation(&inner, r);
							quote! {
								let #inner = p.parse::<#ty>()?;
								#range_check
								#var = Some(#inner);
							}
						} else {
							quote! { #var = Some(p.parse::<#ty>()?); }
						};
						quote! {
							if #var.is_none() && <#ty>::peek(p, c) {
								#inner
								continue;
							}
						}
					})
					.collect();
				let checks: Vec<TokenStream> = vars.clone().map(|var| quote! { #var.is_none() }).collect();
				let unwraps = vars.clone().map(|var| quote! { #var.unwrap() });
				quote! {
					#(#bindings)*
					loop {
						let c = p.peek_n(1);
						#(#parse_steps)*
						break;
					}
					#post_parse_steps
					if #(#checks)||* {
						let c = p.peek_n(1);
						Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
					}
					Ok(Self { #(#members: #unwraps),* })
				}
			} else {
				// Generate sequential parsing logic (existing behavior)
				let parse_steps: Vec<TokenStream> = split_fields
					.iter()
					.map(|(var, ty, arg)| {
						let parse_step = quote! { let #var = p.parse::<#ty>()?; };
						let check_step = arg.in_range.as_ref().map(|r| generate_range_validation(var, r));
						quote! { #parse_step #check_step }
					})
					.collect();

				quote! {
					#( #parse_steps )*
					#post_parse_steps
					Ok(Self { #(#members: #vars),* })
				}
			}
		}

		Data::Enum(DataEnum { variants, .. }) => variants
			.iter()
			.with_position()
			.map(|(position, variant)| {
				let variant_ident = &variant.ident;
				let ParseArg { all_must_occur, .. } = (&variant.attrs).into();
				let members = variant.fields.members();
				let split_fields = variant.fields.to_vars_and_types();
				let first_type = split_fields.first().map(|(_, ty, _)| ty);
				let vars = split_fields.iter().map(|(var, _, _)| var);

				let step = if all_must_occur {
					// Generate AllMustOccur parsing logic for this variant
					let bindings: Vec<TokenStream> = split_fields
						.iter()
						.map(|(var, ty, _)| {
							quote! { let mut #var: Option<#ty> = None; }
						})
						.collect();

					let parse_steps: Vec<TokenStream> = split_fields
						.iter()
						.map(|(var, ty, arg)| {
							let inner = if let Some(r) = &arg.in_range {
								let inner = format_ident!("val");
								let range_check = generate_range_validation(&inner, r);
								quote! {
									let #inner = p.parse::<#ty>()?;
									#range_check
									#var = Some(#inner);
								}
							} else {
								quote! { #var = Some(p.parse::<#ty>()?); }
							};
							quote! {
								if #var.is_none() && <#ty>::peek(p, c) {
									#inner
									continue;
								}
							}
						})
						.collect();
					let checks: Vec<TokenStream> = vars.clone().map(|var| quote! { #var.is_none() }).collect();
					let unwraps = vars.clone().map(|var| quote! { #var.unwrap() });

					quote! {
						#(#bindings)*
						loop {
							let c = p.peek_n(1);
							#(#parse_steps)*
							break;
						}
						#post_parse_steps
						if #(#checks)||* {
							let c = p.peek_n(1);
							Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
						}
						Ok(Self::#variant_ident { #(#members: #unwraps),* })
					}
				} else {
					// Generate sequential parsing logic (existing behavior)
					let parse_steps: Vec<TokenStream> = split_fields
						.iter()
						.map(|(var, ty, arg)| {
							let parse_step = quote! { let #var = p.parse::<#ty>()?; };
							let check_step = arg.in_range.as_ref().map(|r| generate_range_validation(var, r));
							quote! { #parse_step #check_step }
						})
						.collect();

					quote! {
						#( #parse_steps )*
						#post_parse_steps
						Ok(Self::#variant_ident { #(#members: #vars),* })
					}
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
