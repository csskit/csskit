use crate::CursorSink;
use bumpalo::collections::Vec;

/// This trait allows AST nodes to decompose themselves back into a set of (ordered) [Cursors][Cursor].
///
/// This trait is useful to implement because downstream operations can use it to reconstruct source text from Nodes,
/// including after mutating Nodes, such as transforming them (e.g. minification or formatting).
///
/// Nodes that implement this trait should call `s.append()` in the order that those [Cursors][Cursor] were parsed,
/// unless there's a good reason not to. Some good reasons not to:
///
///  - The specification supplies a specific grammar order.
///  - Doing so would require creating many intermediary enums or structs.
///
pub trait ToCursors {
	fn to_cursors(&self, s: &mut impl CursorSink);
}

impl<T> ToCursors for Option<T>
where
	T: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(t) = self {
			ToCursors::to_cursors(t, s);
		}
	}
}

impl<'a, T> ToCursors for Vec<'a, T>
where
	T: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for item in self.iter() {
			ToCursors::to_cursors(item, s);
		}
	}
}

impl<T, U> ToCursors for (T, U)
where
	T: ToCursors,
	U: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
		ToCursors::to_cursors(&self.1, s);
	}
}
