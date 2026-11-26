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

[unicase]: https://crates.io/crates/unicase

`derive_atom_set` expects you to enumerate all keywords upfront, in an enum,
rather than a map. Knowing every keyword ahead of time lets it make some
assumptions about the data and really quickly match an Atom, making for wicked
fast string matching.

Matching is ASCII case-insensitive. For case-sensitive languages (e.g. XML
names) add `#[atom_set(case_sensitive)]` to the enum to match bytes exactly.

## Usage

The macro implements the `AtomSet` trait from the [`atom_set`] crate,
generating implementations bespoke to your enum's discriminants:

```rust
use atom_set::AtomSet;
use derive_atom_set::AtomSet as DeriveAtomSet;

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

```rust,ignore
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
Atoms that are equal are a compile error.

`as_bits()` values are enum discriminants. Do not treat them as a stable
serialization format unless the enum assigns and maintains explicit
discriminants.

## How matching works

`derive_atom_set` builds its lookup tables at compile time, aiming for a few
nanoseconds per lookup against 1000s of atoms, whether or not the string is an
atom:

- All atoms are bucketed by their length, so a lookup only considers atoms of
  the input's length, and a string with an un-bucketed length is discarded
  straight away.
- Single letter keywords use a lookup table (an index into a static array).
- Longer strings are packed into integers: a u64 up to 8 bytes, and 16 byte
  u128 chunks beyond that. Unless `case_sensitive`, packed keys are ASCII case
  folded. If every atom in a length bucket is simple enough (`is_fold_safe`)
  then a cheap fold is used, otherwise the bucket uses a more complex folding
  technique.
- Each length bucket gets a perfect hash table, found at compile time by "hash
  and displace": a multiply hashes the packed key, the top bits of the hash
  select a displacement, and XORing the displacement into the low bits selects
  the only slot the key can occupy. A lookup is one hash, one displacement load
  and one key comparison, with no branching on the key itself.

The csskit workspace uses the macro for its full CSS atom set. See
[`CssAtomSet`] for a large real-world example
and the [crate documentation] for API details.

[`CssAtomSet`]: ../css_ast/src/css_atom_set.rs
[`atom_set`]: https://csskit.rs/docs/internal/atom_set/
[crate documentation]: https://csskit.rs/docs/internal/derive_atom_set/

## Part of csskit

This crate is part of [csskit], a comprehensive CSS tool chain.

[csskit]: https://csskit.rs/

## License

MIT
