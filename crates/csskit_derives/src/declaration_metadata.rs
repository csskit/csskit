use darling::FromAttributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Ident, LitStr, Result, Token};

use crate::darling_ext::{InheritsArg, PipeList};

/// One slot of a shorthand's grammar: the property the slot sets, between string literals of the tokens the grammar
/// writes before and after it, and followed by `?` when leaving it out states the initial value.
struct SlotArg {
	before: TokenStream,
	property: Ident,
	optional: bool,
	after: TokenStream,
}

impl Parse for SlotArg {
	fn parse(input: ParseStream) -> Result<Self> {
		let before = match input.parse::<Option<LitStr>>()? {
			Some(before) => quote! { #before },
			None => quote! { "" },
		};
		let property = input.parse::<Ident>()?;
		let optional = input.parse::<Option<Token![?]>>()?.is_some();
		let after = match input.parse::<Option<LitStr>>()? {
			Some(after) => quote! { #after },
			None => quote! { "" },
		};
		Ok(Self { before, property, optional, after })
	}
}

/// The whole `declaration_writes` attribute: the slots, `repeat` for a grammar which repeats one longhand over the
/// positions of the shorthand's longhands, or `same` for a grammar whose one value every longhand takes.
enum WritesArg {
	Repeat,
	Same,
	Slots(Punctuated<SlotArg, Token![,]>),
}

impl Parse for WritesArg {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.peek(Ident) {
			match input.fork().parse::<Ident>()? {
				keyword if keyword == "repeat" => {
					input.parse::<Ident>()?;
					return Ok(Self::Repeat);
				}
				keyword if keyword == "same" => {
					input.parse::<Ident>()?;
					return Ok(Self::Same);
				}
				_ => {}
			}
		}
		Ok(Self::Slots(Punctuated::parse_terminated(input)?))
	}
}

impl WritesArg {
	fn to_metadata(&self) -> TokenStream {
		match self {
			Self::Repeat => quote! { crate::Writes::Repeat },
			Self::Same => quote! { crate::Writes::Same },
			Self::Slots(slots) => {
				let slots = slots.iter().map(|SlotArg { before, property, optional, after }| {
					quote! { crate::Slot { property: CssAtomSet::#property, before: #before, after: #after, optional: #optional } }
				});
				quote! { crate::Writes::Slots(&[#(#slots),*]) }
			}
		}
	}
}

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
	longhands: Option<PipeList<Ident>>,
	#[darling(default)]
	shorthands: Option<PipeList<Ident>>,
	#[darling(default)]
	resets: Option<PipeList<Ident>>,
	#[darling(default)]
	resets_all: bool,
	#[darling(default)]
	reset_by: Option<PipeList<Ident>>,
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
	let writes_attr = input.attrs.iter().find(|attr| attr.path().is_ident("declaration_writes"));
	let writes =
		writes_attr.map(|attr| attr.parse_args::<WritesArg>()).transpose()?.as_ref().map(WritesArg::to_metadata);
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
	let shorthand = {
		let longhands = attrs.longhands.map(|PipeList(values)| quote! { &[#(CssAtomSet::#values),*] });
		let resets = if attrs.resets_all {
			Some(quote! { crate::ShorthandReset::All })
		} else {
			attrs
				.resets
				.map(|PipeList(values)| quote! { crate::ShorthandReset::Properties(&[#(CssAtomSet::#values),*]) })
		};
		(longhands.is_some() || resets.is_some()).then(|| {
			let longhands = longhands.unwrap_or_else(|| quote! { &[] });
			let writes = writes.map_or_else(|| quote! { None }, |writes| quote! { Some(#writes) });
			let resets = resets.unwrap_or_else(|| quote! { crate::ShorthandReset::Properties(&[]) });
			quote! {
				fn shorthand() -> Option<&'static crate::Shorthand> {
					Some(&crate::Shorthand { longhands: #longhands, writes: #writes, resets: #resets })
				}
			}
		})
	};
	if shorthand.is_none()
		&& let Some(attr) = writes_attr
	{
		return Err(syn::Error::new_spanned(attr, "declaration_writes states no longhands for the value to write"));
	}
	let longhand = {
		let shorthands = attrs.shorthands.map(|PipeList(values)| quote! { &[#(CssAtomSet::#values),*] });
		let reset_by = attrs.reset_by.map(|PipeList(values)| quote! { &[#(CssAtomSet::#values),*] });
		(shorthands.is_some() || reset_by.is_some()).then(|| {
			let shorthands = shorthands.unwrap_or_else(|| quote! { &[] });
			let reset_by = reset_by.unwrap_or_else(|| quote! { &[] });
			quote! {
				fn longhand() -> Option<&'static crate::Longhand> {
					Some(&crate::Longhand { shorthands: #shorthands, reset_by: #reset_by })
				}
			}
		})
	};
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
			#shorthand
			#longhand
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
