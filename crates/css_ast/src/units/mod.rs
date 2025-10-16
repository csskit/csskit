mod angles;
mod custom;
mod decibel;
mod flex;
mod float;
mod frequency;
mod int;
mod length;
mod line_width;
mod number;
mod percentage;
mod resolution;
mod time;

pub use angles::*;
pub use custom::*;
pub use decibel::*;
pub use flex::*;
pub use float::*;
pub use frequency::*;
pub use int::*;
pub use length::*;
pub use line_width::*;
pub use number::*;
pub use percentage::*;
pub use resolution::*;
pub use time::*;

mod prelude {
	pub(crate) use crate::{CssAtomSet, CssDiagnostic};
	pub(crate) use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result as ParserResult, T, ToNumberValue};
	pub(crate) use csskit_derives::{IntoCursor, Parse, Peek, ToCursors};
}

pub trait AbsoluteUnit {
	fn to_base(&self) -> Self;
}
