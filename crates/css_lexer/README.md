# css_lexer

A spec-compliant CSS tokenizer with zero-copy cursors and optional feature gates.

📖 **[Full Documentation](https://csskit.rs/docs/internal/css_lexer/)**

## Features

- **Spec-compliant**: Fully implements CSS Syntax Module Level 3 tokenization rules
- **Zero-copy cursors**: Efficient token streaming with minimal allocations
- **Atom-set aware**: Optimized lexing for CSS keywords and identifiers
- **Allocation control**: Support for custom allocators via allocator_api2 integration

## Optional Features

- `miette` - Enables `From<>` implementations for miette span types
- `bump` - Enables `From<>` implementations for bump Vec
- `serde` - Enables serialization/deserialization support

## Part of csskit

This crate is part of the csskit project, a comprehensive CSS toolchain.

For more information, visit [csskit.rs](https://csskit.rs/).

## License

MIT
