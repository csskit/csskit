#![deny(warnings)]
use csskit_source_finder::find_visitable_nodes;
use heck::{ToKebabCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::{
	collections::HashSet,
	env,
	fs::write,
	io::Error,
	path::{Path, PathBuf},
};

fn write_tokens(file: &str, source: TokenStream) -> Result<(), Error> {
	let contents = syn::parse_file(&source.to_string()).map_err(|e| Error::other(e.to_string()))?;
	let contents = prettyplease::unparse(&contents);
	write(Path::new(&env::var("OUT_DIR").unwrap()).join(file), contents)
}

fn main() {
	println!("cargo::rerun-if-changed=build.rs");
	use std::time::Instant;
	let now = Instant::now();
	let mut matches = HashSet::<_>::new();
	find_visitable_nodes("src/**/*.rs", &mut matches, |path: &PathBuf| {
		println!("cargo::rerun-if-changed={}", path.display());
	});

	{
		let variants = matches.iter().map(|input| input.ident.clone());
		#[rustfmt::skip]
		let source = quote! {
				#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
				pub enum NodeKind {
					#(#variants),*
				}
		};
		write_tokens("css_node_kind.rs", source).unwrap()
	}

	{
		let methods = matches.iter().flat_map(|input| {
			let ident = &input.ident;
			let method_name = input.ident.to_string().to_snake_case();
			let visit_method_name = format_ident!("visit_{}", method_name);
			let exit_method_name = format_ident!("exit_{}", method_name);
			let (impl_generics, ty_generics, _) = input.generics.split_for_impl();
			[
				quote! { #visit_method_name #impl_generics (#ident #ty_generics) },
				quote! { #exit_method_name #impl_generics (#ident #ty_generics) },
			]
		});
		let source = quote! {
			#[macro_export]
			macro_rules! apply_visit_methods {
				($macro: ident) => {
					$macro! {
						#(#methods,)*
					}
				}
			}
		};
		write_tokens("css_apply_visit_methods.rs", source).unwrap();
	}

	{
		let variants = matches.iter().filter_map(|input| {
			let ident = &input.ident;
			input.ident.to_string().strip_suffix("StyleValue").and_then(|name| {
				let generics = &input.generics;
				if name.is_empty() {
					return None;
				}
				let variant_name = format_ident!("{}", name);
				let mut variant_atom = variant_name.clone();
				let kebab = variant_atom.to_string().to_kebab_case();
				if matches!(kebab.split("-").next().unwrap_or_default(), "Webkit" | "Moz" | "Ms" | "O") {
					variant_atom = format_ident!("_{variant_atom}");
				}
				Some(quote! { #variant_name: #ident #generics = #variant_atom })
			})
		});
		let source = quote! {
			macro_rules! apply_properties {
				($macro: ident) => {
					$macro! {
						#(#variants,)*
					}
				}
			}
		};
		write_tokens("css_apply_properties.rs", source).unwrap();
	}

	let elapsed = now.elapsed();
	println!("cargo::warning=Took {:.0?} to generate build files", &elapsed);
}
