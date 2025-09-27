#![deny(warnings)]
//! An implementation of [CSS Syntax Level 3][1], plus various additional traits and macros to assist in parsing. It is
//! intended to be used to build CSS or CSS-alike languages (for example SASS), but isn't able to parse the full CSS
//! grammar itself. It relies on the foundational [css_lexer] crate.
//!
//! This crate provides the [Parser] struct, which builds upon [Lexer][css_lexer::Lexer]. It borrows a `&str` which it
//! will parse to produce AST nodes (any type that implements the [Parse] and [ToCursors] traits). AST nodes should
//! parse themselves and any children using [recursive descent][2].
//!
//! [1]: https://drafts.csswg.org/css-syntax-3/
//! [2]: https://en.wikipedia.org/wiki/Recursive_descent_parser
//!
//! Parsing requires a heap allocator to allocate into, [bumpalo::Bump] being the allocator of choice. This needs to be
//! created before parsing, the parser result will have a lifetime bound to the allocator.
//!
//! The [Parser] _may_ be configured with additional [Features][Feature] to allow for different parsing or lexing
//! styles. All features supported by the [Lexer][css_lexer::Lexer] are supported in the [Parser] also (for example
//! enabling [Feature::SingleLineComments] will enable [the css_lexer feature of the same
//! name][css_lexer::Feature::SingleLineComments]).
//!
//! This crate provides some low level AST nodes that are likely to be common in any CSS-alike language, including the
//! various base tokens (such as dimensions, and operators). These can be referred to via the [T!] macro, and each [T!]
//! implements the necessary traits to be parsed as an AST node. For example [T![DashedIdent]][token_macros::DashedIdent]
//! represents a CSS ident with two leading dashes, and can be parsed and decomposted into its constituent
//! [Token][Token] (or [Cursor][Cursor] or [Span][Span]).
//!
//! Additionally some generic structs are available to implement the general-purpose parts of [CSS Syntax][1], such as
//! [ComponentValues][syntax::ComponentValues]. More on that below in the section titled
//! [Generic AST Nodes](#generic-ast-nodes).
//!
//! Lastly, traits and macros are provided to implement various parsing algorithms to make common parsing operations
//! easier, for example the [ranged_feature] macro makes it easy to build a node that implements the [RangedFeature]
//! trait, a trait that provides [an algorithm for parsing a media feature in a range context][3].
//!
//! [3]: https://drafts.csswg.org/mediaqueries/#range-context
//!
//! Downstream implementations will likely want to build their own AST nodes to represent specific cover grammars, for
//! example implementing the `@property` rule or the `width:` property declaration. Here's a small guide on what is
//! required to build such nodes:
//!
//! # AST Nodes
//!
//! To use this as a library a set of AST nodes will need to be created, the root node (and ideally all nodes) need to
//! implement [Parse] - which will be given a mutable reference to an active [Parser]. Each Node will likely be a
//! collection of other Nodes, calling [Parser::parse&lt;T>()][Parser::parse] (where `T` is each child Node). Leaf Nodes will likely be
//! wrappers around a single token (tip: use the [T!] nodes which cover all single token needs):
//!
//! ```
//! use css_parse::*;
//! struct MyProperty {
//!   ident: T![Ident],
//!   colon: T![Colon],
//!   dimension: T![Dimension],
//! }
//! impl<'a> Parse<'a> for MyProperty {
//!   fn parse(p: &mut Parser<'a>) -> Result<Self> {
//!     let ident = p.parse::<T![Ident]>()?;
//!     let colon = p.parse::<T![Colon]>()?;
//!     let dimension = p.parse::<T![Dimension]>()?;
//!     Ok(Self { ident, colon, dimension })
//!   }
//! }
//! ```
//!
//! AST nodes will also need to implement [ToCursors] - which is given an abstract [CursorSink] to put the cursors back
//! into, in order, so that they can be built back up into the original source text. Implementing [ToCursors] allows
//! for all manner of other useful downstream operations such as concatenation, transforms (e.g. minification) and so
//! on.
//!
//! ```
//! use css_parse::*;
//! struct MyProperty {
//!   ident: T![Ident],
//!   colon: T![Colon],
//!   dimension: T![Dimension],
//! }
//! impl ToCursors for MyProperty {
//!   fn to_cursors(&self, s: &mut impl CursorSink) {
//!     s.append(self.ident.into());
//!     s.append(self.colon.into());
//!     s.append(self.dimension.into());
//!   }
//! }
//! ```
//!
//! Both [Parse] and [ToCursors] are the _required_ trait implemenetations, but several more are also available and make
//! the work of Parsing (or downstream analysis) easier...
//!
//! ## Peekable nodes
//!
//! Everything that implements [Parse] is required to implement [Parse::parse()], but gets [Parse::try_parse()] for
//! free, which allows parent nodes to more easily branch by parsing a node, resetting during failure.
//! [Parse::try_parse()] can be expensive though - parsing a Node is pretty much guaranteed to advance the [Parser]
//! some number of tokens forward, and so a parser checkpoint needs to be stored so that - should
//! [Parse::parse()] fail - the [Parser] can be rewound to that checkpoint as if the operation never happened. Reading
//! N tokens forward only to forget that and re-do it all over can be costly and is likely the _wrong tool_ to use when
//! faced with a set of branching Nodes with an ambiguity of which to parse. So Nodes are also encouraged to implement
//! [Peek], which their parent nodes can call to check as an indicator that this Node may viably parse.
//!
//! Most nodes will know they can only accept a certain number of tokens, per their cover grammar. [Peek] is a useful
//! way to encode this; [Peek::peek] gets an _immutable_ reference to the [Parser], from which it can call
//! [Parser::peek_n()] (an immutable operation that can't change the position of the parser) to look ahead to other
//! tokens and establish if they would cause [Parse::parse()] to fail. There is still a cost to this, and so
//! [Peek::peek] should only look ahead the smallest number of tokens to confidently know that it can begin parsing,
//! rather than looking ahead a large number of tokens. For the most part peeking 1 or two tokens should be sufficient.
//! An easy implementation for [Peek] is to simply set the [Peek::PEEK_KINDSET] const, which the provided
//! implementation of [Peek::peek()] will use to check the cursor matches this [KindSet][KindSet].
//!
//! ```
//! use css_parse::*;
//! use {Kind, KindSet};
//! enum LengthOrAuto {
//!   Length(T![Dimension]), // A Dimension, like `px`
//!   Auto(T![Ident]),       // The Ident of `auto`
//! }
//! impl<'a> Peek<'a> for LengthOrAuto {
//!   const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Dimension, Kind::Ident]);
//! }
//! ```
//!
//! ## Single token Nodes
//!
//! If a node represents just a single token, for example a keyword, then its [Parse] implementation
//! should call [Parser::peek] to check if it can be parsed, then [Parser::next] to get the cursor, and construct
//! the node from that cursor. The [Peek] trait should accurately determine if the Node can be parsed from the
//! given [Cursor][Cursor]. Single token parsing may need to branch if it is an enum of variants:
//!
//! ```
//! use css_parse::*;
//! enum LengthOrAuto {
//!   Length(T![Dimension]), // A Dimension, like `px`
//!   Auto(T![Ident]),       // The Ident of `auto`
//! }
//! impl<'a> Peek<'a> for LengthOrAuto {
//!   const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Dimension, Kind::Ident]);
//! }
//! impl<'a> Parse<'a> for LengthOrAuto {
//!   fn parse(p: &mut Parser<'a>) -> Result<Self> {
//!     if p.peek::<T![Dimension]>() {
//!       p.parse::<T![Dimension]>().map(Self::Length)
//!     } else {
//!       p.parse::<T![Ident]>().map(Self::Auto)
//!     }
//!   }
//! }
//! ```
//!
//! ## Convenience algorithms
//!
//! For more complex algorithms where nodes might parse many child nodes or have some delicate or otherwise awkward
//! steps, additional traits exist to make implementing AST nodes trivial for these use cases.
//!
//! - [StyleSheet] - AST nodes representing a stylesheet should use this to, well, [parse a stylesheet][4].
//! - [Declaration] - AST nodes representing a declaration (aka "property") should use this to [parse a
//!   declaration][5].
//! - [QualifiedRule] - AST nodes representing a "Qualified Rule" (e.g. a style rule) should use this to
//!   [parse a QualifiedRule][7].
//! - [CompoundSelector] - AST nodes representing a CSS selector should use this to parse  a list of nodes implementing
//!   [SelectorComponent].
//! - [SelectorComponent] - AST nodes representing an individual selector component, such as a tag or class or pseudo
//!   element, should use this to parse the set of specified selector components.
//!
//! The `*List` traits are also available to more easily parse lists of things, such as preludes or blocks:
//!
//! - [PreludeList] - AST nodes representing a rule's prelude should use this. It simply repeatedly parses its items
//!   until it enounters the start of a block (<{-token> or <;-token>).
//! - [FeatureConditionList] - AST nodes representing a prelude "condition list" should use this. It parses the complex
//!   condition logic in rules like `@media`, `@supports` or `@container`.
//! - [DeclarationList] - AST nodes representing a block which can only accept "Declarations" should use this. This is
//!   an implementation of [`<declaration-list>`][8].
//! - [DeclarationRuleList] - AST nodes representing a block which can accept either "At Rules" or "Declarations" but
//!   cannot accept "Qualified Rules" should use this. This is an implementation of [`<declaration-rule-list>`][11]
//! - [RuleList] - AST nodes representing a block which can accept either "At Rules" or "Qualfiied Rules" but cannot
//!   accept "Declarations" should use this. This is an implementation of [`<rule-list>`][12].
//!
//! The `*Feature` traits are also available to more easily parse "features conditions", these are the conditions
//! supports in a [FeatureConditionList], e.g. the conditions inside of `@media`, `@container` or `@supports` rules.
//!
//!  - [RangedFeature] - AST nodes representing a feature condition in the "ranged" context.
//!  - [BooleanFeature] - AST nodes representing a feature condition in the "boolean" context.
//!  - [DiscreteFeature] - AST nodes representing a feature condition with discrete keywords.
//!
//! [4]: https://drafts.csswg.org/css-syntax-3/#consume-stylesheet-contents
//! [5]: https://drafts.csswg.org/css-syntax-3/#consume-declaration
//! [6]: https://drafts.csswg.org/css-syntax-3/#consume-at-rule
//! [7]: https://drafts.csswg.org/css-syntax-3/#consume-qualified-rule
//! [8]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
//! [9]: https://drafts.csswg.org/css-syntax-3/#typedef-qualified-rule-list
//! [10]: https://drafts.csswg.org/css-syntax-3/#typedef-at-rule-list
//! [11]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-rule-list
//! [12]: https://drafts.csswg.org/css-syntax-3/#typedef-rule-list
//!
//! # Generic AST nodes
//!
//! In addition to the traits which allow for parsing bespoke AST Nodes, this crate provides a set of generic AST node
//! structs/enums which are capable of providing "general purpose" AST nodes, useful for when an AST node fails to parse
//! and needs to consume some tokens in a generic manner, according to the rules of :
//!
//!  - [syntax::QualifiedRule] provides the generic [`<qualified-rule>` grammar][14].
//!  - [syntax::Declaration] provides the generic [`<declaration>` grammar][15].
//!  - [syntax::BangImportant] provides the [`<!important>` grammar][16].
//!  - [syntax::ComponentValue] provides the [`<component-value>` grammar][17], used by other generic nodes.
//!  - [syntax::SimpleBlock] provides the generic [`<simple-block>` grammar][18].
//!  - [syntax::FunctionBlock] provides the generic [`<function-block>` grammar][19].
//!  - [syntax::ComponentValues] provides a list of `<component-value>` nodes, [per "parse a list of component
//!    values"][20].
//!  - [syntax::BadDeclaration] provides a struct to capture the [bad declaration steps][21].
//!
//! [13]: https://drafts.csswg.org/css-syntax-3/#at-rule-diagram
//! [14]: https://drafts.csswg.org/css-syntax-3/#qualified-rule-diagram
//! [15]: https://drafts.csswg.org/css-syntax-3/#declaration-diagram
//! [16]: https://drafts.csswg.org/css-syntax-3/#!important-diagram
//! [17]: https://drafts.csswg.org/css-syntax-3/#component-value-diagram
//! [18]: https://drafts.csswg.org/css-syntax-3/#simple-block-diagram
//! [19]: https://drafts.csswg.org/css-syntax-3/#function-block-diagram
//! [20]: https://drafts.csswg.org/css-syntax-3/#parse-list-of-component-values
//! [21]: https://drafts.csswg.org/css-syntax-3/#consume-the-remnants-of-a-bad-declaration
//!
//! # Test Helpers
//!
//! In order to make it much easier to test the functionality of AST nodes, enabling the `testing` feature will provide
//! two testing macros which make setting up a test trivial.
//!
//! - [assert_parse!] will parse the given string against the given node, asserting that it parses successfully and can
//!   be written back out to the same output.
//!
//! - [assert_parse_error!] will parse the given string against the node, expecting the parse to fail.
//!
//! It is advised to add the `testing` flag as a `dev-dependencies` feature to enable these only during test:
//!
//! ```toml
//! [dependencies]
//! css_parse = "*"
//!
//! [dev-dependencies]
//! css_parse = { version = "*", features = ["testing"] }
//! ```
//!
//! # Example
//!
//! A small example on how to define an AST node:
//!
//! ```
//! use css_parse::*;
//! #[derive(Debug)]
//! struct MyProperty {
//!   ident: T![Ident],
//!   colon: T![Colon],
//!   dimension: T![Dimension],
//! }
//! impl<'a> Parse<'a> for MyProperty {
//!   fn parse(p: &mut Parser<'a>) -> Result<Self> {
//!     let ident = p.parse::<T![Ident]>()?;
//!     let colon = p.parse::<T![Colon]>()?;
//!     let dimension = p.parse::<T![Dimension]>()?;
//!     Ok(Self { ident, colon, dimension })
//!   }
//! }
//! impl ToCursors for MyProperty {
//!   fn to_cursors(&self, s: &mut impl CursorSink) {
//!     self.ident.to_cursors(s);
//!     self.colon.to_cursors(s);
//!     self.dimension.to_cursors(s);
//!   }
//! }
//!
//! assert_parse!(EmptyAtomSet::ATOMS, MyProperty, "width:1px");
//! ```

// Re-export commonly used components from css_lexer:
pub use css_lexer::{
	AssociatedWhitespaceRules, AtomSet, Cursor, EmptyAtomSet, Kind, KindSet, PairWise, QuoteStyle, SourceCursor,
	SourceOffset, Span, ToSpan, Token, Whitespace,
};

mod comparison;
mod cursor_compact_write_sink;
mod cursor_interleave_sink;
mod cursor_overlay_sink;
mod cursor_pretty_write_sink;
mod cursor_write_sink;
mod diagnostics;
mod feature;
mod macros;
mod parser;
mod parser_checkpoint;
mod parser_return;
/// Various structs/enums that represent generic AST nodes.
pub mod syntax;
/// Test macros available if built with `features = ["testing"]`
#[cfg(any(feature = "testing", test))]
pub mod test_helpers;
/// Various macros that expand to AST nodes that wrap [Tokens][Token].
pub mod token_macros;
mod traits;

pub type Result<T> = std::result::Result<T, diagnostics::Diagnostic>;

pub use comparison::*;
pub use cursor_compact_write_sink::*;
pub use cursor_interleave_sink::*;
pub use cursor_overlay_sink::*;
pub use cursor_pretty_write_sink::*;
pub use cursor_write_sink::*;
pub use diagnostics::*;
pub use feature::*;
pub use macros::optionals::*;
#[cfg(feature = "miette")]
pub use miette::Error;
pub use parser::*;
pub use parser_checkpoint::*;
pub use parser_return::*;
pub use syntax::*;
pub use traits::*;
