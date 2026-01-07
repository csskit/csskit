#![deny(warnings)]
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod generate;
mod syntax;
mod type_renames;

#[cfg(test)]
mod test;

use css_value_definition_parser::{Def, StrWrapped};

#[proc_macro_attribute]
pub fn syntax(args: TokenStream, input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(args as StrWrapped<Def>);
	let ast = parse_macro_input!(input as DeriveInput);
	syntax::generate(args.0.optimize(), ast).into()
}

#[proc_macro_attribute]
pub fn visit(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
}
