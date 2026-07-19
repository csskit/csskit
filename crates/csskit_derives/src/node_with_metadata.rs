use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Data, DataEnum, DataStruct, DeriveInput, Error, Fields, GenericParam, Generics, Ident, Index, Path, Result,
	parse_quote,
};

use crate::darling_ext::PipeList;

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
	pub delegate: bool,
}

impl MetadataArgs {
	fn pipe_tokens(field: &Option<PipeList<Ident>>, type_path: Path) -> TokenStream {
		match field.as_ref().map(|p| p.0.as_slice()) {
			None | Some([]) => quote! { #type_path::none() },
			Some(ids) => quote! { #(#type_path::#ids)|* },
		}
	}

	/// Which struct fields to delegate `metadata()` to. The returned accessors are merged together with
	/// `self_metadata()`.
	fn delegate_fields(fields: &Fields, generics: &Generics) -> Vec<TokenStream> {
		let mut explicit = Vec::new();
		match fields {
			Fields::Named(named) => {
				for field in &named.named {
					if MetadataArgs::from_attributes(&field.attrs).map(|a| a.delegate).unwrap_or(false)
						&& let Some(ident) = field.ident.as_ref()
					{
						explicit.push(quote! { #ident });
					}
				}
			}
			Fields::Unnamed(unnamed) => {
				for (i, field) in unnamed.unnamed.iter().enumerate() {
					if MetadataArgs::from_attributes(&field.attrs).map(|a| a.delegate).unwrap_or(false) {
						let idx = Index::from(i);
						explicit.push(quote! { #idx });
					}
				}
			}
			Fields::Unit => return explicit,
		}
		if !explicit.is_empty() {
			return explicit;
		}

		// Auto-delegate for generic single-field structs (newtypes).
		let has_type_params = generics.type_params().next().is_some();
		if !has_type_params {
			return explicit;
		}

		let total_fields = match fields {
			Fields::Named(named) => named.named.len(),
			Fields::Unnamed(unnamed) => unnamed.unnamed.len(),
			Fields::Unit => 0,
		};
		if total_fields == 1 {
			match fields {
				Fields::Named(named) => {
					if let Some(ident) = named.named.first().and_then(|f| f.ident.as_ref()) {
						explicit.push(quote! { #ident });
					}
				}
				Fields::Unnamed(_) => {
					let idx = Index::from(0);
					explicit.push(quote! { #idx });
				}
				Fields::Unit => {}
			}
		}
		explicit
	}

	fn needs_delegation_bounds(&self, data: &Data, generics: &Generics) -> bool {
		if self.delegate {
			return true;
		}
		if let Data::Struct(DataStruct { fields, .. }) = data {
			return !MetadataArgs::delegate_fields(fields, generics).is_empty();
		}
		false
	}

	fn generics_with_metadata_bounds(&self, generics: &Generics) -> Generics {
		let mut generics = generics.clone();
		for param in &mut generics.params {
			if let GenericParam::Type(type_param) = param {
				type_param.bounds.push(parse_quote!(css_parse::NodeWithMetadata<crate::CssMetadata>));
			}
		}
		generics
	}
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

	let effective_generics = if args.needs_delegation_bounds(&input.data, &input.generics) {
		args.generics_with_metadata_bounds(&input.generics)
	} else {
		input.generics.clone()
	};
	let (impl_generics, type_generics, where_clause) = effective_generics.split_for_impl();

	let node_kinds = MetadataArgs::pipe_tokens(&args.node_kinds, parse_quote! { crate::NodeKinds });
	let used_at_rules = MetadataArgs::pipe_tokens(&args.used_at_rules, parse_quote! { crate::AtRuleId });
	let vendor_prefixes = MetadataArgs::pipe_tokens(&args.vendor_prefixes, parse_quote! { crate::VendorPrefixes });
	let declaration_kinds = MetadataArgs::pipe_tokens(&args.declaration_kinds, parse_quote! { crate::DeclarationKind });
	let property_kinds = MetadataArgs::pipe_tokens(&args.property_kinds, parse_quote! { crate::PropertyKind });

	let field_delegates = match &input.data {
		Data::Struct(DataStruct { fields, .. }) => MetadataArgs::delegate_fields(fields, &input.generics),
		_ => Vec::new(),
	};

	let self_metadata = quote! {
		fn self_metadata(&self) -> crate::CssMetadata {
			crate::CssMetadata {
				node_kinds: #node_kinds,
				used_at_rules: #used_at_rules,
				vendor_prefixes: #vendor_prefixes,
				declaration_kinds: #declaration_kinds,
				property_kinds: #property_kinds,
				..Default::default()
			}
		}

	};

	let metadata_body = if args.delegate {
		if let Data::Enum(DataEnum { variants, .. }) = &input.data {
			let match_arms: TokenStream = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					let field_count = variant.fields.len();

					if MetadataArgs::from_attributes(&variant.attrs).map(|a| a.skip).unwrap_or(false) {
						let pattern = if field_count == 0 {
							quote! { Self::#variant_ident }
						} else {
							quote! { Self::#variant_ident(..) }
						};
						return quote! {
							#pattern => crate::CssMetadata::default(),
						};
					}

					if field_count == 0 {
						quote! {
							Self::#variant_ident => crate::CssMetadata::default(),
						}
					} else {
						let bindings: Vec<_> = (0..field_count).map(|i| format_ident!("v{}", i)).collect();
						let metadata_expr = if field_count == 1 {
							quote! { <_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(v0) }
						} else {
							let mut expr = quote! { <_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(v0) };
							for binding in bindings.iter().skip(1) {
								expr = quote! { css_parse::NodeMetadata::merge(#expr, <_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(#binding)) };
							}
							expr
						};

						// Generate pattern based on whether variant has named or unnamed fields.
						let pattern = match &variant.fields {
							Fields::Named(named) => {
								let field_bindings: Vec<_> = named
									.named
									.iter()
									.zip(bindings.iter())
									.map(|(field, binding)| {
										let fname = field.ident.as_ref().unwrap();
										quote! { #fname: #binding }
									})
									.collect();
								quote! { Self::#variant_ident { #(#field_bindings),*, .. } }
							}
							_ => quote! { Self::#variant_ident(#(#bindings),*) },
						};

						quote! {
							#pattern => #metadata_expr,
						}
					}
				})
				.collect();

			quote! {
				fn metadata(&self) -> crate::CssMetadata {
					match self {
						#match_arms
					}
				}
			}
		} else if let Data::Struct(DataStruct { fields, .. }) = &input.data {
			let field_accessors: Vec<TokenStream> = match fields {
				Fields::Named(named) => {
					named.named.iter().filter_map(|f| f.ident.as_ref().map(|i| quote! { #i })).collect()
				}
				Fields::Unnamed(unnamed) => (0..unnamed.unnamed.len())
					.map(|i| {
						let idx = Index::from(i);
						quote! { #idx }
					})
					.collect(),
				Fields::Unit => Vec::new(),
			};
			let child_meta = field_accessors.iter().fold(quote! { self.self_metadata() }, |acc, field_path| {
				quote! {
					css_parse::NodeMetadata::merge(
						#acc,
						<_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(&self.#field_path),
					)
				}
			});
			quote! {
				fn metadata(&self) -> crate::CssMetadata {
					#child_meta
				}
			}
		} else {
			return Err(Error::new_spanned(ident, "#[metadata(delegate)] can only be used on enums or structs."));
		}
	} else if !field_delegates.is_empty() {
		let child_meta = field_delegates.iter().fold(quote! { self.self_metadata() }, |acc, field_path| {
			quote! {
				css_parse::NodeMetadata::merge(
					#acc,
					<_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(&self.#field_path),
				)
			}
		});
		quote! {
			fn metadata(&self) -> crate::CssMetadata {
				#child_meta
			}
		}
	} else {
		quote! {
			fn metadata(&self) -> crate::CssMetadata {
				self.self_metadata()
			}
		}
	};

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics css_parse::NodeWithMetadata<crate::CssMetadata> for #ident #type_generics #where_clause {
			#self_metadata

			#metadata_body
		}
	})
}
