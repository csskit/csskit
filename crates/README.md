# Crates

This directory contains all the Rust crates that make up the csskit project. Each crate serves a specific purpose in
the CSS parsing, AST generation, and tooling ecosystem. The modular architecture allows for flexible usage - components
can be used independently or as part of the complete csskit toolchain.

## Architecture Overview

The crates follow a layered architecture:

1. **Lexing Layer**: `css_lexer` - Tokenization of CSS source
2. **Parsing Layer**: `css_parse` - Grammar-based parsing into ASTs
3. **AST Layer**: `css_ast` - Type-safe representation of CSS constructs
4. **Tool Layer**: Various specialized tools and integrations
5. **Application Layer**: `csskit` - CLI and high-level APIs

## Core Library Crates

- [`csskit`](./csskit/) - Main CLI application and entry point ([docs](https://csskit.rs/docs/internal/csskit/))
- [`css_lexer`](./css_lexer/) - CSS/CSS-alike tokenizer/lexer for breaking CSS into tokens ([docs](https://csskit.rs/docs/internal/css_lexer/))
- [`css_parse`](./css_parse/) - CSS/CSS-alike parser that builds on the lexer to create parse trees ([docs](https://csskit.rs/docs/internal/css_parse/))
- [`css_ast`](./css_ast/) - Abstract Syntax Tree definitions for the CSS language ([docs](https://csskit.rs/docs/internal/css_ast/))

## Tool & Extension Crates

- [`csskit_lsp`](./csskit_lsp/) - [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) implementation for csskit ([docs](https://csskit.rs/docs/internal/csskit_lsp/))
- [`csskit_highlight`](./csskit_highlight/) - Syntax highlighting utilities ([docs](https://csskit.rs/docs/internal/csskit_highlight/))
- [`csskit_transform`](./csskit_transform/) - CSS transformation and optimization utilities ([docs](https://csskit.rs/docs/internal/csskit_transform/))
- [`csskit_wasm`](./csskit_wasm/) - WebAssembly bindings for browser/Node/Deno ([docs](https://csskit.rs/docs/internal/csskit_wasm/))
- [`chromashift`](./chromashift/) - Color format conversion library ([docs](https://csskit.rs/docs/internal/chromashift/))
- [`css_feature_data`](./css_feature_data/) - CSS feature compatibility and browser support data, utilising ["Web-Features"](https://web-platform-dx.github.io/web-features/) ([docs](https://csskit.rs/docs/internal/css_feature_data/))

## Support & Utility Crates

- [`csskit_source_finder`](./csskit_source_finder/) - Tools to search the csskit source for codegen ([docs](https://csskit.rs/docs/internal/csskit_source_finder/))
- [`csskit_derives`](./csskit_derives/) - Derive macros for CSS-related types ([docs](https://csskit.rs/docs/internal/csskit_derives/))
- [`csskit_proc_macro`](./csskit_proc_macro/) - Procedural macros for code generation ([docs](https://csskit.rs/docs/internal/csskit_proc_macro/))
