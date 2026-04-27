use super::prelude::*;

/// The CSS `auto` keyword as a standalone parseable type.
///
/// Referenced by the proc macro codegen when `auto?` appears as an optional
/// keyword prefix in a sequence grammar, e.g. `auto? [ none | <length> ]`.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Auto(#[atom(CssAtomSet::Auto)] pub T![Ident]);
