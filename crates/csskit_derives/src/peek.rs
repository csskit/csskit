use crate::{
	FieldsExt, WhereCollector,
	attributes::{Atom, extract_atom, extract_field_parse_mode, extract_peek_skip},
	ensure_lifetime_a,
	field_view::option_inner,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Error, Fields, Ident, Result, Type, parse_quote};

/// The lexer kind which always starts a token type, if it is one.
fn kind_of(ty: &Type) -> Option<&'static str> {
	if let Type::Path(path) = ty
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

/// The types which can start a node, each with the atom it must equal: the leading run of optional
/// fields up to and including the first field which is always present, or every field when the parse
/// mode lets any field come first. An atom on the whole node takes precedence over a field's own.
fn starts<'f>(attrs: &[Attribute], fields: &'f Fields) -> Result<Vec<(&'f Type, Option<Atom>)>> {
	let any_field_can_start = extract_field_parse_mode(attrs)?.any_field_can_start();
	let atom = extract_atom(attrs)?;
	let mut starts = vec![];
	for (view, field) in fields.views().into_iter().zip(fields.iter()) {
		if extract_peek_skip(&field.attrs) {
			continue;
		}
		let atom = match &atom {
			Some(atom) => Some(atom.clone()),
			None => extract_atom(&field.attrs)?,
		};
		starts.push((option_inner(view.ty).unwrap_or(view.ty), atom));
		if !any_field_can_start && !view.is_option {
			break;
		}
	}
	Ok(starts)
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let mut where_collector = WhereCollector::new();
	let ident = input.ident;
	let generics = &input.generics;
	let generic_with_a = ensure_lifetime_a(generics);
	let (impl_generics, _, _) = generic_with_a.split_for_impl();
	let (_, type_generics, _) = generics.split_for_impl();

	let starts = match &input.data {
		Data::Union(_) => return Err(Error::new(ident.span(), "Cannot derive Peek on a Union")),
		Data::Struct(data) => starts(&input.attrs, &data.fields)?,
		Data::Enum(data) => {
			let mut starts = vec![];
			for variant in data.variants.iter().filter(|variant| !extract_peek_skip(&variant.attrs)) {
				starts.extend(self::starts(&variant.attrs, &variant.fields)?);
			}
			starts
		}
	};

	// Bare token types collapse into one kind set. Other types check `peek`, once per type, with any
	// atoms the type was given unioned; a type which appears once without an atom is unconstrained.
	let mut kinds: Vec<Ident> = vec![];
	let mut checks: Vec<(&Type, Option<Vec<Atom>>)> = vec![];
	for (ty, atom) in starts {
		match (kind_of(ty), atom) {
			(Some(kind), None) => {
				let kind = Ident::new(kind, Span::call_site());
				if !kinds.contains(&kind) {
					kinds.push(kind);
				}
			}
			(_, atom) => match checks.iter_mut().find(|(seen, _)| *seen == ty) {
				None => checks.push((ty, atom.map(|atom| vec![atom]))),
				Some((_, atoms)) => match (atoms, atom) {
					(Some(atoms), Some(atom)) => {
						if !atoms.contains(&atom) {
							atoms.push(atom);
						}
					}
					(atoms, None) => *atoms = None,
					(None, _) => {}
				},
			},
		}
	}

	let peek_kindset = (!kinds.is_empty()).then(|| {
		quote! {
			const PEEK_KINDSET: ::css_lexer::KindSet = ::css_lexer::KindSet::new(&[ #(::css_lexer::Kind::#kinds),* ]);
		}
	});
	let peek_fn = (!checks.is_empty()).then(|| {
		let kindset_cond = peek_kindset.is_some().then(|| quote! { c == Self::PEEK_KINDSET || });
		let checks = checks.iter().map(|(ty, atoms)| {
			where_collector.add(ty);
			match atoms {
				None => quote! { <#ty>::peek(p, c) },
				Some(atoms) => quote! { (<#ty>::peek(p, c) && (#(p.equals_atom(c.into(), &#atoms))||*)) },
			}
		});
		quote! {
			#[inline(always)]
			fn peek<I>(p: &::css_parse::Parser<'a, I>, c: ::css_parse::Cursor) -> bool
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
				{
				use ::css_parse::{Peek};
				#kindset_cond #(#checks)||*
			}
		}
	});

	let where_clause = where_collector.extend_where_clause(generics, parse_quote! { ::css_parse::Peek<'a> });

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics ::css_parse::Peek<'a> for #ident #type_generics #where_clause {
			#peek_kindset
			#peek_fn
		}
	})
}
