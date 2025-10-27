use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Attribute, DeriveInput, Error, Ident, LitStr, Meta, Token,
	parse::{Parse, ParseStream},
};

#[derive(Debug, Default)]
struct MetadataArg {
	pub initial: Option<String>,
	pub inherits: Option<Ident>,
	pub applies_to: Vec<Ident>,
	pub animation_type: Option<Ident>,
	pub percentages: Option<Ident>,
	pub shorthand_group: Option<Ident>,
	pub property_group: Option<Ident>,
	pub computed_value_type: Option<Ident>,
	pub canonical_order: Option<String>,
	pub logical_property_group: Option<Ident>,
	pub box_side: Vec<Ident>,
	pub box_portion: Vec<Ident>,
}

impl Parse for MetadataArg {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut args = Self::default();
		while !input.is_empty() {
			match input.parse::<Ident>()? {
				i if i == "initial" => {
					if args.initial.is_some() {
						Err(Error::new(i.span(), "redefinition of 'initial'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.initial = Some(input.parse::<LitStr>()?.value());
				}
				i if i == "inherits" => {
					if args.inherits.is_some() {
						Err(Error::new(i.span(), "redefinition of 'inherits'".to_string()))?;
					}
					if input.parse::<Token![=]>().is_ok() {
						args.inherits = Some(input.parse::<Ident>()?);
					} else {
						args.inherits = Some(format_ident!("True"));
					}
				}
				i if i == "applies_to" => {
					if !args.applies_to.is_empty() {
						Err(Error::new(i.span(), "redefinition of 'applies_to'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					loop {
						args.applies_to.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				i if i == "animation_type" => {
					if args.animation_type.is_some() {
						Err(Error::new(i.span(), "redefinition of 'animation_type'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.animation_type = Some(input.parse::<Ident>()?);
				}
				i if i == "percentages" => {
					if args.percentages.is_some() {
						Err(Error::new(i.span(), "redefinition of 'percentages'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.percentages = Some(input.parse::<Ident>()?);
				}
				i if i == "shorthand_group" => {
					if args.shorthand_group.is_some() {
						Err(Error::new(i.span(), "redefinition of 'shorthand_group'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.shorthand_group = Some(input.parse::<Ident>()?);
				}
				i if i == "property_group" => {
					if args.property_group.is_some() {
						Err(Error::new(i.span(), "redefinition of 'property_group'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.property_group = Some(input.parse::<Ident>()?);
				}
				i if i == "computed_value_type" => {
					if args.computed_value_type.is_some() {
						Err(Error::new(i.span(), "redefinition of 'computed_value_type'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.computed_value_type = Some(input.parse::<Ident>()?);
				}
				i if i == "canonical_order" => {
					if args.canonical_order.is_some() {
						Err(Error::new(i.span(), "redefinition of 'canonical_order'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.canonical_order = Some(input.parse::<LitStr>()?.value());
				}
				i if i == "logical_property_group" => {
					if args.logical_property_group.is_some() {
						Err(Error::new(i.span(), "redefinition of 'logical_property_group'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					args.logical_property_group = Some(input.parse::<Ident>()?);
				}
				i if i == "box_side" => {
					if !args.box_side.is_empty() {
						Err(Error::new(i.span(), "redefinition of 'box_side'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					loop {
						args.box_side.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				i if i == "box_portion" => {
					if !args.box_portion.is_empty() {
						Err(Error::new(i.span(), "redefinition of 'box_portion'".to_string()))?;
					}
					input.parse::<Token![=]>()?;
					loop {
						args.box_portion.push(input.parse::<Ident>()?);
						if input.parse::<Token![|]>().is_err() {
							break;
						}
					}
				}
				ident => Err(Error::new(ident.span(), format!("Unrecognized Value arg {ident:?}")))?,
			}

			if !input.is_empty() {
				input.parse::<Token![,]>()?;
			}
		}
		Ok(args)
	}
}

impl From<&Vec<Attribute>> for MetadataArg {
	fn from(attrs: &Vec<Attribute>) -> Self {
		let mut result = Self::default();
		// Check for #[declaration_metadata(...)] attribute
		if let Some(Attribute { meta, .. }) = &attrs.iter().find(|a| a.path().is_ident("declaration_metadata")) {
			match meta {
				Meta::List(meta) => {
					result = meta.parse_args::<MetadataArg>().unwrap();
				}
				_ => panic!("could not parse meta"),
			}
		}
		result
	}
}

pub fn derive(input: DeriveInput) -> TokenStream {
	let attrs = MetadataArg::from(&input.attrs);
	let ident = &input.ident;
	let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
	let initial = attrs.initial.map(|initial| {
		quote! {
			fn initial() -> &'static str { #initial }
		}
	});
	let inherits = attrs.inherits.map(|inherits| {
		quote! {
			fn inherits() -> Inherits { Inherits::#inherits }
		}
	});
	let applies_to = if attrs.applies_to.is_empty() {
		quote! {}
	} else {
		let applies_to = attrs.applies_to;
		quote! { fn applies_to() -> AppliesTo { #(AppliesTo::#applies_to)|* } }
	};
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
	let box_side = if attrs.box_side.is_empty() {
		quote! {}
	} else {
		let box_side = attrs.box_side;
		quote! { fn box_side() -> BoxSide { #(BoxSide::#box_side)|* } }
	};
	let box_portion = if attrs.box_portion.is_empty() {
		quote! {}
	} else {
		let box_portion = attrs.box_portion;
		quote! { fn box_portion() -> BoxPortion { #(BoxPortion::#box_portion)|* } }
	};
	quote! {
	  #[automatically_derived]
	  impl #impl_generics crate::DeclarationMetadata for #ident #type_generics #where_clause {
			#initial
			#inherits
			#applies_to
			#animation_type
			#percentages
			#shorthand_group
			#property_group
			#computed_value_type
			#canonical_order
			#logical_property_group
			#box_side
			#box_portion
	  }
	}
}
