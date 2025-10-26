# css_ast

CSS Abstract Syntax Trees with visitable nodes and style value types.

📖 **[Full Documentation](https://csskit.rs/docs/internal/css_ast/)**

## Features

- **Complete AST definitions**: Generated and handwritten node types for all CSS constructs
- **Visitable nodes**: Implement visitor patterns for AST traversal and transformation
- **CSS value types**: Comprehensive support for all CSS value types
- **Property definitions**: Type-safe representations of CSS properties
- **Generated from spec**: Many value definitions automatically generated from CSS specifications
- **Feature gates**: Optional dependencies for chromashift, miette, serde support

## Optional Features

- `visitable` - Enables visitor pattern support for AST traversal
- `chromashift` - Enables color conversion utilities
- `miette` - Enables rich diagnostic integration
- `serde` - Enables serialization/deserialization support

## Part of csskit

This crate is part of the csskit project, a comprehensive CSS toolchain.

For more information, visit [csskit.rs](https://csskit.rs/).

## License

MIT
