#![deny(warnings)]
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::Error;

mod into_cursor;
mod parse;
mod peek;
mod to_cursors;
mod to_span;
mod visitable;

#[proc_macro_derive(ToCursors, attributes(to_cursors))]
pub fn derive_to_cursors(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	to_cursors::derive(input).into()
}

#[proc_macro_derive(Parse, attributes(parse))]
pub fn derive_parse(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	parse::derive(input).into()
}

#[proc_macro_derive(Peek, attributes(peek))]
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

#[proc_macro_derive(Visitable, attributes(visit))]
pub fn derive_visitable(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	visitable::derive(input).into()
}

fn err(span: Span, msg: &str) -> proc_macro2::TokenStream {
	let err = Error::new(span, msg).into_compile_error();
	quote::quote! {#err}
}
