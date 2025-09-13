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
	DimensionLiteral(f32, String),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum DefGroupStyle {
	None,         // [ ] - regular group notation
	OneMustOccur, // [ ]! - at least one in the group must occur
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub(crate) enum DefCombinatorStyle {
	Ordered,      // <space>
	AllMustOccur, // && - all must occur
	Options,      // || - one or more must occur
	Alternatives, // | - exactly one must occur
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
	AutoOr(Box<Def>),
	NoneOr(Box<Def>),
	AutoNoneOr(Box<Def>),
	Length(DefRange),
	LengthPercentage(DefRange),
	LengthPercentageOrFlex(DefRange),
	NumberLength(DefRange),
	NumberPercentage(DefRange),
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
	Custom(DefIdent),
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
			} else if input.peek(Token![+]) {
				input.parse::<Token![+]>()?;
				Self::Multiplier(inner, DefMultiplierSeparator::None, DefRange::RangeFrom(1.))
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
					Self::IntLiteral(lit.base10_parse::<i32>()?)
				} else {
					let unit = lit.suffix();
					if unit.is_empty() {
						Err(Error::new(lit.span(), "Invalid dimension unit"))?
					}
					Self::DimensionLiteral(lit.base10_parse::<f32>()?, unit.to_string())
				}
			} else {
				Err(Error::new(input.span(), "unknown token in Def parse"))?
			}
		} else {
			input.step(|cursor| {
				if let Some((p, next)) = cursor.punct() {
					return Ok((Self::Punct(p.as_char()), next));
				}
				Err(Error::new(input.span(), "unknown token in Def parse"))?
			})?
		}
		.optimize();
		loop {
			if input.is_empty() {
				return Ok(root);
			} else if input.peek(Token![?]) {
				input.parse::<Token![?]>()?;
				let inner = root;
				root = Self::Optional(Box::new(inner.optimize()));
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
				root = Self::Multiplier(Box::new(inner.optimize()), sep, range).optimize();
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

impl Def {
	pub fn optimize(&self) -> Self {
		match self {
			Self::Combinator(defs, DefCombinatorStyle::Alternatives) if defs.len() == 2 => {
				let [first, second] = defs.as_slice() else { panic!("defs.len() was 2!") };
				match (first, second) {
					// "none | AutoOr<X>" can become "AutoNoneOr<X>"
					(Def::Ident(DefIdent(ident)), Def::Type(DefType::AutoOr(def)))
					| (Def::Type(DefType::AutoOr(def)), Def::Ident(DefIdent(ident)))
						if ident == "none" =>
					{
						Def::Type(DefType::AutoNoneOr(Box::new(*def.clone())))
					}
					// "auto | NoneOr<X>" can become "AutoNoneOr<X>"
					(Def::Ident(DefIdent(ident)), Def::Type(DefType::NoneOr(def)))
					| (Def::Type(DefType::NoneOr(def)), Def::Ident(DefIdent(ident)))
						if ident == "auto" =>
					{
						Def::Type(DefType::AutoNoneOr(Box::new(*def.clone())))
					}
					// "<X> | auto" can be simplified to "AutoOr<X>"
					(Def::Ident(DefIdent(ident)), def) | (def, Def::Ident(DefIdent(ident)))
						if ident == "auto" &&
						// Avoid AutoOr<Ident>, or AutoOr<NoneOr<>> though
						!matches!(def, Def::Ident(_) | Def::Type(DefType::AutoOr(_)) | Def::Type(DefType::NoneOr(_))) =>
					{
						Def::Type(DefType::AutoOr(Box::new(def.clone())))
					}
					// "<X> | none" can be simplified to "NoneOr<X>"
					(Def::Ident(DefIdent(ident)), def) | (def, Def::Ident(DefIdent(ident)))
						if ident == "none"  &&
						// Avoid NoneOr<Ident>, or NoneOr<AutoOr<>> though
						!matches!(def, Def::Ident(_) | Def::Type(DefType::AutoOr(_)) | Def::Type(DefType::NoneOr(_))) =>
					{
						Def::Type(DefType::NoneOr(Box::new(def.clone())))
					}
					// "<length-percentage> | <flex>" can be simplified to "<length-percentage-or-flex>"
					(Def::Type(DefType::Custom(ident)), Def::Type(DefType::LengthPercentage(r)))
					| (Def::Type(DefType::LengthPercentage(r)), Def::Type(DefType::Custom(ident)))
						if ident.to_string() == "Flex" =>
					{
						Def::Type(DefType::LengthPercentageOrFlex(r.clone()))
					}
					// "<number> | <percentage>" can be simplified to "<number-or-percentage>"
					(Def::Type(DefType::Number(r)), Def::Type(DefType::Percentage(_)))
					| (Def::Type(DefType::Percentage(_)), Def::Type(DefType::Number(r))) => {
						Def::Type(DefType::NumberPercentage(r.clone()))
					}
					// "<number> | <length>" can be simplified to "<number-or-length>"
					(Def::Type(DefType::Number(r)), Def::Type(DefType::Length(_)))
					| (Def::Type(DefType::Length(_)), Def::Type(DefType::Number(r))) => Def::Type(DefType::NumberLength(r.clone())),
					_ => {
						return Self::Combinator(
							vec![first.optimize(), second.optimize()],
							DefCombinatorStyle::Alternatives,
						);
					}
				}
			}
			Self::Combinator(defs, DefCombinatorStyle::Alternatives) if defs.len() == 3 => {
				let [first, second, third] = defs.as_slice() else { panic!("defs.len() was 3!") };
				match (first, second, third) {
					// "auto | none | <X>" can be simplified to "<number-length-or-auto>"
					(def, Def::Ident(DefIdent(first)), Def::Ident(DefIdent(second)))
					| (Def::Ident(DefIdent(first)), def, Def::Ident(DefIdent(second)))
					| (Def::Ident(DefIdent(first)), Def::Ident(DefIdent(second)), def)
						if matches!((first.as_str(), second.as_str()), ("auto", "none") | ("none", "auto")) &&
						// Avoid AutoNoneOr<Ident>, or AutoNoneOr<AutoOr<>> though
						!matches!(def, Def::Ident(_) | Def::Type(DefType::AutoOr(_)) | Def::Type(DefType::NoneOr(_))) =>
					{
						Def::Type(DefType::AutoNoneOr(Box::new(def.clone())))
					}
					// "<number> | <length> | auto" can be simplified to "AutoOr<NumberLength>"
					(Def::Type(DefType::Number(r)), Def::Type(DefType::Length(_)), Def::Ident(DefIdent(ident)))
					| (Def::Type(DefType::Length(_)), Def::Type(DefType::Number(r)), Def::Ident(DefIdent(ident)))
					| (Def::Ident(DefIdent(ident)), Def::Type(DefType::Length(_)), Def::Type(DefType::Number(r)))
					| (Def::Ident(DefIdent(ident)), Def::Type(DefType::Number(r)), Def::Type(DefType::Length(_)))
					| (Def::Type(DefType::Length(_)), Def::Ident(DefIdent(ident)), Def::Type(DefType::Number(r)))
					| (Def::Type(DefType::Number(r)), Def::Ident(DefIdent(ident)), Def::Type(DefType::Length(_)))
						if ident == "auto" =>
					{
						Def::Type(DefType::AutoOr(Box::new(Def::Type(DefType::NumberLength(r.clone())))))
					}
					// "<x> | <length-percentage> | <flex>" can be simplified to "<x> | <length-percentage-or-flex>"
					(def, Def::Type(DefType::Custom(ident)), Def::Type(DefType::LengthPercentage(r)))
					| (Def::Type(DefType::LengthPercentage(r)), def, Def::Type(DefType::Custom(ident)))
					| (Def::Type(DefType::LengthPercentage(r)), Def::Type(DefType::Custom(ident)), def)
						if ident.to_string() == "Flex" =>
					{
						Def::Combinator(
							vec![def.optimize(), Def::Type(DefType::LengthPercentageOrFlex(r.clone()))],
							DefCombinatorStyle::Alternatives,
						)
					}
					// "<x> | <number> | <percentage>" can be simplified to "<number-or-percentage>"
					(def, Def::Type(DefType::Number(r)), Def::Type(DefType::Percentage(_)))
					| (Def::Type(DefType::Percentage(_)), def, Def::Type(DefType::Number(r)))
					| (Def::Type(DefType::Percentage(_)), Def::Type(DefType::Number(r)), def) => Def::Combinator(
						vec![def.optimize(), Def::Type(DefType::NumberPercentage(r.clone()))],
						DefCombinatorStyle::Alternatives,
					),
					// "<x> | <number> | <length>" can be simplified to "<number-or-length>"
					(def, Def::Type(DefType::Number(r)), Def::Type(DefType::Length(_)))
					| (Def::Type(DefType::Length(_)), def, Def::Type(DefType::Number(r)))
					| (Def::Type(DefType::Length(_)), Def::Type(DefType::Number(r)), def) => Def::Combinator(
						vec![def.optimize(), Def::Type(DefType::NumberLength(r.clone()))],
						DefCombinatorStyle::Alternatives,
					),
					_ => {
						return Self::Combinator(
							vec![first.optimize(), second.optimize(), third.optimize()],
							DefCombinatorStyle::Alternatives,
						);
					}
				}
			}
			Self::Combinator(defs, style) => {
				return Self::Combinator(defs.iter().map(|d| d.optimize()).collect(), *style);
			}
			// Optimize multiplier styles to avoid unnecessarily allocating.
			// A Multiplier with a fixed range can be normalised to an Ordered combinator of the same value.
			Self::Multiplier(inner, DefMultiplierSeparator::None, DefRange::Fixed(i)) => {
				let opts: Vec<_> = (1..=*i as u32).map(|_| inner.deref().clone()).collect();
				Self::Combinator(opts, DefCombinatorStyle::Ordered)
			}
			// Optimize multiplier styles to avoid unnecessarily allocating.
			// A multiplier with a bounded range can be normalised to an Ordered combinator of some optionals.
			Self::Multiplier(inner, DefMultiplierSeparator::None, DefRange::Range(Range { start, end })) => {
				let opts: Vec<Def> = (1..=*end as i32)
					.map(|i| if i <= (*start as i32) { inner.deref().clone() } else { Self::Optional(inner.clone()) })
					.collect();
				Self::Combinator(opts, DefCombinatorStyle::Ordered)
			}
			Self::Multiplier(inner, sep, range) => {
				return Self::Multiplier(Box::new(inner.optimize()), *sep, range.clone());
			}
			Self::Optional(inner) => return Self::Optional(Box::new(inner.optimize())),
			_ => return self.clone(),
		}
		.optimize()
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
			"length-or-auto" => Self::AutoOr(Box::new(Def::Type(Self::Length(checks)))),
			"length-percentage" => Self::LengthPercentage(checks),
			"length-percentage-or-auto" => Self::AutoOr(Box::new(Def::Type(Self::LengthPercentage(checks)))),
			"length-percentage-or-flex" => Self::LengthPercentageOrFlex(checks),
			"decibel" => Self::Decibel(checks),
			"angle" => Self::Angle(checks),
			"time" => Self::Time(checks),
			"time-or-auto" => Self::AutoOr(Box::new(Def::Type(Self::Time(checks)))),
			"resolution" => Self::Resolution(checks),
			"integer" => Self::Integer(checks),
			"number" => Self::Number(checks),
			"percentage" => Self::Percentage(checks),
			"string" => Self::String,
			"color" => Self::Color,
			// bg-image is a shorthand type for NoneOr<Image>
			// https://drafts.csswg.org/css-backgrounds-3/#typedef-bg-image
			"bg-image" => Self::NoneOr(Box::new(Def::Type(DefType::Image))),
			"image" => Self::Image,
			"image-1D" => Self::Image1D,
			// URI is an alias for URL
			// https://drafts.csswg.org/css2/#value-def-uri
			"uri" => Self::Url,
			"url" => Self::Url,
			"dashed-ident" => Self::DashedIdent,
			"custom-ident" => Self::CustomIdent,
			str => {
				let mut str = str.to_pascal_case().to_owned();
				if input.peek(token::Paren) {
					let content;
					parenthesized!(content in input);
					if !content.is_empty() {
						Err(Error::new(input.span(), "disallowed content inside deftype function"))?
					}
					str.push_str("Function");
				}
				Self::Custom(DefIdent(str))
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

impl From<Ident> for DefIdent {
	fn from(value: Ident) -> Self {
		Self(value.to_string())
	}
}
