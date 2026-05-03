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

	/// Which struct field to delegate `metadata()` to, if any.
	///
	/// Generic newtypes (e.g. `NonNegative<T>`) auto-delegate so `T`'s metadata
	/// propagates without an explicit attribute. Non-generic single-field structs
	/// don't auto-delegate: their inner type may not implement `NodeWithMetadata`.
	fn delegate_field(fields: &Fields, generics: &Generics) -> Option<TokenStream> {
		match fields {
			Fields::Named(named) => {
				for field in &named.named {
					if MetadataArgs::from_attributes(&field.attrs).map(|a| a.delegate).unwrap_or(false) {
						let ident = field.ident.as_ref()?;
						return Some(quote! { #ident });
					}
				}
			}
			Fields::Unnamed(unnamed) => {
				for (i, field) in unnamed.unnamed.iter().enumerate() {
					if MetadataArgs::from_attributes(&field.attrs).map(|a| a.delegate).unwrap_or(false) {
						let idx = Index::from(i);
						return Some(quote! { #idx });
					}
				}
			}
			Fields::Unit => return None,
		}

		// Auto-delegate for generic single-field structs (newtypes).
		let has_type_params = generics.type_params().next().is_some();
		if !has_type_params {
			return None;
		}

		let total_fields = match fields {
			Fields::Named(named) => named.named.len(),
			Fields::Unnamed(unnamed) => unnamed.unnamed.len(),
			Fields::Unit => 0,
		};
		if total_fields == 1 {
			match fields {
				Fields::Named(named) => {
					let ident = named.named.first()?.ident.as_ref()?;
					Some(quote! { #ident })
				}
				Fields::Unnamed(_) => {
					let idx = Index::from(0);
					Some(quote! { #idx })
				}
				Fields::Unit => None,
			}
		} else {
			None
		}
	}

	fn needs_delegation_bounds(&self, data: &Data, generics: &Generics) -> bool {
		if self.delegate {
			return true;
		}
		if let Data::Struct(DataStruct { fields, .. }) = data {
			return MetadataArgs::delegate_field(fields, generics).is_some();
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

	let field_delegate = match &input.data {
		Data::Struct(DataStruct { fields, .. }) => MetadataArgs::delegate_field(fields, &input.generics),
		_ => None,
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

						quote! {
							Self::#variant_ident(#(#bindings),*) => #metadata_expr,
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
		} else {
			return Err(Error::new_spanned(
				ident,
				"#[metadata(delegate)] on a type-level attribute can only be used on enums. Use field-level #[metadata(delegate)] for structs.",
			));
		}
	} else if let Some(field_path) = field_delegate {
		let field_access = quote! { self.#field_path };
		let child_meta_access =
			quote! { <_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(&#field_access) };
		quote! {
			fn metadata(&self) -> crate::CssMetadata {
				let child_meta = #child_meta_access;
				css_parse::NodeMetadata::merge(child_meta, self.self_metadata())
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
