# source_tools

Shared source locations and generic token cursors for parsers, lexers, and other
source-processing tools.

`source_tools` provides the small, language-neutral types needed to connect
tokens to UTF-8 source text. Token meaning, parsing, escaping, and formatting
remain in each language implementation.

## Types

- `SourceOffset`: a byte offset stored as a `u32`.
- `Span`: a half-open byte range, `[start, end)`.
- `LineIndex`: precomputed line starts for repeated line and column lookup.
- `ToSpan`: a trait for values that can return their source span.
- `SourceToken`: the minimal contract required by a generic cursor.
- `Cursor<T>`: a token and its offset in source text.
- `SourceCursor<'a, T>`: a cursor paired with the source text it covers.

Offsets and span lengths are bytes, not Unicode scalar values or UTF-16 code
units. Sources must fit within `u32::MAX` bytes.

## Example

```rust
use source_tools::{Cursor, SourceCursor, SourceOffset, SourceToken, Span};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Kind {
    Word,
    Eof,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Token {
    kind: Kind,
    len: u32,
}

impl SourceToken for Token {
    type Kind = Kind;

    const EMPTY: Self = Self { kind: Kind::Eof, len: 0 };

    fn kind(self) -> Kind {
        self.kind
    }

    fn len(self) -> u32 {
        self.len
    }

    fn kind_name(self) -> &'static str {
        match self.kind {
            Kind::Word => "Word",
            Kind::Eof => "Eof",
        }
    }
}

let token = Token { kind: Kind::Word, len: 3 };
let cursor = Cursor::new(SourceOffset(4), token);

assert_eq!(cursor.kind(), Kind::Word);
assert_eq!(cursor.span(), Span::new(SourceOffset(4), SourceOffset(7)));
assert_eq!(cursor.str_slice("one two"), "two");

let source_cursor = SourceCursor::from(cursor, "two");
assert_eq!(source_cursor.source(), "two");
assert_eq!(source_cursor.cursor(), cursor);
```

## Line and column lookup

`Span::line_and_column` scans from the start of a source. Build a `LineIndex`
when resolving several spans from the same source:

```rust
use source_tools::{LineIndex, SourceOffset, Span};

let source = "one\ntwo\nthree";
let index = LineIndex::new(source);
let span = Span::new(SourceOffset(8), SourceOffset(13));

assert_eq!(index.line_and_column(span), (2, 0));
```

Lines and columns are zero-based. Columns count Unicode scalar values from the
start of the line.

## Features

The crate has no default features.

- `miette`: conversions from `SourceOffset`, `Span`, and `Cursor<T>` to miette
  source-location types.
- `serde`: serialization for `SourceOffset`, `Span`, and `Cursor<T>`.
- `csskit_arena`: `ToSpan` impls for the `csskit_arena` `Box` and `Vec`
  collections.

## Design boundary

This crate owns source coordinates and generic storage only. Language crates
remain responsible for concrete token representations, decoded values,
comparison helpers, source reconstruction, minification, and pretty-printing.

## Part of csskit

This crate is part of [csskit], a comprehensive CSS tool chain.

[csskit]: https://csskit.rs/

## License

MIT
