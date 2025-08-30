#![deny(warnings)]
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod def;
mod generate;
mod initial;
mod syntax;
// mod applies_to;
mod inherited;
// mod canonical_order;
// mod logical_property_group;

#[cfg(test)]
mod test;

use def::{Def, StrWrapped};

#[proc_macro_attribute]
pub fn syntax(args: TokenStream, input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(args as StrWrapped<Def>);
	let ast = parse_macro_input!(input as DeriveInput);
	syntax::generate(args.0.optimize(), ast).into()
}

#[proc_macro_attribute]
pub fn initial(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args_string = args.to_string();
	// // CSS specs have a variety of "see individual properties" styles
	// let args = if args_string == "\"see individual properties\""
	// {
	// 	StrWrapped(initial::Args::Individual)
	// } else {
	// 	parse_macro_input!(args as StrWrapped<initial::Args>)
	// };
	// let ast = parse_macro_input!(input as DeriveInput);
	// initial::generate(args.0, ast).into()
}

#[proc_macro_attribute]
pub fn applies_to(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args = parse_macro_input!(args as applies_to::Args);
	// let ast = parse_macro_input!(input as DeriveInput);
	// applies_to::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn inherited(args: TokenStream, input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(args as inherited::Args);
	let ast = parse_macro_input!(input as DeriveInput);
	inherited::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn percentages(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args = parse_macro_input!(args as inherited::Args);
	// let ast = parse_macro_input!(input as DeriveInput);
	// inherited::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn computed_value(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args = parse_macro_input!(args as inherited::Args);
	// let ast = parse_macro_input!(input as DeriveInput);
	// inherited::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn canonical_order(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args = parse_macro_input!(args as canonical_order::Args);
	// let ast = parse_macro_input!(input as DeriveInput);
	// canonical_order::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn logical_property_group(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args = parse_macro_input!(args as logical_property_group::Args);
	// let ast = parse_macro_input!(input as DeriveInput);
	// logical_property_group::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn animation_type(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
	// let args = parse_macro_input!(args as logical_property_group::Args);
	// let ast = parse_macro_input!(input as DeriveInput);
	// logical_property_group::generate(args, ast).into()
}

#[proc_macro_attribute]
pub fn visit(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn popularity(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn caniuse(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn baseline(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
}

#[proc_macro_attribute]
pub fn versions(_args: TokenStream, input: TokenStream) -> TokenStream {
	input
}
