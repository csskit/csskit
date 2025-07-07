#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-matrix
// matrix() = matrix( <number>#{6} )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Matrix {
	matrix: T![Function],
	pub a: T![Number],
	a_: Option<T![,]>,
	pub b: T![Number],
	b_: Option<T![,]>,
	pub c: T![Number],
	c_: Option<T![,]>,
	pub d: T![Number],
	d_: Option<T![,]>,
	pub e: T![Number],
	e_: Option<T![,]>,
	pub f: T![Number],
	f_: Option<T![,]>,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for Matrix {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "matrix")
	}
}

impl<'a> Parse<'a> for Matrix {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let matrix = p.parse::<T![Function]>()?;
		let c: Cursor = matrix.into();
		if !p.eq_ignore_ascii_case(c, "matrix") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}

		let a = p.parse::<T![Number]>()?;
		let a_ = p.parse_if_peek::<T![,]>()?;
		let b = p.parse::<T![Number]>()?;
		let b_ = p.parse_if_peek::<T![,]>()?;
		let c = p.parse::<T![Number]>()?;
		let c_ = p.parse_if_peek::<T![,]>()?;
		let d = p.parse::<T![Number]>()?;
		let d_ = p.parse_if_peek::<T![,]>()?;
		let e = p.parse::<T![Number]>()?;
		let e_ = p.parse_if_peek::<T![,]>()?;
		let f = p.parse::<T![Number]>()?;
		let f_ = p.parse_if_peek::<T![,]>()?;
		let close = p.parse_if_peek::<T![')']>()?;

		Ok(Self { matrix, a, a_, b, b_, c, c_, d, d_, e, e_, f, f_, close })
	}
}

impl<'a> ToCursors for Matrix {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.matrix, s);
		ToCursors::to_cursors(&self.a, s);
		if let Some(a_) = self.a_ {
			ToCursors::to_cursors(&a_, s);
		}
		ToCursors::to_cursors(&self.b, s);
		if let Some(b_) = self.b_ {
			ToCursors::to_cursors(&b_, s);
		}
		ToCursors::to_cursors(&self.c, s);
		if let Some(c_) = self.c_ {
			ToCursors::to_cursors(&c_, s);
		}
		ToCursors::to_cursors(&self.d, s);
		if let Some(d_) = self.d_ {
			ToCursors::to_cursors(&d_, s);
		}
		ToCursors::to_cursors(&self.e, s);
		if let Some(e_) = self.e_ {
			ToCursors::to_cursors(&e_, s);
		}
		ToCursors::to_cursors(&self.f, s);
		if let Some(f_) = self.f_ {
			ToCursors::to_cursors(&f_, s);
		}
		if let Some(ref close) = self.close {
			ToCursors::to_cursors(close, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Matrix>(), 196);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Matrix, "matrix(1,2,3,4,5,6)");
		assert_parse!(Matrix, "matrix(1 2 3 4 5 6)");
		assert_parse!(Matrix, "matrix(0,0,0,0,0,0)");
		assert_parse!(Matrix, "matrix(-1,-2,-3,-4,-5,-6)");
		assert_parse!(Matrix, "matrix(1.5,2.5,3.5,4.5,5.5,6.5)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Matrix, "matrix()");
		assert_parse_error!(Matrix, "matrix(1)");
		assert_parse_error!(Matrix, "matrix(1,2)");
		assert_parse_error!(Matrix, "matrix(one,two,three,four,five,size)");
	}
}
