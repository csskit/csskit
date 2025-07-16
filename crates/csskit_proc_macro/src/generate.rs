use heck::{ToKebabCase, ToPascalCase, ToSnakeCase};
use itertools::{Itertools, Position};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use std::ops::{Deref, Range, RangeFrom, RangeTo};
use syn::{Error, GenericParam, Generics, Ident, Lifetime, LifetimeParam, Visibility};

use crate::def::*;

pub fn pluralize(str: String) -> String {
	if str.ends_with("s") { str.clone() } else { format!("{str}s") }
}

pub trait GenerateDefinition {
	fn generate_definition(&self, vis: &Visibility, ident: &Ident, generics: &mut Generics) -> TokenStream;
}

pub trait GeneratePeekImpl {
	fn peek_steps(&self) -> TokenStream;
}

pub trait GenerateParseImpl: GeneratePeekImpl {
	fn parse_steps(&self) -> (TokenStream, TokenStream);
}

pub trait GenerateKeywordSet {
	fn generate_keyword_set(&self, ident: &Ident) -> TokenStream;
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
	fn to_types(&self) -> Box<dyn Iterator<Item = TokenStream> + '_>;

	// Generate as a single type, which may be a tuple of types if to_types generated an iterator with a length > 1
	fn to_singular_type(&self) -> TokenStream {
		let types: Vec<_> = self.to_types().collect();
		if types.len() == 1 {
			quote! { #(#types)* }
		} else {
			quote! { (#(#types),*) }
		}
	}
}

trait EasyPeekImpl {}
impl EasyPeekImpl for DefIdent {}
impl EasyPeekImpl for DefType {}

impl<T> GeneratePeekImpl for T
where
	T: ToType + EasyPeekImpl,
{
	fn peek_steps(&self) -> TokenStream {
		let ty = self.to_types().next().unwrap();
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
			Self::Length(_) => "Length".into(),
			Self::LengthPercentage(_) => "LengthPercentage".into(),
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
			Self::Custom(_, ident) => {
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
			Self::Multiplier(v, _) => v.deref().to_variant_name(2),
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
	fn to_types(&self) -> Box<dyn Iterator<Item = TokenStream> + '_> {
		Box::new([quote! { ::css_parse::T![Ident] }].into_iter())
	}
}

impl ToType for Def {
	fn to_types(&self) -> Box<dyn Iterator<Item = TokenStream> + '_> {
		match self {
			Self::Ident(v) => v.to_types(),
			Self::Type(v) => v.to_types(),
			Self::Optional(v) => {
				let ty = v.to_singular_type();
				Box::new([quote! { Option<#ty> }].into_iter())
			}
			Self::Function(_, ty) => Box::new(
				[quote! { ::css_parse::T![Function] }]
					.into_iter()
					.chain(ty.to_types())
					.chain([quote! {  Option<::css_parse::T![')']> }]),
			),
			Self::Combinator(ds, DefCombinatorStyle::Ordered) => Box::new(ds.iter().map(|d| d.to_singular_type())),
			Self::Combinator(ds, DefCombinatorStyle::Alternatives) => Box::new(ds.iter().map(|d| d.to_singular_type())),
			Self::Combinator(ds, DefCombinatorStyle::Options) => {
				let types = ds.iter().map(|d| d.to_singular_type());
				Box::new([quote! { ::css_parse::Optionals![#(#types),*] }].into_iter())
			}
			Self::Combinator(_def, _) => {
				dbg!("TODO to_types for Combinator()", self);
				todo!("to_types")
			}
			Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Fixed(val))) => {
				Box::new((1..=*val as u32).map(|_| def.deref().to_singular_type()))
			}
			Self::Multiplier(def, DefMultiplierStyle::OneOrMoreCommaSeparated(_)) => {
				let ty = def.deref().to_singular_type();
				Box::new([quote! { ::css_parse::CommaSeparated<'a, #ty> }].into_iter())
			}
			Self::Multiplier(def, _) => {
				let ty = def.deref().to_singular_type();
				Box::new([quote! { ::bumpalo::collections::Vec<'a, #ty> }].into_iter())
			}
			Self::IntLiteral(_) => Box::new([quote! { crate::CSSInt }].into_iter()),
			Self::DimensionLiteral(_, _) => Box::new([quote! { ::css_parse::T![Dimension] }].into_iter()),
			Self::Punct(char) => Box::new([quote! { ::css_parse::T![#char] }].into_iter()),
			Self::Group(_, _) => {
				dbg!("TODO to_types for Group()", self);
				todo!("to_types")
			}
		}
	}
}

impl ToType for DefType {
	fn to_types(&self) -> Box<dyn Iterator<Item = TokenStream> + '_> {
		let type_name = match self {
			Self::Length(_) => quote! { crate::Length },
			Self::LengthPercentage(_) => quote! { crate::LengthPercentage },
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
			Self::DashedIdent => quote! { ::css_parse::T![DashedIdent] },
			Self::CustomIdent => quote! { ::css_parse::T![Ident] },
			Self::String => quote! { ::css_parse::T![String] },
			Self::Custom(ty, _) => quote! { crate::#ty },
		};
		let life = if self.requires_allocator_lifetime() { Some(quote! { <'a> }) } else { None };
		Box::new([quote! { #type_name #life }].into_iter())
	}
}

impl Def {
	pub fn requires_allocator_lifetime(&self) -> bool {
		match self {
			Self::Ident(_) | Self::IntLiteral(_) | Self::DimensionLiteral(_, _) => false,
			Self::Function(_, d) => d.requires_allocator_lifetime(),
			Self::Type(d) => d.requires_allocator_lifetime(),
			Self::Optional(d) => d.requires_allocator_lifetime(),
			Self::Combinator(ds, _) => ds.iter().any(|d| d.requires_allocator_lifetime()),
			Self::Group(d, _) => d.requires_allocator_lifetime(),
			Self::Multiplier(_, style) => {
				// Bounded multipliers get optimized into struct of options
				if let DefMultiplierStyle::Range(range) = style {
					matches!(range, DefRange::RangeFrom(_))
				} else {
					true
				}
			}
			Self::Punct(_) => false,
		}
	}

	pub fn generated_data_type(&self) -> DataType {
		match self {
			Self::Combinator(_, DefCombinatorStyle::Alternatives) => DataType::Enum,
			_ => DataType::SingleUnnamedStruct,
		}
	}

	pub fn generate_peek_trait_implementation(&self, ident: &Ident, generics: &mut Generics) -> TokenStream {
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (impl_generics, _, _) = generics.split_for_impl();
		let steps = self.peek_steps();
		quote! {
			#[automatically_derived]
			impl<'a> ::css_parse::Peek<'a> for #ident #impl_generics {
				fn peek(p: &::css_parse::Parser<'a>, c: ::css_lexer::Cursor) -> bool {
					use ::css_parse::Peek;
					#steps
				}
			}
		}
	}

	pub fn generate_parse_trait_implementation(&self, ident: &Ident, generics: &mut Generics) -> TokenStream {
		let keyword_set_ident = format_ident!("{}Keywords", ident);
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
					let mut else_arm = quote! {};

					let keyword_arms = keywords.into_iter().map(|def| {
						if let Def::Ident(ident) = def {
							let keyword_variant = format_ident!("{}", ident.to_string().to_pascal_case());
							let variant_name = ident.to_variant_name(0);
							quote! { #keyword_set_ident::#keyword_variant(c) => {
								return Ok(Self::#variant_name(<::css_parse::T![Ident]>::build(p, c)));
							} }
						} else if def == &Def::Type(DefType::CustomIdent) {
							error_fallthrough = false;
							else_arm = quote! {
								else { return Ok(Self::CustomIdent(p.parse::<::css_parse::T![Ident]>()?)); }
							};
							quote! {}
						} else {
							quote! {}
						}
					});

					Some(quote! {
						if let Some(keyword) = p.parse_if_peek::<#keyword_set_ident>()? {
							use ::css_parse::Build;
							match keyword {
								#(#keyword_arms)*
							}
						} #else_arm
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
									(#v, ::css_lexer::DimensionUnit::#dim_ident) => { return Ok(Self::#variant_name(tk)); }
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
					let c: ::css_lexer::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
					Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
				};

				if keyword_if.is_some() && lit_if.is_none() {
					error = quote! {
						let c: ::css_lexer::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
						Err(::css_parse::diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
					}
				}

				if keyword_if.is_none() && lit_if.is_some() {
					error = quote! {
						let c: ::css_lexer::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
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
								Some(#keyword_set_ident::#keyword_variant(c)) => {
									if val.#member_name.is_some() {
										Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
									}
									val.#member_name = Some(<::css_parse::T![Ident]>::build(p, c));
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
							let c: ::css_lexer::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
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
				quote! {
					#(#steps)*
					Ok(Self(#(#idents),*))
				}
			}
			Self::Combinator(_, DefCombinatorStyle::AllMustOccur) => {
				dbg!("generate_parse_trait_implementation", self);
				todo!("generate_parse_trait_implementation")
			}
			Self::Group(_, _) => {
				dbg!("generate_parse_trait_implementation", self);
				todo!("generate_parse_trait_implementation")
			}
			Self::Multiplier(_, DefMultiplierStyle::ZeroOrMore) => {
				quote! { compile_error!("cannot generate top level multiplier of zero-or-more") }
			}
			Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Range(Range { start, end }))) => {
				// Optimize for bounded ranges like `<foo>{1,2}` which could be expressed as `Foo, Option<Foo>`
				let opts: Vec<Def> = (1..=*end as i32)
					.map(|i| if i <= (*start as i32) { def.deref().clone() } else { Self::Optional(def.clone()) })
					.collect();
				return Self::Combinator(opts, DefCombinatorStyle::Ordered)
					.generate_parse_trait_implementation(ident, generics);
			}
			Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Fixed(val))) => {
				// Optimize for bounded ranges like `<foo>{2}` which could be expressed as `(Foo, Foo)`
				debug_assert!(*val > 0.0);
				let opts: Vec<Def> = (1..=*val as u32).map(|_| def.deref().clone()).collect();
				return Self::Combinator(opts, DefCombinatorStyle::Ordered)
					.generate_parse_trait_implementation(ident, generics);
			}
			Self::Multiplier(_, _) => {
				let (steps, result) = self.parse_steps();
				quote! {
					#steps
					return Ok(Self(#result));
				}
			}
			Self::Punct(_) => todo!(),
			Self::IntLiteral(_) => todo!(),
			Self::DimensionLiteral(_, _) => todo!(),
		};
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (impl_generics, _, _) = generics.split_for_impl();
		quote! {
			#[automatically_derived]
			impl<'a> ::css_parse::Parse<'a> for #ident #impl_generics {
				fn parse(p: &mut ::css_parse::Parser<'a>) -> ::css_parse::Result<Self> {
					use ::css_parse::{Parse,Peek};
					#steps
				}
			}
		}
	}
}

impl GenerateDefinition for Def {
	fn generate_definition(&self, vis: &Visibility, ident: &Ident, generics: &mut Generics) -> TokenStream {
		if self.requires_allocator_lifetime() && !generics.lifetimes().any(|l| l.lifetime.ident == "a") {
			let lt = Lifetime::new("'a", Span::call_site());
			generics.params.push(GenericParam::from(LifetimeParam::new(lt)));
		}
		let (_, impl_generics, _) = generics.split_for_impl();
		let types = self.to_types();
		match self.generated_data_type() {
			DataType::SingleUnnamedStruct => {
				let members = match self {
					Self::Combinator(_, DefCombinatorStyle::Alternatives) => {
						Error::new(ident.span(), "cannot generate alternative combinators in struct")
							.into_compile_error()
					}
					Self::Combinator(defs, DefCombinatorStyle::Options) => {
						let members = defs.iter().map(|def| {
							let name = def.to_member_name(0);
							let ty = def.to_singular_type();
							quote! { #name: Option<#ty> }
						});
						quote! { { #(pub #members),* } }
					}
					_ => {
						quote! { (#(pub #types),*); }
					}
				};
				quote! { #vis struct #ident #impl_generics #members }
			}
			DataType::Enum => match self {
				Self::Combinator(children, DefCombinatorStyle::Alternatives) => {
					let variants: TokenStream = children
						.iter()
						.map(|d| {
							let name = d.to_variant_name(0);
							let types = d.to_types();
							quote! { #name(#(#types),*), }
						})
						.collect();
					quote! { #vis enum #ident #impl_generics { #variants } }
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
			Self::Function(_, _) => quote! { <::css_parse::T![Function]>::peek(p, c) },
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
			Self::Multiplier(p, _) => p.peek_steps(),
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
			Self::Function(p, ty) => {
				let name = p.to_string().to_kebab_case();
				let (steps, result) = ty.parse_steps();
				(
					quote! {
						let function = p.parse::<::css_parse::T![Function]>()?;
						let c: css_lexer::Cursor = function.into();
						if !p.eq_ignore_ascii_case(c, #name) {
							return Err(::css_parse::diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
						}
						#steps
						let inner = #result;
						let close = p.parse_if_peek::<::css_parse::T![')']>()?;
					},
					quote! { function, inner, close },
				)
			}
			Self::Multiplier(def, DefMultiplierStyle::OneOrMore) => match def.deref() {
				Def::Type(v) => {
					let ty = v.to_types().next().unwrap();
					let (steps, result) = v.parse_steps();
					(
						quote! {
							let mut items = ::bumpalo::collections::Vec::new_in(p.bump());
							loop {
								#steps
								items.push(#result);
								if !p.peek::<#ty>() {
									break;
								}
							}
						},
						quote! { items },
					)
				}
				_ => {
					dbg!("parse_steps for multiplier fixed range", self);
					todo!("parse_steps for multiplier fixed range")
				}
			},
			Self::Multiplier(def, DefMultiplierStyle::Range(DefRange::Fixed(val))) => {
				debug_assert!(*val > 0.0);
				let (steps, results): (Vec<_>, Vec<_>) = (1..=*val as u32)
					.map(|_| match def.deref() {
						Def::Type(v) => v.parse_steps(),
						_ => {
							dbg!("parse_steps for multiplier fixed range", self);
							todo!("parse_steps for multiplier fixed range")
						}
					})
					.collect::<Vec<_>>()
					.into_iter()
					.unzip();
				(quote! { #(#steps)* }, quote! { #(#results)* })
			}
			Self::Multiplier(def, DefMultiplierStyle::OneOrMoreCommaSeparated(range)) => match def.as_ref() {
				Def::Type(def) => {
					let max_check = match range {
						DefRange::Range(Range { end, .. }) => {
							let n = *end as usize;
							quote! {
								if result.len() > #n {
									break;
								}
							}
						}
						_ => quote! {},
					};
					let min_check = match range {
						DefRange::None => quote! {},
						DefRange::RangeTo(_) => quote! { compile_error!("invalid range expression on multiplier") },
						DefRange::RangeFrom(_) => quote! { compile_error!("from range multiplier is todo") },
						DefRange::Fixed(_) => quote! { compile_error!("invalid fixed range expression on multiplier") },
						DefRange::Range(Range { start, .. }) => {
							let n = *start as usize;
							quote! {
								if result.len() < #n {
									let c: ::css_lexer::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
									Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
								}
							}
						}
					};
					let ty = def.to_singular_type();
					let parse = quote! { p.parse::<::css_parse::CommaSeparated<'a, #ty>>()? };
					if max_check.is_empty() && min_check.is_empty() {
						(quote! {}, parse)
					} else {
						(quote! { let result = #parse; #max_check; #min_check }, quote! { result })
					}
				}
				_ => {
					dbg!("parse_steps for Self::Multiplier def", self);
					todo!("parse_steps for Self::Multiplier def")
				}
			},
			Self::Multiplier(def, DefMultiplierStyle::Range(range)) => {
				let peek_steps = def.peek_steps();
				let (steps, result) = def.parse_steps();
				let max_check = match range {
					DefRange::Range(Range { end, .. }) => {
						let n = *end as usize;
						quote! {
							if i > #n {
								break;
							}
						}
					}
					_ => quote! {},
				};
				let min_check = match range {
					DefRange::None => quote! {},
					DefRange::RangeTo(_) => quote! { compile_error!("invalid range expression on multiplier") },
					DefRange::RangeFrom(_) => quote! { compile_error!("from range multiplier is todo") },
					DefRange::Fixed(_) => quote! { compile_error!("invalid fixed range expression on multiplier") },
					DefRange::Range(Range { start, .. }) => {
						let n = *start as usize;
						quote! {
							if i < #n {
								let c: ::css_lexer::Cursor = p.parse::<::css_parse::T![Any]>()?.into();
								Err(::css_parse::diagnostics::Unexpected(c.into(), c.into()))?
							}
						}
					}
				};
				let instantiate_i =
					if matches!(range, DefRange::None) { None } else { Some(quote! { let mut i = 0; }) };
				let increment_i = if matches!(range, DefRange::None) { None } else { Some(quote! { i += 1; }) };
				(
					quote! {
						#instantiate_i
						let mut items = ::bumpalo::collections::Vec::new_in(p.bump());
						loop {
							#max_check
							let c = p.peek_n(1);
							if #peek_steps {
								#steps
								#increment_i
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
			Self::Optional(def) => match def.deref() {
				Def::Type(d) => {
					let ty = d.to_singular_type();
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
				let ty = self.to_singular_type();
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
			Self::Group(def, DefGroupStyle::None) => def.parse_steps(),
			_ => {
				dbg!("parse_steps", self);
				todo!("parse_steps");
			}
		}
	}
}

impl GenerateKeywordSet for Def {
	fn generate_keyword_set(&self, ident: &Ident) -> TokenStream {
		let kws: Vec<&Def> = match self {
			Self::Combinator(opts, DefCombinatorStyle::Alternatives)
			| Self::Combinator(opts, DefCombinatorStyle::Options) => {
				opts.iter().filter(|def| matches!(def, Def::Ident(_))).collect()
			}
			_ => vec![],
		};
		if kws.is_empty() {
			quote! {}
		} else {
			let keywords: Vec<TokenStream> = kws
				.iter()
				.filter_map(|def| {
					if let Def::Ident(def) = def {
						let ident = format_ident!("{}", def.to_string().to_pascal_case());
						let str = def.to_string().to_kebab_case();
						Some(quote! { #ident: #str, })
					} else {
						None
					}
				})
				.collect();
			debug_assert!(keywords.len() == kws.len());
			let keyword_name = format_ident!("{}Keywords", ident);
			quote! {
				::css_parse::keyword_set!(pub enum #keyword_name { #(#keywords)* });
			}
		}
	}
}

impl DefType {
	pub fn checks(&self) -> &DefRange {
		match self {
			Self::Length(c)
			| Self::LengthPercentage(c)
			| Self::Percentage(c)
			| Self::Decibel(c)
			| Self::Angle(c)
			| Self::Time(c)
			| Self::Resolution(c)
			| Self::Integer(c)
			| Self::Number(c) => c,
			_ => &DefRange::None,
		}
	}

	pub fn requires_allocator_lifetime(&self) -> bool {
		if let Self::Custom(DefIdent(ident), _) = self {
			return matches!(
				ident.as_str(),
				"BorderTopColorStyleValue"
					| "CounterStyle"
					| "DynamicRangeLimitMix"
					| "EasingFunction"
					| "OutlineColor"
					| "TransformList"
					| "SingleTransition"
					| "ContentList"
			);
		}
		matches!(self, Self::Image | Self::Image1D)
	}
}

impl GenerateParseImpl for DefType {
	fn parse_steps(&self) -> (TokenStream, TokenStream) {
		if self == &Self::CustomIdent {
			// No steps needed, simple enough to flatten into result.
			return (quote! {}, quote! { p.parse::<::css_parse::T![Ident]>()? });
		}

		let name = self.to_singular_type();
		let checks = self.checks();
		let check_code = match checks {
			DefRange::RangeTo(RangeTo { end }) => quote! {
			let valf32: f32 = ty.into();
					if #end < valf32 {
						return Err(::css_parse::diagnostics::NumberTooLarge(#end, ::css_lexer::Span::new(start, p.offset())))?
					}
				},
			DefRange::Range(Range { start, end }) => quote! {
			let valf32: f32 = ty.into();
					if !(#start..#end).contains(&valf32) {
						return Err(::css_parse::diagnostics::NumberOutOfBounds(valf32, format!("{}..{}", #start, #end), ::css_lexer::Span::new(start, p.offset())))?
					}
				},
			DefRange::RangeFrom(RangeFrom { start }) => quote! {
			let valf32: f32 = ty.into();
					if #start > valf32 {
						return Err(::css_parse::diagnostics::NumberTooSmall(#start, ::css_lexer::Span::new(start, p.offset())))?
					}
				},
			DefRange::None => quote! {},
			DefRange::Fixed(_) => quote! {},
		};
		// Ensure that the simple case can flatten into the result:
		if check_code.is_empty() {
			(quote! {}, quote! { p.parse::<#name>()? })
		} else {
			(
				quote! {
					let start = p.offset();
					let ty = p.parse::<#name>()?;
					#check_code
				},
				quote! { ty },
			)
		}
	}
}

impl GenerateParseImpl for DefIdent {
	fn parse_steps(&self) -> (TokenStream, TokenStream) {
		let name = self.to_string().to_kebab_case();
		let ty = self.to_singular_type();
		(
			quote! {
				let ident = p.parse::<#ty>()?;
				let c: ::css_lexer::Cursor = ident.into();
				if !p.eq_ignore_ascii_case(c, #name) {
					Err(::css_parse::diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
				}
			},
			quote! { ident },
		)
	}
}
