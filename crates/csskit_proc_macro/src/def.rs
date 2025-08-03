use css_lexer::DimensionUnit;
use heck::ToPascalCase;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt, format_ident};
use std::{
	fmt::Display,
	ops::{Deref, Range},
};
use syn::{
	Error, Ident, Lit, LitFloat, LitInt, LitStr, Result, Token, braced, bracketed,
	ext::IdentExt,
	parenthesized,
	parse::{Parse, ParseStream},
	parse2, token,
};

pub(crate) struct StrWrapped<T: Parse>(pub T);
impl<T: Parse> Parse for StrWrapped<T> {
	fn parse(input_raw: ParseStream) -> Result<Self> {
		Ok(Self(parse2::<T>(
			input_raw.parse::<LitStr>()?.value().replace("'", "\"").replace("∞", "").parse::<TokenStream>()?,
		)?))
	}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Def {
	Ident(DefIdent),
	Function(DefIdent, Box<Def>),
	Type(DefType),
	Optional(Box<Def>), // ?
	Combinator(Vec<Def>, DefCombinatorStyle),
	Group(Box<Def>, DefGroupStyle),
	Multiplier(Box<Def>, DefMultiplierSeparator, DefRange),
	Punct(char),
	IntLiteral(i32),
	DimensionLiteral(f32, DimensionUnit),
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefGroupStyle {
	None,         // [ ] - regular group notation
	OneMustOccur, // [ ]! - at least one in the group must occur
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub(crate) enum DefCombinatorStyle {
	Ordered,      // <space>
	AllMustOccur, // && - all must occur
	Options,      // || - one or more must occur
	Alternatives, // | - exactly one must occur
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum DefMultiplierSeparator {
	None,   // *, +, or {,}
	Commas, // #, #? or #{,}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefRange {
	None,
	Range(Range<f32>), // {A,B}
	RangeFrom(f32),    // {A,}
	RangeTo(f32),      // {,B}
	Fixed(f32),        // {A}
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct DefIdent(pub String);

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum DefType {
	Length(DefRange),
	LengthOrAuto(DefRange),
	LengthPercentage(DefRange),
	LengthPercentageOrAuto(DefRange),
	Decibel(DefRange),
	Angle(DefRange),
	Time(DefRange),
	Resolution(DefRange),
	Integer(DefRange),
	Number(DefRange),
	Percentage(DefRange),
	Color,
	String,
	Image,
	Image1D,
	Url,
	DashedIdent,
	CustomIdent,
	Custom(DefIdent, DefIdent),
}

impl Parse for Def {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut root = if input.peek(Token![<]) {
			Self::Type(input.parse::<DefType>()?)
		} else if input.peek(token::Bracket) {
			let content;
			bracketed!(content in input);
			let inner = Box::new(content.parse::<Def>()?);
			if input.peek(Token![!]) {
				input.parse::<Token![!]>()?;
				Self::Group(inner, DefGroupStyle::OneMustOccur)
			} else if input.peek(Token![#]) {
				input.parse::<Token![#]>()?;
				Self::Multiplier(inner, DefMultiplierSeparator::Commas, DefRange::RangeFrom(1.))
			} else if input.peek(token::Brace) {
				let content;
				braced!(content in input);
				let range = content.parse::<DefRange>()?;
				debug_assert!(matches!(range, DefRange::Range(_)));
				Self::Multiplier(inner, DefMultiplierSeparator::None, range)
			} else {
				Self::Group(inner, DefGroupStyle::None)
			}
		} else if input.peek(Ident::peek_any) {
			let ident = input.parse::<DefIdent>()?;
			if input.peek(token::Paren) {
				let content;
				parenthesized!(content in input);
				Self::Function(ident, Box::new(content.parse::<Def>()?))
			} else {
				Self::Ident(ident)
			}
		} else if input.peek(Lit) {
			if let Lit::Int(lit) = input.parse::<Lit>()? {
				if lit.suffix() == "" {
					return Ok(Self::IntLiteral(lit.base10_parse::<i32>()?));
				}

				let unit = DimensionUnit::from(lit.suffix());
				if unit == DimensionUnit::Unknown {
					Err(Error::new(lit.span(), "Invalid dimension unit"))?
				}
				return Ok(Self::DimensionLiteral(lit.base10_parse::<f32>()?, unit));
			}

			Err(Error::new(input.span(), "unknown token in Def parse"))?
		} else {
			input.step(|cursor| {
				if let Some((p, next)) = cursor.punct() {
					return Ok((Self::Punct(p.as_char()), next));
				}
				Err(Error::new(input.span(), "unknown token in Def parse"))?
			})?
		};
		loop {
			if input.is_empty() {
				return match root {
					Self::Combinator(ref defs, DefCombinatorStyle::Alternatives) if defs.len() == 2 => {
						let [first, second] = defs.as_slice() else { panic!("defs.len() was 2!") };
						match (first, second) {
							// "<length> | auto" can be simplified to "<length-or-auto>"
							(Def::Ident(DefIdent(ident)), Def::Type(DefType::Length(r)))
							| (Def::Type(DefType::Length(r)), Def::Ident(DefIdent(ident)))
								if ident == "auto" =>
							{
								Ok(Def::Type(DefType::LengthOrAuto(r.clone())))
							}
							// "<length-percentage> | auto" can be simplified to "<length-percentage-or-auto>"
							(Def::Ident(DefIdent(ident)), Def::Type(DefType::LengthPercentage(r)))
							| (Def::Type(DefType::LengthPercentage(r)), Def::Ident(DefIdent(ident)))
								if ident == "auto" =>
							{
								Ok(Def::Type(DefType::LengthPercentageOrAuto(r.clone())))
							}
							_ => Ok(root),
						}
					}
					_ => Ok(root),
				};
			} else if input.peek(Token![?]) {
				input.parse::<Token![?]>()?;
				let inner = root;
				root = Self::Optional(Box::new(inner));
			} else if input.peek(Token![+])
				|| input.peek(Token![#])
				|| input.peek(token::Brace)
				|| input.peek(Token![*])
			{
				let inner = root;
				let (sep, range) = if input.peek(Token![*]) {
					input.parse::<Token![*]>()?;
					(DefMultiplierSeparator::None, DefRange::RangeFrom(0.))
				} else if input.peek(Token![+]) {
					input.parse::<Token![+]>()?;
					(DefMultiplierSeparator::None, DefRange::RangeFrom(1.))
				} else if input.peek(Token![#]) {
					input.parse::<Token![#]>()?;
					let range = if input.peek(token::Brace) {
						let content;
						braced!(content in input);
						content.parse::<DefRange>()?
					} else if input.peek(Token![?]) {
						input.parse::<Token![?]>()?;
						DefRange::RangeFrom(0.)
					} else {
						DefRange::RangeFrom(1.)
					};
					(DefMultiplierSeparator::Commas, range)
				} else if input.peek(token::Brace) {
					let content;
					braced!(content in input);
					(DefMultiplierSeparator::None, content.parse::<DefRange>()?)
				} else {
					Err(Error::new(input.span(), "Unknown token in DefMultiplierStyle parse!"))?
				};
				// Optimize multiplier styles to avoid unnecessarily allocating
				// arrays for structs that could be a set of optional values
				match (sep, &range) {
					// A fixed range can be normalised to an Ordered combinator of the same value
					(DefMultiplierSeparator::None, DefRange::Fixed(i)) => {
						let opts: Vec<_> = (1..=*i as u32)
							.map(|_| match &inner {
								Def::Type(_) => inner.clone(),
								_ => {
									dbg!("TODO fixed range variant", &inner);
									todo!("multiplier fixed range")
								}
							})
							.collect();
						root = Self::Combinator(opts, DefCombinatorStyle::Ordered);
					}
					// Bounded range can be normalised to an Ordered combinator of some optionals
					(DefMultiplierSeparator::None, DefRange::Range(Range { start, end })) => {
						let opts: Vec<Def> = (1..=*end as i32)
							.map(|i| {
								if i <= (*start as i32) {
									inner.clone()
								} else {
									Self::Optional(Box::new(inner.clone()))
								}
							})
							.collect();
						root = Self::Combinator(opts, DefCombinatorStyle::Ordered)
					}
					_ => {
						debug_assert!(matches!(
							range,
							DefRange::Range(_) | DefRange::RangeTo(_) | DefRange::RangeFrom(_)
						));
						root = Self::Multiplier(Box::new(inner), sep, range);
					}
				}
			} else {
				let style = if input.peek(Token![||]) {
					input.parse::<Token![||]>()?;
					DefCombinatorStyle::Options
				} else if input.peek(Token![|]) {
					input.parse::<Token![|]>()?;
					DefCombinatorStyle::Alternatives
				} else if input.peek(Token![&&]) {
					input.parse::<Token![&&]>()?;
					DefCombinatorStyle::AllMustOccur
				} else {
					DefCombinatorStyle::Ordered
				};
				let mut next = input.parse::<Def>()?;
				match (&mut root, &mut next) {
					(_, &mut Self::Combinator(ref mut children, ref s)) if s == &style => {
						children.insert(0, root);
						root = next;
					}
					(&mut Self::Combinator(ref mut children, ref s), _) if s == &style => {
						children.push(next);
					}
					(_, &mut Self::Combinator(ref mut children, ref other_style)) if &style < other_style => {
						let options = Self::Combinator(vec![root, children.remove(0)], style);
						children.insert(0, options);
						root = next;
					}
					(_, Self::Group(inner, DefGroupStyle::None)) => {
						let children = vec![root, *inner.deref().clone()];
						root = Self::Combinator(children, style);
					}
					(Self::Group(inner, DefGroupStyle::None), _) => {
						let children = vec![*inner.deref().clone(), next];
						root = Self::Combinator(children, style);
					}
					_ => {
						let children = vec![root, next];
						root = Self::Combinator(children, style);
					}
				}
			}
		}
	}
}

impl Parse for DefIdent {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut str = "".to_owned();
		let mut last_was_ident = false;
		loop {
			if input.peek(Token![>]) || input.peek(token::Bracket) {
				return Ok(Self(str));
			} else if input.peek(Ident::peek_any) && !last_was_ident {
				last_was_ident = true;
				let ident = input.call(Ident::parse_any)?;
				str.push_str(&ident.to_string());
			// LitInt might pick up identifier parts like "3d"
			} else if input.peek(LitInt) && last_was_ident {
				last_was_ident = true;
				let int = input.parse::<LitInt>()?;
				str.push_str(&int.to_string());
			} else if input.peek(Token![-]) {
				last_was_ident = false;
				input.parse::<Token![-]>()?;
				str.push('-');
			} else {
				return Ok(Self(str));
			}
		}
	}
}

impl Parse for DefType {
	fn parse(input: ParseStream) -> Result<Self> {
		input.parse::<Token![<]>()?;
		let ident = if input.peek(LitStr) {
			let str = input.parse::<StrWrapped<DefIdent>>()?.0.0;
			DefIdent(format!("{str}-style-value"))
		} else {
			input.parse::<DefIdent>()?
		};
		let mut checks = DefRange::None;
		if input.peek(token::Bracket) {
			let content;
			bracketed!(content in input);
			checks = content.parse::<DefRange>()?;
		}
		let ty = match ident.0.as_str() {
			"length" => Self::Length(checks),
			"length-or-auto" => Self::LengthOrAuto(checks),
			"length-percentage" => Self::LengthPercentage(checks),
			"length-percentage-or-auto" => Self::LengthPercentageOrAuto(checks),
			"decibel" => Self::Decibel(checks),
			"angle" => Self::Angle(checks),
			"time" => Self::Time(checks),
			"resolution" => Self::Resolution(checks),
			"integer" => Self::Integer(checks),
			"number" => Self::Number(checks),
			"percentage" => Self::Percentage(checks),
			"string" => Self::String,
			"color" => Self::Color,
			"image" => Self::Image,
			"image-1D" => Self::Image1D,
			// URI is an alias for URL
			// https://drafts.csswg.org/css2/#value-def-uri
			"uri" => Self::Url,
			"url" => Self::Url,
			"dashed-ident" => Self::DashedIdent,
			"custom-ident" => Self::CustomIdent,
			str => {
				let iden = DefIdent(str.to_pascal_case());
				let mut str = str.to_pascal_case().to_owned();
				if input.peek(token::Paren) {
					let content;
					parenthesized!(content in input);
					if !content.is_empty() {
						Err(Error::new(input.span(), "disallowed content inside deftype function"))?
					}
					str.push_str("Function");
				}
				Self::Custom(iden, DefIdent(str))
			}
		};
		input.parse::<Token![>]>()?;
		Ok(ty)
	}
}

impl Parse for DefRange {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut lhs = None;
		let mut rhs = None;
		if input.peek(LitFloat) {
			lhs = Some(input.parse::<LitFloat>()?.base10_parse()?);
		} else if input.peek(LitInt) {
			lhs = Some(input.parse::<LitInt>()?.base10_parse::<f32>()?);
		}
		if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			if input.peek(LitFloat) {
				rhs = Some(input.parse::<LitFloat>()?.base10_parse()?);
			} else if input.peek(LitInt) {
				rhs = Some(input.parse::<LitInt>()?.base10_parse::<f32>()?);
			}
		} else if let Some(lhs) = lhs {
			return Ok(Self::Fixed(lhs));
		}
		Ok(match (lhs, rhs) {
			(Some(start), Some(end)) => Self::Range(Range { start, end }),
			(None, Some(end)) => Self::RangeTo(end),
			(Some(start), None) => Self::RangeFrom(start),
			(None, None) => Self::None,
		})
	}
}

pub enum DataType {
	SingleUnnamedStruct,
	Enum,
}

impl DataType {
	pub fn is_struct(&self) -> bool {
		matches!(self, Self::SingleUnnamedStruct)
	}

	pub fn is_enum(&self) -> bool {
		matches!(self, Self::Enum)
	}
}

impl Display for DefIdent {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}

impl ToTokens for DefIdent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append(Ident::new(&self.to_string(), Span::call_site()));
	}
}

impl From<DefIdent> for Ident {
	fn from(value: DefIdent) -> Self {
		format_ident!("{}", value.0)
	}
}
