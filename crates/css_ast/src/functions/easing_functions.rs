use super::prelude::*;
use crate::Percentage;

// https://drafts.csswg.org/css-easing-2/#typedef-easing-function
// <easing-function> = <linear-easing-function>
//                      | <cubic-bezier-easing-function>
//                      | <step-easing-function>
//
// <linear-easing-function> = linear | <linear()>
//
// linear() = linear( [ <number> && <percentage>{0,2} ]# )
//
// <cubic-bezier-easing-function> =
// 	ease | ease-in | ease-out | ease-in-out | <cubic-bezier()>
//
// cubic-bezier() = cubic-bezier( [ <number [0,1]>, <number> ]#{2} )
//
// <step-easing-function> = step-start | step-end | <steps()>
//
// steps() = steps( <integer>, <step-position>?)
//
// <step-position> = jump-start | jump-end | jump-none | jump-both | start | end
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum EasingFunction<'a> {
	#[visit(skip)]
	#[atom(CssAtomSet::Linear)]
	Linear(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::Ease)]
	Ease(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::EaseIn)]
	EaseIn(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::EaseOut)]
	EaseOut(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::EaseInOut)]
	EaseInOut(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::StepStart)]
	StepStart(T![Ident]),
	#[visit(skip)]
	#[atom(CssAtomSet::StepEnd)]
	StepEnd(T![Ident]),
	#[atom(CssAtomSet::Linear)]
	LinearFunction(LinearFunction<'a>),
	#[atom(CssAtomSet::CubicBezier)]
	CubicBezierFunction(CubicBezierFunction),
	#[atom(CssAtomSet::Steps)]
	StepsFunction(StepsFunction),
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LinearFunction<'a> {
	#[atom(CssAtomSet::Linear)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, LinearFunctionParams>,
	pub close: T![')'],
}

#[derive(Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LinearFunctionParams(T![Number], Option<Percentage>, Option<Percentage>);

impl<'a> Parse<'a> for LinearFunctionParams {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut num = p.parse_if_peek::<T![Number]>()?;
		let percent = p.parse_if_peek::<Percentage>()?;
		let percent2 = p.parse_if_peek::<Percentage>()?;
		if num.is_none() {
			num = Some(p.parse::<T![Number]>()?);
		}
		Ok(Self(num.unwrap(), percent, percent2))
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct CubicBezierFunction {
	#[atom(CssAtomSet::CubicBezier)]
	pub name: T![Function],
	pub params: CubicBezierFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CubicBezierFunctionParams {
	x1: T![Number],
	c1: Option<T![,]>,
	x2: T![Number],
	c2: Option<T![,]>,
	y1: T![Number],
	c3: Option<T![,]>,
	y2: T![Number],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct StepsFunction {
	#[atom(CssAtomSet::Steps)]
	pub name: T![Function],
	pub params: StepsFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct StepsFunctionParams(CSSInt, Option<T![,]>, Option<StepPosition>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum StepPosition {
	#[atom(CssAtomSet::JumpStart)]
	JumpStart(T![Ident]),
	#[atom(CssAtomSet::JumpEnd)]
	JumpEnd(T![Ident]),
	#[atom(CssAtomSet::JumpNone)]
	JumpNone(T![Ident]),
	#[atom(CssAtomSet::JumpBoth)]
	JumpBoth(T![Ident]),
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<EasingFunction>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "ease-in-out");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,0.25,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,0.5 25% 75%,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.25,0.1,0.25,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.1,-0.6,0.2,0)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0,0,1,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "steps(4,end)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "steps(10,jump-both)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "linear(0,0.25,1)");
		assert_parse!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.1 -0.6 0.2 0)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, EasingFunction, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, EasingFunction, "linear()");
		assert_parse_error!(CssAtomSet::ATOMS, EasingFunction, "cubic-bezier(0.1, red, 1.0, green)");
	}
}
