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

macro_rules! impl_tuple {
    ($($T:ident),*) => {
        impl<$($T),*> ToCursors for ($($T),*)
        where
            $($T: ToCursors,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
            fn to_cursors(&self, s: &mut impl CursorSink) {
                let ($($T),*) = self;
                $($T.to_cursors(s);)*
            }
        }
    };
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
