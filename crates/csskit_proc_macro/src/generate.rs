use heck::{ToKebabCase, ToPascalCase, ToSnakeCase};
use itertools::{Itertools, Position};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::ops::{Deref, Range};
use syn::{Error, Generics, Ident, Visibility, parse_quote};

use crate::def::*;

pub fn pluralize(str: String) -> String {
	if str.ends_with("s") { str.clone() } else { format!("{str}s") }
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

pub trait GeneratePeekImpl {
	fn peek_steps(&self) -> TokenStream;
}

pub trait GenerateParseImpl: GeneratePeekImpl {
	fn parse_steps(&self) -> (TokenStream, TokenStream);
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

trait EasyPeekImpl {}
impl EasyPeekImpl for DefIdent {}
impl EasyPeekImpl for DefType {}

impl<T> GeneratePeekImpl for T
where
	T: ToType + EasyPeekImpl,
{
	fn peek_steps(&self) -> TokenStream {
		let ty = self.to_type();
		quote! { <#ty>::peek(p, c) }
	}
}

impl ToFieldName for DefIdent {
	fn to_variant_name(&self, size_hint: usize) -> Ident {
		let pascal = self.0.to_pascal_case();
		format_ident!("{}", if size_hint > 0 { pluralize(pascal) } else { pascal })
	}
}

impl ToFieldName for DefType {
	fn to_variant_name(&self, size_hint: usize) -> Ident {
		let str: String = match self {
			Self::AutoOr(ty) => format!("AutoOr{}", ty.deref().to_variant_name(size_hint)),
			Self::NoneOr(ty) => format!("NoneOr{}", ty.deref().to_variant_name(size_hint)),
			Self::AutoNoneOr(ty) => format!("AutoNoneOr{}", ty.deref().to_variant_name(size_hint)),
			Self::Length(_) => "Length".into(),
			Self::LengthPercentage(_) => "LengthPercentage".into(),
			Self::LengthPercentageOrFlex(_) => "LengthPercentageOrFlex".into(),
			Self::NumberLength(_) => "NumberLength".into(),
			Self::NumberPercentage(_) => "NumberPercentage".into(),
			Self::Percentage(_) => "Percentage".into(),
			Self::Decibel(_) => "Decibel".into(),
			Self::Angle(_) => "Angle".into(),
			Self::Time(_) => "Time".into(),
			Self::Resolution(_) => "Resolution".into(),
			Self::Integer(_) => "Integer".into(),
			Self::Number(_) => "Number".into(),
			Self::String => "String".into(),
			Self::Color => "Color".into(),
			Self::Image => "Image".into(),
			Self::Image1D => "Image".into(),
			Self::Url => "Url".into(),
			Self::DashedIdent => "DashedIdent".into(),
			Self::CustomIdent => "CustomIdent".into(),
			Self::Custom(ident) => {
				ident.to_string().strip_suffix("StyleValue").unwrap_or(&ident.to_string()).to_string()
			}
		};
		format_ident!("{}", if size_hint > 0 { pluralize(str) } else { str })
	}
}

impl ToFieldName for Def {
	fn to_variant_name(&self, size_hint: usize) -> Ident {
		match self {
			Self::Ident(v) => v.to_variant_name(size_hint),
			Self::Type(v) => v.to_variant_name(size_hint),
			Self::Function(v, _) => format_ident!("{}Function", v.0.to_pascal_case()),
			Self::Multiplier(v, _, _) => v.deref().to_variant_name(2),
			Self::Group(def, _) => def.deref().to_variant_name(size_hint),
			Self::Optional(def) => def.deref().to_variant_name(size_hint),
			Self::IntLiteral(v) => format_ident!("Literal{}", v.to_string()),
			Self::DimensionLiteral(int, dim) => {
				let dim_name: &str = (*dim).into();
				let variant_str = format!("{int}{dim_name}");
				format_ident!("Literal{}", variant_str)
			}
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => {
				let (optional, others): (Vec<&Def>, Vec<&Def>) = ds.iter().partition(|d| matches!(d, Def::Optional(_)));
				let logical_first = others.first().or(optional.first());
				logical_first.expect("At least one Def is required").to_variant_name(0)
			}
			Self::Punct(_) => {
				dbg!("TODO variant name for Punct()", self);
				todo!("variant name")
			}
			Self::Combinator(_, _) => {
				dbg!("TODO variant name for Combinator()", self);
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
			Self::Punct(char) => vec![quote! { ::css_parse::T![#char] }],
			Self::Group(_, _) => {
				dbg!("TODO to_type for Group()", self);
				todo!("to_type")
			}
		}
	}
}

impl ToType for DefType {
	fn to_types(&self) -> Vec<TokenStream> {
		let type_name = match self {
			Self::AutoOr(ty) => {
				let ty = ty.to_type();
				quote! { crate::AutoOr<#ty> }
			}
			Self::NoneOr(ty) => {
				let ty = ty.to_type();
				quote! { crate::NoneOr<#ty> }
			}
			Self::AutoNoneOr(ty) => {
				let ty = ty.to_type();
				quote! { crate::AutoNoneOr<#ty> }
			}
			Self::Length(_) => quote! { crate::Length },
			Self::LengthPercentage(_) => quote! { crate::LengthPercentage },
			Self::LengthPercentageOrFlex(_) => quote! { crate::LengthPercentageOrFlex },
			Self::NumberLength(_) => quote! { crate::NumberLength },
			Self::NumberPercentage(_) => quote! { crate::NumberPercentage },
			Self::Percentage(_) => quote! { ::css_parse::T![Dimension::%] },
			Self::Decibel(_) => quote! { ::css_parse::T![Dimension::Db] },
			Self::Angle(_) => quote! { crate::Angle },
			Self::Time(_) => quote! { crate::Time },
			Self::Resolution(_) => quote! { crate::Resolution },
			Self::Integer(_) => quote! { crate::CSSInt },
			Self::Number(_) => quote! { ::css_parse::T![Number] },
			Self::Color => quote! { crate::Color },
			Self::Image => quote! { crate::Image },
			Self::Image1D => quote! { crate::Image1D },
			Self::Url => quote! { ::css_parse::T![Url] },
			Self::DashedIdent => quote! { crate::DashedIdent },
			Self::CustomIdent => quote! { crate::CustomIdent },
			Self::String => quote! { ::css_parse::T![String] },
			Self::Custom(ty) => quote! { crate::#ty },
		};
		let generics = self.get_generics();
		vec![quote! { #type_name #generics }]
	}
}

impl Def {
	fn single_ident(ident: &Ident) -> Ident {
		let ident = ident.to_string();
		let ident = ident.strip_suffix("StyleValue").unwrap_or(&ident).to_string();
		let ident = ident.strip_prefix("Single").unwrap_or(&ident);
		format_ident!("Single{}", ident)
	}

	fn keyword_ident(ident: &Ident) -> Ident {
		let ident = ident.to_string();
		let ident = ident.strip_suffix("StyleValue").unwrap_or(&ident).to_string();
		let ident = ident.strip_prefix("Single").unwrap_or(&ident);
		format_ident!("{}Keywords", ident)
	}

	fn should_skip_visit(&self) -> bool {
		match self {
			Self::Ident(_) => true,
			Self::IntLiteral(_) => true,
			Self::DimensionLiteral(_, _) => true,
			Self::Function(_, _) => false,
			Self::Type(DefType::AutoOr(ty)) => ty.as_ref().should_skip_visit(),
			Self::Type(DefType::NoneOr(ty)) => ty.as_ref().should_skip_visit(),
			Self::Type(DefType::AutoNoneOr(ty)) => ty.as_ref().should_skip_visit(),
			Self::Type(DefType::Url) => true,
			Self::Type(DefType::Percentage(_)) => true,
			Self::Type(DefType::Number(_)) => true,
			Self::Type(DefType::Decibel(_)) => true,
			Self::Type(DefType::Custom(DefIdent(ident))) => ident.ends_with("Keywords"),
			Self::Type(_) => false,
			Self::Optional(d) => d.should_skip_visit(),
			Self::Combinator(d, _) => d.iter().all(|d| d.should_skip_visit()),
			Self::Group(d, _) => d.should_skip_visit(),
			Self::Multiplier(d, _, _) => d.should_skip_visit(),
			Self::Punct(_) => false,
		}
	}

	fn type_attributes(&self, derives_parse: bool, derives_visitable: bool) -> TokenStream {
		let skip = if derives_visitable && self.should_skip_visit() {
			quote! { #[visit(skip)] }
		} else {
			quote! {}
		};
		let in_range = match self {
			Def::IntLiteral(i) if derives_parse => {
				let f = *i as f32;
				quote! { #[parse(in_range = #f..=#f)] }
			}
			Def::DimensionLiteral(f, _) if derives_parse => {
				quote! { #[parse(in_range = #f..=#f)] }
			}
			Def::Optional(def) => match def.deref() {
				Def::Type(deftype) if derives_parse => deftype.generate_in_range_attr(),
				_ => quote! {},
			},
			Def::Type(deftype) if derives_parse => deftype.generate_in_range_attr(),
			_ => quote! {},
		};
		quote! { #skip #in_range }
	}

	fn is_all_keywords(&self) -> bool {
		match self {
			Self::Ident(_) => true,
			Self::IntLiteral(_) => false,
			Self::DimensionLiteral(_, _) => false,
			Self::Function(_, _) => false,
			Self::Type(DefType::Custom(DefIdent(ident))) => ident.ends_with("Keywords"),
			Self::Type(_) => false,
			Self::Optional(def) => def.deref().is_all_keywords(),
			Self::Combinator(defs, _) => defs.iter().all(Self::is_all_keywords),
			Self::Group(def, _) => def.deref().is_all_keywords(),
			Self::Multiplier(def, _, _) => def.deref().is_all_keywords(),
			Self::Punct(_) => false,
		}
	}

	pub fn get_generics(&self) -> Generics {
		// NonrOr/AutoOr might requires_allocator_lifetime for the internal to the type, but shoulnd't express it's own generics
		if self.requires_allocator_lifetime() && !matches!(self, Self::Type(DefType::NoneOr(_) | DefType::AutoOr(_))) {
			parse_quote!(<'a>)
		} else {
			Default::default()
		}
	}

	pub fn requires_allocator_lifetime(&self) -> bool {
		match self {
			Self::Ident(_) | Self::IntLiteral(_) | Self::DimensionLiteral(_, _) => false,
			Self::Function(DefIdent(ident), _) => matches!(ident.as_str(), "dynamic-range-limit-mix" | "params"),
			Self::Type(d) => d.requires_allocator_lifetime(),
			Self::Optional(d) => d.requires_allocator_lifetime(),
			Self::Combinator(ds, _) => ds.iter().any(|d| d.requires_allocator_lifetime()),
			Self::Group(d, _) => d.requires_allocator_lifetime(),
			Self::Multiplier(_, _, _) => true,
			Self::Punct(_) => false,
		}
	}

	pub fn generated_data_type(&self) -> DataType {
		match self {
			Self::Combinator(_, DefCombinatorStyle::Alternatives) => DataType::Enum,
			_ => DataType::SingleUnnamedStruct,
		}
	}

	pub fn generate_parse_trait_implementation(&self, ident: &Ident, generics: &Generics) -> TokenStream {
		let keyword_set_ident = Self::keyword_ident(ident);
		let steps = match self {
			Self::Ident(_) | Self::Type(_) | Self::Function(_, _) | Self::Optional(_) => {
				let (steps, result) = self.parse_steps();
				quote! {
					#steps
					Ok(Self(#result))
				}
			}
			Self::Combinator(opts, DefCombinatorStyle::Alternatives) => {
				let (keywords, others): (Vec<&Def>, Vec<&Def>) = opts.iter().partition(|def| {
					matches!(def, Def::Ident(_) | Def::Type(DefType::CustomIdent) | Def::Type(DefType::DashedIdent))
				});
				let (lits, other_others): (Vec<&Def>, Vec<&Def>) =
					others.iter().partition(|def| matches!(def, Def::IntLiteral(_) | Def::DimensionLiteral(_, _)));

				let mut error_fallthrough = true;

				let other_if: Vec<TokenStream> = other_others
					.into_iter()
					.with_position()
					.map(|(p, def)| {
						let peek = def.peek_steps();
						let (steps, result) = def.parse_steps();
						let var = def.to_variant_name(0);
						// If it's the only parse block we don't need to peek, just return it.
						if p == Position::Only {
							quote! { #steps; Ok(Self::#var(#result)) }
						} else {
							quote! {
								let c = p.peek_n(1);
								if #peek { #steps; return Ok(Self::#var(#result)); }
							}
						}
					})
					.collect();

				let keyword_if = if keywords.is_empty() {
					None
				} else {
					let mut none_arm = quote! {};

					let keyword_arms = keywords.into_iter().map(|def| {
						if let Def::Ident(ident) = def {
							let keyword_variant = format_ident!("{}", ident.to_string().to_pascal_case());
							let variant_name = ident.to_variant_name(0);
							quote! { Some(#keyword_set_ident::#keyword_variant(ident)) => {
								return Ok(Self::#variant_name(ident));
							} }
						} else if def == &Def::Type(DefType::CustomIdent) {
							error_fallthrough = false;
							let ty = def.to_type();
							none_arm = quote! {
								return Ok(Self::CustomIdent(p.parse::<#ty>()?));
							};
							quote! {}
						} else {
							quote! {}
						}
					});

					Some(quote! {
						match p.parse_if_peek::<#keyword_set_ident>()? {
							#(#keyword_arms)*
							None => { #none_arm }
						}
					})
				};

				let lit_if = if lits.is_empty() {
					None
				} else {
					let mut int_literals = Vec::new();
					let mut dimension_literals = Vec::new();

					for def in lits.iter() {
						match def {
							Def::IntLiteral(v) => {
								let variant_name = def.to_variant_name(0);
								int_literals.push(quote! { #v => { return Ok(Self::#variant_name(tk)); } });
							}
							Def::DimensionLiteral(v, dim) => {
								let variant_name = def.to_variant_name(0);
								let dim_name: &str = (*dim).into();
								let dim_ident = format_ident!("{}", dim_name.to_pascal_case());
								dimension_literals.push(quote! {
									(#v, ::css_parse::DimensionUnit::#dim_ident) => { return Ok(Self::#variant_name(tk)); }
								});
							}
							_ => todo!(),
						}
					}

					let mut res = TokenStream::new();

					if !int_literals.is_empty() {
						res.extend(quote! {
							if let Some(tk) = p.parse_if_peek::<crate::CSSInt>()? {
								match tk.into() {
									#(#int_literals),*
									_ => {
										// Error handled below
									}
								}
							}
						});
					}

					if !dimension_literals.is_empty() {
						res.extend(quote! {
							if let Some(tk) = p.parse_if_peek::<::css_parse::T![Dimension]>()? {
								match tk.into() {
									#(#dimension_literals),*
									_ => {
										// Error handled below
									}
								}
							}
						});
					}

					Some(res)
				};

				let mut error = quote! {
					let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
					Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
				};

				if keyword_if.is_some() && lit_if.is_none() {
					error = quote! {
						let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
						Err(::css_parse::diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
					}
				}

				if keyword_if.is_none() && lit_if.is_some() {
					error = quote! {
						let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
						Err(::css_parse::diagnostics::UnexpectedLiteral(p.parse_str(c).into(), c.into()))?
					}
				}

				// Using an error fallthrough when we have difinitive else statements will cause errors due to unreachable
				// statements. Ensure this doesn't happen by blowing away the error fallthrough when we know we can.
				if !error_fallthrough {
					error = quote! {}
				}

				if other_if.is_empty() {
					quote! {
						#keyword_if
						#lit_if
						#error
					}
				} else if other_if.len() == 1 {
					quote! {
						#keyword_if
						#lit_if
						#(#other_if)*
					}
				} else {
					quote! {
						#keyword_if
						#lit_if
						#(#other_if)*;
						#error
					}
				}
			}
			// Special case for when a set of options are just keywords
			Self::Combinator(opts, DefCombinatorStyle::Options) => {
				let members: Vec<_> = opts
					.iter()
					.map(|def| match def {
						Def::Ident(id) => id.to_member_name(0),
						Def::Type(ty) => ty.to_member_name(0),
						_ => {
							dbg!("generate_parse_trait_implementation type on group options", self);
							todo!("generate_parse_trait_implementation type on group options")
						}
					})
					.collect();
				let member_steps: Vec<_> = opts
					.iter()
					.enumerate()
					.map(|(i, ty)| {
						if matches!(ty, Def::Ident(_)) {
							// Handled in keyword_arms
							return quote! {};
						}
						let ident = &members[i];
						let peek = ty.peek_steps();
						let (parse_steps, result) = ty.parse_steps();
						#[rustfmt::skip]
						quote! {
							if val.#ident.is_none() && #peek {
								#parse_steps
								val.#ident = Some(#result);
								continue;
							}
						}
					})
					.collect();
				let keyword_arms: Vec<_> = opts
					.iter()
					.filter_map(|def| {
						if let Def::Ident(ident) = def {
							let keyword_variant = format_ident!("{}", ident.to_string().to_pascal_case());
							let member_name = ident.to_member_name(0);
							Some(quote! {
								Some(#keyword_set_ident::#keyword_variant(ident)) => {
									if val.#member_name.is_some() {
										use ::css_parse::ToSpan;
										Err(::css_parse::diagnostics::Unexpected(ident.into(), c.to_span()))?
									}
									val.#member_name = Some(ident);
									continue;
								}
							})
						} else {
							None
						}
					})
					.collect();
				let keyword_match = if keyword_arms.is_empty() {
					quote! {}
				} else {
					quote! {
						match p.parse_if_peek::<#keyword_set_ident>()? {
							#(#keyword_arms),*
							None => {},
						}
					}
				};
				#[rustfmt::skip]
				quote! {
					use ::css_parse::Build;
					let mut val = Self { #(#members: None),* };
					while #(val.#members.is_none())||* {
							let c = p.peek_n(1);
							#keyword_match
							#(#member_steps)*
							break;
					}
					if #(val.#members.is_none())&&* {
							let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
							Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
					}
					Ok(val)
        }
			}
			Self::Combinator(defs, DefCombinatorStyle::Ordered) => {
				let idents: Vec<Ident> = (0..defs.len()).map(|i| format_ident!("val{}", i)).collect();
				let steps: Vec<_> = defs
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let ident = &idents[i];
						if def.is_all_keywords() && matches!(def, Def::Optional(_)) {
							quote! {
								let #ident = p.parse_if_peek::<#keyword_set_ident>()?.map(|kw| kw.into());
							}
						} else if def.is_all_keywords() {
							quote! {
								let #ident = p.parse::<#keyword_set_ident>()?.into();
							}
						} else {
							let (steps, result) = def.parse_steps();
							if steps.is_empty() {
								quote! { let #ident = #result; }
							} else {
								quote! {
									let #ident = {
									#steps
									#result
									};
								}
							}
						}
					})
					.collect();
				quote! {
					#(#steps)*
					Ok(Self(#(#idents),*))
				}
			}
			Self::Combinator(defs, DefCombinatorStyle::AllMustOccur) => {
				let idents: Vec<Ident> = (0..defs.len()).map(|i| format_ident!("val{}", i)).collect();
				let (bindings, (steps, checks)): (Vec<_>, (Vec<_>, Vec<_>)) = defs
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let ident = &idents[i];
						let ty = def.to_type();
						let binding = quote! { let mut #ident: Option<#ty> = None; };
						let step = if def.is_all_keywords() {
							quote! {
								if #ident.is_none() && <#keyword_set_ident>::peek(p, c) {
									#ident = Some(p.parse::<#keyword_set_ident>()?.into());
									continue;
								}
							}
						} else {
							let peek = def.peek_steps();
							let (steps, result) = def.parse_steps();
							let inner = if steps.is_empty() {
								quote! { #ident = Some(#result); }
							} else {
								quote! {
									#steps
									#ident = Some(#result);
								}
							};
							quote! {
								if #ident.is_none() && #peek {
									#inner
									continue;
								}
							}
						};
						let check = quote! { #ident.is_none() };
						(binding, (step, check))
					})
					.unzip();
				quote! {
					#(#bindings)*
					loop {
						let c = p.peek_n(1);
						#(#steps)*
						break;
					}
					if #(#checks)||* {
						let c = p.peek_n(1);
						Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
					}
					Ok(Self(#(#idents.unwrap()),*))
				}
			}
			Self::Group(_, _) => {
				dbg!("generate_parse_trait_implementation", self);
				todo!("generate_parse_trait_implementation")
			}
			Self::Multiplier(def, sep, range) => {
				debug_assert!(matches!(range, DefRange::Range(_) | DefRange::RangeFrom(_) | DefRange::RangeTo(_)));
				match def.deref() {
					Def::Combinator(defs, DefCombinatorStyle::Alternatives)
						if defs.iter().all(|def| matches!(def, Def::Ident(_))) =>
					{
						let phantom_type = Def::Multiplier(
							Box::new(Def::Type(DefType::Custom(keyword_set_ident.clone().into()))),
							*sep,
							range.clone(),
						);
						let (steps, result) = phantom_type.parse_steps();
						quote! {
							#steps
							return Ok(Self(#result));
						}
					}
					Def::Combinator(_, _) if matches!(range, DefRange::RangeFrom(_) | DefRange::RangeTo(_)) => {
						let ty_ident = Self::single_ident(ident);
						let phantom_type = Def::Multiplier(
							Box::new(Def::Type(DefType::Custom(ty_ident.clone().into()))),
							*sep,
							range.clone(),
						);
						let (steps, result) = phantom_type.parse_steps();
						quote! {
							#steps
							return Ok(Self(#result));
						}
					}
					_ => {
						let (steps, result) = self.parse_steps();
						quote! {
							#steps
							return Ok(Self(#result));
						}
					}
				}
			}
			Self::Punct(_) => todo!(),
			Self::IntLiteral(_) => todo!(),
			Self::DimensionLiteral(_, _) => todo!(),
		};
		let mut generic_with_alloc = generics.clone();
		let (impl_generics, type_generics, where_clause) = if generics.lifetimes().all(|l| l.lifetime.ident != "a") {
			generic_with_alloc.params.insert(0, parse_quote!('a));
			let (impl_generics, _, _) = generic_with_alloc.split_for_impl();
			let (_, type_generics, where_clause) = generics.split_for_impl();
			(impl_generics, type_generics, where_clause)
		} else {
			generics.split_for_impl()
		};
		quote! {
			#[automatically_derived]
			impl #impl_generics ::css_parse::Parse<'a> for #ident #type_generics #where_clause {
				fn parse(p: &mut ::css_parse::Parser<'a>) -> ::css_parse::Result<Self> {
					use ::css_parse::{Parse,Peek};
					#steps
				}
			}
		}
	}

	fn gather_keywords(&self) -> Vec<&Self> {
		match self {
			// Self::Ident shouldn't return itself because it can be used in a literal position.
			Self::Ident(_) => vec![],
			Self::Function(_, _) => vec![],
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

	pub fn generate_additional_types(&self, vis: &Visibility, ident: &Ident, _generics: &Generics) -> TokenStream {
		let kws = self.gather_keywords();
		let keyword_type = if kws.is_empty() {
			quote! {}
		} else {
			let keywords: Vec<TokenStream> = kws
				.iter()
				.unique_by(|def| if let Self::Ident(DefIdent(str)) = def { str } else { "" })
				.filter_map(|def| {
					if let Self::Ident(def) = def {
						let ident = format_ident!("{}", def.to_string().to_pascal_case());
						let str = def.to_string().to_kebab_case();
						Some(quote! { #ident: #str, })
					} else {
						None
					}
				})
				.collect();
			let keyword_name = Self::keyword_ident(ident);
			quote! {
				::css_parse::keyword_set!(
					#[derive(::csskit_derives::Visitable)]
					#[visit(skip)]
					pub enum #keyword_name {
						#(#keywords)*
					}
				);
			}
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
						let def = defs.generate_definition(vis, &ident, &generics, false, true);
						let parse_impl = defs.generate_parse_trait_implementation(&ident, &generics);
						quote! {
							#[derive(::csskit_derives::Peek, ::csskit_derives::ToSpan, ::csskit_derives::ToCursors, ::csskit_derives::Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
							#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
							#[visit(children)]
							#def
							#parse_impl
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
		let keyword_name = Self::keyword_ident(ident);
		match self.generated_data_type() {
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
							let mut attrs = def.type_attributes(derives_parse, derives_visitable);
							if derives_parse && matches!(def, Def::Ident(_)) {
								let kw_name = def.to_variant_name(0);
								attrs.extend(quote! { #[parse(keyword = #keyword_name::#kw_name)] });
							}
							quote! { #attrs pub #name: Option<#ty> }
						});
						if derives_parse {
							struct_attrs.extend(quote! { #[parse(one_must_occur)] })
						}
						quote! { { #(#members),* } }
					}
					Self::Combinator(defs, DefCombinatorStyle::Ordered) => {
						let types = defs.iter().map(|def| {
							let ty = if def.is_all_keywords() && matches!(def, Self::Optional(_)) {
								quote! { Option<css_parse::T![Ident]> }
							} else if def.is_all_keywords() {
								quote! { css_parse::T![Ident] }
							} else {
								def.to_type()
							};
							let attrs = def.type_attributes(derives_parse, derives_visitable);
							quote! { #attrs pub #ty }
						});
						quote! { ( #(#types),* ); }
					}
					Self::Combinator(defs, DefCombinatorStyle::AllMustOccur) => {
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
							let phantom_type = Self::Multiplier(
								Box::new(Def::Type(DefType::Custom(keyword_name.clone().into()))),
								*sep,
								range.clone(),
							);
							let ty = phantom_type.to_type();
							quote! { ( pub #ty ); }
						}
						Self::Combinator(_, _) if matches!(range, DefRange::RangeFrom(_) | DefRange::RangeTo(_)) => {
							let ty_ident = Self::single_ident(ident);
							let phantom_type = Self::Multiplier(
								Box::new(Def::Type(DefType::Custom(ty_ident.clone().into()))),
								*sep,
								range.clone(),
							);
							let ty = phantom_type.to_types();
							let attrs = phantom_type.type_attributes(derives_parse, derives_visitable);
							quote! { ( #(#attrs pub #ty),* ); }
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
							let mut var_attrs = quote! {};
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
								Self::Ident(_) => {
									if derives_parse {
										var_attrs.extend(quote! { #[parse(keyword = #keyword_name::#name)] });
									}
									d.to_types()
								}
								Self::IntLiteral(_) | Self::DimensionLiteral(_, _) => {
									let attrs = attrs.take().unwrap();
									let ty = d.to_type();
									vec![quote! { #attrs #ty }]
								}
								_ => d.to_types(),
							};
							quote! { #var_attrs #attrs #name(#(#types),*), }
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

impl GeneratePeekImpl for Def {
	fn peek_steps(&self) -> TokenStream {
		match self {
			Self::Type(p) => p.peek_steps(),
			Self::Ident(p) => p.peek_steps(),
			Self::Function(_, _) => {
				let ty = self.to_type();
				quote! { <#ty>::peek(p, c) }
			}
			Self::Optional(p) => p.peek_steps(),
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => {
				// We can optimize ordered combinators by peeking only up until the first required def
				// <type>? keyword ==> peek(type) || peek(keyword)
				// keyword <type>? ==> peek(keyword)
				let peek_steps: Vec<TokenStream> = ds
					.iter()
					.scan(true, |keep_going, d| {
						if !*keep_going {
							return None;
						}
						match d {
							Def::Optional(_) => {}
							_ => {
								// Pretty much take_until, but inclusive of the last item before we stop
								*keep_going = false;
							}
						}

						Some(d.peek_steps())
					})
					.collect();

				let peeks: Vec<TokenStream> = peek_steps
					.iter()
					.unique_by(|tok| tok.to_string())
					.with_position()
					.map(|(i, steps)| {
						if i == Position::First || i == Position::Only {
							quote! { #steps }
						} else {
							quote! { || #steps }
						}
					})
					.collect();

				quote! { #(#peeks)* }
			}
			Self::Combinator(p, _) => {
				let peeks: Vec<TokenStream> = p
					.iter()
					.map(|p| p.peek_steps())
					.unique_by(|tok| tok.to_string())
					.with_position()
					.map(|(i, steps)| {
						if i == Position::First || i == Position::Only {
							quote! { #steps }
						} else {
							quote! { || #steps }
						}
					})
					.collect();
				quote! { #(#peeks)* }
			}
			Self::Group(p, _) => p.peek_steps(),
			Self::Multiplier(p, _, _) => p.peek_steps(),
			Self::Punct(_) => todo!(),
			Self::IntLiteral(_) => quote! { <crate::CSSInt>::peek(p, c) },
			Self::DimensionLiteral(_, _) => quote! { <::css_parse::T![Dimension]>::peek(p, c) },
		}
	}
}

impl GenerateParseImpl for Def {
	fn parse_steps(&self) -> (TokenStream, TokenStream) {
		match self {
			Self::Type(p) => p.parse_steps(),
			Self::Ident(p) => p.parse_steps(),
			Self::Function(p, _) => {
				let func_name = format_ident!("{}Function", p.to_string().to_pascal_case());
				(quote! {}, quote! { p.parse::<crate::#func_name>()? })
			}
			Self::Multiplier(def, sep, range) => {
				let max = match range {
					DefRange::RangeTo(end) | DefRange::Range(Range { end, .. }) => Some(*end),
					DefRange::RangeFrom(_) => None,
					_ => panic!("Multiplier should only have Range/RangeFrom/RangeTo"),
				};
				let min = match range {
					DefRange::RangeFrom(start) | DefRange::Range(Range { start, .. }) => Some(*start),
					DefRange::RangeTo(_) => None,
					_ => panic!("Multiplier should only have Range/RangeFrom/RangeTo"),
				};
				match def.deref() {
					Def::Type(def) => {
						let ty = def.to_type();
						match sep {
							DefMultiplierSeparator::Commas => {
								let outer_type = self.to_type();
								let parse = quote! { p.parse::<#outer_type>()? };
								let min_check = min.and_then(|min| {
									if min == 1. {
										None
									} else {
										Some(quote! {
											if result.len() < #min {
												let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
												Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
											}
										})
									}
								});
								let max_check = max.map(|max| {
									quote! {
										if result.len() > #max {
											let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
											Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
										}
									}
								});
								if min_check.is_none() && max_check.is_none() {
									(quote! {}, parse)
								} else if min == Some(0.) {
									(
										quote! { let result = if p.peek::<#ty>() { #parse } else { ::css_parse::CommaSeparated::new_in(p.bump()) }; #max_check; },
										quote! { result },
									)
								} else {
									(quote! { let result = #parse; #max_check; #min_check }, quote! { result })
								}
							}
							DefMultiplierSeparator::None => {
								let max_check = max.map(|max| {
									quote! {
										if i > #max {
											break;
										}
									}
								});
								let (steps, result) = def.parse_steps();
								if min == Some(1.) {
									let max_check = max.map(|max| {
										quote! {
											if items.len() > #max {
												break;
											}
										}
									});
									(
										quote! {
											let mut items = ::bumpalo::collections::Vec::new_in(p.bump());
											loop {
												#steps
												#max_check
												#steps
												items.push(#result);
												if !p.peek::<#ty>() {
													break;
												}
											}
										},
										quote! { items },
									)
								} else {
									let peek_steps = def.peek_steps();
									let min_check = min.map(|min| {
										quote! {
											if i < #min {
												let c: ::css_parse::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
												Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
											}
										}
									});
									(
										quote! {
											let mut i = 0;
											let mut items = ::bumpalo::collections::Vec::new_in(p.bump());
											loop {
												#max_check
												let c = p.peek_n(1);
												if #peek_steps {
													#steps
													i += 1;
													items.push(#result)
												} else {
													break;
												}
											}
											#min_check
										},
										quote! { items },
									)
								}
							}
						}
					}
					_ => {
						dbg!("parse_steps for Self::Multiplier def", self);
						todo!("parse_steps for Self::Multiplier def")
					}
				}
			}
			Self::Optional(def) => match def.deref() {
				Def::Type(d) => {
					let ty = d.to_type();
					let (steps, result) = d.parse_steps();
					// Simple enough that no steps are needed, just flatten into the result
					if steps.is_empty() {
						(quote! {}, quote! { p.parse_if_peek::<#ty>()? })
					} else {
						(
							quote! {},
							quote! {
								if p.peek::<#ty>() {
									#steps
									Some(#result)
								} else { None }
							},
						)
					}
				}
				_ => {
					dbg!("parse_steps for Self::Optional def", self);
					todo!("parse_steps for Self::Optional def")
				}
			},
			Self::Combinator(_, DefCombinatorStyle::Options) => {
				let ty = self.to_type();
				(quote! {}, quote! { p.parse::<#ty>()? })
			}
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => {
				let idents: Vec<Ident> = (0..ds.len()).map(|i| format_ident!("combo{}", i)).collect();
				let steps: Vec<_> = ds
					.iter()
					.enumerate()
					.map(|(i, def)| {
						let ident = &idents[i];
						let (steps, result) = def.parse_steps();
						if steps.is_empty() {
							quote! { let #ident = #result; }
						} else {
							quote! {
								let #ident = {
									#steps
									#result
								};
							}
						}
					})
					.collect();
				(quote! { #(#steps)* }, quote! { #(#idents),* })
			}
			_ => {
				dbg!("parse_steps", self);
				todo!("parse_steps");
			}
		}
	}
}

impl DefType {
	pub fn checks(&self) -> &DefRange {
		match self {
			Self::AutoOr(def) | Self::NoneOr(def) => match def.as_ref() {
				Def::Type(ty) => ty.checks(),
				_ => &DefRange::None,
			},
			Self::Length(c)
			| Self::LengthPercentage(c)
			| Self::Percentage(c)
			| Self::NumberPercentage(c)
			| Self::NumberLength(c)
			| Self::Decibel(c)
			| Self::Angle(c)
			| Self::Time(c)
			| Self::Resolution(c)
			| Self::Integer(c)
			| Self::Number(c) => c,
			_ => &DefRange::None,
		}
	}

	pub fn generate_in_range_attr(&self) -> TokenStream {
		match self.checks() {
			DefRange::None | DefRange::Fixed(_) => quote! {},
			DefRange::Range(Range { start, end }) => quote! { #[parse(in_range = #start..=#end)] },
			DefRange::RangeFrom(start) => quote! { #[parse(in_range = #start..)] },
			DefRange::RangeTo(end) => quote! { #[parse(in_range = ..=#end)] },
		}
	}

	pub fn check_step(&self, ident: &Ident) -> TokenStream {
		if matches!(self, Self::AutoOr(_) | Self::AutoNoneOr(_) | Self::NoneOr(_)) {
			self.check_step_try_into(ident)
		} else {
			self.check_step_direct(ident)
		}
	}

	fn check_step_err(&self, ident: &Ident) -> TokenStream {
		match self.checks() {
			DefRange::Range(Range { start, end }) => quote! { {
				use css_parse::ToSpan;
				Err(::css_parse::diagnostics::NumberOutOfBounds(#ident.into(), format!("{}..{}", #start, #end), #ident.to_span()))?
			} },
			DefRange::RangeTo(end) => quote! { {
				use css_parse::ToSpan;
				Err(::css_parse::diagnostics::NumberTooLarge(#end, #ident.to_span()))?
			} },
			DefRange::RangeFrom(start) => quote! { {
				use css_parse::ToSpan;
				Err(::css_parse::diagnostics::NumberTooSmall(#start, #ident.to_span()))?
			} },
			DefRange::Fixed(_) | DefRange::None => quote! {},
		}
	}

	fn check_step_try_into(&self, ident: &Ident) -> TokenStream {
		let ty = match self {
			Self::NoneOr(_) => quote! { crate::NoneOr },
			Self::AutoOr(_) => quote! { crate::AutoOr },
			Self::AutoNoneOr(_) => quote! { crate::AutoNoneOr },
			_ => return quote! {},
		};
		let err = self.check_step_err(ident);
		match self.checks() {
			DefRange::RangeTo(end) => quote! {
				match #ident {
					#ty::Some(inner) if #end < Into::<f32>::into(inner) => #err
					_ => {}
				}
			},
			DefRange::RangeFrom(start) => quote! {
				match #ident {
					#ty::Some(inner) if #start > Into::<f32>::into(inner) => #err
					_ => {}
				}
			},
			DefRange::Range(Range { start, end }) => quote! {
				match #ident {
					// None variants are always valid
					#ty::Some(inner) if !(#start..=#end).contains(&(Into::<f32>::into())) => #err
					_ => {}
				}
			},
			DefRange::Fixed(_) | DefRange::None => quote! {},
		}
	}

	fn check_step_direct(&self, ident: &Ident) -> TokenStream {
		let cond = match self.checks() {
			DefRange::RangeTo(end) => quote! { #end < Into::<f32>::into(#ident) },
			DefRange::Range(Range { start, end }) => quote! { !(#start..=#end).contains(&Into::<f32>::into(#ident)) },
			DefRange::RangeFrom(start) => quote! { #start > Into::<f32>::into(#ident) },
			DefRange::Fixed(_) | DefRange::None => return quote! {},
		};
		let err = self.check_step_err(ident);
		quote! { if #cond #err }
	}
	pub fn get_generics(&self) -> Generics {
		// NonrOr/AutoOr might requires_allocator_lifetime for the internal to the type, but shoulnd't express it's own generics
		if self.requires_allocator_lifetime() && !matches!(self, DefType::NoneOr(_) | DefType::AutoOr(_)) {
			parse_quote!(<'a>)
		} else {
			Default::default()
		}
	}

	pub fn requires_allocator_lifetime(&self) -> bool {
		match self {
			Self::NoneOr(ty) | Self::AutoOr(ty) => ty.as_ref().requires_allocator_lifetime(),
			Self::Custom(DefIdent(ident)) => {
				matches!(
					ident.as_str(),
					"SingleFontFamily"
						| "BorderTopColorStyleValue"
						| "ContentList" | "CounterStyle"
						| "DynamicRangeLimitStyleValue"
						| "DynamicRangeLimitMixFunction"
						| "CursorImage" | "EasingFunction"
						| "FamilyName" | "OutlineColor"
						| "OutlineColorStyleValue"
						| "ParamFunction" | "SingleTransition"
						| "TransformList"
				)
			}
			Self::Image | Self::Image1D => true,
			_ => false,
		}
	}
}

impl GenerateParseImpl for DefType {
	fn parse_steps(&self) -> (TokenStream, TokenStream) {
		if self == &Self::CustomIdent {
			let ty = self.to_type();
			// No steps needed, simple enough to flatten into result.
			return (quote! {}, quote! { p.parse::<#ty>()? });
		}

		let name = self.to_type();
		let check_step = self.check_step(&format_ident!("ty"));
		// Ensure that the simple case can flatten into the result:
		if check_step.is_empty() {
			(quote! {}, quote! { p.parse::<#name>()? })
		} else {
			(
				quote! {
					let ty = p.parse::<#name>()?;
					#check_step
				},
				quote! { ty },
			)
		}
	}
}

impl GenerateParseImpl for DefIdent {
	fn parse_steps(&self) -> (TokenStream, TokenStream) {
		let name = self.to_string().to_kebab_case();
		let ty = self.to_type();
		(
			quote! {
				let ident = p.parse::<#ty>()?;
				let c: ::css_parse::Cursor = ident.into();
				if !p.eq_ignore_ascii_case(c, #name) {
					Err(::css_parse::diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
				}
			},
			quote! { ident },
		)
	}
}
