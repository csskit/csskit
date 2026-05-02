use crate::{
	TypeIsOption, WhereCollector,
	attributes::{Atom, extract_atom},
	err,
};
use itertools::{Itertools, Position};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Meta, Token, Type, TypePath, parse::Parse,
	parse_quote,
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
				}
				_ => panic!("could not parse meta"),
			}
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
	_arg: &ParseArg,
	parse_mode: FieldParseMode,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	match parse_mode {
		FieldParseMode::Sequential => {
			let condition = atom.equals_atom(format_ident!("c"));
			let inner_ty = ty.unpack_option();
			where_collector.add(&inner_ty);
			if ty.is_option() {
				quote! {
					let #var = {
						let c = p.peek_n(1);
						if #condition {
							Some(p.parse::<#inner_ty>()?)
						} else {
							None
						}
					};
				}
			} else {
				quote! {
					let #var = {
						let c = p.peek_n(1);
						if #condition {
							p.parse::<#inner_ty>()?
						} else {
							return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?;
						}
					};
				}
			}
		}
		FieldParseMode::AllMustOccur | FieldParseMode::OneMustOccur => {
			let atom = atom.path();
			let ty = ty.unpack_option();
			where_collector.add(&ty);
			quote! {
				if #var.is_none() && atom == #atom {
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
			if let Some(state_ident) = &arg.state {
				quote! {
					let #var = {
						let old_state = p.set_state(State::#state_ident);
						let result = p.parse::<#ty>()?;
						p.set_state(old_state);
						result
					};
				}
			} else {
				quote! { let #var = p.parse::<#ty>()?; }
			}
		}
		FieldParseMode::AllMustOccur | FieldParseMode::OneMustOccur => {
			let ty = ty.unpack_option();
			where_collector.add(&ty);
			quote! {
			  if #var.is_none() && <#ty>::peek(p, c) {
					#var = Some(p.parse::<#ty>()?);
					continue;
			  }
			}
		}
	}
}

/// Generate a pre-loop that parses shared fields before variant discrimination.
/// `shared_fields` are fields present in all sibling variants (same atom path).
/// Returns `(binding_decls, loop_body)` so callers can emit them before the variant dispatch.
fn generate_must_occur_parsing(
	split_fields: &[(Ident, Type, ParseArg, Option<Atom>)],
	members: Vec<TokenStream>,
	post_parse_steps: &TokenStream,
	parse_mode: FieldParseMode,
	constructor: TokenStream,
	where_collector: &mut WhereCollector,
) -> TokenStream {
	let mut atom_binding = None;
	let mut atom_set_ty = None;
	let bindings: Vec<TokenStream> = split_fields
		.iter()
		.map(|(var, ty, _, atom)| {
			if atom.is_some() && atom_binding.is_none() {
				let a = atom.as_ref().unwrap();
				let atom_expr = a.to_atom(format_ident!("c"));
				let atom_set = a.first_segment();
				atom_set_ty = Some(atom_set.clone());
				atom_binding = Some(quote! { let atom = #atom_expr; });
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

	let atom_binding_guarded = if let Some(atom_set) = atom_set_ty {
		quote! {
			let atom = if p.peek::<::css_parse::token_macros::Ident>() {
				p.to_atom::<#atom_set>(c)
			} else {
				<#atom_set>::default()
			};
		}
	} else {
		quote! { #atom_binding }
	};
	quote! {
	  #(#bindings)*
	  loop {
			let c = p.peek_n(1);
			#atom_binding_guarded
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
							members.clone(),
							&post_parse_steps,
							parse_mode,
							constructor,
							&mut where_collector,
						)
					};

					let effective_atom = if let Some(variant_atom) = atom {
						Some(variant_atom)
					} else if parse_mode == FieldParseMode::OneMustOccur
						&& split_fields.iter().all(|(_, ty, _, _)| ty.is_option())
					{
						None
					} else {
						variant.fields.iter().next().and_then(|field| extract_atom(&field.attrs))
					};

					// Store variant_ident, members, parse_mode alongside so the sibling block can
					// regenerate steps with hoisted vars when needed.
					let variant_ident_ts = quote! { #variant_ident };
					(first_type, effective_atom, step, split_fields, variant_ident_ts, members, parse_mode)
				})
				.collect();

			// Group by first type and atom status to separate atom variants from non-atom variants of the same type
			let grouped_variants = variant_data
				.into_iter()
				.sorted_by_key(|(ty, atom, _, _, _, _, _)| (quote!(#ty).to_string(), atom.is_none()))
				.chunk_by(|(ty, atom, _, _, _, _, _)| (quote!(#ty).to_string(), atom.is_none()));

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
								.flat_map(|(_, atom, _, _, _, _, _)| atom)
								.map(|atom| atom.to_atom(format_ident!("c")))
								.collect();
							let atom_checks: TokenStream = variants
								.into_iter()
								.map(|(_, atom, step, _, _, _, _)| {
									let atom = atom.unwrap();
									let atom_path = atom.path();
									quote! { #atom_path => { #step }, }
								})
								.collect();

							let type_check = quote! { p.peek::<#ty>() };
							if matches!(pos, Position::Last | Position::Only) {
								quote! {
									if #type_check {
										let c = p.peek_n(1);
										match #extract_atom {
											#atom_checks
											_ => {
												return Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?;
											}
										}
									} else {
										return Err(crate::Diagnostic::new(p.peek_n(1), crate::Diagnostic::unexpected))?;
									}
								}
							} else {
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
					let variants: Vec<_> = variants.into_iter().collect();
					let last_idx = variants.len() - 1;

					// For multiple sibling one_must_occur variants, hoist fields whose atoms
					// appear in every variant so they can be parsed before the discriminator
					// (enabling orderings like `balance wrap` and `balance wrap-reverse`).
					let shared_atom_paths: Vec<String> = if variants.len() > 1 {
						let first_atoms: Vec<String> = variants[0]
							.3
							.iter()
							.filter_map(|(_, _, _, atom)| atom.as_ref().map(|a| quote!(#a).to_string()))
							.collect();
						first_atoms
							.into_iter()
							.filter(|ap| {
								variants[1..].iter().all(|(_, _, _, sf, _, _, _)| {
									sf.iter().any(|(_, _, _, a)| a.as_ref().map(|a| quote!(#a).to_string()).as_deref() == Some(ap))
								})
							})
							.collect()
					} else {
						vec![]
					};

					// Build hoisted binding declarations and pre-loop for shared fields.
					// Use first variant's split_fields as representative (all share these atoms).
					let hoisted_var_names: Vec<&Ident> = variants[0]
						.3
						.iter()
						.filter(|(_, _, _, atom)| {
							atom.as_ref().map(|a| shared_atom_paths.contains(&quote!(#a).to_string())).unwrap_or(false)
						})
						.map(|(var, _, _, _)| var)
						.collect();

					let hoisted_bindings: TokenStream = variants[0]
						.3
						.iter()
						.filter(|(_, _, _, atom)| {
							atom.as_ref().map(|a| shared_atom_paths.contains(&quote!(#a).to_string())).unwrap_or(false)
						})
						.map(|(var, ty, _, _)| {
							if ty.is_option() {
								quote! { let mut #var: #ty = None; }
							} else {
								quote! { let mut #var: Option<#ty> = None; }
							}
						})
						.collect();

					let hoisted_preloop: TokenStream = if !shared_atom_paths.is_empty() {
						let atom_set = variants[0]
							.3
							.iter()
							.find_map(|(_, _, _, atom)| {
								atom.as_ref().filter(|a| shared_atom_paths.contains(&quote!(#a).to_string())).map(|a| a.first_segment().clone())
							});
						let parse_steps: Vec<TokenStream> = variants[0]
							.3
							.iter()
							.filter(|(_, _, _, atom)| {
								atom.as_ref().map(|a| shared_atom_paths.contains(&quote!(#a).to_string())).unwrap_or(false)
							})
							.map(|(var, ty, _, atom)| {
								let atom_path = atom.as_ref().unwrap().path();
								let inner_ty = ty.unpack_option();
								quote! {
									if #var.is_none() && atom == #atom_path {
										#var = Some(p.parse::<#inner_ty>()?);
										continue;
									}
								}
							})
							.collect();
						if let Some(atom_set) = atom_set {
							quote! {
								loop {
									let c = p.peek_n(1);
									let atom = if p.peek::<::css_parse::token_macros::Ident>() {
										p.to_atom::<#atom_set>(c)
									} else {
										<#atom_set>::default()
									};
									#(#parse_steps)*
									break;
								}
							}
						} else {
							quote! {}
						}
					} else {
						quote! {}
					};

					let variant_steps: TokenStream = variants
						.iter()
						.enumerate()
						.map(|(idx, (_, _, _step, split_fields, variant_ident_ts, members, _parse_mode))| {
							let is_last_variant = matches!(pos, Position::Last | Position::Only) && idx == last_idx;
							if is_last_variant && idx == 0 && shared_atom_paths.is_empty() {
								// Single variant, last in outer sequence, no hoisting needed
								quote! { { #_step } }
							} else {
								// Find discriminating atom: first atom NOT in shared set
								let discriminating_field = split_fields.iter().find(|(_, _, _, atom)| {
									atom.as_ref().map(|a| !shared_atom_paths.contains(&quote!(#a).to_string())).unwrap_or(false)
								});
								let type_check = if let Some((_, field_ty, _, Some(atom))) = discriminating_field {
									let peek_ty =
										if field_ty.is_option() { field_ty.unpack_option() } else { field_ty.clone() };
									let atom_path = atom.path();
									let atom_set = atom.first_segment();
									quote! { p.peek::<#peek_ty>() && p.to_atom::<#atom_set>(p.peek_n(1)) == #atom_path }
								} else if let Some((_, field_ty, _, Some(atom))) = split_fields.iter().find(|(_, _, _, atom)| atom.is_some()) {
									// All fields are shared; fall back to first atom field (single variant case)
									let peek_ty =
										if field_ty.is_option() { field_ty.unpack_option() } else { field_ty.clone() };
									let atom_path = atom.path();
									let atom_set = atom.first_segment();
									quote! { p.peek::<#peek_ty>() && p.to_atom::<#atom_set>(p.peek_n(1)) == #atom_path }
								} else {
									// Fallback: peek all types up to and including first non-optional
									let mut peek_types = Vec::new();
									for (_, field_ty, _, _) in split_fields {
										let peek_ty = if field_ty.is_option() {
											field_ty.unpack_option()
										} else {
											field_ty.clone()
										};
										peek_types.push(peek_ty);
										if !field_ty.is_option() {
											break;
										}
									}
									let type_checks: Vec<TokenStream> =
										peek_types.iter().map(|t| quote! { p.peek::<#t>() }).collect();
									if type_checks.len() == 1 {
										type_checks.into_iter().next().unwrap()
									} else {
										quote! { #(#type_checks)||* }
									}
								};

								let final_step: TokenStream = if hoisted_var_names.is_empty() {
									quote! { #_step }
								} else {
									// Regenerate step excluding hoisted binding declarations.
									// Hoisted vars are already in scope from outer bindings.
									let atom_set = split_fields.iter().find_map(|(_, _, _, atom)| {
										atom.as_ref().map(|a| a.first_segment().clone())
									});
									let non_hoisted_bindings: Vec<TokenStream> = split_fields
										.iter()
										.filter(|(var, _, _, _)| !hoisted_var_names.contains(&var))
										.map(|(var, ty, _, _)| {
											if ty.is_option() {
												quote! { let mut #var: #ty = None; }
											} else {
												quote! { let mut #var: Option<#ty> = None; }
											}
										})
										.collect();
									let loop_parse_steps: Vec<TokenStream> = split_fields
										.iter()
										.map(|(var, ty, _, atom)| {
											if let Some(atom) = atom {
												let atom_path = atom.path();
												let inner_ty = ty.unpack_option();
												quote! {
													if #var.is_none() && atom == #atom_path {
														#var = Some(p.parse::<#inner_ty>()?);
														continue;
													}
												}
											} else {
												quote! {}
											}
										})
										.collect();
									let all_vars: Vec<&Ident> = split_fields.iter().map(|(v, _, _, _)| v).collect();
									let all_checks: Vec<TokenStream> = all_vars.iter().map(|v| quote! { #v.is_none() }).collect();
									let all_assignments: Vec<TokenStream> = all_vars.iter().map(|v| quote! { #v }).collect();
									let atom_binding_guarded = if let Some(atom_set) = atom_set {
										quote! {
											let atom = if p.peek::<::css_parse::token_macros::Ident>() {
												p.to_atom::<#atom_set>(c)
											} else {
												<#atom_set>::default()
											};
										}
									} else {
										quote! {}
									};
									quote! {
										#(#non_hoisted_bindings)*
										loop {
											let c = p.peek_n(1);
											#atom_binding_guarded
											#(#loop_parse_steps)*
											break;
										}
										if #(#all_checks)&&* {
											let c = p.peek_n(1);
											Err(crate::Diagnostic::new(c, crate::Diagnostic::unexpected))?
										}
										return Ok(Self::#variant_ident_ts { #(#members: #all_assignments),* });
									}
								};

								if is_last_variant {
									quote! { if #type_check { #final_step } else { return Err(crate::Diagnostic::new(p.peek_n(1), crate::Diagnostic::unexpected))?; } }
								} else {
									quote! { if #type_check { #final_step } }
								}
							}
						})
						.collect();

					quote! {
						#hoisted_bindings
						#hoisted_preloop
						#variant_steps
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
