#![deny(warnings)]
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{AngleBracketedGenericArguments, Error, GenericArgument, PathArguments, PathSegment, Type, TypePath};

mod attributes;
mod css_feature;
mod declaration_metadata;
mod into_cursor;
mod parse;
mod peek;
mod semantic_eq;
mod to_cursors;
mod to_span;
mod visitable;
mod where_collector;

use where_collector::WhereCollector;

#[cfg(test)]
mod test;

#[proc_macro_derive(ToCursors, attributes(to_cursors))]
pub fn derive_to_cursors(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	to_cursors::derive(input).into()
}

#[proc_macro_derive(Parse, attributes(parse, in_range, atom))]
pub fn derive_parse(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	parse::derive(input).into()
}

#[proc_macro_derive(Peek, attributes(peek, in_range, atom))]
pub fn derive_peek(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	peek::derive(input).into()
}

#[proc_macro_derive(IntoCursor)]
pub fn derive_into_cursor(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	into_cursor::derive(input).into()
}

#[proc_macro_derive(ToSpan)]
pub fn derive_into_span(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	to_span::derive(input).into()
}

#[proc_macro_derive(Visitable, attributes(visit, metadata))]
pub fn derive_visitable(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	visitable::derive(input).into()
}

#[proc_macro_derive(ToCSSFeature, attributes(css_feature))]
pub fn derive_css_feature(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	css_feature::derive(input).into()
}

#[proc_macro_derive(DeclarationMetadata, attributes(declaration_metadata))]
pub fn derive_declaration_metadata(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	declaration_metadata::derive(input).into()
}

#[proc_macro_derive(SemanticEq, attributes(semantic_eq))]
pub fn derive_semantic_eq(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	semantic_eq::derive(input).into()
}

fn err(span: Span, msg: &str) -> proc_macro2::TokenStream {
	let err = Error::new(span, msg).into_compile_error();
	quote::quote! {#err}
}

trait TypeIsOption {
	fn is_option(&self) -> bool;
	fn unpack_option(&self) -> Self;
}

impl TypeIsOption for Type {
	fn is_option(&self) -> bool {
		match self {
			Self::Path(TypePath { path, .. }) => path.segments.last().is_some_and(|s| s.ident == "Option"),
			_ => false,
		}
	}

	fn unpack_option(&self) -> Self {
		if let Self::Path(TypePath { path, .. }) = self
			&& let Some(PathSegment {
				ident,
				arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
				..
			}) = path.segments.last()
			&& ident == "Option"
			&& args.len() == 1
			&& let GenericArgument::Type(inner_ty) = &args[0]
		{
			return inner_ty.clone();
		}
		self.clone()
	}
}
