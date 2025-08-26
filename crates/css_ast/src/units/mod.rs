mod angles;
mod custom;
mod flex;
mod float;
mod frequency;
mod int;
mod length;
mod line_width;
mod number;
mod resolution;
mod time;

pub use angles::*;
pub use custom::*;
pub use flex::*;
pub use float::*;
pub use frequency::*;
pub use int::*;
pub use length::*;
pub use line_width::*;
pub use number::*;
pub use resolution::*;
pub use time::*;

pub trait AbsoluteUnit {
	fn to_base(&self) -> Self;
}
