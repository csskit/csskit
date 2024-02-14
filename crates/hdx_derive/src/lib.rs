use proc_macro::TokenStream;

mod string_transform;

mod atomizable;
mod parsable;
mod writable;

use proc_macro2::Span;
pub(crate) use string_transform::*;
use syn::Error;

#[proc_macro_derive(Atomizable, attributes(atomizable))]
pub fn derive_atomizable(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	atomizable::derive(input).into()
}

#[proc_macro_derive(Parsable, attributes(parsable))]
pub fn derive_parsable(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	parsable::derive(input).into()
}

#[proc_macro_derive(Writable, attributes(writable))]
pub fn derive_writable(stream: TokenStream) -> TokenStream {
	let input = syn::parse(stream).unwrap();
	writable::derive(input).into()
}

fn err(span: Span, msg: &str) -> proc_macro2::TokenStream {
	let err = Error::new(span, msg).into_compile_error();
	quote::quote! {#err}
}
