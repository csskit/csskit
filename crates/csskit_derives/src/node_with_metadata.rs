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
	pub block: bool,
	#[darling(default)]
	pub prelude: bool,
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

	fn field_is_block(attrs: &[syn::Attribute]) -> bool {
		MetadataArgs::from_attributes(attrs).map(|a| a.block).unwrap_or(false)
	}

	fn field_is_prelude(attrs: &[syn::Attribute]) -> bool {
		MetadataArgs::from_attributes(attrs).map(|a| a.prelude).unwrap_or(false)
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

/// Accessors (`self.<member>`) of every field, paired with the field itself.
fn members(fields: &Fields) -> Vec<(TokenStream, &syn::Field)> {
	match fields {
		Fields::Named(named) => {
			named.named.iter().filter_map(|f| f.ident.as_ref().map(|i| (quote! { #i }, f))).collect()
		}
		Fields::Unnamed(unnamed) => unnamed
			.unnamed
			.iter()
			.enumerate()
			.map(|(i, f)| {
				let idx = Index::from(i);
				(quote! { #idx }, f)
			})
			.collect(),
		Fields::Unit => Vec::new(),
	}
}

/// Accessor (`self.<member>`) of the field marked `#[metadata(block)]`, if any.
fn block_field(fields: &Fields) -> Result<Option<TokenStream>> {
	let mut found: Option<TokenStream> = None;
	for (member, field) in members(fields) {
		if !MetadataArgs::field_is_block(&field.attrs) {
			continue;
		}
		if MetadataArgs::field_is_skipped(&field.attrs) {
			return Err(Error::new_spanned(field, "#[metadata(block)] cannot be combined with #[metadata(skip)]."));
		}
		if found.is_some() {
			return Err(Error::new_spanned(field, "#[metadata(block)] can only be applied to one field."));
		}
		found = Some(member);
	}
	Ok(found)
}

/// Accessors (`self.<member>`) of every field marked `#[metadata(prelude)]`.
fn prelude_fields(fields: &Fields) -> Vec<TokenStream> {
	members(fields)
		.into_iter()
		.filter(|(_, field)| MetadataArgs::field_is_prelude(&field.attrs))
		.map(|(member, _)| member)
		.collect()
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

	let block = match &input.data {
		Data::Struct(DataStruct { fields, .. }) => block_field(fields)?,
		Data::Enum(DataEnum { variants, .. }) => {
			for variant in variants {
				if block_field(&variant.fields)?.is_some() {
					return Err(Error::new_spanned(variant, "#[metadata(block)] is only supported on structs."));
				}
			}
			None
		}
		Data::Union(_) => None,
	};

	let preludes = match &input.data {
		Data::Struct(DataStruct { fields, .. }) => prelude_fields(fields),
		Data::Enum(DataEnum { variants, .. }) => {
			for variant in variants {
				if !prelude_fields(&variant.fields).is_empty() {
					return Err(Error::new_spanned(variant, "#[metadata(prelude)] is only supported on structs."));
				}
			}
			Vec::new()
		}
		Data::Union(_) => Vec::new(),
	};

	let declares_rule = args
		.node_kinds
		.as_ref()
		.is_some_and(|kinds| kinds.0.iter().any(|kind| kind == "AtRule" || kind == "StyleRule"));

	// `Inert` marks a rule which renders nothing, `Effective` a rule which renders something, so
	// that a rule holding only inert rules is itself detectable as inert. `EmptyBlock` is the
	// narrower "holds nothing at all". A rule without a block always has an effect on the rule
	// containing it.
	let node_kinds = if let Some(block) = &block {
		quote! {
			{
				let child = <_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(&self.#block);
				let mut kinds = #node_kinds;
				kinds |= if child.has_effect() { crate::NodeKinds::Effective } else { crate::NodeKinds::Inert };
				if !child.node_kinds.intersects(
					crate::NodeKinds::Declaration | crate::NodeKinds::StyleRule | crate::NodeKinds::AtRule,
				) {
					kinds |= crate::NodeKinds::EmptyBlock;
				}
				kinds
			}
		}
	} else if declares_rule {
		quote! { #node_kinds | crate::NodeKinds::Effective }
	} else {
		node_kinds
	};

	// A prelude which covers no source text is absent: `None` spans `Span::DUMMY` and an empty
	// list spans `Span::ZERO`, both of zero length.
	let node_kinds = if preludes.is_empty() {
		node_kinds
	} else {
		quote! {
			{
				let kinds = #node_kinds;
				if #(css_parse::ToSpan::to_span(&self.#preludes).len() == 0)&&* {
					kinds | crate::NodeKinds::EmptyPrelude
				} else {
					kinds
				}
			}
		}
	};

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
