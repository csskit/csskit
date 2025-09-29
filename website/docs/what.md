---
layout: markdown-base
permalink: /docs/
---

# What is csskit?

csskit is a program aimed at developers to make writing CSS more fun,
productive, and easier. It's a "toolchain" - a collection of different tools
bundled into one:

- <b>💅</b> <a href="/docs/fmt">Formatting</a>: keeps your code well organized
  with consistent style - tabs vs spaces, quote style, color syntax.
- <b>🔍</b> <a href="/docs/lint">Linting</a>: finds mistakes in your CSS and
  suggests corrections, like misspelled selectors or properties.
- <b>⚡</b> <a href="/docs/min">Minifier</a>: write CSS with generous whitespace
  and comments, then compress it for web delivery with `csskit min`.
- <b>🔄</b> <a href="/docs/transform">Transpiler</a>: converts modern CSS syntax
  to ensure browser compatibility.
- <b>📦</b> <a href="/docs/bundle">Bundler</a>: combines multiple CSS files into
  optimized bundles.
- <b>🔬</b> <a href="/docs/analyze">Analyzer</a>: queries your CSS to summarize
  colors, suggest complementary palettes, and identify browser compatibility
  issues.
- <b>💻</b> <a href="/docs/lsp">LSP</a>: integrates with
  <a href="/docs/lsp/zed">Zed</a> and <a href="/docs/lsp/vscode">VSCode</a> for
  real-time guidance and code completion.

## Philosophy

The philosophy behind csskit is to be "CSS's missing toolchain". Like Rust, Go,
or Deno, csskit provides almost everything you need without reaching into
external ecosystems. Just as Rust uses `cargo test` and `cargo fmt`, csskit aims
for similar simplicity.

Compare this to JavaScript's fragmented ecosystem, where you might encounter
Jest, Vitest, or Mocha for testing, combined with npm, Yarn, or pnpm package
managers. This creates barriers when moving between projects and forces teams
to spend time configuring tools instead of building.

csskit brings the ergonomic benefits of Rust and Go toolchains to CSS - an
underinvested area. While CSS has excellent tools, they're often fragmented and
difficult to configure, making deployment unnecessarily complex.

## How does it compare?

> [!WARNING]
> csskit is alpha software - great for experimenting but may have bugs in
> production. Choose battle-tested tools for production code.

### [postcss](https://postcss.org/) & [cssnano](https://cssnano.github.io/cssnano/)

[postcss](https://postcss.org/) has over a decade of development.
[cssnano](https://cssnano.github.io/cssnano/) builds on postcss for low-config
CSS transformation and minification.

Both focus on transforming CSS but can't lint. PostCSS's plugin system offers
flexibility but requires configuration time. The JavaScript plugin system
enables experimental features that keep it ahead of the curve.

csskit aims to handle most postcss use cases while always targeting
browser-supported CSS. For experimental features no browsers support, postcss
remains the best choice.

So pick postcss or cssnano if:

- <b>🪨</b> You want something rock solid and well tested.
- <b>🧪</b> You want to build on top of more experimental features before the
  browsers even get them.
- <b>🐌</b> You don't mind waiting a few seconds for your css to build.

### [lightningcss](https://lightningcss.dev/) or [Parcel](https://parceljs.org/)

[lightningcss](https://lightningcss.dev/) is a blazing fast CSS
minifier/transpiler that replaces cssnano or postcss. It powers
[Parcel](https://parceljs.org/)'s CSS minifier.

While it lacks postcss's experimental features, lightningcss excels for most
developers due to its speed. Written in Rust, it uses the excellent
[cssparser](https://crates.io/crates/cssparser) library that also powers
[Servo](https://github.com/servo/servo/) and
[Firefox](https://www.firefox.com/).

csskit is also written in Rust, and is very close to lightningcss's speed (often
a little faster) but minifies less effectively due to being newer. Both are
much faster than postcss.

[Read the benchmarks for more details on csskit's performance](/benchmarks).

So pick lightningcss or parcel if:

- <b>🚄</b> You want best in class performance.
- <b>💁</b> You're less interested in the other features, like formatting or
  IDE integration.

#### [prettier](https://prettier.io/)

[Prettier](https://prettier.io/) is an "opinionated code formatter" and an
industry staple. It works well for lots of different languages, and CSS is no
exception. If you're using Prettier to format other files in your project
there's a good chance you'll want to keep using it for css.

csskit aims to format CSS faster and better than Prettier. Its deeper CSS
knowledge should enable better formatting decisions, though this isn't yet
realized in the current experimental version.

So pick Prettier if:

- <b>🔨</b> You're already using it for CSS formatting without issues.

#### [stylelint](https://stylelint.io/)

Stylelint is a powerful CSS linter with extensive plugins - currently the only
real choice for CSS linting. Its JavaScript API enables extensibility.

csskit aims to match Stylelint's error detection capabilities with some
extensibility for custom rules (though not via JavaScript API). Currently,
Stylelint remains the better choice.

- <b>🔨</b> You're already using it to lint your code.
- <b>🧪</b> You want to build a set of custom lint rules using a JavaScript API.
- <b>🐌</b> You don't mind waiting a few seconds for it to run.
