use crate::type_renames::get_type_rename;
use css_value_definition_parser::*;
use heck::{ToPascalCase, ToSnakeCase};
use itertools::Itertools;
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote};
use std::ops::{Deref, Range};
use syn::{Error, Generics, Ident, Visibility, parse_quote};

pub fn pluralize(str: String) -> String {
	if str.ends_with("s") { str.clone() } else { format!("{str}s") }
}

/// Trait for extending Def with code generation methods.
pub trait DefExt {
	fn single_ident(ident: &Ident) -> Ident;
	fn keyword_ident(ident: &Ident) -> Ident;
	fn should_skip_visit(&self) -> bool;
	fn type_attributes(&self, derives_parse: bool, derives_visitable: bool) -> TokenStream;
	fn is_all_keywords(&self) -> bool;
	fn get_generics(&self) -> Generics;
	fn gather_keywords(&self) -> Vec<&Self>;
	fn generate_additional_types(&self, vis: &Visibility, ident: &Ident, generics: &Generics) -> TokenStream;
}

/// Trait for extending DefType with code generation methods.
pub trait DefTypeExt {
	fn generate_in_range_attr(&self) -> TokenStream;
	fn get_generics(&self) -> Generics;
}

pub trait GenerateDefinition {
	fn generate_definition(
		&self,
		vis: &Visibility,
		ident: &Ident,
		generics: &Generics,
		derives_parse: bool,
		derives_visitable: bool,
	) -> TokenStream;
}

/// Generate a suitable name for an enum variant or struct member given the Def.
pub trait ToFieldName {
	/// Generates an Ident suitable for naming an enum variant.
	fn to_variant_name(&self, size_hint: usize) -> Ident;

	/// Generates an Ident suitable for naming a struct member.
	fn to_member_name(&self, size_hint: usize) -> Ident {
		format_ident!("{}", self.to_variant_name(size_hint).to_string().to_snake_case())
	}
}

// Generate a suitable type for the given Def
pub trait ToType {
	fn to_type(&self) -> TokenStream {
		let types = self.to_types();
		if types.len() == 1 {
			quote! { #(#types)* }
		} else {
			quote! { (#(#types,)*) }
		}
	}

	fn to_types(&self) -> Vec<TokenStream>;
}

impl ToFieldName for DefIdent {
	fn to_variant_name(&self, size_hint: usize) -> Ident {
		let pascal = self.0.to_pascal_case();
		format_ident!("{}", if size_hint > 0 { pluralize(pascal) } else { pascal })
	}
}

impl ToFieldName for DefType {
	fn to_variant_name(&self, size_hint: usize) -> Ident {
		let str = self.ident.to_string();
		format_ident!("{}", if size_hint > 0 { pluralize(str) } else { str })
	}
}

impl ToFieldName for Def {
	fn to_variant_name(&self, size_hint: usize) -> Ident {
		match self {
			Self::Ident(v) => v.to_variant_name(size_hint),
			Self::Type(v) => v.to_variant_name(size_hint),
			Self::StyleValue(v) => v.to_variant_name(size_hint),
			Self::FunctionType(v) => format_ident!("{}Function", v.to_variant_name(size_hint)),
			Self::AutoOr(ty) => format_ident!("AutoOr{}", ty.deref().to_variant_name(size_hint)),
			Self::NoneOr(ty) => format_ident!("NoneOr{}", ty.deref().to_variant_name(size_hint)),
			Self::AutoNoneOr(ty) => format_ident!("AutoNoneOr{}", ty.deref().to_variant_name(size_hint)),
			Self::Function(v, _) => format_ident!("{}Function", v.0.to_pascal_case()),
			Self::Multiplier(v, _, _) => v.deref().to_variant_name(2),
			Self::Group(def, _) => def.deref().to_variant_name(size_hint),
			Self::Optional(def) => def.deref().to_variant_name(size_hint),
			Self::IntLiteral(v) => format_ident!("Literal{}", v.to_string()),
			Self::DimensionLiteral(int, dim) => format_ident!("Literal{int}{dim}"),
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => {
				let (optional, others): (Vec<&Def>, Vec<&Def>) = ds.iter().partition(|d| matches!(d, Def::Optional(_)));
				let logical_first = others.first().or(optional.first());
				logical_first.expect("At least one Def is required").to_variant_name(0)
			}
			Self::Combinator(ds, DefCombinatorStyle::Options) => {
				let auto_generated_name = ds.iter().fold(String::new(), |str, d| match d {
					Def::Type(d) => {
						format!("{}{}", str, d.to_variant_name(0))
					}
					_ => {
						dbg!("TODO variant name for Combinator() of Options", d);
						todo!("variant name")
					}
				});
				format_ident!("{}", get_type_rename(&auto_generated_name).unwrap_or(&auto_generated_name))
			}
			Self::Combinator(_, _) => {
				dbg!("TODO variant name for Combinator()", self);
				todo!("variant name")
			}
			Self::Punct(_) => {
				dbg!("TODO variant name for Punct()", self);
				todo!("variant name")
			}
		}
	}
}

impl ToType for DefIdent {
	fn to_types(&self) -> Vec<TokenStream> {
		vec![quote! { ::css_parse::T![Ident] }]
	}
}

impl ToType for Def {
	fn to_types(&self) -> Vec<TokenStream> {
		match self {
			Self::Ident(v) => v.to_types(),
			Self::Type(v) => v.to_types(),
			Self::StyleValue(ty) => {
				let ident = format_ident!("{}StyleValue", ty.ident.0);
				let generics = self.get_generics();
				vec![quote! { crate::#ident #generics }]
			}
			Self::FunctionType(ty) => {
				let ident = format_ident!("{}Function", ty.ident.0);
				let generics = self.get_generics();
				vec![quote! { crate::#ident #generics }]
			}
			Self::AutoOr(ty) => {
				let ty = ty.to_type();
				vec![quote! { crate::AutoOr<#ty> }]
			}
			Self::NoneOr(ty) => {
				let ty = ty.to_type();
				vec![quote! { crate::NoneOr<#ty> }]
			}
			Self::AutoNoneOr(ty) => {
				let ty = ty.to_type();
				vec![quote! { crate::AutoNoneOr<#ty> }]
			}
			Self::Optional(v) => {
				let ty = v.to_type();
				vec![quote! { Option<#ty> }]
			}
			Self::Function(_, _) => {
				let func_name = self.to_variant_name(0);
				let generics = self.get_generics();
				vec![quote! { crate::#func_name #generics }]
			}
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => ds.iter().map(|d| d.to_type()).collect(),
			Self::Combinator(_, DefCombinatorStyle::Alternatives) => {
				dbg!("TODO to_type for Combinator::Alternatives()", self);
				todo!("to_type")
			}
			Self::Combinator(ds, DefCombinatorStyle::Options) => {
				let types = ds.iter().map(|d| d.to_type());
				vec![quote! { ::css_parse::Optionals![#(#types),*] }]
			}
			Self::Combinator(ds, DefCombinatorStyle::AllMustOccur) => {
				let types = ds.iter().map(|d| d.to_type());
				vec![quote! { #(#types),* }]
			}
			Self::Multiplier(def, DefMultiplierSeparator::Commas, range) => {
				let ty = def.deref().to_type();
				let min = match range {
					DefRange::Range(Range { start, .. }) if *start != 1.0 => Some(*start as usize),
					DefRange::RangeFrom(f) if *f != 1.0 => Some(*f as usize),
					DefRange::Fixed(f) if *f != 1.0 => Some(*f as usize),
					_ => None,
				};
				vec![quote! { ::css_parse::CommaSeparated<'a, #ty, #min> }]
			}
			Self::Multiplier(def, DefMultiplierSeparator::None, _) => {
				let ty = def.deref().to_type();
				vec![quote! { ::bumpalo::collections::Vec<'a, #ty> }]
			}
			Self::IntLiteral(_) => vec![quote! { crate::CSSInt }],
			Self::DimensionLiteral(_, _) => vec![quote! { ::css_parse::T![Dimension] }],
			Self::Punct(char) => {
				let punct = Punct::new(*char, Spacing::Alone);
				vec![quote! { ::css_parse::T![#punct] }]
			}
			Self::Group(inner, _) => inner.deref().to_types(),
		}
	}
}

impl ToType for DefType {
	fn to_types(&self) -> Vec<TokenStream> {
		let ty = &self.ident;
		let type_name = quote! { crate::#ty };
		let generics = self.get_generics();
		vec![quote! { #type_name #generics }]
	}
}

impl DefExt for Def {
	fn single_ident(ident: &Ident) -> Ident {
		let ident = ident.to_string();
		let ident = ident.strip_prefix("Single").unwrap_or(&ident);
		format_ident!("Single{}", ident)
	}

	fn keyword_ident(ident: &Ident) -> Ident {
		let ident = ident.to_string();
		let ident = ident.strip_prefix("Single").unwrap_or(&ident);
		format_ident!("{}Keywords", ident)
	}

	fn should_skip_visit(&self) -> bool {
		match self {
			Self::Ident(_) => true,
			Self::IntLiteral(_) => true,
			Self::DimensionLiteral(_, _) => true,
			Self::Function(_, _) => false,
			Self::AutoOr(ty) => ty.as_ref().should_skip_visit(),
			Self::NoneOr(ty) => ty.as_ref().should_skip_visit(),
			Self::AutoNoneOr(ty) => ty.as_ref().should_skip_visit(),
			Self::Type(DefType { ident, .. }) => ident.0.ends_with("Keywords"),
			Self::StyleValue(_) => false,
			Self::FunctionType(_) => false,
			Self::Optional(d) => d.should_skip_visit(),
			Self::Combinator(d, _) => d.iter().all(|d| d.should_skip_visit()),
			Self::Group(d, _) => d.should_skip_visit(),
			Self::Multiplier(d, _, _) => d.should_skip_visit(),
			Self::Punct(_) => false,
		}
	}

	fn type_attributes(&self, derives_parse: bool, derives_visitable: bool) -> TokenStream {
		let skip = if derives_visitable && self.should_skip_visit() {
			quote! { #[cfg_attr(feature = "visitable", visit(skip))] }
		} else {
			quote! {}
		};
		let in_range = match self {
			Def::IntLiteral(i) if derives_parse => {
				let f = *i as f32;
				quote! { #[in_range(#f..=#f)] }
			}
			Def::DimensionLiteral(f, _) if derives_parse => {
				quote! { #[in_range(#f..=#f)] }
			}
			Def::Optional(def) | Def::AutoNoneOr(def) | Def::AutoOr(def) | Def::NoneOr(def) => match def.deref() {
				Def::Type(deftype) if derives_parse => deftype.generate_in_range_attr(),
				_ => quote! {},
			},
			Def::Type(deftype) if derives_parse => deftype.generate_in_range_attr(),
			_ => quote! {},
		};
		let atom = match self {
			Def::Type(ty) => match ty.ident_str() {
				"Decibel" => quote! { #[atom(CssAtomSet::Db)] },
				_ => quote! {},
			},
			Def::DimensionLiteral(_, unit) if derives_parse => {
				let name = format_ident!("{}", unit.to_pascal_case());
				quote! { #[atom(CssAtomSet::#name)] }
			}
			Def::Ident(DefIdent(str)) if derives_parse => {
				let name = format_ident!("{}", str.to_pascal_case());
				quote! { #[atom(CssAtomSet::#name)] }
			}
			_ => quote! {},
		};
		quote! { #skip #in_range #atom }
	}

	fn is_all_keywords(&self) -> bool {
		match self {
			Self::Ident(_) => true,
			Self::IntLiteral(_) => false,
			Self::DimensionLiteral(_, _) => false,
			Self::Function(_, _) => false,
			Self::Type(DefType { ident, .. }) => ident.0.ends_with("Keywords"),
			Self::FunctionType(_) => false,
			Self::StyleValue(_) => false,
			Self::AutoOr(def) => def.deref().is_all_keywords(),
			Self::NoneOr(def) => def.deref().is_all_keywords(),
			Self::AutoNoneOr(def) => def.deref().is_all_keywords(),
			Self::Optional(def) => def.deref().is_all_keywords(),
			Self::Combinator(defs, _) => defs.iter().all(Self::is_all_keywords),
			Self::Group(def, _) => def.deref().is_all_keywords(),
			Self::Multiplier(def, _, _) => def.deref().is_all_keywords(),
			Self::Punct(_) => false,
		}
	}

	fn gather_keywords(&self) -> Vec<&Self> {
		match self {
			// Self::Ident shouldn't return itself because it can be used in a literal position.
			Self::Ident(_) => vec![],
			Self::Function(_, _) => vec![],
			Self::AutoOr(_) => vec![],
			Self::NoneOr(_) => vec![],
			Self::AutoNoneOr(_) => vec![],
			Self::StyleValue(_) => vec![],
			Self::FunctionType(_) => vec![],
			Self::Type(_) => vec![],
			Self::Optional(def) => def.gather_keywords(),
			Self::Combinator(opts, DefCombinatorStyle::Alternatives)
			| Self::Combinator(opts, DefCombinatorStyle::Options) => {
				opts.iter().filter(|def| matches!(def, Self::Ident(_))).collect()
			}
			Self::Combinator(opts, DefCombinatorStyle::Ordered) => {
				opts.iter().flat_map(Self::gather_keywords).collect()
			}
			Self::Combinator(opts, DefCombinatorStyle::AllMustOccur) => {
				opts.iter().flat_map(Self::gather_keywords).collect()
			}
			Self::Group(def, _) => def.gather_keywords(),
			Self::Multiplier(def, _, _) => def.gather_keywords(),
			Self::Punct(_) => vec![],
			Self::IntLiteral(_) => vec![],
			Self::DimensionLiteral(_, _) => vec![],
		}
	}

	fn get_generics(&self) -> Generics {
		// NonrOr/AutoOr might maybe_unsized for the internal to the type, but shoulnd't express it's own generics
		if self.maybe_unsized() && !matches!(self, Def::NoneOr(_) | Def::AutoOr(_) | Def::AutoNoneOr(_)) {
			parse_quote!(<'a>)
		} else {
			Default::default()
		}
	}

	fn generate_additional_types(&self, vis: &Visibility, ident: &Ident, _generics: &Generics) -> TokenStream {
		let needs_keyword_type = match self {
			Self::Combinator(defs, DefCombinatorStyle::Ordered) => defs.iter().all(|def| def.is_all_keywords()),
			Self::Multiplier(def, _, _) => match def.deref() {
				Self::Combinator(defs, DefCombinatorStyle::Alternatives) => {
					defs.iter().all(|def| matches!(def, Def::Ident(_)))
				}
				_ => false,
			},
			_ => false,
		};
		let keyword_type = if needs_keyword_type {
			let keywords: Vec<TokenStream> = self
				.gather_keywords()
				.iter()
				.unique_by(|def| if let Self::Ident(DefIdent(str)) = def { str } else { "" })
				.filter_map(|def| {
					if let Self::Ident(def) = def {
						let ident = format_ident!("{}", def.to_string().to_pascal_case());
						let ty = def.to_type();
						Some(quote! { #[atom(CssAtomSet::#ident)] #ident(#ty), })
					} else {
						None
					}
				})
				.collect();
			let keyword_name = Self::keyword_ident(ident);
			quote! {
				#[derive(
					::csskit_derives::Parse,
					::csskit_derives::Peek,
					::csskit_derives::ToCursors,
					::csskit_derives::ToSpan,
					::csskit_derives::SemanticEq,
					Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
				#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
				#[cfg_attr(feature = "visitable", derive(::csskit_derives::Visitable), visit(skip))]
				pub enum #keyword_name {
					#(#keywords)*
				}
			}
		} else {
			quote! {}
		};
		let single_type = match self {
			Self::Multiplier(defs, _, range) => {
				match defs.deref() {
					// Combinator of alternatives where all alternatives are keywords does not need
					// an additional type beyond the keyword_type.
					Def::Combinator(defs, DefCombinatorStyle::Alternatives)
						if defs.iter().all(|def| matches!(def, Def::Ident(_))) =>
					{
						quote! {}
					}
					Def::Combinator(_, _) if matches!(range, DefRange::RangeFrom(_) | DefRange::RangeTo(_)) => {
						let ident = Self::single_ident(ident);
						let generics = defs.get_generics();
						let def = defs.generate_definition(vis, &ident, &generics, true, true);
						quote! {
							#[derive(
								::csskit_derives::Parse,
								::csskit_derives::Peek,
								::csskit_derives::ToSpan,
								::csskit_derives::ToCursors,
								::csskit_derives::SemanticEq,
								Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
							#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
							#[cfg_attr(feature = "visitable", derive(::csskit_derives::Visitable), visit(children))]
							#def
						}
					}
					_ => quote! {},
				}
			}
			_ => quote! {},
		};
		quote! {
			#keyword_type
			#single_type
		}
	}
}

impl GenerateDefinition for Def {
	fn generate_definition(
		&self,
		vis: &Visibility,
		ident: &Ident,
		generics: &Generics,
		derives_parse: bool,
		derives_visitable: bool,
	) -> TokenStream {
		let (_, type_generics, where_clause) = generics.split_for_impl();
		match self.suggested_data_type() {
			DataType::SingleUnnamedStruct => {
				let mut struct_attrs = quote! {};
				let members = match self {
					Self::Combinator(_, DefCombinatorStyle::Alternatives) => {
						Error::new(ident.span(), "cannot generate alternative combinators in struct")
							.into_compile_error()
					}
					Self::Combinator(defs, DefCombinatorStyle::Options) => {
						let members = defs.iter().map(|def| {
							let name = def.to_member_name(0);
							let ty = def.to_type();
							let attrs = def.type_attributes(derives_parse, derives_visitable);
							quote! { #attrs pub #name: Option<#ty> }
						});
						if derives_parse {
							struct_attrs.extend(quote! { #[parse(one_must_occur)] })
						}
						quote! { { #(#members),* } }
					}
					Self::Combinator(defs, DefCombinatorStyle::Ordered) => {
						let types = defs.iter().map(|def| {
							let ty = if def.is_all_keywords() {
								let keyword_name = Self::keyword_ident(ident);
								match def {
									Self::Optional(_) => quote! { Option<#keyword_name> },
									_ => quote! { #keyword_name },
								}
							} else {
								def.to_type()
							};
							let attrs = def.type_attributes(derives_parse, derives_visitable);
							quote! { #attrs pub #ty }
						});
						quote! { ( #(#types),* ); }
					}
					Self::Combinator(defs, DefCombinatorStyle::AllMustOccur) => {
						struct_attrs.extend(quote! { #[parse(all_must_occur)] });
						let types = defs.iter().map(|def| {
							let ty = def.to_type();
							let attrs = def.type_attributes(derives_parse, derives_visitable);
							quote! { #attrs pub #ty }
						});
						quote! { ( #(#types),* ); }
					}
					Self::Multiplier(def, sep, range) => match def.deref() {
						Self::Combinator(defs, DefCombinatorStyle::Alternatives)
							if defs.iter().all(|def| matches!(def, Def::Ident(_))) =>
						{
							let keyword_name = Self::keyword_ident(ident);
							let phantom_type = Self::Multiplier(
								Box::new(Def::Type(DefType::new(&keyword_name.to_string(), DefRange::None))),
								*sep,
								range.clone(),
							);
							let ty = phantom_type.to_type();
							quote! { ( pub #ty ); }
						}
						Self::Combinator(_, _) if matches!(range, DefRange::RangeFrom(_) | DefRange::RangeTo(_)) => {
							let ty_ident = Self::single_ident(ident);
							// Check if the inner combinator needs a lifetime - if so, add it manually
							let needs_lifetime = def.maybe_unsized();
							let generics = if needs_lifetime {
								quote! { <'a> }
							} else {
								quote! {}
							};
							let inner_type_ref = quote! { crate::#ty_ident #generics };
							let ty = match sep {
								DefMultiplierSeparator::Commas => {
									let min = match range {
										DefRange::Range(Range { start, .. }) if *start != 1.0 => Some(*start as usize),
										DefRange::RangeFrom(f) if *f != 1.0 => Some(*f as usize),
										DefRange::Fixed(f) if *f != 1.0 => Some(*f as usize),
										_ => None,
									};
									vec![quote! { ::css_parse::CommaSeparated<'a, #inner_type_ref, #min> }]
								}
								DefMultiplierSeparator::None => {
									vec![quote! { ::bumpalo::collections::Vec<'a, #inner_type_ref> }]
								}
							};
							quote! { ( #(pub #ty),* ); }
						}
						_ => {
							let ty = self.to_types();
							let attrs = self.type_attributes(derives_parse, derives_visitable);
							quote! { ( #(#attrs pub #ty),* ); }
						}
					},
					_ => {
						let ty = self.to_types();
						let attrs = self.type_attributes(derives_parse, derives_visitable);
						quote! { ( #(#attrs pub #ty),* ); }
					}
				};
				quote! { #struct_attrs #vis struct #ident #type_generics #where_clause #members }
			}
			DataType::Enum => match self {
				Self::Combinator(children, DefCombinatorStyle::Alternatives) => {
					let variants: TokenStream = children
						.iter()
						.map(|d| {
							let mut attrs = Some(d.type_attributes(derives_parse, derives_visitable));
							let name = d.to_variant_name(0);
							let types = match d {
								Self::Combinator(defs, DefCombinatorStyle::Ordered) => defs
									.iter()
									.map(|d| {
										let ty = d.to_type();
										let attrs = d.type_attributes(derives_parse, derives_visitable);
										quote! { #attrs #ty }
									})
									.collect(),
								Self::Ident(_) => d.to_types(),
								Self::IntLiteral(_) | Self::DimensionLiteral(_, _) => {
									let attrs = attrs.take().unwrap();
									let ty = d.to_type();
									vec![quote! { #attrs #ty }]
								}
								Self::Type(_) => {
									let attrs = attrs.take().unwrap();
									let ty = d.to_type();
									vec![quote! { #attrs #ty }]
								}
								Self::Optional(inner) if matches!(inner.deref(), Def::Type(_)) => {
									let attrs = attrs.take().unwrap();
									let ty = d.to_type();
									vec![quote! { #attrs #ty }]
								}
								_ => d.to_types(),
							};
							quote! { #attrs #name(#(#types),*), }
						})
						.collect();
					quote! { #vis enum #ident #type_generics #where_clause { #variants } }
				}
				Self::Combinator(_, _) => {
					Error::new(ident.span(), "cannot generate non-Alternatives combinators in enum")
						.into_compile_error()
				}
				_ => {
					dbg!("TODO non union enum", self);
					todo!("non union enum")
				}
			},
		}
	}
}

impl DefTypeExt for DefType {
	fn generate_in_range_attr(&self) -> TokenStream {
		match self.range {
			DefRange::None | DefRange::Fixed(_) => quote! {},
			DefRange::Range(Range { start, end }) => quote! { #[in_range(#start..=#end)] },
			DefRange::RangeFrom(start) => quote! { #[in_range(#start..)] },
			DefRange::RangeTo(end) => quote! { #[in_range(..=#end)] },
		}
	}

	fn get_generics(&self) -> Generics {
		if self.maybe_unsized() { parse_quote!(<'a>) } else { Default::default() }
	}
}
