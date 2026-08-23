//! Native bindings that give Node access to the csskit CSS AST object model.
#![cfg(feature = "napi")]

#[forbid(unsafe_code)]
mod ast;
#[cfg_attr(test, allow(dead_code, reason = "N-API exports are used by Node, not Rust tests"))]
mod bindings;
