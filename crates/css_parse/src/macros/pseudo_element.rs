/// A macro for defining pseudo elements.
///
/// This makes it much easier to define a pseudo element. Parsing is also a little bit delicate, as the two
/// [Cursors][crate::Cursor] must appear next to each other - no whitespace nor comments can be present betwixt the
/// colon and ident.
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use csskit_derives::*;
/// use derive_atom_set::*;
/// use bumpalo::Bump;
///
/// #[derive(Debug, Default, AtomSet, Copy, Clone, PartialEq)]
/// pub enum MyAtomSet {
///   #[default]
///   _None,
///   Foo,
///   Bar,
///   Baz,
/// }
/// impl MyAtomSet {
///   const ATOMS: MyAtomSet = MyAtomSet::_None;
/// }
///
/// pseudo_element!(
///   /// Some docs on this type...
///   #[derive(Debug, ToCursors, ToSpan)]
///   pub enum MyPseudoElement {
///     Foo: MyAtomSet::Foo,
///     Bar: MyAtomSet::Bar,
///     Baz: MyAtomSet::Baz,
///   }
/// );
///
/// // Matches are case insensitive
/// assert_parse!(MyAtomSet::ATOMS, MyPseudoElement, "::FoO");
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(MyAtomSet::ATOMS, MyPseudoElement, "::bar");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(MyAtomSet::ATOMS, MyPseudoElement, "::bing");
/// ```
#[macro_export]
macro_rules! pseudo_element {
	($(#[$meta:meta])* $vis:vis enum $name: ident { $first_variant: ident: $atoms: ident::$first:ident, $( $variant: ident: $variant_pat: pat$(,)?)* }) => {
		$(#[$meta])*
		$vis enum $name {
			$first_variant($crate::T![::], $crate::T![Ident]),
			$($variant($crate::T![::], $crate::T![Ident]),)*
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek<I>(p: &$crate::Parser<'a, I>, c: $crate::Cursor) -> bool
			where
				I: Iterator<Item = $crate::Cursor> + Clone,
			{
				let c2 = p.peek_n(2);
				let c3 = p.peek_n(3);
				c == $crate::Kind::Colon
				&& c2 == $crate::Kind::Colon
				&& c3 == $crate::Kind::Ident
				&& matches!(p.to_atom::<$atoms>(c3), $atoms::$first $(| $variant_pat)*)
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse<I>(p: &mut $crate::Parser<'a, I>) -> $crate::Result<Self>
			where
				I: Iterator<Item = $crate::Cursor> + Clone,
			{
				let colons = p.parse::<$crate::T![::]>()?;
				let skip = p.set_skip($crate::KindSet::NONE);
				let ident = p.parse::<$crate::T![Ident]>();
				p.set_skip(skip);
				let ident = ident?;
				match p.to_atom::<$atoms>(ident.into()) {
					$atoms::$first => Ok(Self::$first_variant(colons, ident)),
					$($variant_pat => Ok(Self::$variant(colons, ident)),)*
					_ => {
						use $crate::ToSpan;
						Err($crate::Diagnostic::new(ident.into(), Diagnostic::unexpected_ident))?
					}
				}
			}
		}
	}
}
