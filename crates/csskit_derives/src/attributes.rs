use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, Error, ExprPath, Ident, Meta, Result};

#[derive(Debug)]
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
