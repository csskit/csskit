use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Error, ExprPath, Ident, Meta, Result};

/// How fields within a struct variant must be satisfied.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldParseMode {
	/// Fields are consumed in order; first non-optional field ends the variant.
	#[default]
	Sequential,
	/// All optional fields must appear (in any order).
	AllMustOccur,
	/// At least one optional field must appear (in any order).
	OneMustOccur,
}

impl FieldParseMode {
	/// Any field can be the first token (unordered multi-field modes).
	pub fn any_field_can_start(self) -> bool {
		matches!(self, Self::AllMustOccur | Self::OneMustOccur)
	}
}

#[derive(Debug, Default, FromAttributes)]
#[darling(attributes(parse))]
struct ParseModeArgs {
	#[darling(default)]
	all_must_occur: bool,
	#[darling(default)]
	one_must_occur: bool,
}

pub fn extract_field_parse_mode(attrs: &[Attribute]) -> Result<FieldParseMode> {
	let args = ParseModeArgs::from_attributes(attrs)
		.map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e.to_string()))?;
	Ok(match (args.all_must_occur, args.one_must_occur) {
		(true, _) => FieldParseMode::AllMustOccur,
		(_, true) => FieldParseMode::OneMustOccur,
		_ => FieldParseMode::Sequential,
	})
}

#[derive(Debug, Clone)]
pub struct Atom(ExprPath);

impl Atom {
	pub fn path(&self) -> ExprPath {
		self.0.clone()
	}

	pub fn equals_atom(&self, cursor: Ident) -> TokenStream {
		let atom = self.path();
		quote! { p.equals_atom(#cursor.into(), &#atom) }
	}

	pub fn to_atom(&self, cursor: Ident) -> TokenStream {
		let atom_set = self.first_segment();
		quote! { p.to_atom::<#atom_set>(#cursor) }
	}

	pub fn first_segment(&self) -> Ident {
		self.0.path.segments.first().expect("atom path must have at least one segment").ident.clone()
	}

	pub fn binding_block(&self) -> TokenStream {
		let atom_set = self.first_segment();
		quote! {
			let atom = if p.peek::<::css_parse::token_macros::Ident>() {
				p.to_atom::<#atom_set>(c)
			} else {
				<#atom_set>::default()
			};
		}
	}

	pub fn opt_binding_block(atom: Option<&Self>) -> TokenStream {
		atom.map(Self::binding_block).unwrap_or_default()
	}
}

impl ToTokens for Atom {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens)
	}
}

pub fn extract_atom(attrs: &[Attribute]) -> Result<Option<Atom>> {
	let Some(attr) = attrs.iter().find(|a| a.path().is_ident("atom")) else {
		return Ok(None);
	};
	match &attr.meta {
		Meta::List(meta) => Ok(Some(Atom(meta.parse_args::<ExprPath>()?))),
		_ => Err(Error::new_spanned(&attr.meta, "#[atom] requires a path argument, e.g. #[atom(MyAtomSet::my_atom)]")),
	}
}
