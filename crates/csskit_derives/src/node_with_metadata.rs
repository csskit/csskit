use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, Meta, Token,
	parse::{Parse, ParseStream},
};

/// Parsed metadata arguments from #[metadata(...)] attribute.
#[derive(Debug, Default)]
struct MetadataArgs {
	/// Whether to skip generating the implementation (type has manual impl).
	pub skip: bool,
	/// node_kinds field values (e.g., Dimension, Function, AtRule).
	pub node_kinds: Vec<Ident>,
	/// used_at_rules field values (e.g., Media, Keyframes).
	pub used_at_rules: Vec<Ident>,
	/// vendor_prefixes field values (e.g., Moz, WebKit).
	pub vendor_prefixes: Vec<Ident>,
	/// declaration_kinds field values (e.g., Important, Custom).
	pub declaration_kinds: Vec<Ident>,
	/// property_kinds field values (e.g., Name).
	pub property_kinds: Vec<Ident>,
	/// For enums: delegate to the inner value's metadata() for each variant.
	/// When set, generates: match self { Self::Variant(v) => v.metadata(), ... }
	pub delegate: bool,
}

impl Parse for MetadataArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut args = Self::default();
		while !input.is_empty() {
			let ident = input.parse::<Ident>()?;
			match ident.to_string().as_str() {
				"skip" => {
					args.skip = true;
				}
				"delegate" => {
					args.delegate = true;
				}
				"node_kinds" => {
					input.parse::<Token![=]>()?;
					loop {
						args.node_kinds.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				"used_at_rules" => {
					input.parse::<Token![=]>()?;
					loop {
						args.used_at_rules.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				"vendor_prefixes" => {
					input.parse::<Token![=]>()?;
					loop {
						args.vendor_prefixes.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				"declaration_kinds" => {
					input.parse::<Token![=]>()?;
					loop {
						args.declaration_kinds.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				"property_kinds" => {
					input.parse::<Token![=]>()?;
					loop {
						args.property_kinds.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				_ => {
					return Err(Error::new(ident.span(), format!("Unrecognized metadata argument: {ident}")));
				}
			}

			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(args)
	}
}

impl From<&[Attribute]> for MetadataArgs {
	fn from(attrs: &[Attribute]) -> Self {
		let mut result = Self::default();
		if let Some(Attribute { meta, .. }) = attrs.iter().find(|a| a.path().is_ident("metadata"))
			&& let Meta::List(meta) = meta
		{
			result = meta.parse_args::<MetadataArgs>().unwrap();
		}
		result
	}
}

/// Check if a field has #[metadata(delegate)] attribute
fn has_delegate_attr(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|attr| {
		if attr.path().is_ident("metadata") {
			match &attr.meta {
				Meta::List(meta) => meta.parse_args::<MetadataArgs>().map(|args| args.delegate).unwrap_or(false),
				_ => false,
			}
		} else {
			false
		}
	})
}

fn has_skip_attr(attrs: &[Attribute]) -> bool {
	attrs.iter().any(|attr| {
		if attr.path().is_ident("metadata") {
			match &attr.meta {
				Meta::List(meta) => meta.parse_args::<MetadataArgs>().map(|args| args.skip).unwrap_or(false),
				_ => false,
			}
		} else {
			false
		}
	})
}

/// Determines which struct field to delegate `metadata()` to, if any.
///
/// Generic newtypes (e.g. `NonNegative<T>`) auto-delegate so that `T`'s metadata
/// propagates without requiring an explicit attribute. Non-generic single-field structs
/// don't auto-delegate because their inner types may not implement `NodeWithMetadata`.
fn find_delegate_field(fields: &Fields, generics: &syn::Generics) -> Option<TokenStream> {
	// Explicit #[metadata(delegate)] attribute takes priority
	match fields {
		Fields::Named(named) => {
			for field in &named.named {
				if has_delegate_attr(&field.attrs) {
					let ident = field.ident.as_ref()?;
					return Some(quote! { #ident });
				}
			}
		}
		Fields::Unnamed(unnamed) => {
			for (i, field) in unnamed.unnamed.iter().enumerate() {
				if has_delegate_attr(&field.attrs) {
					let idx = syn::Index::from(i);
					return Some(quote! { #idx });
				}
			}
		}
		Fields::Unit => return None,
	}

	// Second: auto-delegate for generic single-field structs (newtypes)
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
				let idx = syn::Index::from(0);
				Some(quote! { #idx })
			}
			Fields::Unit => None,
		}
	} else {
		None
	}
}

/// Check if delegation is active (struct field delegate or enum-level delegate).
/// Used to determine if we need to add NodeWithMetadata bounds on generic type params.
fn needs_delegation_bounds(args: &MetadataArgs, data: &Data, generics: &syn::Generics) -> bool {
	if args.delegate {
		return true;
	}
	if let Data::Struct(DataStruct { fields, .. }) = data {
		return find_delegate_field(fields, generics).is_some();
	}
	false
}

/// Ensures delegated fields can provide metadata by bounding type params with `NodeWithMetadata`.
fn generics_with_metadata_bounds(generics: &syn::Generics) -> syn::Generics {
	let mut generics = generics.clone();
	for param in &mut generics.params {
		if let syn::GenericParam::Type(type_param) = param {
			type_param.bounds.push(syn::parse_quote!(css_parse::NodeWithMetadata<crate::CssMetadata>));
		}
	}
	generics
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let ident = input.ident;
	let args = MetadataArgs::from(input.attrs.as_slice());

	// Check if we should skip generating NodeWithMetadata (type has manual implementation)
	if args.skip {
		return Error::new_spanned(
			ident,
			"#[metadata(skip)] should not be used with derive(NodeWithMetadata). Remove the derive instead.",
		)
		.into_compile_error();
	}

	// Add NodeWithMetadata bounds on generic params when delegation is active
	let effective_generics = if needs_delegation_bounds(&args, &input.data, &input.generics) {
		generics_with_metadata_bounds(&input.generics)
	} else {
		input.generics.clone()
	};
	let (impl_generics, type_generics, where_clause) = effective_generics.split_for_impl();

	// Generate field initializers for any specified metadata
	let node_kinds = if args.node_kinds.is_empty() {
		quote! { crate::NodeKinds::none() }
	} else {
		let kinds = &args.node_kinds;
		quote! { #(crate::NodeKinds::#kinds)|* }
	};

	let used_at_rules = if args.used_at_rules.is_empty() {
		quote! { crate::AtRuleId::none() }
	} else {
		let rules = &args.used_at_rules;
		quote! { #(crate::AtRuleId::#rules)|* }
	};

	let vendor_prefixes = if args.vendor_prefixes.is_empty() {
		quote! { crate::VendorPrefixes::none() }
	} else {
		let prefixes = &args.vendor_prefixes;
		quote! { #(crate::VendorPrefixes::#prefixes)|* }
	};

	let declaration_kinds = if args.declaration_kinds.is_empty() {
		quote! { crate::DeclarationKind::none() }
	} else {
		let kinds = &args.declaration_kinds;
		quote! { #(crate::DeclarationKind::#kinds)|* }
	};

	let property_kinds = if args.property_kinds.is_empty() {
		quote! { crate::PropertyKind::none() }
	} else {
		let kinds = &args.property_kinds;
		quote! { #(crate::PropertyKind::#kinds)|* }
	};

	let field_delegate = match &input.data {
		Data::Struct(DataStruct { fields, .. }) => find_delegate_field(fields, &input.generics),
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

	// Check if this is an enum with delegate flag - in that case, generate delegation
	let metadata_body = if args.delegate {
		if let Data::Enum(DataEnum { variants, .. }) = &input.data {
			// Enum delegation: match on each variant and call metadata() on the inner value
			let match_arms: TokenStream = variants
				.iter()
				.map(|variant| {
					let variant_ident = &variant.ident;
					let field_count = variant.fields.len();

					// Check if variant has #[metadata(skip)] - if so, return default
					if has_skip_attr(&variant.attrs) {
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
						// Unit variant - return default metadata
						quote! {
							Self::#variant_ident => crate::CssMetadata::default(),
						}
					} else {
						// Generate bindings for each field (v0, v1, v2, ...)
						let bindings: Vec<_> = (0..field_count).map(|i| format_ident!("v{}", i)).collect();

						// For enum delegation, we merge metadata from all fields
						// First field is the base, subsequent fields are merged in
						let metadata_expr = if field_count == 1 {
							quote! { <_ as css_parse::NodeWithMetadata<crate::CssMetadata>>::metadata(v0) }
						} else {
							// Merge metadata from all fields using NodeMetadata::merge
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
			return Error::new_spanned(
				ident,
				"#[metadata(delegate)] on a type-level attribute can only be used on enums. Use field-level #[metadata(delegate)] for structs.",
			)
			.into_compile_error();
		}
	} else if let Some(field_path) = field_delegate {
		// Struct delegate (explicit or auto for single-field newtypes) - merge from that field
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

	// Generate NodeWithMetadata implementation
	quote! {
		#[automatically_derived]
		impl #impl_generics css_parse::NodeWithMetadata<crate::CssMetadata> for #ident #type_generics #where_clause {
			#self_metadata

			#metadata_body
		}
	}
}
