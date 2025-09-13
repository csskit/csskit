use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, ExprPath, ExprRange, Ident, Meta};

/// Extract #[in_range(...)] attribute from a field
pub fn extract_in_range(attrs: &[Attribute]) -> Option<ExprRange> {
	if let Some(Attribute { meta, .. }) = attrs.iter().find(|a| a.path().is_ident("in_range")) {
		match meta {
			Meta::List(meta) => {
				let range = meta.parse_args::<syn::Expr>().unwrap();
				if let syn::Expr::Range(range_expr) = range {
					Some(range_expr)
				} else {
					panic!("Expected range expression in #[in_range(...)]");
				}
			}
			_ => panic!("could not parse in_range meta"),
		}
	} else {
		None
	}
}

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
		self.0.path.segments.first().expect("keyword variant path should have at least one segment").ident.clone()
	}
}

impl ToTokens for Atom {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens)
	}
}

/// Extract #[atom(...)] attribute from a field
pub fn extract_atom(attrs: &[Attribute]) -> Option<Atom> {
	if let Some(Attribute { meta, .. }) = attrs.iter().find(|a| a.path().is_ident("atom")) {
		match meta {
			Meta::List(meta) => meta.parse_args::<ExprPath>().ok().map(Atom),
			_ => panic!("could not parse in_range meta"),
		}
	} else {
		None
	}
}
