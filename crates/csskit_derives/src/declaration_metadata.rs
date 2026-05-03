use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Result};

use crate::darling_ext::{InheritsArg, PipeList};

#[derive(Debug, Default, FromAttributes)]
#[darling(attributes(declaration_metadata))]
struct MetadataArg {
	#[darling(default)]
	pub initial: Option<String>,
	#[darling(default)]
	pub inherits: Option<InheritsArg>,
	#[darling(default)]
	pub applies_to: Option<PipeList<Ident>>,
	#[darling(default)]
	pub animation_type: Option<Ident>,
	#[darling(default)]
	pub percentages: Option<Ident>,
	#[darling(default)]
	pub shorthand_group: Option<Ident>,
	#[darling(default)]
	pub longhands: Option<PipeList<Ident>>,
	#[darling(default)]
	pub property_group: Option<Ident>,
	#[darling(default)]
	pub computed_value_type: Option<Ident>,
	#[darling(default)]
	pub canonical_order: Option<String>,
	#[darling(default)]
	pub logical_property_group: Option<Ident>,
	#[darling(default)]
	pub box_side: Option<PipeList<Ident>>,
	#[darling(default)]
	pub box_portion: Option<PipeList<Ident>>,
	#[darling(default)]
	pub unitless_zero_resolves: Option<Ident>,
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
	let attrs = MetadataArg::from_attributes(&input.attrs)?;
	let ident = &input.ident;
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
	let initial = attrs.initial.map(|initial| {
		quote! {
			fn initial() -> &'static str { #initial }
		}
	});
	let inherits = attrs.inherits.map(|InheritsArg(inherits)| {
		quote! {
			fn inherits() -> Inherits { Inherits::#inherits }
		}
	});
	let applies_to = attrs.applies_to.map(|PipeList(applies_to)| {
		quote! { fn applies_to() -> AppliesTo { #(AppliesTo::#applies_to)|* } }
	});
	let animation_type = attrs.animation_type.map(|animation_type| {
		quote! {
			fn animation_type() -> AnimationType { AnimationType::#animation_type }
		}
	});
	let percentages = attrs.percentages.map(|percentages| {
		quote! {
			fn percentages() -> Percentages { Percentages::#percentages }
		}
	});
	let shorthand_group = attrs.shorthand_group.map(|shorthand_group| {
		quote! {
			fn shorthand_group() -> CssAtomSet { CssAtomSet::#shorthand_group }
		}
	});
	let property_group = attrs.property_group.map(|property_group| {
		quote! {
			fn property_group() -> PropertyGroup { PropertyGroup::#property_group }
		}
	});
	let computed_value_type = attrs.computed_value_type.map(|computed_value_type| {
		quote! {
			fn computed_value_type() -> ComputedValueType { ComputedValueType::#computed_value_type }
		}
	});
	let canonical_order = attrs.canonical_order.map(|canonical_order| {
		quote! {
			fn canonical_order() -> Option<&'static str> { Some(#canonical_order) }
		}
	});
	let logical_property_group = attrs.logical_property_group.map(|logical_property_group| {
		quote! {
			fn logical_property_group() -> Option<CssAtomSet> { Some(CssAtomSet::#logical_property_group) }
		}
	});
	let box_side = attrs.box_side.map(|PipeList(box_side)| {
		quote! { fn box_side() -> BoxSide { #(BoxSide::#box_side)|* } }
	});
	let box_portion = attrs.box_portion.map(|PipeList(box_portion)| {
		quote! { fn box_portion() -> BoxPortion { #(BoxPortion::#box_portion)|* } }
	});
	let longhands = attrs.longhands.map(|PipeList(longhands)| {
		quote! {
			fn longhands() -> Option<&'static [CssAtomSet]> {
				Some(&[#(CssAtomSet::#longhands),*])
			}
			fn is_shorthand() -> bool { true }
		}
	});
	let unitless_zero_resolves = attrs.unitless_zero_resolves.map(|unitless_zero_resolves| {
		quote! {
			fn unitless_zero_resolves() -> crate::UnitlessZeroResolves { crate::UnitlessZeroResolves::#unitless_zero_resolves }
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
			#shorthand_group
			#longhands
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
