use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Result};

use crate::darling_ext::{InheritsArg, PipeList};

#[derive(Debug, Default, FromAttributes)]
#[darling(attributes(declaration_metadata))]
struct MetadataArg {
	#[darling(default)]
	initial: Option<String>,
	#[darling(default)]
	inherits: Option<InheritsArg>,
	#[darling(default)]
	applies_to: Option<PipeList<Ident>>,
	#[darling(default)]
	animation_type: Option<Ident>,
	#[darling(default)]
	percentages: Option<Ident>,
	#[darling(default)]
	shorthand: bool,
	#[darling(default)]
	longhands: Option<PipeList<Ident>>,
	#[darling(default)]
	shorthand_group: Option<Ident>,
	#[darling(default)]
	shorthand_resets_known: bool,
	#[darling(default)]
	shorthand_resets: Option<PipeList<Ident>>,
	#[darling(default)]
	shorthand_resets_all: bool,
	#[darling(default)]
	reset_by_shorthands: Option<PipeList<Ident>>,
	#[darling(default)]
	property_group: Option<Ident>,
	#[darling(default)]
	computed_value_type: Option<Ident>,
	#[darling(default)]
	canonical_order: Option<String>,
	#[darling(default)]
	logical_property_group: Option<Ident>,
	#[darling(default)]
	box_side: Option<PipeList<Ident>>,
	#[darling(default)]
	box_portion: Option<PipeList<Ident>>,
	#[darling(default)]
	unitless_zero_resolves: Option<Ident>,
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let attrs = MetadataArg::from_attributes(&input.attrs)?;
	let ident = &input.ident;
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
	let initial = attrs.initial.map(|initial| quote! { fn initial() -> &'static str { #initial } });
	let inherits = attrs.inherits.map(|InheritsArg(inherits)| {
		quote! { fn inherits() -> Inherits { Inherits::#inherits } }
	});
	let applies_to = attrs
		.applies_to
		.map(|PipeList(applies_to)| quote! { fn applies_to() -> AppliesTo { #(AppliesTo::#applies_to)|* } });
	let animation_type = attrs
		.animation_type
		.map(|animation_type| quote! { fn animation_type() -> AnimationType { AnimationType::#animation_type } });
	let percentages =
		attrs.percentages.map(|percentages| quote! { fn percentages() -> Percentages { Percentages::#percentages } });
	let longhands = attrs.longhands.map(|PipeList(values)| {
		quote! {
			fn longhands() -> Option<&'static [CssAtomSet]> {
				Some(&[#(CssAtomSet::#values),*])
			}
			fn is_shorthand() -> bool { true }
		}
	});
	let shorthand = attrs.shorthand.then(|| quote! { fn is_shorthand() -> bool { true } });
	let shorthand_group = attrs.shorthand_group.map(|shorthand_group| {
		quote! { fn shorthand_group() -> CssAtomSet { CssAtomSet::#shorthand_group } }
	});
	let shorthand_reset = {
		let resets_opt = attrs.shorthand_resets.map(|PipeList(values)| values);
		let should_emit = attrs.shorthand_resets_known || resets_opt.is_some() || attrs.shorthand_resets_all;
		let reset = if attrs.shorthand_resets_all {
			quote! { crate::ShorthandReset::All }
		} else if attrs.shorthand_resets_known || resets_opt.is_some() {
			let resets = resets_opt.unwrap_or_default();
			quote! { crate::ShorthandReset::Properties(&[#(CssAtomSet::#resets),*]) }
		} else {
			quote! { crate::ShorthandReset::Unknown }
		};
		should_emit.then(|| {
			quote! {
				fn shorthand_reset() -> crate::ShorthandReset {
					#reset
				}
			}
		})
	};
	let reset_by_shorthands = attrs.reset_by_shorthands.map(|PipeList(values)| {
		quote! {
			fn reset_by_shorthands() -> &'static [CssAtomSet] {
				&[#(CssAtomSet::#values),*]
			}
		}
	});
	let property_group = attrs
		.property_group
		.map(|property_group| quote! { fn property_group() -> PropertyGroup { PropertyGroup::#property_group } });
	let computed_value_type = attrs.computed_value_type.map(|computed_value_type| {
		quote! { fn computed_value_type() -> ComputedValueType { ComputedValueType::#computed_value_type } }
	});
	let canonical_order = attrs
		.canonical_order
		.map(|canonical_order| quote! { fn canonical_order() -> Option<&'static str> { Some(#canonical_order) } });
	let logical_property_group = attrs.logical_property_group.map(|logical_property_group| {
		quote! { fn logical_property_group() -> Option<CssAtomSet> { Some(CssAtomSet::#logical_property_group) } }
	});
	let box_side =
		attrs.box_side.map(|PipeList(box_side)| quote! { fn box_side() -> BoxSide { #(BoxSide::#box_side)|* } });
	let box_portion = attrs
		.box_portion
		.map(|PipeList(box_portion)| quote! { fn box_portion() -> BoxPortion { #(BoxPortion::#box_portion)|* } });
	let unitless_zero_resolves = attrs.unitless_zero_resolves.map(|unitless_zero_resolves| {
		quote! {
			fn unitless_zero_resolves() -> crate::UnitlessZeroResolves {
				crate::UnitlessZeroResolves::#unitless_zero_resolves
			}
		}
	});
	Ok(quote! {
		#[automatically_derived]
		impl #impl_generics crate::DeclarationMetadata for #ident #type_generics #where_clause {
			#initial
			#inherits
			#applies_to
			#animation_type
			#percentages
			#longhands
			#shorthand
			#shorthand_group
			#shorthand_reset
			#reset_by_shorthands
			#property_group
			#computed_value_type
			#canonical_order
			#logical_property_group
			#box_side
			#box_portion
			#unitless_zero_resolves
		}
	})
}
