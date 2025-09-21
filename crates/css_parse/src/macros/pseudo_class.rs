/// A macro for defining pseudo classes.
///
/// This makes it much easier to define a pseudo class. Parsing is also a little bit delicate, as the two
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
/// pseudo_class!(
///   /// Some docs on this type...
///   #[derive(Debug, ToCursors, ToSpan)]
///   pub enum MyPseudoClass {
///     Foo: MyAtomSet::Foo,
///     Bar: MyAtomSet::Bar,
///     Baz: MyAtomSet::Baz,
///   }
/// );
///
/// // The result will be one of the variants in the enum, matching the keyword.
/// assert_parse!(MyAtomSet::ATOMS, MyPseudoClass, ":foo");
///
/// // Matches are case insensitive
/// assert_parse!(MyAtomSet::ATOMS, MyPseudoClass, ":BaR");
///
/// // Words that do not match will fail to parse.
/// assert_parse_error!(MyAtomSet::ATOMS, MyPseudoClass, ":bing");
///
/// // The `:` is also required
/// assert_parse_error!(MyAtomSet::ATOMS, MyPseudoClass, "baz");
///
/// // Any tokens between the `:` and ident result in a parse error:
/// assert_parse_error!(MyAtomSet::ATOMS, MyPseudoClass, ": foo");
/// ```
#[macro_export]
macro_rules! pseudo_class {
	($(#[$meta:meta])* $vis:vis enum $name: ident { $first_variant: ident: $atoms: ident::$first:ident, $( $variant: ident: $variant_pat: pat$(,)?)* }) => {
		$(#[$meta])*
		pub enum $name {
			$first_variant($crate::T![:], $crate::T![Ident]),
			$($variant($crate::T![:], $crate::T![Ident]),)*
		}

		impl<'a> $crate::Peek<'a> for $name {
			fn peek(p: &$crate::Parser<'a>, c: $crate::Cursor) -> bool {
				let c2 = p.peek_n(2);
				c == $crate::Kind::Colon &&
				c2 == $crate::Kind::Ident &&
				matches!(p.to_atom::<$atoms>(c2), $atoms::$first $(| $variant_pat)*)
			}
		}

		impl<'a> $crate::Parse<'a> for $name {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				let colon = p.parse::<$crate::T![:]>()?;
				let skip = p.set_skip($crate::KindSet::NONE);
				let ident = p.parse::<$crate::T![Ident]>();
				p.set_skip(skip);
				let ident = ident?;
				match p.to_atom::<$atoms>(ident.into()) {
					$atoms::$first => Ok(Self::$first_variant(colon, ident)),
					$($variant_pat => Ok(Self::$variant(colon, ident)),)*
					_ => {
						Err($crate::Diagnostic::new(ident.into(), $crate::Diagnostic::unexpected_ident))?
					}
				}
			}
		}
	}
}
