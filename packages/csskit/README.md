# csskit

csskit is a suite of high performance tools for CSS, written in Rust.

## Goals

The goal of this project is to provide a high quality set of tools for writing
native CSS and shipping said CSS into production the best way possible.
This means:

- Preventing mistakes at author time (parsing & linting).
- Advising best practices and highlighting pitfalls (linting).
- Allow consistent homogeneous code to be written (formatting).
- Allowing the authorship of modern CSS that is downsampled for browser support ("transpiling").
- Provides integration with IDEs (LSP).
- Producing the smallest available artefacts (minification & bundling).
- Provide a way for authors to migrate from alternative authoring formats (such as SCSS).

## Usage

### Binary

When installed, `csskit` will be available as a binary. Run `csskit --help` to
see how to use it, with full instructions and examples.

### Library

csskit can also be used as a library, for creating custom scripts:

```js
import { parse, StyleRule, StyleSheet } from "csskit";

const sheet = parse("a { color: red }");
sheet instanceof StyleSheet; // true

for (const rule of sheet.querySelectorAll("style-rule")) {
 console.log(rule.constructor.name, rule.text); // StyleRule a { color: red }
}
```

Every AST node kind has its own class, similar to way the DOM has
`HTMLDivElement`. Each one has a `parse` method. This allows you to parse
individual grammars, for example:

```js
import { parse, Color, MediaRule, WidthStyleValue } from "csskit/nodes";

Color.parse("#ff0000");
WidthStyleValue.parse("100px");
parse("@media print { a { color: red } }", { context: MediaRule });
```

#### Client Side

While the main entry is a native addon, therefore only possible to use in Node,
a simplified WASM API is available for client side code:

```js
import { minify } from "csskit/bundle";
const min = minify("a{color:#ff0000}");
console.assert(min === "a{color:red}");
```

The WASM build exposes `lex`, `minify`, `format` and `parseErrorReport`, but no
object model, so classes like `Color` are not available.

Visit the [Getting started guide](https://csskit.rs/docs/getting-started/) for more detail.
