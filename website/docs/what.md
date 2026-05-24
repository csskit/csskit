---
layout: markdown-base
permalink: /docs/
---

# What is csskit?

csskit is a program aimed at developers to make writing CSS more fun,
productive, and easier. It's a "toolchain" - a collection of different tools
bundled into one:

<ul>
 <li>
  <div class="icon-wrap" style="color:var(--gold-5);background:light-dark(var(--gold-1),var(--gold-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="none">
    <path d="m16 22-1-4"/><path d="M19 14a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2h-3a1 1 0 0 1-1-1V4a2 2 0 0 0-4 0v5a1 1 0 0 1-1 1H6a2 2 0 0 0-2 2v1a1 1 0 0 0 1 1"/><path d="M19 14H5l-1.973 6.767A1 1 0 0 0 4 22h16a1 1 0 0 0 .973-1.233z"/><path d="m8 22 1-4"/>
   </svg>
  </div>
  <span>
   <a href="/docs/fmt">Formatting</a>: keeps your code well organized with consistent style - tabs vs spaces, quote style, color syntax.
 </span>
 </li>
 <li>
  <div class="icon-wrap" style="color:var(--stone-7);background:light-dark(var(--stone-1),var(--stone-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="none">
    <path d="m8 11 2 2 4-4"/><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>
   </svg>
  </div>
  <span>
  <a href="/docs/lint">Linting</a>: finds mistakes in your CSS and suggests corrections, like misspelled selectors or properties.
 </span>
 </li>
 <li>
  <div class="icon-wrap" style="color:var(--yellow-4);background:light-dark(var(--yellow-1),var(--yellow-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="none">
    <path d="M4 14a1 1 0 0 1-.78-1.63l9.9-10.2a.5.5 0 0 1 .86.46l-1.92 6.02A1 1 0 0 0 13 10h7a1 1 0 0 1 .78 1.63l-9.9 10.2a.5.5 0 0 1-.86-.46l1.92-6.02A1 1 0 0 0 11 14z"/>
   </svg>
  </div>
  <span>
  <a href="/docs/min">Minifier</a>: write CSS with generous whitespace and comments, then compress it for web delivery with `csskit min`.
 </span>
 </li>
 <li>
  <div class="icon-wrap" style="color:var(--teal-3);background:light-dark(var(--teal-1),var(--teal-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
    <path d="M12 6v6l2 1"/><path d="M12.337 21.994a10 10 0 1 1 9.588-8.767"/><path d="m14 18 4 4 4-4"/><path d="M18 14v8"/>
   </svg>
  </div>
  <span>
  <a href="/docs/transform">Transpiler</a>: converts modern CSS syntax to ensure browser compatibility.
 </span>
 </li>
 <li>
  <div class="icon-wrap" style="color:var(--orange-3);background:light-dark(var(--orange-1),var(--orange-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="none">
    <path d="M11 21.73a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73z"/><path d="M12 22V12"/><polyline points="3.29 7 12 12 20.71 7"/><path d="m7.5 4.27 9 5.15"/>
   </svg>
  </div>
  <span>
  <a href="/docs/bundle">Bundler</a>: combines multiple CSS files into optimized bundles.
  </span>
 </li>
 <li>
  <div class="icon-wrap" style="color:var(--blue-3);background:light-dark(var(--blue-1),var(--blue-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="none">
    <path d="M6 18h8"/><path d="M3 22h18"/><path d="M14 22a7 7 0 1 0 0-14h-1"/><path d="M9 14h2"/><path d="M9 12a2 2 0 0 1-2-2V6h6v4a2 2 0 0 1-2 2Z"/><path d="M12 6V3a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v3"/>
   </svg>
  </div>
  <span>
  <a href="/docs/analyze">Analyzer</a>: queries your CSS to summarize colors, suggest complementary palettes, and identify browser compatibility issues.
  </span>
 </li>
 <li>
  <div class="icon-wrap" style="color:var(--blue-5);background:light-dark(var(--blue-1),var(--blue-10))">
   <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentcolor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" role="none" role="none">
    <rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.01"/><path d="M10 8h.01"/><path d="M14 8h.01"/>
   </svg>
  </div>
  <span>
  <a href="/docs/lsp">LSP</a>: integrates with
  <a href="/docs/lsp/zed">Zed</a>, <a href="/docs/lsp/vscode">VSCode</a>, and <a href="/docs/lsp/neovim">Neovim</a> for real-time guidance and code completion.
  </span>
 </li>
</ul>

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
