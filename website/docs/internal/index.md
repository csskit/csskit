---
layout: markdown-base
---

# Contributing

If you're interested in working _on_ csskit, then this is the right jumping off
point.

## What is csskit written in?

csskit is written in Rust, a systems programming language. It's a little
trickier than JavaScript but it is a lot more powerful due to the ability to
more tightly control how memory is used.

csskit is broken into a set of modules, each doing it's own thing.

- [css_lexer](./css_lexer) is the foundational "lexer" crate which takes a set
  the raw character data from a css sheet and turns them into "tokens".
- [css_parse](./css_parse) builds on top of css_lexer to provide a library for
  parsing CSS-alike languages (CSS, SCSS, and so on).
- [css_ast](./css_ast) builds on top of css_parse to provide all of the
  structured types in the CSS language such as StyleValues, Selectors,
PseudoClasses and so on.
- [csskit_lsp](./csskit_lsp) provides the functionality for the LSP server, for IDE
  integration.
- [csskit_higlight](./csskit_higlight) is a library to provide syntax
  highlighting for languages supported by csskit.
- [css_feature_data](./css_feature_data) is a library which generates Rust data structures
  from the [web-features](https://web-platform-dx.github.io/web-features/) database,
  powering the "Baseline" & browser support data.

There are many other utility modules, so explore the documentation to get a
feel for the project. This contribution section will be expanded over time.
