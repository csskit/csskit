use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set};
use csskit_derives::{IntoSpan, ToCursors};

use crate::CSSInt;

function_set!(EasingFunctionKeyword { Linear: "linear", CubicBezier: "cubic-bezier", Steps: "steps" });

keyword_set!(StepPosition {
	JumpStart: "jump-start",
	JumpEnd: "jump-end",
	JumpNone: "jump-none",
	JumpBoth: "jump-both",
	Start: "start",
	End: "end",
});

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
#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum EasingFunction<'a> {
	Linear(T![Ident]),
	Ease(T![Ident]),
	EaseIn(T![Ident]),
	EaseOut(T![Ident]),
	EaseInOut(T![Ident]),
	StepStart(T![Ident]),
	StepEnd(T![Ident]),
	LinearFunction(
		T![Function],
		Vec<'a, (T![Number], Option<T![Dimension::%]>, Option<T![Dimension::%]>, Option<T![,]>)>,
		Option<T![')']>,
	),
	CubicBezierFunction(
		T![Function],
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![,]>,
		T![Number],
		Option<T![')']>,
	),
	StepFunction(T![Function], CSSInt, Option<T![,]>, Option<StepPosition>, Option<T![')']>),
}

impl<'a> Peek<'a> for EasingFunction<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		EasingKeyword::peek(p, c) || EasingFunctionKeyword::peek(p, c)
	}
}

impl<'a> Parse<'a> for EasingFunction<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<EasingKeyword>() {
			let keyword = p.parse::<EasingKeyword>()?;
			let c = keyword.into();
			let ident = <T![Ident]>::build(p, c);
			return match keyword {
				EasingKeyword::Linear(_) => Ok(Self::Linear(ident)),
				EasingKeyword::Ease(_) => Ok(Self::Ease(ident)),
				EasingKeyword::EaseIn(_) => Ok(Self::EaseIn(ident)),
				EasingKeyword::EaseOut(_) => Ok(Self::EaseOut(ident)),
				EasingKeyword::EaseInOut(_) => Ok(Self::EaseInOut(ident)),
				EasingKeyword::StepStart(_) => Ok(Self::StepStart(ident)),
				EasingKeyword::StepEnd(_) => Ok(Self::StepEnd(ident)),
			};
		}
		let keyword = p.parse::<EasingFunctionKeyword>()?;
		let c = keyword.into();
		let function = <T![Function]>::build(p, c);
		match keyword {
			EasingFunctionKeyword::Linear(f) => {
				let mut stops = Vec::new_in(p.bump());
				loop {
					if p.at_end() || p.peek::<T![')']>() {
						break;
					}
					let mut num = p.parse_if_peek::<T![Number]>()?;
					let percent = p.parse_if_peek::<T![Dimension::%]>()?;
					let percent2 = p.parse_if_peek::<T![Dimension::%]>()?;
					if num.is_none() {
						num = Some(p.parse::<T![Number]>()?);
					}
					stops.push((num.unwrap(), percent, percent2, p.parse_if_peek::<T![,]>()?));
				}
				if stops.len() < 2 {
					Err(diagnostics::NotEnoughArguments("linear".into(), 2, stops.len(), f.into()))?
				}
				Ok(Self::LinearFunction(function, stops, p.parse_if_peek::<T![')']>()?))
			}
			EasingFunctionKeyword::CubicBezier(_) => {
				let x1 = p.parse::<T![Number]>()?;
				let c1 = p.parse_if_peek::<T![,]>()?;
				let x2 = p.parse::<T![Number]>()?;
				let c2 = p.parse_if_peek::<T![,]>()?;
				let y1 = p.parse::<T![Number]>()?;
				let c3 = p.parse_if_peek::<T![,]>()?;
				let y2 = p.parse::<T![Number]>()?;
				Ok(Self::CubicBezierFunction(function, x1, c1, x2, c2, y1, c3, y2, p.parse_if_peek::<T![')']>()?))
			}
			EasingFunctionKeyword::Steps(_) => {
				let number = p.parse::<CSSInt>()?;
				let comma = p.parse_if_peek::<T![,]>()?;
				let position = p.parse_if_peek::<StepPosition>()?;
				dbg!(number, position);
				Ok(Self::StepFunction(function, number, comma, position, p.parse_if_peek::<T![')']>()?))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<EasingFunction>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EasingFunction, "ease-in-out");
		assert_parse!(EasingFunction, "linear(0,1)");
		assert_parse!(EasingFunction, "linear(0,0.25,1)");
		assert_parse!(EasingFunction, "linear(0,0.5 25% 75%,1)");
		assert_parse!(EasingFunction, "cubic-bezier(0.25,0.1,0.25,1)");
		assert_parse!(EasingFunction, "cubic-bezier(0.1,-0.6,0.2,0)");
		assert_parse!(EasingFunction, "cubic-bezier(0,0,1,1)");
		assert_parse!(EasingFunction, "steps(4,end)");
		assert_parse!(EasingFunction, "steps(10,jump-both)");
		// // Incomplete but recoverable
		assert_parse!(EasingFunction, "linear(0,0.25,1");
		assert_parse!(EasingFunction, "linear(0 0.25 1)");
		assert_parse!(EasingFunction, "cubic-bezier(0.1 -0.6 0.2 0)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(EasingFunction, "foo");
		assert_parse_error!(EasingFunction, "linear()");
		assert_parse_error!(EasingFunction, "cubic-bezier(0.1, red, 1.0, green)");
	}
}
