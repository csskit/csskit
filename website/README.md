# Website

Documentation website and project homepage for csskit.

## Purpose

This directory contains the source code and content for the csskit project website, including documentation, examples, and project information.

## Running locally

The build compiles csskit to WebAssembly, which needs the Rust `wasm32-unknown-unknown` target, `wasm-bindgen`, and `wasm-opt` (from binaryen). Exact versions are in `.mise.toml` at the repository root.

With [mise], from the repository root:

```sh
mise install
cd website
npm i
npm start
```

Without mise, install the Node and Rust versions from `.mise.toml`, then:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version <version from .mise.toml>
brew install binaryen # or: apt install binaryen
```

## Structure

- [`docs/`](./docs/) - Documentation content and guides.
- [`css/`](./css/) - Website stylesheets
- [`fonts/`](./fonts/) - Web fonts used by the site
- [`images/`](./images/) - Website images and assets
- [`playground/`](./playground/) - Interactive CSS playground/demo
- [`src/`](./src/) - Website JS source code
- [`_includes/`](./_includes/) - Reusable templates & partials
- [`_layouts/`](./_layouts/) - Page layout templates

## Purpose

The website serves as the main resource for:

- Project documentation and API references
- Getting started guides and tutorials
- Interactive examples and playground
- Community resources and contribution guidelines

[mise]: https://mise.jdx.dev
