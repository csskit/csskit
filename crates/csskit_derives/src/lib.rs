#![deny(warnings)]
use proc_macro::TokenStream;
use syn::{DeriveInput, Generics, Result, parse_quote};

mod attributes;
mod css_feature;
mod darling_ext;
mod declaration_metadata;
mod field_view;
mod into_cursor;
mod node_with_metadata;
mod parse;
mod peek;
mod semantic_eq;
mod to_cursors;
mod to_span;
mod visitable;
mod where_collector;

use field_view::FieldsExt;
use where_collector::WhereCollector;

fn ensure_lifetime_a(generics: &Generics) -> Generics {
	let mut g = generics.clone();
	if generics.lifetimes().all(|l| l.lifetime.ident != "a") {
		g.params.insert(0, parse_quote!('a));
	}
	g
}

#[cfg(test)]
mod test;

fn run<F>(stream: TokenStream, f: F) -> TokenStream
where
	F: FnOnce(DeriveInput) -> Result<proc_macro2::TokenStream>,
{
	let input = syn::parse::<DeriveInput>(stream).unwrap_or_else(|e| panic!("{e}"));
	f(input).unwrap_or_else(|e| e.into_compile_error()).into()
}

#[proc_macro_derive(ToCursors, attributes(to_cursors))]
pub fn derive_to_cursors(stream: TokenStream) -> TokenStream {
	run(stream, to_cursors::derive)
}

#[proc_macro_derive(Parse, attributes(parse, atom))]
pub fn derive_parse(stream: TokenStream) -> TokenStream {
	run(stream, parse::derive)
}

#[proc_macro_derive(Peek, attributes(peek, atom))]
pub fn derive_peek(stream: TokenStream) -> TokenStream {
	run(stream, peek::derive)
}

#[proc_macro_derive(IntoCursor)]
pub fn derive_into_cursor(stream: TokenStream) -> TokenStream {
	run(stream, into_cursor::derive)
}

#[proc_macro_derive(ToSpan)]
pub fn derive_into_span(stream: TokenStream) -> TokenStream {
	run(stream, to_span::derive)
}

#[proc_macro_derive(Visitable, attributes(visit, queryable))]
pub fn derive_visitable(stream: TokenStream) -> TokenStream {
	run(stream, visitable::derive)
}

#[proc_macro_derive(NodeWithMetadata, attributes(metadata))]
pub fn derive_node_with_metadata(stream: TokenStream) -> TokenStream {
	run(stream, node_with_metadata::derive)
}

#[proc_macro_derive(ToCSSFeature, attributes(css_feature))]
pub fn derive_css_feature(stream: TokenStream) -> TokenStream {
	run(stream, css_feature::derive)
}

#[proc_macro_derive(DeclarationMetadata, attributes(declaration_metadata))]
pub fn derive_declaration_metadata(stream: TokenStream) -> TokenStream {
	run(stream, declaration_metadata::derive)
}

#[proc_macro_derive(SemanticEq, attributes(semantic_eq))]
pub fn derive_semantic_eq(stream: TokenStream) -> TokenStream {
	run(stream, semantic_eq::derive)
}
