use crate::CursorSink;
use bumpalo::collections::Vec;

/// This trait allows AST nodes to decompose themselves back into a set of (ordered) [Cursors][css_lexer::Cursor].
///
/// This trait is useful to implement because downstream operations can use it to reconstruct source text from Nodes,
/// including after mutating Nodes, such as transforming them (e.g. minification or formatting).
///
/// Nodes that implement this trait should call `s.append()` in the order that those [Cursors][css_lexer::Cursor] were parsed,
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

// TODO: This is here for types that are todo, and may want to derive ToCursors
impl ToCursors for () {
	fn to_cursors(&self, _: &mut impl CursorSink) {}
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

impl<T, U, V> ToCursors for (T, U, V)
where
	T: ToCursors,
	U: ToCursors,
	V: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
		ToCursors::to_cursors(&self.1, s);
		ToCursors::to_cursors(&self.2, s);
	}
}

impl<T, U, V, W> ToCursors for (T, U, V, W)
where
	T: ToCursors,
	U: ToCursors,
	V: ToCursors,
	W: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.0, s);
		ToCursors::to_cursors(&self.1, s);
		ToCursors::to_cursors(&self.2, s);
		ToCursors::to_cursors(&self.3, s);
	}
}
