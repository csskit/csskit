use crate::{
	TypeIsOption, WhereCollector,
	attributes::{Atom, extract_atom, extract_in_range},
	err,
};
use itertools::{Itertools, Position};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, ExprRange, Fields, Meta, Token, Type, TypePath,
	parse::Parse, parse_quote,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum FieldParseMode {
	#[default]
	Sequential,
	AllMustOccur,
	OneMustOccur,
}

trait ToVarsAndTypes {
	fn to_vars_and_types(&self) -> Vec<(Ident, Type, ParseArg, Option<Atom>)>;
}

impl ToVarsAndTypes for Fields {
	fn to_vars_and_types(&self) -> Vec<(Ident, Type, ParseArg, Option<Atom>)> {
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
					extract_atom(&field.attrs),
				)
			})
			.collect::<Vec<_>>()
	}
}

#[derive(Debug, Default)]
struct ParseArg {
	pub state: Option<Ident>,
	pub stop: Option<(Ident, Ident)>,
	pub in_range: Option<ExprRange>,
	pub parse_mode: FieldParseMode,
}

impl Parse for ParseArg {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let mut args = ParseArg::default();
		while !input.is_empty() {
			match input.parse::<Ident>()? {
				i if i == "state" => {
					if args.state.is_some() {
						Err(Error::new(i.span(), "redefinition of 'state'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					let TypePath { path, .. } = input.parse::<TypePath>()?;
					let ident = path.segments.first().map(|s| s.ident.clone()).unwrap();
					if ident != "State" {
						Err(Error::new(ident.span(), format!("state must use the State type, saw {ident:?}")))?;
					}
					let ident = path.segments.last().map(|s| s.ident.clone()).unwrap();
					args.state = Some(ident);
				}
				i if i == "stop" => {
					if args.stop.is_some() {
						Err(Error::new(i.span(), "redefinition of 'stop'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					let TypePath { path, .. } = input.parse::<TypePath>()?;
					let kind_or_kindset = path.segments.first().map(|s| s.ident.clone()).unwrap();
					if kind_or_kindset != "Kind" && kind_or_kindset != "KindSet" {
						panic!("stop must use the Kind or KindSet type");
					}
					let ident = path.segments.last().map(|s| s.ident.clone()).unwrap();
					args.stop = Some((kind_or_kindset, ident));
				}
				i if i == "in_range" => {
					if args.in_range.is_some() {
						Err(Error::new(i.span(), "redefinition of 'in_range'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					let range = input.parse::<syn::Expr>()?;
					let range = if let syn::Expr::Range(range_expr) = range {
						range_expr
					} else {
						return Err(Error::new_spanned(range, "Expected range expression"));
					};
					args.in_range = Some(range);
				}
				i if i == "all_must_occur" => {
					if args.parse_mode != Default::default() {
						Err(Error::new(i.span(), "redefinition of 'all_must_occur' or 'one_must_occur'".to_string()))?;
					}
					args.parse_mode = FieldParseMode::AllMustOccur;
				}
				i if i == "one_must_occur" => {
					if args.parse_mode != Default::default() {
						Err(Error::new(i.span(), "redefinition of 'all_must_occur' or 'one_must_occur'".to_string()))?;
					}
					args.parse_mode = FieldParseMode::OneMustOccur;
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

impl From<&Vec<Attribute>> for ParseArg {
	fn from(attrs: &Vec<Attribute>) -> Self {
		let mut result = Self::default();

		// Check for #[parse(...)] attribute
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("parse")) {
			match meta {
				Meta::List(meta) => {
					let parsed = meta.parse_args::<ParseArg>().unwrap();
					result.state = parsed.state;
					result.stop = parsed.stop;
					result.parse_mode = parsed.parse_mode;
					result.in_range = parsed.in_range;
				}
				_ => panic!("could not parse meta"),
			}
		}

		// Check for #[in_range(...)]
		if let Some(range) = extract_in_range(attrs) {
			result.in_range = Some(range);
		}

		result
	}
}

fn generate_field_parsing(
	var: &Ident,
	ty: &Type,
	arg: &ParseArg,
	atom: &Option<Atom>,
	parse_mode: FieldParseMode,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	if let Some(atom) = atom {
		generate_keyword_parsing(var, ty, atom, arg, parse_mode, where_collector)
	} else {
		generate_normal_parsing(var, ty, arg, parse_mode, where_collector)
	}
}

fn generate_keyword_parsing(
	var: &Ident,
	ty: &Type,
	atom: &Atom,
	arg: &ParseArg,
	parse_mode: FieldParseMode,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	let range_validation = arg.in_range.as_ref().map(|r| generate_range_validation(&format_ident!("ident"), ty, r));

	match parse_mode {
		FieldParseMode::Sequential => {
			let condition = atom.equals_atom(format_ident!("c"));
			let ty = ty.unpack_option();
			where_collector.add(&ty);
			quote! {
				let #var = {
					let c = p.peek_n(1);
					if #condition {
						#range_validation
						p.parse::<#ty>()?
					} else {
						return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?;
					}
				};
			}
		}
		FieldParseMode::AllMustOccur | FieldParseMode::OneMustOccur => {
			let atom = atom.path();
			let ty = ty.unpack_option();
			where_collector.add(&ty);
			quote! {
				if #var.is_none() && atom == #atom {
					#range_validation
					#var = Some(p.parse::<#ty>()?);
					continue;
				}
			}
		}
	}
}

fn generate_normal_parsing(
	var: &Ident,
	ty: &Type,
	arg: &ParseArg,
	parse_mode: FieldParseMode,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	match parse_mode {
		FieldParseMode::Sequential => {
			where_collector.add(ty);
			let parse_step = quote! { let #var = p.parse::<#ty>()?; };
			let check_step = arg.in_range.as_ref().map(|r| generate_range_validation(var, ty, r));
			quote! { #parse_step #check_step }
		}
		FieldParseMode::AllMustOccur | FieldParseMode::OneMustOccur => {
			let ty = ty.unpack_option();
			where_collector.add(&ty);
			let inner = if let Some(r) = &arg.in_range {
				let inner = format_ident!("inner");
				let range_check = generate_range_validation(&inner, &ty, r);
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
		}
	}
}

fn generate_must_occur_parsing(
	split_fields: &[(Ident, Type, ParseArg, Option<Atom>)],
	members: Vec<TokenStream>,
	post_parse_steps: &TokenStream,
	parse_mode: FieldParseMode,
	constructor: TokenStream,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	let mut atom_binding = None;
	let bindings: Vec<TokenStream> = split_fields
		.iter()
		.map(|(var, ty, _, atom)| {
			if atom.is_some() && atom_binding.is_none() {
				let atom = atom.as_ref().unwrap().to_atom(format_ident!("c"));
				atom_binding = Some(quote! { let atom = #atom; });
			}
			if ty.is_option() {
				quote! { let mut #var: #ty = None; }
			} else {
				quote! { let mut #var: Option<#ty> = None; }
			}
		})
		.collect();

	let parse_steps: Vec<TokenStream> = split_fields
		.iter()
		.map(|(var, ty, arg, atom)| generate_field_parsing(var, ty, arg, atom, parse_mode, where_collector))
		.collect();

	let vars = split_fields.iter().map(|(var, _, _, _)| var);
	let checks: Vec<TokenStream> = vars.clone().map(|var| quote! { #var.is_none() }).collect();
	let assignments: Vec<_> = match parse_mode {
		FieldParseMode::Sequential => unreachable!(),
		FieldParseMode::OneMustOccur => vars.map(|var| quote! { #var }).collect(),
		FieldParseMode::AllMustOccur => vars.map(|var| quote! { #var.unwrap() }).collect(),
	};
	let occurance_cond = match parse_mode {
		FieldParseMode::Sequential => unreachable!(),
		FieldParseMode::OneMustOccur => quote! { #(#checks)&&* },
		FieldParseMode::AllMustOccur => quote! { #(#checks)||* },
	};

	quote! {
	  #(#bindings)*
	  loop {
			let c = p.peek_n(1);
			#atom_binding
			#(#parse_steps)*
			break;
	  }
	  #post_parse_steps
	  if #occurance_cond {
			let c = p.peek_n(1);
			Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?
	  }
	  return Ok(#constructor { #(#members: #assignments),* });
	}
}

fn generate_sequential_parsing(
	split_fields: &[(Ident, Type, ParseArg, Option<Atom>)],
	members: Vec<TokenStream>,
	post_parse_steps: &TokenStream,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	let parse_steps: Vec<TokenStream> = split_fields
		.iter()
		.map(|(var, ty, arg, atom)| {
			generate_field_parsing(var, ty, arg, atom, FieldParseMode::Sequential, where_collector)
		})
		.collect();

	let vars = split_fields.iter().map(|(var, _, _, _)| var);

	quote! {
	  #( #parse_steps )*
	  #post_parse_steps
	  return Ok(Self { #(#members: #vars),* });
	}
}

fn generate_range_validation(field_ident: &Ident, ty: &Type, range_expr: &ExprRange) -> TokenStream {
	let start = &range_expr.start;
	let end = &range_expr.end;
	let check = match (start, end) {
		// 1..=10 (inclusive end)
		(Some(start), Some(end)) => {
			quote! {
				if !(#start..=#end).contains(&i) {
					use ::css_parse::ToSpan;
					Err(crate::Diagnostic::new(c, crate::Diagnostic::number_out_of_bounds))?
				}
			}
		}
		(Some(start), None) => {
			quote! {
				if #start > i {
					use ::css_parse::ToSpan;
					Err(crate::Diagnostic::new(c, crate::Diagnostic::number_too_small))?
				}
			}
		}
		(None, Some(end)) => {
			quote! {
				if #end < i {
					use ::css_parse::ToSpan;
					Err(crate::Diagnostic::new(c, crate::Diagnostic::number_too_large))?
				}
			}
		}
		// .. (full range) - no validation needed
		(None, None) => {
			return quote! {};
		}
	};
	if ty.is_option() {
		quote! {
			if let Some(number_val) = #field_ident {
				if let Some(i) = ::css_parse::ToNumberValue::to_number_value(&number_val) {
					let c: ::css_parse::Cursor = number_val.into();
					#check
				}
			}
		}
	} else {
		quote! {
			if let Some(i) =::css_parse::ToNumberValue::to_number_value(&#field_ident) {
				let c: ::css_parse::Cursor = #field_ident.into();
				#check
			}
		}
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = &input.generics;
	let mut generic_with_alloc = generics.clone();
	let (impl_generics, type_generics, _) = if generics.lifetimes().all(|l| l.lifetime.ident != "a") {
		generic_with_alloc.params.insert(0, parse_quote!('a));
		let (impl_generics, _, where_clause) = generic_with_alloc.split_for_impl();
		let (_, type_generics, _) = generics.split_for_impl();
		(impl_generics, type_generics, where_clause)
	} else {
		generic_with_alloc.split_for_impl()
	};
	let mut pre_parse_steps = quote! {};
	let mut post_parse_steps = quote! {};
	let ParseArg { state, stop, parse_mode, .. } = (&input.attrs).into();
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

	let body = match &input.data {
		Data::Union(_) => return err(ident.span(), "Cannot derive Parse on a Union"),

		Data::Struct(DataStruct { fields, .. }) => {
			let members = fields.members();
			let split_fields = fields.to_vars_and_types();
			let members: Vec<TokenStream> = members.into_iter().map(|m| quote! { #m }).collect();
			if parse_mode == FieldParseMode::Sequential {
				generate_sequential_parsing(&split_fields, members, &post_parse_steps, &mut where_collector)
			} else {
				generate_must_occur_parsing(
					&split_fields,
					members,
					&post_parse_steps,
					parse_mode,
					quote! { Self },
					&mut where_collector,
				)
			}
		}
		Data::Enum(DataEnum { variants, .. }) => {
			let variant_data: Vec<_> = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					let ParseArg { parse_mode, .. } = (&variant.attrs).into();
					let atom = extract_atom(&variant.attrs);
					let members = variant.fields.members();
					let split_fields = variant.fields.to_vars_and_types();
					let first_type = split_fields
						.first()
						.map(|(_, ty, _, _)| ty.clone())
						.expect("Field has to have at least one type!");
					let members: Vec<TokenStream> = members.into_iter().map(|m| quote! { #m }).collect();

					let step = if parse_mode == FieldParseMode::Sequential {
						let parse_steps: Vec<TokenStream> = split_fields
							.iter()
							.map(|(var, ty, arg, atom)| {
								generate_field_parsing(
									var,
									ty,
									arg,
									atom,
									FieldParseMode::Sequential,
									&mut where_collector,
								)
							})
							.collect();
						let vars = split_fields.iter().map(|(var, _, _, _)| var);
						quote! {
						  #( #parse_steps )*
						  #post_parse_steps
						  return Ok(Self::#variant_ident { #(#members: #vars),* });
						}
					} else {
						let constructor = quote! { Self::#variant_ident };
						generate_must_occur_parsing(
							&split_fields,
							members,
							&post_parse_steps,
							parse_mode,
							constructor,
							&mut where_collector,
						)
					};

					let effective_atom = if let Some(variant_atom) = atom {
						Some(variant_atom)
					} else {
						variant.fields.iter().next().and_then(|field| extract_atom(&field.attrs))
					};

					(first_type, effective_atom, step, split_fields)
				})
				.collect();

			// Group by first type and atom status to separate atom variants from non-atom variants of the same type
			let grouped_variants = variant_data
				.into_iter()
				.sorted_by_key(|(ty, atom, _, _)| (quote!(#ty).to_string(), atom.is_none()))
				.chunk_by(|(ty, atom, _, _)| (quote!(#ty).to_string(), atom.is_none()));

			{
				grouped_variants
					.into_iter()
					.with_position()
					.map(|(pos, ((type_str, is_atom_group), group))| {
						let ty: Type = syn::parse_str(&type_str).unwrap();
						let variants: Vec<_> = group.collect();

						if !is_atom_group {
							let extract_atom: TokenStream = variants
								.first()
								.iter()
								.flat_map(|(_, atom, _, _)| atom)
								.map(|atom| atom.to_atom(format_ident!("c")))
								.collect();
							let atom_checks: TokenStream = variants
								.into_iter()
								.map(|(_, atom, step, _)| {
									let atom = atom.unwrap();
									let atom_path = atom.path();
									quote! { #atom_path => { #step }, }
								})
								.collect();

							if matches!(pos, Position::Last | Position::Only) {
								quote! {
									{
										let c = p.peek_n(1);
										match #extract_atom {
											#atom_checks
											_ => {
												return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?;
											}
										}
									}
								}
							} else {
								let type_check = quote! { p.peek::<#ty>() };
								quote! {
									if #type_check {
										let c = p.peek_n(1);
										match #extract_atom {
											#atom_checks
											_ => {}
										}
									}
								}
							}
						} else {
							let (_, _, step, split_fields) = variants.into_iter().next().unwrap();
							if matches!(pos, Position::Last | Position::Only) {
								quote! { { #step } }
							} else {
								// Generate peek condition for all types up to and including the first non-optional
								let mut peek_types = Vec::new();
								for (_, field_ty, _, _) in &split_fields {
									// Always add the type to peek for (unwrapping Option if needed)
									let peek_ty =
										if field_ty.is_option() { field_ty.unpack_option() } else { field_ty.clone() };
									peek_types.push(peek_ty);

									// If this field is non-optional, we've found our stopping point
									if !field_ty.is_option() {
										break;
									}
								}

								let type_checks: Vec<TokenStream> =
									peek_types.iter().map(|peek_ty| quote! { p.peek::<#peek_ty>() }).collect();

								let type_check = if type_checks.len() == 1 {
									type_checks.into_iter().next().unwrap()
								} else {
									quote! { #(#type_checks)||* }
								};

								quote! { if #type_check { #step } }
							}
						}
					})
					.collect()
			}
		}
	};

	let mut generics = input.generics.clone();
	let where_clause = where_collector.extend_where_clause(&mut generics, parse_quote! { ::css_parse::Parse<'a> });

	quote! {
	  #[automatically_derived]
	  impl #impl_generics ::css_parse::Parse<'a> for #ident #type_generics #where_clause {
		fn parse<I>(p: &mut css_parse::Parser<'a, I>) -> css_parse::Result<Self>
		where
			I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
		{
		  use css_parse::{Parse, Peek};
			#pre_parse_steps
		  #body
		}
	  }
	}
}
