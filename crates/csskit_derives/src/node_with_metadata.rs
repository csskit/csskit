use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Ident, Member, Path, Result, parse_quote};

use crate::{FieldsExt, darling_ext::PipeList, field_view::Arm, where_collector::WhereCollector};

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

/// Member of the field marked `#[metadata(block)]`, if any.
fn block_field(fields: &Fields) -> Result<Option<Member>> {
	let mut found = None;
	for (view, field) in fields.views().into_iter().zip(fields.iter()) {
		if !MetadataArgs::field_is_block(&field.attrs) {
			continue;
		}
		if MetadataArgs::field_is_skipped(&field.attrs) {
			return Err(Error::new_spanned(field, "#[metadata(block)] cannot be combined with #[metadata(skip)]."));
		}
		if found.is_some() {
			return Err(Error::new_spanned(field, "#[metadata(block)] can only be applied to one field."));
		}
		found = Some(view.member);
	}
	Ok(found)
}

/// Members of every field marked `#[metadata(prelude)]`.
fn prelude_fields(fields: &Fields) -> Vec<Member> {
	fields
		.views()
		.into_iter()
		.zip(fields.iter())
		.filter(|(_, field)| MetadataArgs::field_is_prelude(&field.attrs))
		.map(|(view, _)| view.member)
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
	let ident = &input.ident;
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

	// A node's metadata is its own merged with that of every field which is not skipped.
	let mut wc = WhereCollector::new();
	let arms: Vec<TokenStream> = Arm::all(&input)?
		.iter()
		.map(|arm| {
			let skip_arm = MetadataArgs::from_attributes(arm.attrs).map(|a| a.skip).unwrap_or(false);
			let merged: Vec<bool> =
				arm.fields.iter().map(|field| !skip_arm && !MetadataArgs::field_is_skipped(&field.attrs)).collect();
			for (field, _) in arm.fields.iter().zip(&merged).filter(|(_, merged)| **merged) {
				wc.add(&field.ty);
			}
			let pattern = arm.pattern(|i, view| merged[i].then(|| view.binding.clone()));
			let bindings =
				arm.fields.views().into_iter().zip(&merged).filter(|(_, merged)| **merged).map(|(view, _)| {
					let binding = view.binding;
					quote! { #binding }
				});
			let body = merge_expr(quote! { self.self_metadata() }, bindings);
			quote! { #pattern => #body, }
		})
		.collect();

	let where_clause =
		wc.extend_where_clause(&input.generics, parse_quote!(css_parse::NodeWithMetadata<crate::CssMetadata>));
	let (impl_generics, type_generics, _) = input.generics.split_for_impl();

	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics css_parse::NodeWithMetadata<crate::CssMetadata> for #ident #type_generics #where_clause {
			#self_metadata

			fn metadata(&self) -> crate::CssMetadata {
				match self { #(#arms)* }
			}
		}
	})
}
