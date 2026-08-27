# bytescan

This library provides primitives for scanning a range of characters for a
matching sequence of bytes, using fast SIMD techniques, powered by
[fearless_simd]. The ideal use case for this is quickly scanning for keywords,
comments, or strings in a Lexer / Tokenizer.

Let's say you want to quickly scan a string for Idents in your language. Your
language allows idents that are alpha-numeric plus dash, and underscore
(`a-zA-Z0-9_-`).

At first a RegExp might seem reasonable:

```rust,ignore
let ident = Regex::new(r"^[a-zA-Z0-9_-]+").unwrap();
let len = ident.find(input).map_or(0, |m| m.end());
```

This reads well, but it is extremely slow. The pattern compiles to a state
machine walking one `char` at a time, while also dragging in a large dependency.
As the saying goes:

> Some people, when confronted with a problem, think “I know, I’ll use regular
> expressions.” Now they have two problems.

Writing a manual loop & match is significantly faster because the compiler
turns the arms into a couple of comparisons. A little more code to read and
reason about, but far more effective:

```rust
fn scan_ident(bytes: &[u8]) -> usize {
    for (offset, byte) in bytes.iter().enumerate() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' => {}
            _ => return offset,
        }
    }
    bytes.len()
}

assert_eq!(scan_ident(b"name-1:value"), 6);
```

This is fast, but it can be _much_ faster. It still tests one byte per step, but
modern CPUs can test sixteen, thirty-two, or sixty-four bytes in the same step.
This is, of course, with SIMD. Writing out by hand can be laborious and
delicate, and usually means writing out one implementation for each instruction
set. Helper libraries like `fearless_simd` help massively, but it can be made
easer.

`bytescan` tries to provide an ergonomic macro to get closer to RegExp
ergonomics, but with the raw speed of SIMD:

```rust
use bytescan::{byte_class, scan_until};

byte_class! {
    /// Bytes that end an identifier.
    pub struct IdentEnd = !(alpha | b'0'..=b'9' | b'-' | b'_');
}

assert_eq!(scan_until(b"name-1:value", IdentEnd), 6);
```

## Usage

### Byte classes

A "type" of token to scan for is a [`ByteClass`]. This can be declared with the
[`byte_class!`] macro, which can use a `|` separated list of terms to match
against. Each term is one of:

| Term                | Matches                           |
| ------------------- | --------------------------------- |
| `b'x'`, `0x7F`, `0` | that one byte                     |
| `0x80..=0xFF`       | that inclusive range              |
| `alpha`             | ASCII letters of either case      |
| `!(…)`              | every byte outside the nested set |
| a field name        | the byte held in that field       |

Field names can be used to hold data only where the language needs it, such as
the quote delimiter of a string. For example, add one field in the struct to
hold a byte, and use that name as a term:

```rust
use bytescan::{byte_class, scan_until};

byte_class! {
    /// Bytes that end a string body.
    pub struct StringEnd(delimiter) = delimiter | b'\\' | b'\n' | 0x80..=0xFF;
}

assert_eq!(scan_until(b"say 'hi'", StringEnd(b'\'')), 4);
```

Everything stays as a compile-time constant. If you find something that cannot
be expressed with the macro, for example a range with a runtime bound, you can
implement the [`ByteClass`] trait by hand. `mask` is generic over the vector
width, so it is written against the [`SimdInt`] comparisons rather than one
instruction set:

```rust
use bytescan::{ByteClass, ByteVector, Simd, SimdInt, scan_until};

#[derive(Clone, Copy)]
struct AtLeast(u8);

impl ByteClass for AtLeast {
    fn matches(self, byte: u8) -> bool {
        byte >= self.0
    }

    fn mask<S: Simd, V: ByteVector<S>>(self, value: V) -> V::Mask {
        value.simd_ge(self.0)
    }
}

assert_eq!(scan_until(b"abcXYZ", AtLeast(b'a')), 0);
assert_eq!(scan_until(b"XYZabc", AtLeast(b'a')), 3);
```

A hand-written class may also find the [`byte_mask`] and [`range_mask`] helpers
useful for simplifying manual implementations. The macro emits these helpers
for fixed byte and fixed ranges; the `StringEnd` class above expands to this:

```rust
use bytescan::{ByteClass, ByteVector, Simd, byte_mask, range_mask, scan_until};

#[derive(Clone, Copy)]
struct StringEnd(u8);

impl ByteClass for StringEnd {
    fn matches(self, byte: u8) -> bool {
        byte == self.0 || byte == b'\\' || byte == b'\n' || byte >= 0x80
    }

    fn mask<S: Simd, V: ByteVector<S>>(self, value: V) -> V::Mask {
        byte_mask(value, self.0)
            | byte_mask(value, b'\\')
            | byte_mask(value, b'\n')
            | range_mask::<S, V, 0x80, 0xFF>(value)
    }
}

assert_eq!(scan_until(b"say 'hi'", StringEnd(b'\'')), 4);
```

### Scanning

[`scan_until`] returns the offset of the first byte matched by a class.
[`scan_until_and_mark`] also reports whether another class matched any byte
before the stopping offset. [`scan_byte`] is the single-byte form of
[`scan_until`]. All three return `bytes.len()` when nothing matches.

```rust
use bytescan::{byte_class, scan_byte, scan_until_and_mark};

byte_class! {
    pub struct Colon = b':';
}

byte_class! {
    pub struct AsciiUpper = b'A'..=b'Z';
}

assert_eq!(scan_until_and_mark(b"Background:", Colon, AsciiUpper), (10, true));
assert_eq!(scan_byte(b"name:value", b':'), 4);
assert_eq!(scan_byte(b"name", b':'), 4);
```

### Runtime dispatch

SIMD support is detected at runtime through `fearless_simd` and cached by it.
The scan runs 64 bytes per step on AVX-512, 32 on AVX2, and 16 on SSE2 and
NEON. Callers scan without target-specific features, and name SIMD traits only
to write a class by hand.

## Part of csskit

This crate is part of [csskit], a comprehensive CSS tool chain.

[csskit]: https://csskit.rs/
[fearless_simd]: https://docs.rs/fearless_simd/

## License

MIT
