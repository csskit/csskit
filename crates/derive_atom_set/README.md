# derive_atom_set

Often it is useful to represent strings once in memory, as a special value or
type. This is the concept of [String Interning], often called an "Atom" type.
Most Atom implementations will use a hashing algorithm and a map, returning
something like an opaque integer in place of the string.

[String Interning]: https://en.wikipedia.org/wiki/String_interning

Many languages (HTML & CSS) also allow keywords to be case insensitive.
Libraries like [unicase] can be used in combination with hashing to produce a
case insensitive string intern map, but this can often add additional overhead
making the intern map take even longer to match. For performance critical
implementations, this overhead can be undesirable.

[unicase]: https://en.wikipedia.org/wiki/String_interning

`derive_atom_set` bakes in the assumption that keywords are case insensitive,
and rather than a map, it expects you to enumerate all keywords upfront, in an
enum. The _benefit_ of these two things is that it can make some assumptions
about the data and really quickly match an Atom, making for wicked fast string
matching.

## Usage

The macro implements an `AtomSet` trait supplied by the consuming crate. That
trait must be in scope where the derive is used. `DeriveAtomSet` will impl this
trait, generating implementations bespoke to your enums discriminants:

```rust
pub trait AtomSet: Default + std::fmt::Debug {
 fn from_str(keyword: &str) -> Self;
 fn to_str(self) -> &'static str;
 fn len(&self) -> u32;
 fn from_bits(bits: u32) -> Self;
 fn as_bits(&self) -> u32;
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, DeriveAtomSet)]
enum Unit {
    #[default]
    Unknown,
    Px,
    Em,
    Rem,
    Vmin,
    #[atom("%")]
    Percentage,
}

assert_eq!(Unit::from_str("px"), Unit::Px);
assert_eq!(Unit::from_str("PX"), Unit::Px);
assert_eq!(Unit::Vmin.to_str(), "vmin");
assert_eq!(Unit::Percentage.to_str(), "%");
assert_eq!(Unit::from_str("not-a-unit"), Unit::Unknown);
assert_eq!(Unit::Unknown.to_str(), "");

let bits = Unit::Rem.as_bits();
assert_eq!(Unit::from_bits(bits), Unit::Rem);
```

Exactly one unit variant must carry `#[default]`. It represents an empty or
unrecognised atom. Derive `Default` and `Debug` as required by the `AtomSet`
trait.

Use `#[atom("...")]` to override a non-default variant's string:

```rust
#[atom("%")]
Percentage,
```

Without an `atom` attribute, variant names are converted to kebab-case:

| Variant           | Atom                 |
| ----------------- | -------------------- |
| `Px`              | `"px"`               |
| `FontSize`        | `"font-size"`        |
| `WebkitTransform` | `"webkit-transform"` |

The macro supports fieldless enums. All atoms must be known at compile time.

`as_bits()` values are enum discriminants. Do not treat them as a stable
serialization format unless the enum assigns and maintains explicit
discriminants.

## How matching works

`derive_atom_set` uses compile time checks to use a bunch of gnarly tricks to
efficiently string match, aiming for sub-20ns lookup matching against 1000s of
atoms. It does so with a few interesting techniques:

- All atoms are bucketed by their length, which means an atom set can cheaply
  check character length and only match against atoms in that length bucket,
  significantly reducing the number of potential branches. If you have 100 atoms
  and only 5 of them are 10 characters or less, `derive_atom_set` intrinsically
  will not match against the remaining 95 atoms. Likewise, this means matching
  against unknown strings happens really quickly, as a string with an
  un-bucketed length is immediately discarded.
- It routes strings to different character matching algorithms based on the
  length:
  - Single letter keywords use a lookup table for the lowest number of
    operations (an index lookup in a static array).
  - Small strings get packed into a u64 binary value, with a compiled match
    table. This makes for extremely fast lookup on 64-bit hardware.
  - Medium strings are packed into a u128, which processes well on SIMD capable
    hardware (most modern CPUs).
  - Large (16 chars or more) strings are packed into u128 chunks, the slowest,
    but also likely the most infrequent keywords in your corpus (unless you're
    parsing Java).
- Atoms (grouped by length) are also assessed for ASCII compatibility. If the
  group is ASCII alphanumeric compatible, extremely cheap case folding is done,
  otherwise a slightly slower (but still very vast) ASCII case folding is done,
  otherwise it resorts to unicode case folding. The detection is done at compile
  time meaning

The csskit workspace uses the macro for its full CSS atom set. See
[`CssAtomSet`] for a large real-world example
and the [crate documentation] for API details.

[`CssAtomSet`]: ../css_ast/src/css_atom_set.rs
[crate documentation]: https://csskit.rs/docs/internal/derive_atom_set/

## Part of csskit

This crate is part of [csskit], a comprehensive CSS tool chain.

[csskit]: https://csskit.rs/

## License

MIT
