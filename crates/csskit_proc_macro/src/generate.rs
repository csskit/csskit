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
	fn options_ident(ident: &Ident) -> Ident;
	fn should_skip_visit(&self) -> bool;
	fn type_attributes(&self, derives_parse: bool, derives_visitable: bool) -> TokenStream;
	fn is_all_keywords(&self) -> bool;
	fn get_generics(&self) -> Generics;
	fn gather_keywords(&self) -> Vec<&Self>;
	fn rewrite_options_type(&self, replacement: &TokenStream) -> TokenStream;
	fn generate_additional_types(&self, vis: &Visibility, ident: &Ident, generics: &Generics) -> TokenStream;
}

/// Trait for extending DefType with code generation methods.
pub trait DefTypeExt {
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
			Self::NormalOr(ty) => format_ident!("NormalOr{}", ty.deref().to_variant_name(size_hint)),
			Self::Function(v, _) => format_ident!("{}Function", v.0.to_pascal_case()),
			Self::Multiplier(v, _, _) => v.deref().to_variant_name(2),
			Self::Group(def, _) => def.deref().to_variant_name(size_hint),
			Self::Optional(def) => def.deref().to_variant_name(size_hint),
			Self::IntLiteral(v) => format_ident!("Literal{}", v.to_string()),
			Self::DimensionLiteral(int, dim) => format_ident!("Literal{int}{dim}"),
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => {
				let non_optional: Vec<String> = ds
					.iter()
					.filter(|d| !matches!(d, Def::Optional(_) | Def::Punct(_)))
					.map(|d| d.to_variant_name(0).to_string())
					.collect();
				let distinct_count = {
					let mut uniq = non_optional.clone();
					uniq.dedup();
					uniq.len()
				};
				if distinct_count > 1 {
					let name: String = ds
						.iter()
						.filter(|d| !matches!(d, Def::Punct(_)))
						.map(|d| d.to_variant_name(0).to_string())
						.collect();
					format_ident!("{}", name)
				} else {
					let (optional, others): (Vec<&Def>, Vec<&Def>) =
						ds.iter().filter(|d| !matches!(d, Def::Punct(_))).partition(|d| matches!(d, Def::Optional(_)));
					let logical_first = others.first().or(optional.first());
					logical_first.expect("At least one Def is required").to_variant_name(0)
				}
			}
			Self::Combinator(ds, DefCombinatorStyle::Options) => {
				let auto_generated_name: String = ds
					.iter()
					.filter(|d| !matches!(d, Def::Punct(_)))
					.map(|d| d.to_variant_name(0).to_string())
					.collect();
				format_ident!("{}", get_type_rename(&auto_generated_name).unwrap_or(&auto_generated_name))
			}
			Self::Combinator(ds, DefCombinatorStyle::AllMustOccur) => {
				let name: String = ds
					.iter()
					.filter(|d| !matches!(d, Def::Punct(_)))
					.map(|d| d.to_variant_name(0).to_string())
					.collect();
				format_ident!("{}", name)
			}
			Self::Combinator(_, _) => {
				dbg!("TODO variant name for Combinator()", self);
				todo!("variant name")
			}
			Self::Punct(c) => panic!("Punct('{c}') has no variant name; filter before calling to_variant_name"),
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
			Self::NormalOr(ty) => {
				let ty = ty.to_type();
				vec![quote! { crate::NormalOr<#ty> }]
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
				let types: Vec<_> = ds.iter().map(|d| d.to_type()).collect();
				if types.len() == 1 { types } else { vec![quote! { (#(#types),*) }] }
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
			Self::IntLiteral(value) => {
				let val = *value;
				vec![quote! { crate::Exact<crate::CSSInt, #val> }]
			}
			Self::DimensionLiteral(value, _) => {
				let val = *value as i32;
				vec![quote! { crate::Exact<::css_parse::T![Dimension], #val> }]
			}
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
		let base_type = quote! { #type_name #generics };

		let wrapped_type = match self.range {
			DefRange::None | DefRange::Fixed(_) => base_type,
			DefRange::Range(Range { start, end }) => {
				if start == end {
					let value = start as i32;
					quote! { crate::Exact<#base_type, #value> }
				} else {
					let min = start as i32;
					let max = end as i32;
					quote! { crate::Ranged<#base_type, #min, #max> }
				}
			}
			DefRange::RangeFrom(start) => {
				if start == 0.0 {
					quote! { crate::NonNegative<#base_type> }
				} else if start > 0.0 && start <= 1.0 {
					quote! { crate::Positive<#base_type> }
				} else {
					let min = start as i32;
					let max = i32::MAX;
					quote! { crate::Ranged<#base_type, #min, #max> }
				}
			}
			DefRange::RangeTo(end) => {
				let min = i32::MIN;
				let max = end as i32;
				quote! { crate::Ranged<#base_type, #min, #max> }
			}
		};

		vec![wrapped_type]
	}
}

/// Find Options combinators containing keyword (Ident) children that need helper structs.
/// Searches: direct Alternatives children, and NoneOr/AutoOr/NormalOr/AutoNoneOr wrappers.
fn find_options_with_keywords(def: &Def) -> Vec<&Def> {
	fn is_options_with_keywords(def: &Def) -> bool {
		if let Def::Combinator(defs, DefCombinatorStyle::Options) = def {
			defs.iter().any(|d| !matches!(d, Def::Type(_) | Def::StyleValue(_)))
		} else {
			false
		}
	}
	fn unwrap_to_options(def: &Def) -> Option<&Def> {
		match def {
			Def::Group(inner, _) => unwrap_to_options(inner),
			d if is_options_with_keywords(d) => Some(d),
			_ => None,
		}
	}
	match def {
		Def::Combinator(children, DefCombinatorStyle::Alternatives) => {
			children.iter().filter_map(unwrap_to_options).collect()
		}
		Def::NoneOr(inner) | Def::AutoOr(inner) | Def::NormalOr(inner) | Def::AutoNoneOr(inner) => {
			unwrap_to_options(inner).into_iter().collect()
		}
		_ => vec![],
	}
}

/// For a list of sibling `Combinator(Options, ...)` children, compute each child's
/// distinguishing keyword set: the keywords that appear in this child but NOT in some other
/// sibling. Used to derive concise variant names when distribution produces multiple Options
/// siblings sharing common keywords (e.g. `[a|b] || c` distributes to `(a||c)|(b||c)` whose
/// distinguishing sets are `{a}` and `{b}`).
///
/// Returns `None` for a child if it cannot be uniquely distinguished, in which case the caller
/// should fall back to the full concatenated name.
fn distinguishing_keyword_names(siblings: &[&Def]) -> Vec<Option<Vec<String>>> {
	if siblings.len() < 2 {
		return siblings.iter().map(|_| None).collect();
	}
	let keyword_sets: Vec<Vec<String>> = siblings
		.iter()
		.map(|sibling| match sibling {
			Def::Combinator(children, DefCombinatorStyle::Options) => children
				.iter()
				.filter_map(|d| if let Def::Ident(DefIdent(s)) = d { Some(s.clone()) } else { None })
				.collect(),
			_ => vec![],
		})
		.collect();
	keyword_sets
		.iter()
		.enumerate()
		.map(|(i, mine)| {
			let unique: Vec<String> = mine
				.iter()
				.filter(|kw| keyword_sets.iter().enumerate().any(|(j, other)| j != i && !other.contains(kw)))
				.cloned()
				.collect();
			if unique.is_empty() || unique.len() == mine.len() { None } else { Some(unique) }
		})
		.collect()
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

	fn options_ident(ident: &Ident) -> Ident {
		let ident = ident.to_string();
		let ident = ident.strip_prefix("Single").unwrap_or(&ident);
		format_ident!("{}Options", ident)
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
			Self::NormalOr(ty) => ty.as_ref().should_skip_visit(),
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
			Def::Optional(inner) => match inner.as_ref() {
				Def::Ident(DefIdent(str)) if derives_parse => {
					let name = format_ident!("{}", str.to_pascal_case());
					quote! { #[atom(CssAtomSet::#name)] }
				}
				_ => quote! {},
			},
			_ => quote! {},
		};
		quote! { #skip #atom }
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
			Self::NormalOr(def) => def.deref().is_all_keywords(),
			// Optional(Ident(kw)) is emitted as Option<crate::Kw>, not via the Keywords enum path.
			Self::Optional(def) => match def.as_ref() {
				Self::Ident(_) => false,
				_ => def.deref().is_all_keywords(),
			},
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
			Self::NormalOr(_) => vec![],
			Self::StyleValue(_) => vec![],
			Self::FunctionType(_) => vec![],
			Self::Type(_) => vec![],
			Self::Optional(def) => {
				// Optional(Ident(kw)) is handled as Option<crate::Kw>, not via Keywords enum.
				if matches!(def.as_ref(), Self::Ident(_)) { vec![] } else { def.gather_keywords() }
			}
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
		// NoneOr/AutoOr/NormalOr might maybe_unsized for the internal to the type, but shouldn't express their own generics
		if self.maybe_unsized()
			&& !matches!(self, Def::NoneOr(_) | Def::AutoOr(_) | Def::AutoNoneOr(_) | Def::NormalOr(_))
		{
			parse_quote!(<'a>)
		} else {
			Default::default()
		}
	}

	fn rewrite_options_type(&self, replacement: &TokenStream) -> TokenStream {
		match self {
			Self::NoneOr(_) => quote! { crate::NoneOr<#replacement> },
			Self::AutoOr(_) => quote! { crate::AutoOr<#replacement> },
			Self::NormalOr(_) => quote! { crate::NormalOr<#replacement> },
			Self::AutoNoneOr(_) => quote! { crate::AutoNoneOr<#replacement> },
			Self::Group(inner, _) => inner.rewrite_options_type(replacement),
			Self::Combinator(_, DefCombinatorStyle::Options) => replacement.clone(),
			_ => self.to_type(),
		}
	}

	fn generate_additional_types(&self, vis: &Visibility, ident: &Ident, _generics: &Generics) -> TokenStream {
		let needs_keyword_type = match self {
			Self::Combinator(defs, DefCombinatorStyle::Ordered) => defs.iter().any(|def| def.is_all_keywords()),
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
		// Determine if a Single* helper struct is needed, and which Def to generate it from.
		let single_inner: Option<&Def> = match self {
			Self::Multiplier(defs, _, range) => match defs.deref() {
				// All-keyword alternatives don't need a Single* type beyond keyword_type.
				Def::Combinator(defs, DefCombinatorStyle::Alternatives)
					if defs.iter().all(|def| matches!(def, Def::Ident(_))) =>
				{
					None
				}
				Def::Combinator(_, _) if matches!(range, DefRange::RangeFrom(_) | DefRange::RangeTo(_)) => {
					Some(defs.deref())
				}
				_ => None,
			},
			Self::Combinator(defs, DefCombinatorStyle::Ordered) => defs
				.iter()
				.find_map(|def| {
					if def.keyword_prefix_name().is_some() {
						Some(def)
					} else if let Def::Optional(inner) = def {
						if inner.keyword_prefix_name().is_some() { Some(inner.as_ref()) } else { None }
					} else {
						None
					}
				})
				.map(|def| match def {
					Def::Group(inner, _) => inner.as_ref(),
					other => other,
				}),
			_ => None,
		};
		let single_type = if let Some(inner) = single_inner {
			let single_ident = Self::single_ident(ident);
			let generics = inner.get_generics();
			let def = inner.generate_definition(vis, &single_ident, &generics, true, true);
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
		} else {
			quote! {}
		};
		// Generate Options helper structs when Options contains keyword (Ident) children.
		// Optionals![T![Ident], T![Ident], ...] can't discriminate keywords — a named struct
		// with #[parse(one_must_occur)] and per-field #[atom] handles this correctly.
		// When `self` is an Alternatives (i.e. an enum), Options-with-keywords children are
		// inlined as struct variants on the enum directly (see DataType::Enum codegen below) —
		// so no helper structs are needed here. Helper structs are still emitted for struct
		// cases (e.g. NoneOr-wrapped Options).
		let options_children = if matches!(self, Self::Combinator(_, DefCombinatorStyle::Alternatives)) {
			vec![]
		} else {
			find_options_with_keywords(self)
		};
		let options_types = if options_children.is_empty() {
			quote! {}
		} else {
			let mut result = quote! {};
			for (i, inner) in options_children.iter().enumerate() {
				let opts_ident = if options_children.len() == 1 {
					Self::options_ident(ident)
				} else {
					format_ident!("{}Options{}", ident, i + 1)
				};
				let generics = inner.get_generics();
				let def = inner.generate_definition(vis, &opts_ident, &generics, true, true);
				result.extend(quote! {
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
				});
			}
			result
		};
		quote! {
			#keyword_type
			#single_type
			#options_types
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
						let single_ident = Self::single_ident(ident);
						let types = defs.iter().map(|def| {
							let ty = if let Self::Optional(inner) = def {
								if matches!(inner.as_ref(), Self::Ident(_)) {
									// Optional(Ident(kw)) references standalone keyword type
									def.to_type()
								} else if inner.keyword_prefix_name().is_some() {
									quote! { Option<#single_ident> }
								} else if inner.is_all_keywords() {
									let keyword_name = Self::keyword_ident(ident);
									quote! { Option<#keyword_name> }
								} else {
									def.to_type()
								}
							} else if def.keyword_prefix_name().is_some() {
								quote! { #single_ident }
							} else if def.is_all_keywords() {
								let keyword_name = Self::keyword_ident(ident);
								quote! { #keyword_name }
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
						// Check if this wraps an Options-with-keywords that has a helper struct.
						let options_children = find_options_with_keywords(self);
						if !options_children.is_empty() {
							let opts_ident = Self::options_ident(ident);
							let opts_generics = options_children[0].get_generics();
							let ty = self.rewrite_options_type(&quote! { crate::#opts_ident #opts_generics });
							quote! { ( pub #ty ); }
						} else {
							let ty = self.to_types();
							let attrs = self.type_attributes(derives_parse, derives_visitable);
							quote! { ( #(#attrs pub #ty),* ); }
						}
					}
				};
				quote! { #struct_attrs #vis struct #ident #type_generics #where_clause #members }
			}
			DataType::Enum => match self {
				Self::Combinator(children, DefCombinatorStyle::Alternatives) => {
					// Pre-compute which children are Options-with-keywords; these are inlined as
					// struct variants on the enum (rather than referencing a helper struct).
					let options_children_refs: Vec<(usize, &Def)> = children
						.iter()
						.enumerate()
						.filter_map(|(i, d)| {
							let inner = match d {
								Def::Group(inner, _) => inner.as_ref(),
								other => other,
							};
							if let Def::Combinator(defs, DefCombinatorStyle::Options) = inner
								&& defs.iter().any(|d| !matches!(d, Def::Type(_) | Def::StyleValue(_)))
							{
								return Some((i, inner));
							}
							None
						})
						.collect();
					let options_indices: Vec<usize> = options_children_refs.iter().map(|(i, _)| *i).collect();
					let options_inner_defs: Vec<&Def> = options_children_refs.iter().map(|(_, d)| *d).collect();
					// For multi-sibling Options, derive each variant's distinguishing keywords so
					// names reflect the discriminator rather than concatenating shared keywords.
					let distinguishing = distinguishing_keyword_names(&options_inner_defs);

					let variants: TokenStream = children
						.iter()
						.enumerate()
						.map(|(child_idx, d)| {
							let mut attrs = Some(d.type_attributes(derives_parse, derives_visitable));
							// Locate this child in the Options list (if it is one).
							let options_helper_idx = options_indices.iter().position(|&i| i == child_idx);
							if let Some(idx) = options_helper_idx {
								// Inline as struct variant with #[parse(one_must_occur)].
								let inner = options_inner_defs[idx];
								let Def::Combinator(opts_children, DefCombinatorStyle::Options) = inner else {
									unreachable!("filtered above");
								};
								// Variant name: distinguishing keywords (if computed) else full
								// concatenation of all child names.
								let name = if let Some(keywords) = &distinguishing[idx] {
									format_ident!("{}", keywords.iter().map(|k| k.to_pascal_case()).collect::<String>())
								} else {
									inner.to_variant_name(0)
								};
								let members = opts_children.iter().map(|child| {
									let member_name = child.to_member_name(0);
									let ty = child.to_type();
									let field_attrs = child.type_attributes(derives_parse, derives_visitable);
									quote! { #field_attrs #member_name: Option<#ty> }
								});
								let variant_attrs = if derives_parse {
									quote! { #[parse(one_must_occur)] }
								} else {
									quote! {}
								};
								quote! { #variant_attrs #name { #(#members),* }, }
							} else {
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
									Self::Combinator(defs, DefCombinatorStyle::AllMustOccur) => {
										if derives_parse {
											attrs = Some(quote! { #[parse(all_must_occur)] });
										}
										defs.iter()
											.map(|d| {
												let ty = d.to_type();
												let a = d.type_attributes(derives_parse, derives_visitable);
												quote! { #a #ty }
											})
											.collect()
									}
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
							}
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
	fn get_generics(&self) -> Generics {
		if self.maybe_unsized() { parse_quote!(<'a>) } else { Default::default() }
	}
}
