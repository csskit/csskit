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
use syn::{AngleBracketedGenericArguments, GenericArgument, Ident, PathArguments, Type, TypePath};

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
			Self::Path(TypePath { path, .. }) => path.segments.last().map(|seg| {
				// Strip trait bounds from generic arguments
				match &seg.arguments {
					PathArguments::AngleBracketed(args) => {
						// For each generic argument, strip the bounds
						let stripped_args: Vec<GenericArgument> = args
							.args
							.iter()
							.filter_map(|arg| {
								match arg {
									GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
										// Extract just the identifier, no bounds
										path.segments.last().map(|seg| {
											GenericArgument::Type(Type::Path(TypePath {
												qself: None,
												path: syn::Path {
													leading_colon: None,
													segments: std::iter::once(syn::PathSegment {
														ident: seg.ident.clone(),
														arguments: PathArguments::None,
													})
													.collect(),
												},
											}))
										})
									}
									_ => Some(arg.clone()),
								}
							})
							.collect();

						if stripped_args.is_empty() {
							PathArguments::None
						} else {
							PathArguments::AngleBracketed(AngleBracketedGenericArguments {
								colon2_token: None,
								lt_token: args.lt_token,
								args: stripped_args.into_iter().collect(),
								gt_token: args.gt_token,
							})
						}
					}
					other => other.clone(),
				}
			}),
			_ => None,
		}
	}
}

fn ident_to_snake_case(ident: Ident) -> Ident {
	format_ident!("{}", ident.to_string().to_snake_case())
}

// Strip trait bounds from a type, keeping only the type name and generic parameter names
fn strip_bounds_from_type(ty: &Type) -> Option<Type> {
	match ty {
		Type::Path(TypePath { path, .. }) => {
			let last_segment = path.segments.last()?;
			let ident = last_segment.ident.clone();

			// Get arguments without bounds
			let args = match &last_segment.arguments {
				PathArguments::AngleBracketed(args) => {
					let stripped_args: Vec<GenericArgument> = args
						.args
						.iter()
						.filter_map(|arg| match arg {
							GenericArgument::Type(Type::Path(TypePath { path, .. })) => {
								path.segments.last().map(|seg| {
									GenericArgument::Type(Type::Path(TypePath {
										qself: None,
										path: syn::Path {
											leading_colon: None,
											segments: std::iter::once(syn::PathSegment {
												ident: seg.ident.clone(),
												arguments: PathArguments::None,
											})
											.collect(),
										},
									}))
								})
							}
							_ => Some(arg.clone()),
						})
						.collect();

					if stripped_args.is_empty() {
						PathArguments::None
					} else {
						PathArguments::AngleBracketed(AngleBracketedGenericArguments {
							colon2_token: None,
							lt_token: args.lt_token,
							args: stripped_args.into_iter().collect(),
							gt_token: args.gt_token,
						})
					}
				}
				other => other.clone(),
			};

			Some(Type::Path(TypePath {
				qself: None,
				path: syn::Path {
					leading_colon: None,
					segments: std::iter::once(syn::PathSegment { ident, arguments: args }).collect(),
				},
			}))
		}
		_ => None,
	}
}

fn write_tokens(file: &str, source: TokenStream) -> Result<(), Error> {
	let contents = syn::parse_file(&source.to_string()).map_err(|e| Error::other(e.to_string()))?;
	let contents = prettyplease::unparse(&contents);
	write(Path::new(&env::var("OUT_DIR").unwrap()).join(file), contents)
}

fn main() {
	println!("cargo::rerun-if-changed=build.rs");
	use std::time::Instant;
	let now = Instant::now();
	let mut matches = HashSet::<String>::new();
	find_visitable_nodes("src/**/*.rs", &mut matches, |path: &PathBuf| {
		println!("cargo::rerun-if-changed={}", path.display());
	});

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
		write_tokens("css_node_kind.rs", source).unwrap()
	}

	{
		let methods = matches.iter().filter_map(|type_str| {
			syn::parse_str::<Type>(type_str).ok().and_then(|ty| {
				ty.get_ident().and_then(|ident| {
					let method_name = format_ident!("visit_{}", ident_to_snake_case(ident));
					let life = ty.get_arguments();
					// Strip bounds from the type for use in the macro
					let ty_stripped = strip_bounds_from_type(&ty)?;
					Some(quote! { #method_name #life (#ty_stripped) })
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
		write_tokens("css_apply_visit_methods.rs", source).unwrap();
	}

	{
		let variants = matches.iter().filter_map(|type_str| {
			syn::parse_str::<Type>(type_str).ok().and_then(|ty| {
				ty.get_ident().and_then(|ident| {
					ident.to_string().strip_suffix("StyleValue").and_then(|name| {
						if name.is_empty() {
							return None;
						}
						let variant_name = format_ident!("{}", name);
						let mut variant_atom = variant_name.clone();
						let kebab = variant_atom.to_string().to_kebab_case();
						if matches!(kebab.split("-").next().unwrap_or_default(), "Webkit" | "Moz" | "Ms" | "O") {
							variant_atom = format_ident!("_{variant_atom}");
						}
						Some(quote! { #variant_name: #ty = #variant_atom })
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
		write_tokens("css_apply_properties.rs", source).unwrap();
	}

	let elapsed = now.elapsed();
	println!("cargo::warning=Took {:.0?} to generate build files", &elapsed);
}
