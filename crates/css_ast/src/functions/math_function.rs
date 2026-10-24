use super::prelude::*;
use crate::{
	Angle, AngleOrNumber, AttrFunction, EnvFunction, FirstValidFunction, Frequency, IfFunction, Length, NoneOr, Number,
	Resolution, Time, TreeCountingFunction, Unresolved, VarFunction,
};
use css_parse::{AtomSet, SemanticEq, Token};

/// ```text,ignore
/// <calc()>  = calc( <calc-sum> )
/// <min()>   = min( <calc-sum># )
/// <max()>   = max( <calc-sum># )
/// <clamp()> = clamp( [ <calc-sum> | none ], <calc-sum>, [ <calc-sum> | none ] )
/// <round()> = round( <rounding-strategy>?, <calc-sum>, <calc-sum>? )
/// <mod()>   = mod( <calc-sum>, <calc-sum> )
/// <rem()>   = rem( <calc-sum>, <calc-sum> )
/// <sin()>   = sin( <calc-sum> )
/// <cos()>   = cos( <calc-sum> )
/// <tan()>   = tan( <calc-sum> )
/// <asin()>  = asin( <calc-sum> )
/// <acos()>  = acos( <calc-sum> )
/// <atan()>  = atan( <calc-sum> )
/// <atan2()> = atan2( <calc-sum>, <calc-sum> )
/// <pow()>   = pow( <calc-sum>, <calc-sum> )
/// <sqrt()>  = sqrt( <calc-sum> )
/// <hypot()> = hypot( <calc-sum># )
/// <log()>   = log( <calc-sum>, <calc-sum>? )
/// <exp()>   = exp( <calc-sum> )
/// <abs()>   = abs( <calc-sum> )
/// <sign()>  = sign( <calc-sum> )
/// <calc-sum> = <calc-product> [ [ '+' | '-' ] <calc-product> ]*
/// <calc-product> = <calc-value> [ [ '*' | / ] <calc-value> ]*
/// <calc-value> = <number> | <dimension> | <percentage> |
///                <calc-keyword> | ( <calc-sum> )
/// <calc-keyword> = e | pi | infinity | -infinity | NaN
/// <rounding-strategy> = nearest | up | down | to-zero | line-width
/// ```
#[node]
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub enum MathFunction<'a, T> {
	CalcFunction(CalcFunction<'a, T>),
	MinFunction(MinFunction<'a, T>),
	MaxFunction(MaxFunction<'a, T>),
	ClampFunction(ClampFunction<'a, T>),
	RoundFunction(RoundFunction<'a, T>),
	ModFunction(ModFunction<'a, T>),
	RemFunction(RemFunction<'a, T>),
	SinFunction(SinFunction<'a>),
	CosFunction(CosFunction<'a>),
	TanFunction(TanFunction<'a>),
	AsinFunction(AsinFunction<'a>),
	AcosFunction(AcosFunction<'a>),
	AtanFunction(AtanFunction<'a>),
	Atan2Function(Atan2Function<'a, T>),
	PowFunction(PowFunction<'a>),
	SqrtFunction(SqrtFunction<'a>),
	HypotFunction(HypotFunction<'a, T>),
	LogFunction(LogFunction<'a>),
	ExpFunction(ExpFunction<'a>),
	AbsFunction(AbsFunction<'a, T>),
	SignFunction(SignFunction<'a, T>),
}

/// Returns true if the given atom is a CSS math function name.
pub fn is_math_function(atom: CssAtomSet) -> bool {
	matches!(
		atom,
		CssAtomSet::Calc
			| CssAtomSet::Min
			| CssAtomSet::Max
			| CssAtomSet::Clamp
			| CssAtomSet::Round
			| CssAtomSet::Mod
			| CssAtomSet::Rem
			| CssAtomSet::Sin
			| CssAtomSet::Cos
			| CssAtomSet::Tan
			| CssAtomSet::Asin
			| CssAtomSet::Acos
			| CssAtomSet::Atan
			| CssAtomSet::Atan2
			| CssAtomSet::Pow
			| CssAtomSet::Sqrt
			| CssAtomSet::Hypot
			| CssAtomSet::Log
			| CssAtomSet::Exp
			| CssAtomSet::Abs
			| CssAtomSet::Sign
	)
}

impl<'a, T> Peek<'a> for MathFunction<'a, T> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c) && is_math_function(p.to_atom::<CssAtomSet>(c))
	}
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct CalcFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Calc)]
	pub name: Function,
	pub params: CalcSum<'a, T>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct MinFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Min)]
	pub name: Function,
	pub params: CommaSeparated<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct MaxFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Max)]
	pub name: Function,
	pub params: CommaSeparated<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-clamp>
///
/// ```text,ignore
/// clamp( [ <calc-sum> | none ], <calc-sum>, [ <calc-sum> | none ] )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct ClampFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Clamp)]
	pub name: Function,
	pub min: NoneOr<Box<'a, CalcSum<'a, T>>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma_1: Comma,
	pub value: Box<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma_2: Comma,
	pub max: NoneOr<Box<'a, CalcSum<'a, T>>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-round>
///
/// ```text,ignore
/// round( <rounding-strategy>?, <calc-sum>, <calc-sum>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct RoundFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Round)]
	pub name: Function,
	pub strategy: Option<RoundingStrategy>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma_1: Option<Comma>,
	pub value: Box<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma_2: Option<Comma>,
	pub step: Option<Box<'a, CalcSum<'a, T>>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-mod>
///
/// ```text,ignore
/// mod( <calc-sum>, <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct ModFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Mod)]
	pub name: Function,
	pub dividend: CalcSum<'a, T>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Comma,
	pub divisor: Box<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-rem>
///
/// ```text,ignore
/// rem( <calc-sum>, <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct RemFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Rem)]
	pub name: Function,
	pub dividend: CalcSum<'a, T>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Comma,
	pub divisor: Box<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-sin>
///
/// ```text,ignore
/// sin( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct SinFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Sin)]
	pub name: Function,
	pub params: CalcSum<'a, AngleOrNumber>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-cos>
///
/// ```text,ignore
/// cos( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct CosFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Cos)]
	pub name: Function,
	pub params: CalcSum<'a, AngleOrNumber>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-tan>
///
/// ```text,ignore
/// tan( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct TanFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Tan)]
	pub name: Function,
	pub params: CalcSum<'a, AngleOrNumber>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-asin>
///
/// ```text,ignore
/// asin( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct AsinFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Asin)]
	pub name: Function,
	pub params: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-acos>
///
/// ```text,ignore
/// acos( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct AcosFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Acos)]
	pub name: Function,
	pub params: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-atan>
///
/// ```text,ignore
/// atan( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct AtanFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Atan)]
	pub name: Function,
	pub params: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-atan2>
///
/// ```text,ignore
/// atan2( <calc-sum>, <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct Atan2Function<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Atan2)]
	pub name: Function,
	pub y: CalcSum<'a, T>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Comma,
	pub x: Box<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-pow>
///
/// ```text,ignore
/// pow( <calc-sum>, <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct PowFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Pow)]
	pub name: Function,
	pub base: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Comma,
	pub exponent: Box<'a, CalcSum<'a, Number>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-sqrt>
///
/// ```text,ignore
/// sqrt( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct SqrtFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Sqrt)]
	pub name: Function,
	pub params: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-hypot>
///
/// ```text,ignore
/// hypot( <calc-sum># )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct HypotFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Hypot)]
	pub name: Function,
	pub params: CommaSeparated<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-log>
///
/// ```text,ignore
/// log( <calc-sum>, <calc-sum>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct LogFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Log)]
	pub name: Function,
	pub value: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Option<Comma>,
	pub base: Option<Box<'a, CalcSum<'a, Number>>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-exp>
///
/// ```text,ignore
/// exp( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct ExpFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Exp)]
	pub name: Function,
	pub params: CalcSum<'a, Number>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-abs>
///
/// ```text,ignore
/// abs( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct AbsFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Abs)]
	pub name: Function,
	pub params: CalcSum<'a, T>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#funcdef-sign>
///
/// ```text,ignore
/// sign( <calc-sum> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct SignFunction<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Sign)]
	pub name: Function,
	pub params: CalcSum<'a, T>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#typedef-calc-sum>
///
/// ```text,ignore
/// <calc-sum> = <calc-product> [ [ '+' | '-' ] <calc-product> ]*
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CalcSum<'a, T> {
	pub first: CalcProduct<'a, T>,
	pub rest: Vec<'a, (CalcSumOperator, CalcProduct<'a, T>)>,
}

/// The `+` or `-` operator inside a `<calc-sum>`.
///
/// Per <https://drafts.csswg.org/css-values-4/#calc-syntax>, whitespace is required on both sides
/// of these operators (unlike `*` and `/`), since `1px -2px` (no space before `-`) must lex as two
/// adjacent values rather than a subtraction. This is why `Parse`/`Peek` are hand-written here.
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CalcSumOperator {
	Add(T![+]),
	Subtract(T![-]),
}

impl<'a> Peek<'a> for CalcSumOperator {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == Kind::Delim && (c == '+' || c == '-') && p.peek_n_including_whitespace(1) == Kind::Whitespace
	}
}

impl<'a> Parse<'a> for CalcSumOperator {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		// `+`/`-` in a `<calc-sum>` require whitespace on both sides (spec) and that whitespace is
		// significant: `calc(1px+1px)` is invalid and stripping it would change tokenisation.
		let rules = AssociatedWhitespaceRules::EnforceBefore | AssociatedWhitespaceRules::EnforceAfter;
		let op = if c == '+' {
			Self::Add(p.parse::<T![+]>()?.with_associated_whitespace(rules))
		} else {
			Self::Subtract(p.parse::<T![-]>()?.with_associated_whitespace(rules))
		};
		if p.peek_n_including_whitespace(1) != Kind::Whitespace {
			Err(Diagnostic::new(p.peek_n(1), Diagnostic::unexpected_delim))?;
		}
		Ok(op)
	}
}

/// A single operand of a `<calc-product>`: either a literal `<calc-value>` (number, dimension,
/// percentage, calc-keyword, or parenthesized `<calc-sum>`), or an arbitrary substitution
/// function (`var()`/`env()`/`attr()`/`if()`/`first-valid()`) per
/// <https://drafts.csswg.org/css-values-5/#arbitrary-substitution-function>.
#[node]
#[derive(Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub enum CalcOperand<'a, T> {
	Literal(CalcValue<'a, T>),
	Substituted(Box<'a, CalcOperandSubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
}

crate::values::impl_value_slot_parse!(CalcOperand, CalcOperandSubstitutionFunction, CalcValue<T>);

impl<'a, T: SemanticEq> SemanticEq for CalcOperand<'a, T> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		calc_operand_eq(self, other, source_text)
	}
}

fn calc_operand_eq<'a, T: SemanticEq>(a: &CalcOperand<'a, T>, b: &CalcOperand<'a, T>, source_text: &str) -> bool {
	match (a, b) {
		(CalcOperand::Literal(a), CalcOperand::Literal(b)) => calc_value_eq(a, b, source_text),
		(CalcOperand::Substituted(a), CalcOperand::Substituted(b)) => calc_operand_substitution_eq(a, b, source_text),
		(CalcOperand::Unresolved(a), CalcOperand::Unresolved(b)) => a.semantic_eq(b, source_text),
		_ => false,
	}
}

fn calc_value_eq<'a, T: SemanticEq>(a: &CalcValue<'a, T>, b: &CalcValue<'a, T>, source_text: &str) -> bool {
	match (a, b) {
		(CalcValue::Number(a), CalcValue::Number(b)) => a.semantic_eq(b, source_text),
		(CalcValue::Typed(a), CalcValue::Typed(b)) => a.semantic_eq(b, source_text),
		(CalcValue::Keyword(a), CalcValue::Keyword(b)) => a.semantic_eq(b, source_text),
		(CalcValue::TreeCounting(a), CalcValue::TreeCounting(b)) => a.semantic_eq(b, source_text),
		(CalcValue::Parenthesized(a), CalcValue::Parenthesized(b)) => calc_in_parens_eq(a, b, source_text),
		_ => false,
	}
}

fn calc_in_parens_eq<'a, T: SemanticEq>(a: &CalcInParens<'a, T>, b: &CalcInParens<'a, T>, source_text: &str) -> bool {
	a.open.semantic_eq(&b.open, source_text) && calc_sum_eq(&a.sum, &b.sum, source_text)
}

fn calc_sum_eq<'a, T: SemanticEq>(a: &CalcSum<'a, T>, b: &CalcSum<'a, T>, source_text: &str) -> bool {
	if a.rest.len() != b.rest.len() {
		return false;
	}
	calc_product_eq(&a.first, &b.first, source_text)
		&& a.rest
			.iter()
			.zip(b.rest.iter())
			.all(|((op_a, p_a), (op_b, p_b))| op_a == op_b && calc_product_eq(p_a, p_b, source_text))
}

fn calc_product_eq<'a, T: SemanticEq>(a: &CalcProduct<'a, T>, b: &CalcProduct<'a, T>, source_text: &str) -> bool {
	if a.rest.len() != b.rest.len() {
		return false;
	}
	calc_operand_eq(&a.first, &b.first, source_text)
		&& a.rest
			.iter()
			.zip(b.rest.iter())
			.all(|((op_a, p_a), (op_b, p_b))| op_a == op_b && calc_operand_eq(p_a, p_b, source_text))
}

fn calc_operand_substitution_eq<'a, T: SemanticEq>(
	a: &CalcOperandSubstitutionFunction<'a, T>,
	b: &CalcOperandSubstitutionFunction<'a, T>,
	source_text: &str,
) -> bool {
	match (a, b) {
		(CalcOperandSubstitutionFunction::Math(a), CalcOperandSubstitutionFunction::Math(b)) => {
			math_function_eq(a, b, source_text)
		}
		(CalcOperandSubstitutionFunction::Var(a), CalcOperandSubstitutionFunction::Var(b)) => {
			a.semantic_eq(b, source_text)
		}
		(CalcOperandSubstitutionFunction::Env(a), CalcOperandSubstitutionFunction::Env(b)) => {
			a.semantic_eq(b, source_text)
		}
		(CalcOperandSubstitutionFunction::Attr(a), CalcOperandSubstitutionFunction::Attr(b)) => {
			a.semantic_eq(b, source_text)
		}
		(CalcOperandSubstitutionFunction::If(a), CalcOperandSubstitutionFunction::If(b)) => {
			a.semantic_eq(b, source_text)
		}
		(CalcOperandSubstitutionFunction::FirstValid(a), CalcOperandSubstitutionFunction::FirstValid(b)) => {
			a.semantic_eq(b, source_text)
		}
		_ => false,
	}
}

fn math_function_eq<'a, T: SemanticEq>(a: &MathFunction<'a, T>, b: &MathFunction<'a, T>, source_text: &str) -> bool {
	use MathFunction::*;
	match (a, b) {
		(CalcFunction(a), CalcFunction(b)) => calc_sum_eq(&a.params, &b.params, source_text),
		(MinFunction(a), MinFunction(b)) => a.params.semantic_eq(&b.params, source_text),
		(MaxFunction(a), MaxFunction(b)) => a.params.semantic_eq(&b.params, source_text),
		(ClampFunction(a), ClampFunction(b)) => {
			calc_sum_or_none_eq(&a.min, &b.min, source_text)
				&& calc_sum_eq(&a.value, &b.value, source_text)
				&& calc_sum_or_none_eq(&a.max, &b.max, source_text)
		}
		(RoundFunction(a), RoundFunction(b)) => {
			a.strategy == b.strategy
				&& calc_sum_eq(&a.value, &b.value, source_text)
				&& calc_sum_opt_eq(&a.step, &b.step, source_text)
		}
		(ModFunction(a), ModFunction(b)) => {
			calc_sum_eq(&a.dividend, &b.dividend, source_text) && calc_sum_eq(&a.divisor, &b.divisor, source_text)
		}
		(RemFunction(a), RemFunction(b)) => {
			calc_sum_eq(&a.dividend, &b.dividend, source_text) && calc_sum_eq(&a.divisor, &b.divisor, source_text)
		}
		(SinFunction(a), SinFunction(b)) => a.semantic_eq(b, source_text),
		(CosFunction(a), CosFunction(b)) => a.semantic_eq(b, source_text),
		(TanFunction(a), TanFunction(b)) => a.semantic_eq(b, source_text),
		(AsinFunction(a), AsinFunction(b)) => a.semantic_eq(b, source_text),
		(AcosFunction(a), AcosFunction(b)) => a.semantic_eq(b, source_text),
		(AtanFunction(a), AtanFunction(b)) => a.semantic_eq(b, source_text),
		(Atan2Function(a), Atan2Function(b)) => a.semantic_eq(b, source_text),
		(PowFunction(a), PowFunction(b)) => a.semantic_eq(b, source_text),
		(SqrtFunction(a), SqrtFunction(b)) => a.semantic_eq(b, source_text),
		(HypotFunction(a), HypotFunction(b)) => a.semantic_eq(b, source_text),
		(LogFunction(a), LogFunction(b)) => a.semantic_eq(b, source_text),
		(ExpFunction(a), ExpFunction(b)) => a.semantic_eq(b, source_text),
		(AbsFunction(a), AbsFunction(b)) => a.semantic_eq(b, source_text),
		(SignFunction(a), SignFunction(b)) => calc_sum_eq(&a.params, &b.params, source_text),
		_ => false,
	}
}

fn calc_sum_or_none_eq<'a, T: SemanticEq>(
	a: &NoneOr<Box<'a, CalcSum<'a, T>>>,
	b: &NoneOr<Box<'a, CalcSum<'a, T>>>,
	source_text: &str,
) -> bool {
	match (a, b) {
		(NoneOr::Some(a), NoneOr::Some(b)) => calc_sum_eq(a, b, source_text),
		(NoneOr::None(_), NoneOr::None(_)) => true,
		_ => false,
	}
}

fn calc_sum_opt_eq<'a, T: SemanticEq>(
	a: &Option<Box<'a, CalcSum<'a, T>>>,
	b: &Option<Box<'a, CalcSum<'a, T>>>,
	source_text: &str,
) -> bool {
	match (a, b) {
		(None, None) => true,
		(Some(a), Some(b)) => calc_sum_eq(a, b, source_text),
		_ => false,
	}
}

/// A substitution or nested math function appearing in a [`CalcOperand`] slot.
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub enum CalcOperandSubstitutionFunction<'a, T> {
	Math(Box<'a, MathFunction<'a, T>>),
	Var(VarFunction<'a, CalcOperand<'a, T>>),
	Env(EnvFunction<'a, CalcOperand<'a, T>>),
	Attr(Box<'a, AttrFunction<'a>>),
	If(IfFunction<'a, CalcOperand<'a, T>>),
	FirstValid(FirstValidFunction<'a, CalcOperand<'a, T>>),
}

/// <https://drafts.csswg.org/css-values-4/#typedef-calc-product>
///
/// ```text,ignore
/// <calc-product> = <calc-value> [ [ '*' | / ] <calc-value> ]*
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CalcProduct<'a, T> {
	pub first: CalcOperand<'a, T>,
	pub rest: Vec<'a, (CalcProductOperator, CalcOperand<'a, T>)>,
}

impl<'a, T: SemanticEq> SemanticEq for CalcProduct<'a, T> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		calc_product_eq(self, other, source_text)
	}
}

/// The `*` or `/` operator inside a `<calc-product>`. Unlike `+`/`-`, no surrounding whitespace is
/// required.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CalcProductOperator {
	Multiply(T![*]),
	Divide(T![/]),
}

/// <https://drafts.csswg.org/css-values-4/#typedef-calc-value>
///
/// ```text,ignore
/// <calc-value> = <number> | <dimension> | <percentage> | <calc-keyword> | ( <calc-sum> )
///              | <sibling-count()> | <sibling-index()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CalcValue<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	Number(Number),
	Typed(T),
	Keyword(CalcKeyword),
	TreeCounting(TreeCountingFunction),
	Parenthesized(CalcInParens<'a, T>),
}

/// A parenthesized `<calc-sum>`, e.g. the `( 1px + 2px )` in `calc((1px + 2px) * 2)`.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CalcInParens<'a, T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub open: LeftParen,
	pub sum: Box<'a, CalcSum<'a, T>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

/// <https://drafts.csswg.org/css-values-4/#typedef-calc-keyword>
///
/// ```text,ignore
/// <calc-keyword> = e | pi | infinity | -infinity | NaN
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CalcKeyword {
	#[atom(CssAtomSet::E)]
	E(Ident),
	#[atom(CssAtomSet::Pi)]
	Pi(Ident),
	#[atom(CssAtomSet::Infinity)]
	Infinity(Ident),
	#[atom(CssAtomSet::_NegInfinity)]
	NegativeInfinity(Ident),
	#[atom(CssAtomSet::NaN)]
	NaN(Ident),
}

/// <https://drafts.csswg.org/css-values-4/#typedef-rounding-strategy>
///
/// ```text,ignore
/// <rounding-strategy> = nearest | up | down | to-zero | line-width
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RoundingStrategy {
	#[atom(CssAtomSet::Nearest)]
	Nearest(Ident),
	#[atom(CssAtomSet::Up)]
	Up(Ident),
	#[atom(CssAtomSet::Down)]
	Down(Ident),
	#[atom(CssAtomSet::ToZero)]
	ToZero(Ident),
	#[atom(CssAtomSet::LineWidth)]
	LineWidth(Ident),
}

const MAX_CALC_TERMS: usize = 4;

/// The unit of a term in a [`ReducedCalc`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcUnit {
	/// A unitless `<number>`.
	Number,
	/// A dimension or percentage, identified by its unit atom. Absolute lengths are in [`CssAtomSet::Px`], angles
	/// in [`CssAtomSet::Deg`], times in [`CssAtomSet::S`], frequencies in [`CssAtomSet::Hz`] and resolutions in
	/// [`CssAtomSet::Dppx`].
	Dimension(CssAtomSet),
}

/// A math expression reduced to a sum with at most one term per unit, such as `100% - 50px`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReducedCalc {
	terms: [(CalcUnit, f64); MAX_CALC_TERMS],
	len: usize,
	inverted: bool,
}

impl ReducedCalc {
	fn single(unit: CalcUnit, value: f64) -> Self {
		let mut terms = [(CalcUnit::Number, 0.0); MAX_CALC_TERMS];
		terms[0] = (unit, value);
		Self { terms, len: 1, inverted: false }
	}

	/// Reduces a number or dimension token, converting absolute lengths, angles, times, frequencies and resolutions
	/// to their canonical unit.
	/// Returns `None` for any other token, or a dimension with an unknown unit.
	pub fn from_token(token: Token) -> Option<Self> {
		let value = token.value() as f64;
		match token.kind() {
			Kind::Number => Some(Self::single(CalcUnit::Number, value)),
			Kind::Dimension if token.atom_bits() != 0 => {
				let unit = CssAtomSet::from_bits(token.atom_bits());
				let (unit, value) = if let Some(factor) = Length::px_per(unit) {
					(CssAtomSet::Px, value * factor as f64)
				} else if let Some(factor) = Angle::deg_per(unit) {
					(CssAtomSet::Deg, value * factor as f64)
				} else if let Some(divisor) = Time::per_s(unit) {
					(CssAtomSet::S, value / divisor as f64)
				} else if let Some(factor) = Frequency::hz_per(unit) {
					(CssAtomSet::Hz, value * factor as f64)
				} else if let Some(divisor) = Resolution::per_dppx(unit) {
					(CssAtomSet::Dppx, value / divisor as f64)
				} else {
					(unit, value)
				};
				Some(Self::single(CalcUnit::Dimension(unit), value))
			}
			_ => None,
		}
	}

	/// The terms of the sum, in the order their units first appeared.
	pub fn terms(&self) -> &[(CalcUnit, f64)] {
		&self.terms[..self.len]
	}

	/// The unit and value of a sum of exactly one term.
	pub fn as_single(&self) -> Option<(CalcUnit, f64)> {
		match self.terms() {
			[term] if !self.inverted => Some(*term),
			_ => None,
		}
	}

	fn scalar(&self) -> Option<f64> {
		match self.terms() {
			[(CalcUnit::Number, value)] => Some(*value),
			_ => None,
		}
	}

	fn dimension(&self) -> Option<(CssAtomSet, f64)> {
		match self.terms() {
			[
				(
					CalcUnit::Dimension(
						unit @ (CssAtomSet::Px | CssAtomSet::Deg | CssAtomSet::S | CssAtomSet::Hz | CssAtomSet::Dppx),
					),
					value,
				),
			] => Some((*unit, *value)),
			_ => None,
		}
	}

	fn is_number(&self) -> bool {
		self.terms().iter().any(|(unit, _)| *unit == CalcUnit::Number)
	}

	fn as_single_absolute(&self) -> Option<(CalcUnit, f64)> {
		self.as_single().filter(|(unit, _)| *unit != CalcUnit::Dimension(CssAtomSet::Percent))
	}

	fn scale(mut self, factor: f64) -> Self {
		for term in &mut self.terms[..self.len] {
			term.1 *= factor;
		}
		self
	}

	fn map(self, f: impl FnOnce(f64) -> f64) -> Option<Self> {
		let (unit, value) = self.as_single_absolute()?;
		Some(Self::single(unit, f(value)))
	}

	fn add(mut self, other: Self, sign: f64) -> Option<Self> {
		if self.is_number() != other.is_number() || self.inverted || other.inverted {
			return None;
		}
		for &(unit, value) in other.terms() {
			match self.terms[..self.len].iter_mut().find(|(term_unit, _)| *term_unit == unit) {
				Some(term) => term.1 += sign * value,
				None if self.len < MAX_CALC_TERMS => {
					self.terms[self.len] = (unit, sign * value);
					self.len += 1;
				}
				None => return None,
			}
		}
		Some(self)
	}

	fn multiply(self, other: Self) -> Option<Self> {
		match (self.scalar(), other.scalar()) {
			(Some(factor), _) => Some(other.scale(factor)),
			(_, Some(factor)) => Some(self.scale(factor)),
			_ => match (self.dimension(), other.dimension()) {
				(Some((lhs_unit, lhs)), Some((rhs_unit, rhs)))
					if lhs_unit == rhs_unit && self.inverted != other.inverted =>
				{
					Some(Self::single(CalcUnit::Number, lhs * rhs))
				}
				_ => None,
			},
		}
	}

	fn divide(self, other: Self) -> Option<Self> {
		if let Some(divisor) = other.scalar() {
			return (divisor != 0.0).then(|| self.scale(1.0 / divisor));
		}
		let (unit, divisor) = other.dimension().filter(|(_, value)| *value != 0.0)?;
		if let Some(dividend) = self.scalar() {
			return Some(Self {
				inverted: !other.inverted,
				..Self::single(CalcUnit::Dimension(unit), dividend / divisor)
			});
		}
		match self.dimension() {
			Some((dividend_unit, dividend)) if dividend_unit == unit && self.inverted == other.inverted => {
				Some(Self::single(CalcUnit::Number, dividend / divisor))
			}
			_ => None,
		}
	}

	fn pick(self, other: Self, choose: fn(f64, f64) -> f64) -> Option<Self> {
		let ((lhs_unit, lhs), (rhs_unit, rhs)) = (self.as_single_absolute()?, other.as_single_absolute()?);
		(lhs_unit == rhs_unit)
			.then(|| Self::single(lhs_unit, if lhs.is_nan() || rhs.is_nan() { f64::NAN } else { choose(lhs, rhs) }))
	}
}

fn reduce_comparison<'a, T: 'a>(
	sums: &CommaSeparated<'a, CalcSum<'a, T>>,
	leaf: &impl Fn(&T) -> Option<ReducedCalc>,
	choose: fn(f64, f64) -> f64,
) -> Option<ReducedCalc> {
	let mut sums = sums.into_iter().map(|(sum, _)| sum);
	let first = sums.next()?.reduce_with(leaf)?;
	sums.try_fold(first, |acc, sum| acc.pick(sum.reduce_with(leaf)?, choose))
}

impl<'a, T: Copy + Into<Token>> MathFunction<'a, T> {
	/// Reduces `calc()`, `min()`, `max()`, `clamp()`, `abs()` and `sign()`; see [`MathFunction::reduce_with`].
	pub fn reduce(&self) -> Option<ReducedCalc> {
		self.reduce_with(&|value| ReducedCalc::from_token((*value).into()))
	}
}

impl<'a, T> MathFunction<'a, T> {
	/// Reduces `calc()`, `min()`, `max()`, `clamp()`, `abs()` and `sign()`, using `leaf` to reduce each typed value.
	/// Returns `None` if the expression depends on a substitution or tree counting function, mixes incompatible
	/// units, divides by zero, compares or takes the sign of a percentage, or uses any other math function.
	pub fn reduce_with(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		self.fold(leaf).filter(|reduced| !reduced.inverted)
	}

	fn fold(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		match self {
			Self::CalcFunction(func) => func.params.fold(leaf),
			Self::MinFunction(func) => reduce_comparison(&func.params, leaf, f64::min),
			Self::MaxFunction(func) => reduce_comparison(&func.params, leaf, f64::max),
			Self::ClampFunction(func) => {
				let mut value = func.value.reduce_with(leaf)?;
				if let NoneOr::Some(max) = &func.max {
					value = value.pick(max.reduce_with(leaf)?, f64::min)?;
				}
				if let NoneOr::Some(min) = &func.min {
					value = value.pick(min.reduce_with(leaf)?, f64::max)?;
				}
				Some(value)
			}
			Self::AbsFunction(func) => func.params.reduce_with(leaf)?.map(f64::abs),
			Self::SignFunction(func) => {
				let reduced = func.params.reduce_with(leaf)?;
				let value = reduced.scalar().or_else(|| reduced.dimension().map(|(_, value)| value))?;
				let sign = if value == 0.0 || value.is_nan() { value } else { value.signum() };
				Some(ReducedCalc::single(CalcUnit::Number, sign))
			}
			_ => None,
		}
	}
}

impl<'a, T> CalcSum<'a, T> {
	/// Reduces this sum, using `leaf` to reduce each typed value; see [`MathFunction::reduce_with`].
	pub fn reduce_with(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		self.fold(leaf).filter(|reduced| !reduced.inverted)
	}

	fn fold(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		self.rest.iter().try_fold(self.first.fold(leaf)?, |acc, (op, product)| {
			let sign = if matches!(op, CalcSumOperator::Add(_)) { 1.0 } else { -1.0 };
			acc.add(product.fold(leaf)?, sign)
		})
	}
}

impl<'a, T> CalcProduct<'a, T> {
	fn fold(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		self.rest.iter().try_fold(self.first.fold(leaf)?, |acc, (op, operand)| {
			let rhs = operand.fold(leaf)?;
			match op {
				CalcProductOperator::Multiply(_) => acc.multiply(rhs),
				CalcProductOperator::Divide(_) => acc.divide(rhs),
			}
		})
	}
}

impl<'a, T> CalcOperand<'a, T> {
	fn fold(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		match self {
			Self::Literal(value) => value.fold(leaf),
			Self::Substituted(func) => match &**func {
				CalcOperandSubstitutionFunction::Math(math) => math.fold(leaf),
				_ => None,
			},
			Self::Unresolved(_) => None,
		}
	}
}

impl<'a, T> CalcValue<'a, T> {
	fn fold(&self, leaf: &impl Fn(&T) -> Option<ReducedCalc>) -> Option<ReducedCalc> {
		match self {
			Self::Number(number) => ReducedCalc::from_token((*number).into()),
			Self::Typed(typed) => leaf(typed),
			Self::Keyword(keyword) => Some(ReducedCalc::single(
				CalcUnit::Number,
				match keyword {
					CalcKeyword::E(_) => core::f64::consts::E,
					CalcKeyword::Pi(_) => core::f64::consts::PI,
					CalcKeyword::Infinity(_) => f64::INFINITY,
					CalcKeyword::NegativeInfinity(_) => f64::NEG_INFINITY,
					CalcKeyword::NaN(_) => f64::NAN,
				},
			)),
			Self::TreeCounting(_) => None,
			Self::Parenthesized(parens) => parens.sum.fold(leaf),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, Length};
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	type LengthMathFunction<'a> = MathFunction<'a, Length>;

	fn reduce(source_text: &str) -> Option<std::vec::Vec<(CalcUnit, f64)>> {
		use css_parse::{Arena, Parser};
		let arena = Arena::default();
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&arena, source_text, lexer);
		let math = parser.parse_entirely::<MathFunction<crate::LengthPercentage>>().output.unwrap();
		math.reduce().map(|reduced| reduced.terms().to_vec())
	}

	#[test]
	fn test_reduce() {
		let px = CalcUnit::Dimension(CssAtomSet::Px);
		let percent = CalcUnit::Dimension(CssAtomSet::Percent);
		assert_eq!(reduce("calc(1in + 4px)"), Some(vec![(px, 100.0)]));
		assert_eq!(reduce("calc((100% - 50px) / 2)"), Some(vec![(percent, 50.0), (px, -25.0)]));
		assert_eq!(reduce("calc(1 / (1 / 50px))"), Some(vec![(px, 50.0)]));
		assert_eq!(reduce("calc(1in / 48px)"), Some(vec![(CalcUnit::Number, 2.0)]));
		assert_eq!(reduce("min(1in, 100px)"), Some(vec![(px, 96.0)]));
		assert_eq!(reduce("clamp(10px, 50px, 20px)"), Some(vec![(px, 20.0)]));
		assert_eq!(reduce("calc(abs(-5px) + 5px)"), Some(vec![(px, 10.0)]));
		assert_eq!(reduce("sign(-5px)"), Some(vec![(CalcUnit::Number, -1.0)]));
	}

	#[test]
	fn test_reduce_unresolvable() {
		assert_eq!(reduce("calc(1 / 50px)"), None);
		assert_eq!(reduce("calc(10 / 4px)"), None);
		assert_eq!(reduce("calc(2 * (10 / 4px))"), None);
		assert_eq!(reduce("calc(2 * calc(10 / 4px))"), None);
		assert_eq!(reduce("calc(1px / 0)"), None);
		assert_eq!(reduce("calc(1px + 1)"), None);
		assert_eq!(reduce("calc(10px * 1em)"), None);
		assert_eq!(reduce("min(10px, 1em)"), None);
		assert_eq!(reduce("calc(var(--x) + 1px)"), None);
		assert_eq!(reduce("calc(sibling-index() * 1px)"), None);
		assert_eq!(reduce("round(5.5px, 1px)"), None);
		assert_eq!(reduce("min(10%, 20%)"), None);
		assert_eq!(reduce("clamp(10%, 50%, 20%)"), None);
		assert_eq!(reduce("abs(-5%)"), None);
		assert_eq!(reduce("sign(-5%)"), None);
	}

	#[test]
	fn test_calc_functions() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(2 + 3)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "min(1, 2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "max(3, 4)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "clamp(0, 5, 10)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "round(5.5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "abs(-5)");
	}

	#[test]
	fn test_clamp() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "clamp(none, 5px, 10px)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "clamp(0px, 5px, none)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "clamp(1px, 2px, 3px)");
	}

	#[test]
	fn test_round() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "round(5.5, 2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "round(up, 5.5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "round(nearest, 5.5, 2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "round(to-zero, 5.5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "round(line-width, 5.5)");
	}

	#[test]
	fn test_mod_rem() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "mod(18, 5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "rem(18, 5)");
	}

	#[test]
	fn test_trig_and_exponential() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "sin(45deg)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "cos(45deg)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "tan(45deg)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "asin(0.5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "acos(0.5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "atan(0.5)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "atan2(1, 1)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "pow(2, 3)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "sqrt(2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "hypot(1, 2, 3)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "log(8, 2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "log(8)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "exp(1)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "sign(-5)");
	}

	#[test]
	fn test_calc_keywords() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(e)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(pi)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(infinity)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(-infinity)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(NaN)");
	}

	#[test]
	fn test_calc_nesting_and_precedence() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px + 2px * 3)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc((1px + 2px) * 3)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px * 2 / 3)");
	}

	#[test]
	fn test_calc_substitution() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(var(--foo) * 2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px + var(--foo))");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(var(--foo, 1px) + var(--bar, 2px))");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "min(var(--foo), 10px)");
	}

	#[test]
	fn test_calc_nested_math_function() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(min(1px, 2px) + 3px)");
	}

	#[test]
	fn test_calc_tree_counting_functions() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(sibling-index() * 10px)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(10px * sibling-count())");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "min(sibling-index(), 4)");
	}

	#[test]
	fn test_calc_operator_whitespace_sensitivity() {
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px + -2px)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px*2)");
		assert_parse!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px/2)");
	}

	#[test]
	fn test_calc_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px -2px)");
		assert_parse_error!(CssAtomSet::ATOMS, LengthMathFunction, "calc()");
		assert_parse_error!(CssAtomSet::ATOMS, LengthMathFunction, "clamp(1px, 2px)");
		assert_parse_error!(CssAtomSet::ATOMS, LengthMathFunction, "calc(1px + 10%)");
		assert_peek_false!(CssAtomSet::ATOMS, LengthMathFunction, "--calc(1px)");
	}
}
