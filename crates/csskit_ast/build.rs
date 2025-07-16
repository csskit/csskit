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
	let mut matches = HashSet::<Type>::new();
	find_visitable_nodes("../css_ast/src/**/*.rs", &mut matches, |path: &PathBuf| {
		println!("cargo::rerun-if-changed={}", path.display());
	});

	println!("cargo::warning=Constructring css_ast_node_tag.rs");
	{
		let variants = matches.iter().filter_map(GetIdent::get_ident).map(|ident| {
			let str = ident.to_string().to_kebab_case();
			quote! { #ident: #str }
		});
		#[rustfmt::skip]
		let source = quote! {
				::css_parse::keyword_set! {
					/// All possible nodes that can be selected.
					#[derive(Visitable)]
					#[visit(self)]
					pub enum Tag {
						#(#variants),*
					}
				}
		};
		write(Path::new(&env::var("OUT_DIR").unwrap()).join("css_ast_node_tag.rs"), source.to_string()).unwrap();
	}

	println!("cargo::warning=Constructring css_apply_visit_methods.rs");
	{
		let mut matches = HashSet::<Type>::new();
		find_visitable_nodes("src/**/*.rs", &mut matches, |path: &PathBuf| {
			println!("cargo::rerun-if-changed={}", path.display());
		});
		let methods = matches.iter().filter_map(|ty| {
			ty.get_ident().map(|ident| {
				let method_name = format_ident!("visit_{}", ident_to_snake_case(ident));
				let life = ty.get_arguments();
				quote! { #method_name #life (#ty) }
			})
		});
		let source = quote! {
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

	let elapsed = now.elapsed();
	println!("cargo::warning=Built in {:.?}", &elapsed);
}
