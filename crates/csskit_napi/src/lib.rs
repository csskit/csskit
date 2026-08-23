//! Native bindings that give Node access to the csskit CSS AST object model.
//!
//! [`bindings`] is the napi-rs surface. [`ast`] is the Rust half: the owned parse that the bindings
//! wrap, and the selector queries over it. The parse is self-referential (the root borrows the arena
//! and the source that the same value owns); `self_cell` holds it, thus this crate needs no `unsafe`.
//!
//! Built only with the `napi` feature, so the default workspace build links without a Node runtime.
//! `src/bin/generate_node_classes.rs` needs no part of this, and reads `css_ast` directly.
#![cfg(feature = "napi")]

// The self-referential parse lives here; `self_cell` holds it, thus no `unsafe` is needed. The
// napi-rs macros generate their own `unsafe` glue, thus the ban is on this module, not the crate.
#[forbid(unsafe_code)]
mod ast;
mod bindings;
