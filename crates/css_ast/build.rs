#![deny(warnings)]
use csskit_source_finder::{find_queryable_nodes, find_visitable_nodes};
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

	// Find all visitable nodes (for Visit trait)
	let mut all_visitable = HashSet::<_>::new();
	find_visitable_nodes("src/**/*.rs", &mut all_visitable, |path: &PathBuf| {
		println!("cargo::rerun-if-changed={}", path.display());
	});

	// Find only queryable nodes (for NodeId enum and QueryableNode trait)
	let mut queryable = HashSet::<_>::new();
	find_queryable_nodes("src/**/*.rs", &mut queryable, |_| {});

	// NodeId enum - only queryable types
	{
		let variants = queryable.iter().enumerate().map(|(idx, node)| {
			let ident = node.ident();
			let discriminant = idx as isize;
			quote! { #ident = #discriminant }
		});

		let tag_name_cases = queryable.iter().map(|node| {
			let ident = node.ident();
			let tag_name = ident.to_string().to_kebab_case();
			quote! { Self::#ident => #tag_name }
		});

		let from_tag_name_cases = queryable.iter().map(|node| {
			let ident = node.ident();
			let tag_name = ident.to_string().to_kebab_case();
			quote! { #tag_name => Some(Self::#ident) }
		});

		let all_tags = queryable.iter().map(|node| {
			let ident = node.ident();
			quote! { Self::#ident }
		});

		let display_arms = queryable.iter().map(|node| {
			let ident = node.ident();
			let ident_str = ident.to_string();
			quote! { Self::#ident => write!(f, #ident_str) }
		});

		#[rustfmt::skip]
		let source = quote! {
			/// Unique identifier for each AST node type that can be queried.
			///
			/// This enum is automatically generated from types that derive `Visitable`
			/// and have `#[visit]`, `#[visit(self)]`, or `#[visit(all)]` attributes.
			/// Each variant has a unique discriminant value assigned at build time.
			#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
			#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
			pub enum NodeId {
				#(#variants),*
			}

			impl std::fmt::Display for NodeId {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					match self {
						#(#display_arms),*
					}
				}
			}

			impl NodeId {
				/// Returns the kebab-case tag name for this node type.
				///
				/// This is used in selector matching (e.g., "style-rule", "declaration").
				pub const fn tag_name(self) -> &'static str {
					match self {
						#(#tag_name_cases),*
					}
				}

				/// Attempts to parse a tag name string into a NodeId.
				///
				/// Returns `None` if the tag name doesn't match any known node type.
				pub fn from_tag_name(name: &str) -> Option<Self> {
					match name {
						#(#from_tag_name_cases),*,
						_ => None
					}
				}

				/// Returns an iterator over all possible NodeId values.
				pub fn all_variants() -> impl Iterator<Item = Self> {
					[#(#all_tags),*].into_iter()
				}
			}
		};
		write_tokens("css_node_kind.rs", source).unwrap()
	}

	// apply_visit_methods - all visitable types (visit_xxx AND exit_xxx methods)
	{
		let methods = all_visitable.iter().flat_map(|node| {
			let ident = node.ident();
			let method_name = node.ident().to_string().to_snake_case();
			let visit_method_name = format_ident!("visit_{}", method_name);
			let exit_method_name = format_ident!("exit_{}", method_name);
			let (impl_generics, ty_generics, _) = node.generics().split_for_impl();
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

	// apply_queryable_visit_methods - only queryable types (with NodeId)
	{
		let methods = queryable.iter().map(|node| {
			let ident = node.ident();
			let method_name = format_ident!("visit_{}", node.ident().to_string().to_snake_case());
			let (impl_generics, ty_generics, _) = node.generics().split_for_impl();
			quote! { #method_name #impl_generics (#ident #ty_generics) }
		});
		let source = quote! {
			#[macro_export]
			macro_rules! apply_queryable_visit_methods {
				($macro: ident) => {
					$macro! {
						#(#methods,)*
					}
				}
			}
		};
		write_tokens("css_apply_queryable_visit_methods.rs", source).unwrap();
	}

	// apply_queryable_exit_methods - only queryable types exit methods
	{
		let methods = queryable.iter().map(|node| {
			let ident = node.ident();
			let method_name = format_ident!("exit_{}", node.ident().to_string().to_snake_case());
			let (impl_generics, ty_generics, _) = node.generics().split_for_impl();
			quote! { #method_name #impl_generics (#ident #ty_generics) }
		});
		let source = quote! {
			#[macro_export]
			macro_rules! apply_queryable_exit_methods {
				($macro: ident) => {
					$macro! {
						#(#methods,)*
					}
				}
			}
		};
		write_tokens("css_apply_queryable_exit_methods.rs", source).unwrap();
	}

	{
		let variants = all_visitable.iter().filter_map(|node| {
			let ident = node.ident();
			if matches!(
				ident.to_string().as_str(),
				"FontFaceRuleStyleValue" | "PropertyRuleStyleValue" | "CounterStyleRuleStyleValue"
			) {
				return None;
			}
			node.ident().to_string().strip_suffix("StyleValue").and_then(|name| {
				let generics = node.generics();
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
}
