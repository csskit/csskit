use csskit_source_finder::find_visitable_nodes;
use heck::{ToKebabCase, ToSnakeCase};
use quote::{format_ident, quote};
use std::{
	collections::HashSet,
	env,
	fs::write,
	path::{Path, PathBuf},
};
use syn::{Ident, PathArguments, Type, TypePath};

trait GetIdent {
	fn get_ident(&self) -> Option<Ident>;
}

impl GetIdent for Type {
	fn get_ident(&self) -> Option<Ident> {
		match self {
			Self::Path(TypePath { path, .. }) => path.segments.last().map(|seg| seg.ident.clone()),
			_ => None,
		}
	}
}

trait GetArguments {
	fn get_arguments(&self) -> Option<PathArguments>;
}

impl GetArguments for Type {
	fn get_arguments(&self) -> Option<PathArguments> {
		match self {
			Self::Path(TypePath { path, .. }) => path.segments.last().map(|seg| seg.arguments.clone()),
			_ => None,
		}
	}
}

fn ident_to_snake_case(ident: Ident) -> Ident {
	format_ident!("{}", ident.to_string().to_snake_case())
}

fn main() {
	println!("cargo::rerun-if-changed=build.rs");
	use std::time::Instant;
	let now = Instant::now();
	let mut matches = HashSet::<String>::new();
	find_visitable_nodes("src/**/*.rs", &mut matches, |path: &PathBuf| {
		println!("cargo::rerun-if-changed={}", path.display());
	});

	println!("cargo::warning=Constructring css_node_kind.rs");
	{
		let variants =
			matches.iter().filter_map(|type_str| syn::parse_str::<Type>(type_str).ok().and_then(|ty| ty.get_ident()));
		#[rustfmt::skip]
		let source = quote! {
				#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
				pub enum NodeKind {
					#(#variants),*
				}
		};
		write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_node_kind.rs"), source.to_string()).unwrap();
	}

	println!("cargo::warning=Constructring css_apply_visit_methods.rs");
	{
		let methods = matches.iter().filter_map(|type_str| {
			syn::parse_str::<Type>(type_str).ok().and_then(|ty| {
				ty.get_ident().map(|ident| {
					let method_name = format_ident!("visit_{}", ident_to_snake_case(ident));
					let life = ty.get_arguments();
					quote! { #method_name #life (#ty) }
				})
			})
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
		write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_apply_visit_methods.rs"), source.to_string()).unwrap();
	}

	println!("cargo::warning=Constructring css_apply_properties.rs");
	{
		let variants = matches.iter().filter_map(|type_str| {
			syn::parse_str::<Type>(type_str).ok().and_then(|ty| {
				ty.get_ident().and_then(|ident| {
					ident.to_string().strip_suffix("StyleValue").and_then(|name| {
						if name.is_empty() {
							return None;
						}
						let variant_name = format_ident!("{}", name);
						let mut variant_str = variant_name.to_string().to_kebab_case();
						if variant_str.starts_with("webkit-") || variant_str.starts_with("moz-") {
							variant_str = format!("-{variant_str}");
						}
						Some(quote! { #variant_name: #ty = #variant_str })
					})
				})
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
		write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_apply_properties.rs"), source.to_string()).unwrap();
	}

	let elapsed = now.elapsed();
	println!("cargo::warning=Built in {:.?}", &elapsed);
}
