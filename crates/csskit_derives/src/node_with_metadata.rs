use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, Index, Path, Result, Type, parse_quote};

use crate::{darling_ext::PipeList, where_collector::WhereCollector};

#[derive(Debug, Default, FromAttributes)]
#[darling(attributes(metadata))]
struct MetadataArgs {
	#[darling(default)]
	pub skip: bool,
	#[darling(default)]
	pub node_kinds: Option<PipeList<Ident>>,
	#[darling(default)]
	pub used_at_rules: Option<PipeList<Ident>>,
	#[darling(default)]
	pub vendor_prefixes: Option<PipeList<Ident>>,
	#[darling(default)]
	pub declaration_kinds: Option<PipeList<Ident>>,
	#[darling(default)]
	pub property_kinds: Option<PipeList<Ident>>,
	#[darling(default)]
	pub value_kinds: Option<PipeList<Ident>>,
	#[darling(default)]
	pub uses_substitution: bool,
}

impl MetadataArgs {
	fn pipe_tokens(field: &Option<PipeList<Ident>>, type_path: Path) -> TokenStream {
		match field.as_ref().map(|p| p.0.as_slice()) {
			None | Some([]) => quote! { #type_path::none() },
			Some(ids) => quote! { #(#type_path::#ids)|* },
		}
	}

	fn field_is_skipped(attrs: &[syn::Attribute]) -> bool {
		MetadataArgs::from_attributes(attrs).map(|a| a.skip).unwrap_or(false)
	}
}

/// Accessors (`self.<member>`) of every field that contributes metadata, plus their types.
fn aggregated_fields(fields: &Fields) -> Vec<(TokenStream, &Type)> {
	match fields {
		Fields::Named(named) => named
			.named
			.iter()
			.filter(|f| !MetadataArgs::field_is_skipped(&f.attrs))
			.filter_map(|f| f.ident.as_ref().map(|i| (quote! { #i }, &f.ty)))
			.collect(),
		Fields::Unnamed(unnamed) => unnamed
			.unnamed
			.iter()
			.enumerate()
			.filter(|(_, f)| !MetadataArgs::field_is_skipped(&f.attrs))
			.map(|(i, f)| {
				let idx = Index::from(i);
				(quote! { #idx }, &f.ty)
			})
			.collect(),
		Fields::Unit => Vec::new(),
	}
}

fn merge_expr(base: TokenStream, values: impl IntoIterator<Item = TokenStream>) -> TokenStream {
	values.into_iter().fold(base, |acc, value| {
		quote! {
			css_parse::NodeMetadata::merge(
				#acc,
				<_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(#value),
			)
		}
	})
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let ident = input.ident;
	let args = MetadataArgs::from_attributes(&input.attrs)?;

	if args.skip {
		return Err(Error::new_spanned(
			ident,
			"#[metadata(skip)] should not be used with derive(NodeWithMetadata). Remove the derive instead.",
		));
	}

	let node_kinds = MetadataArgs::pipe_tokens(&args.node_kinds, parse_quote! { crate::NodeKinds });
	let used_at_rules = MetadataArgs::pipe_tokens(&args.used_at_rules, parse_quote! { crate::AtRuleId });
	let vendor_prefixes = MetadataArgs::pipe_tokens(&args.vendor_prefixes, parse_quote! { crate::VendorPrefixes });
	let declaration_kinds = MetadataArgs::pipe_tokens(&args.declaration_kinds, parse_quote! { crate::DeclarationKind });
	let property_kinds = MetadataArgs::pipe_tokens(&args.property_kinds, parse_quote! { crate::PropertyKind });
	let value_kinds = MetadataArgs::pipe_tokens(&args.value_kinds, parse_quote! { crate::CssTypes });
	let uses_substitution = args.uses_substitution;

	let self_metadata = quote! {
		fn self_metadata(&self) -> crate::CssMetadata {
			crate::CssMetadata {
				node_kinds: #node_kinds,
				used_at_rules: #used_at_rules,
				vendor_prefixes: #vendor_prefixes,
				declaration_kinds: #declaration_kinds,
				property_kinds: #property_kinds,
				value_kinds: #value_kinds,
				uses_substitution: #uses_substitution,
				..Default::default()
			}
		}
	};

	let mut wc = WhereCollector::new();
	let metadata_body = match &input.data {
		Data::Struct(DataStruct { fields, .. }) => {
			let members = aggregated_fields(fields);
			for (_, ty) in &members {
				wc.add(ty);
			}
			let body =
				merge_expr(quote! { self.self_metadata() }, members.iter().map(|(member, _)| quote! { &self.#member }));
			quote! {
				fn metadata(&self) -> crate::CssMetadata {
					#body
				}
			}
		}
		Data::Enum(DataEnum { variants, .. }) => {
			let arms: TokenStream = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					if MetadataArgs::from_attributes(&variant.attrs).map(|a| a.skip).unwrap_or(false) {
						let pattern = match &variant.fields {
							Fields::Unit => quote! { Self::#variant_ident },
							_ => quote! { Self::#variant_ident(..) },
						};
						return quote! { #pattern => crate::CssMetadata::default(), };
					}
					let bindings: Vec<_> = variant
						.fields
						.iter()
						.enumerate()
						.map(|(i, field)| {
							if MetadataArgs::field_is_skipped(&field.attrs) {
								None
							} else {
								wc.add(&field.ty);
								Some(format_ident!("v{}", i))
							}
						})
						.collect();
					let pattern = match &variant.fields {
						Fields::Unit => quote! { Self::#variant_ident },
						Fields::Named(named) => {
							let field_bindings: Vec<TokenStream> = named
								.named
								.iter()
								.zip(bindings.iter())
								.filter_map(|(field, binding)| {
									let name = field.ident.as_ref()?;
									let binding = binding.as_ref()?;
									Some(quote! { #name: #binding })
								})
								.collect();
							quote! { Self::#variant_ident { #(#field_bindings,)* .. } }
						}
						Fields::Unnamed(_) => {
							let positional: Vec<TokenStream> = bindings
								.iter()
								.map(|binding| match binding {
									Some(binding) => quote! { #binding },
									None => quote! { _ },
								})
								.collect();
							quote! { Self::#variant_ident(#(#positional),*) }
						}
					};
					let body = merge_expr(
						quote! { self.self_metadata() },
						bindings.iter().flatten().map(|binding| quote! { #binding }),
					);
					quote! { #pattern => #body, }
				})
				.collect();
			quote! {
				fn metadata(&self) -> crate::CssMetadata {
					match self {
						#arms
					}
				}
			}
		}
		Data::Union(_) => return Err(Error::new_spanned(ident, "NodeWithMetadata cannot be derived for unions.")),
	};

	let where_clause =
		wc.extend_where_clause(&input.generics, parse_quote!(css_parse::NodeWithMetadata<crate::CssMetadata>));
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics css_parse::NodeWithMetadata<crate::CssMetadata> for #ident #type_generics #where_clause {
			#self_metadata

			#metadata_body
		}
	})
}
