# atom_set

This library provides the [`AtomSet`] and [`DynAtomSet`] traits: the interface
for a fixed set of keywords ("atoms") which a lexer or parser matches ASCII
case-insensitively, and stores as a small integer rather than a string.

An atom set is an enum. Each variant is one keyword, the `#[default]` variant is
the empty atom which stands in for any unrecognised keyword, and the
discriminant is the bit representation the token stores:

```rust
use atom_set::AtomSet;
use derive_atom_set::AtomSet as DeriveAtomSet;

#[derive(Debug, Default, Copy, Clone, PartialEq, DeriveAtomSet)]
pub enum Units {
    #[default]
    _None,
    Px,
    Rem,
    #[atom("%")]
    Percent,
}

assert_eq!(Units::from_str("PX"), Units::Px);
assert_eq!(Units::Rem.to_str(), "rem");
assert_eq!(Units::from_str("furlong"), Units::_None);
assert_eq!(Units::from_bits(Units::Percent.as_bits()), Units::Percent);
```

Write the `impl` with [`derive_atom_set`], which generates lookup code
specialised to the keyword lengths in the set. Hand written impls must keep
[`AtomSet::from_str`] and [`AtomSet::to_str`] round-tripping, and must map the
`#[default]` variant to the empty string.

[`DynAtomSet`] is the object-safe form. It exchanges bits and `&'static str`
instead of `Self`, so a lexer can hold `&'static dyn DynAtomSet` and stay
generic over the language it lexes. A blanket impl covers every [`AtomSet`], so
implementing it by hand is not required.

[`derive_atom_set`]: https://csskit.rs/docs/internal/derive_atom_set/

## Part of csskit

This crate is part of [csskit], a comprehensive CSS tool chain.

[csskit]: https://csskit.rs/

## License

MIT
