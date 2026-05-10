use crate::{
	FieldsExt, WhereCollector,
	attributes::{Atom, extract_atom, extract_field_parse_mode},
	ensure_lifetime_a,
	field_view::option_inner,
};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use std::collections::{HashMap, hash_map::Entry};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Ident, Result, Type, parse_quote};

/// Map indivudal type names to a Kind, if possible.
fn map_type_to_kind(ty: &Type) -> Option<&'static str> {
	if let Type::Path(path) = option_inner(ty).unwrap_or(ty)
		&& let Some(seg) = path.path.segments.last()
	{
		match seg.ident.to_string().as_str() {
			"Ident" => Some("Ident"),
			"String" => Some("String"),
			"Number" => Some("Number"),
			"Dimension" => Some("Number"),
			"Function" => Some("Function"),
			"AtKeyword" => Some("AtKeyword"),
			"Hash" => Some("Hash"),
			"Delim" => Some("Delim"),
			_ => None,
		}
	} else {
		None
	}
}

/// Build a peek check for a single type, optionally constrained to a set of atoms.
/// `atoms` empty means unconstrained (just `peek`); non-empty means `peek && (atom || atom || ...)`.
fn generate_type_peek(ty: &Type, atoms: &[Atom], where_collector: &mut WhereCollector) -> TokenStream {
	where_collector.add(ty);
	if atoms.is_empty() {
		quote! { <#ty>::peek(p, c) }
	} else {
		let atom_checks = atoms.iter().map(|a| {
			let path = a.path();
			quote! { p.equals_atom(c.into(), &#path) }
		});
		quote! { (<#ty>::peek(p, c) && (#(#atom_checks)||*)) }
	}
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = &input.generics;
	let generic_with_a = ensure_lifetime_a(generics);
	let (impl_generics, _, _) = generic_with_a.split_for_impl();
	let (_, type_generics, _) = generics.split_for_impl();
	let mut kinds = vec![];
	let body = match input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive Peek on a Union")),

		Data::Struct(DataStruct { ref fields, .. }) => {
			let parse_mode = extract_field_parse_mode(&input.attrs)?;
			let mut checks: Vec<TokenStream> = vec![];
			for (view, syn_field) in fields.views().into_iter().zip(fields.iter()) {
				let atom = extract_atom(&syn_field.attrs)?;
				let atoms = atom.map(|a| vec![a]).unwrap_or_default();
				let peek_ty = option_inner(view.ty).unwrap_or(view.ty);
				if let Some(kind) = map_type_to_kind(peek_ty) {
					let ident = Ident::new(kind, Span::call_site());
					kinds.push(quote! { ::css_lexer::Kind::#ident });
				} else {
					checks.push(generate_type_peek(peek_ty, &atoms, &mut where_collector));
				}
				if !parse_mode.any_field_can_start() && !view.is_option {
					break;
				}
			}
			quote! { #(#checks)||* }
		}

		Data::Enum(DataEnum { variants, .. }) => {
			let mut seen: Vec<String> = vec![];
			let mut by_type: HashMap<String, Option<Vec<Atom>>> = HashMap::new();

			let mut register = |peek_ty: &Type, atom: Option<Atom>| {
				let key = peek_ty.to_token_stream().to_string();
				if atom.is_none()
					&& let Some(kind) = map_type_to_kind(peek_ty)
				{
					if !seen.contains(&kind.to_string()) {
						seen.push(kind.to_string());
						let ident = Ident::new(kind, Span::call_site());
						kinds.push(quote! { ::css_lexer::Kind::#ident });
					}
					return;
				}
				match by_type.entry(key.clone()) {
					Entry::Vacant(e) => {
						seen.push(key);
						e.insert(atom.map(|a| vec![a]));
					}
					Entry::Occupied(mut e) => match (e.get_mut(), atom) {
						(None, _) => {}
						(slot @ Some(_), None) => *slot = None,
						(Some(existing), Some(a)) => {
							let path = a.path().to_token_stream().to_string();
							if !existing.iter().any(|x| x.path().to_token_stream().to_string() == path) {
								existing.push(a);
							}
						}
					},
				}
			};

			for variant in variants.iter() {
				let views = variant.fields.views();
				let parse_mode = extract_field_parse_mode(&variant.attrs)?;
				for (view, syn_field) in views.iter().zip(variant.fields.iter()) {
					let atom = match extract_atom(&variant.attrs)? {
						Some(a) => Some(a),
						None => extract_atom(&syn_field.attrs)?,
					};
					let option = option_inner(view.ty);
					let peek_ty = option.unwrap_or(view.ty);
					register(peek_ty, atom);
					if !parse_mode.any_field_can_start() && option.is_none() {
						break;
					}
				}
			}

			let mut type_checks: Vec<TokenStream> = vec![];
			for key in &seen {
				let Some(entry) = by_type.remove(key) else {
					continue;
				};
				let ty = variants
					.iter()
					.find_map(|v| {
						v.fields.views().into_iter().find_map(|vw| {
							let peek_ty = option_inner(vw.ty).unwrap_or(vw.ty);
							if peek_ty.to_token_stream().to_string() == *key { Some(peek_ty.clone()) } else { None }
						})
					})
					.unwrap();
				let atoms = entry.unwrap_or_default();
				type_checks.push(generate_type_peek(&ty, &atoms, &mut where_collector));
			}
			quote! { #(#type_checks)||* }
		}
	};

	let generics = input.generics.clone();
	let where_clause = where_collector.extend_where_clause(&generics, parse_quote! { ::css_parse::Peek<'a> });
	let (peek_kindset, kindset_cond) = if kinds.is_empty() {
		(None, None)
	} else {
		(
			Some(quote! {
			  const PEEK_KINDSET: ::css_lexer::KindSet = ::css_lexer::KindSet::new(&[ #(#kinds),* ]);
			}),
			Some(quote! { c == Self::PEEK_KINDSET ||  }),
		)
	};
	let peek_fn = if body.is_empty() {
		None
	} else {
		Some(quote! {
			#[inline(always)]
			fn peek<I>(p: &::css_parse::Parser<'a, I>, c: ::css_parse::Cursor) -> bool
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
				{
				use ::css_parse::{Peek};
				#kindset_cond #body
			}
		})
	};

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Peek<'a> for #ident #type_generics #where_clause {
			#peek_kindset
			#peek_fn
		}
	})
}
