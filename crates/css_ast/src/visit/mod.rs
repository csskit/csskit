include!(concat!(env!("OUT_DIR"), "/css_node_kind.rs"));
include!(concat!(env!("OUT_DIR"), "/css_apply_visit_methods.rs"));

/// Calls `$callback! { $args @hooks { ... } }` with every [`VisitMut`] hook pair that is not generated from a node type.
/// Each pair is `kind #[visit attrs] visit, #[exit attrs] exit [generics] (node type) [where predicates];`, where `kind`
/// is `scoped` for a pair bracketing a part of the walk and `feature` for the feature pair.
#[doc(hidden)]
#[macro_export]
macro_rules! apply_visit_mut_hooks {
	([$($callback: tt)*] $($args: tt)*) => {
		$($callback)*! {
			$($args)*
			@hooks {
				scoped
				/// Called on entry to a declaration. Return [`VisitFlow::SKIP_CHILDREN`] to leave its value unvisited.
				enter_declaration,
				exit_declaration
				[T: $crate::DeclarationValue<'a, $crate::CssMetadata>] ($crate::Declaration<'a, T, $crate::CssMetadata>) [];
				scoped visit_bad_declaration, exit_bad_declaration [] ($crate::__css_parse::syntax::BadDeclaration<'a>) [];
				scoped visit_string, exit_string [] ($crate::__css_parse::token_macros::String) [];
				scoped visit_comparison, exit_comparison [] ($crate::__css_parse::Comparison) [];
				feature
				/// Called before entering a node whose type has feature metadata. Return [`VisitFlow::STOP`] to halt the
				/// traversal; any other flow enters the node.
				visit_feature,
				/// Called after leaving a node whose type has feature metadata. Return [`VisitFlow::STOP`] to halt the
				/// traversal.
				exit_feature
				[T: $crate::FeatureMetadata] (T) [];
				scoped
				/// Offers a value slot which may hold a math function for replacement, before its children are visited.
				/// Return [`VisitFlow::SKIP_CHILDREN`] to leave its children unvisited.
				visit_value_slot,
				/// Called after leaving a value slot offered to [`VisitMut::visit_value_slot`], even when that skipped its
				/// children.
				exit_value_slot
				[S: $crate::ValueSlot<'a>] (S) [S::Literal: $crate::Visitable + $crate::__css_parse::ToSpan];
			}
		}
	};
}

/// Implements [`VisitMut`] for an enum whose every variant holds a visitor, forwarding each call to the visitor in the
/// active variant. The walk is then compiled once for the enum, rather than once per visitor. The impl's generics must
/// name the arena lifetime `'a`. Usable from other crates only.
///
/// ```ignore
/// enum Pass<'a, N> { A(A<'a, N>), B(B<'a, N>) }
/// css_ast::forward_visit_mut!([<'a, N: Bound>] Pass<'a, N> { A, B });
/// ```
#[macro_export]
macro_rules! forward_visit_mut {
	(@match $self:ident { $($variant:ident),+ $(,)? } $method:ident($arg:ident)) => {
		match $self { $(Self::$variant(v) => v.$method($arg),)+ }
	};
	(
		@impl [$($generics:tt)*] $ty:ty, $variants:tt
		{ $( $kind:ident $visit:ident $exit:ident [$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*]; )+ }
		$( $name:ident$(<$lt:lifetime $(, $gen:ident)*>)?($obj:ty) [$($tp:ident),*], )+
	) => {
		impl $($generics)* $crate::VisitMut<'a> for $ty {
			fn consider_node(&self, node: $crate::VisitNode) -> $crate::VisitFlow {
				$crate::forward_visit_mut!(@match self $variants consider_node(node))
			}
			fn enter_node(&mut self, node: $crate::VisitNode) -> $crate::VisitFlow {
				$crate::forward_visit_mut!(@match self $variants enter_node(node))
			}
			fn exit_node(&mut self, node: $crate::VisitNode) -> $crate::VisitFlow {
				$crate::forward_visit_mut!(@match self $variants exit_node(node))
			}
			fn retain(&mut self, node: $crate::VisitNode) -> bool {
				$crate::forward_visit_mut!(@match self $variants retain(node))
			}
			$(
				fn $visit<$($hgen)*>(&mut self, node: &mut $hobj) -> $crate::VisitFlow where $($hwhere)* {
					$crate::forward_visit_mut!(@match self $variants $visit(node))
				}
				fn $exit<$($hgen)*>(&mut self, node: &mut $hobj) -> $crate::VisitFlow where $($hwhere)* {
					$crate::forward_visit_mut!(@match self $variants $exit(node))
				}
			)+
			$(
				fn $name$(<$($gen),*>)?(&mut self, node: &mut $obj) -> $crate::VisitFlow
				where $($tp: $crate::Visitable + $crate::VisitableMut<'a> + $crate::__css_parse::ToSpan,)*
				{
					$crate::forward_visit_mut!(@match self $variants $name(node))
				}
			)+
		}
	};
	(
		@start [$($generics:tt)*] $ty:ty, $variants:tt
		@hooks { $(
			$kind:ident $(#[$visit_attr:meta])* $visit:ident, $(#[$exit_attr:meta])* $exit:ident
			[$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*];
		)+ }
	) => {
		$crate::apply_visit_methods!(
			[$crate::forward_visit_mut] @impl [$($generics)*] $ty, $variants
			{ $( $kind $visit $exit [$($hgen)*] ($hobj) [$($hwhere)*]; )+ }
		);
	};
	([$($generics:tt)*] $ty:ty { $($variant:ident),+ $(,)? }) => {
		$crate::apply_visit_mut_hooks!([$crate::forward_visit_mut] @start [$($generics)*] $ty, { $($variant),+ });
	};
}

#[doc(hidden)]
pub use css_parse as __css_parse;

use css_parse::Vec;
use css_parse::{
	AttributeOperator, Block, Box, CommaSeparated, Comparison, ComponentValue, ComponentValues, Cursor, Declaration,
	DeclarationGroup, DeclarationList, DeclarationOrBad, DeclarationValue, Either, NoBlockAllowed, NodeMetadata,
	NodeWithMetadata, Optionals2, Optionals3, Optionals4, Optionals5, Parse, Peek, QualifiedRule, RuleList, Span,
	ToCursors, ToSpan, syntax::BadDeclaration, token_macros,
};
use std::ops::ControlFlow;
use visit_flow::try_visit;

/// The `#[visitor]` attribute: observer methods without a return type
/// auto-descend. Re-exported so visitor consumers need only depend on `css_ast`.
pub use csskit_derives::visitor;
pub use visit_flow::{VisitAction, VisitBreak, VisitFlow, VisitFlowExt};

mod fused;
mod root;
mod visit_node;
pub use fused::FusedVisitor;
pub use root::{ErasedNode, ParsedRoot, parse_root};
pub(crate) use visit_node::QueryNodeData;
pub use visit_node::{NodeKey, VisitNode};

use crate::*;

macro_rules! visit_mut_trait {
	(
		@hooks { $(
			$kind:ident $(#[$visit_attr:meta])* $visit:ident, $(#[$exit_attr:meta])* $exit:ident
			[$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*];
		)+ }
	) => {
		apply_visit_methods!([visit_mut_trait] @methods { $(
			$(#[$visit_attr])* $visit, $(#[$exit_attr])* $exit [$($hgen)*] ($hobj) [$($hwhere)*];
		)+ });
	};
	(
		@methods { $(
			$(#[$visit_attr:meta])* $visit:ident, $(#[$exit_attr:meta])* $exit:ident
			[$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*];
		)+ }
		$( $name: ident$(<$lt: lifetime $(, $gen: ident)*>)?($obj: ty) [$($tp: ident),*], )+
	) => {
		/// A visitor which may rewrite the nodes of a tree whose arena lifetime is `'a`.
		pub trait VisitMut<'a>: Sized {
			/// Called before entering a queryable node. Return [`VisitFlow::SKIP_CHILDREN`] to leave the node and its
			/// subtree unvisited, or [`VisitFlow::STOP`] to halt the traversal.
			fn consider_node(&self, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			/// Called on entry to every queryable node. Return [`VisitFlow::SKIP_CHILDREN`] to skip the typed `visit_*`
			/// call and children.
			fn enter_node(&mut self, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			/// Called on exit from every queryable node.
			fn exit_node(&mut self, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			/// Called by a list or optional slot for each element it holds, before visiting it. Return `false` to
			/// remove the element; it is then never visited. A [`CommaSeparated`] stops asking once only its minimum
			/// number of items remain.
			fn retain(&mut self, _node: VisitNode) -> bool {
				true
			}
			$(
				$(#[$visit_attr])*
				fn $visit<$($hgen)*>(&mut self, _node: &mut $hobj) -> VisitFlow where $($hwhere)* {
					VisitFlow::DESCEND
				}
				$(#[$exit_attr])*
				fn $exit<$($hgen)*>(&mut self, _node: &mut $hobj) -> VisitFlow where $($hwhere)* {
					VisitFlow::DESCEND
				}
			)+
			$(
				fn $name$(<$($gen),*>)?(&mut self, _rule: &mut $obj) -> VisitFlow
				where $($tp: Visitable + VisitableMut<'a> + ToSpan,)*
				{
					VisitFlow::DESCEND
				}
			)+
		}
	}
}
apply_visit_mut_hooks!([visit_mut_trait]);

/// The object-safe core of [`Visit`].
///
/// [`Visit`] has methods that are type-specific. A visitor that needs the node kind and the span can implement just
/// this trait, and can then walk a node whose type is erased, such as an [`ErasedNode`].
///
/// The methods have no defaults, thus an implementation cannot drop one by mistake.
pub trait NodeVisitor {
	/// Called before entering a node.
	///
	/// Return [`VisitFlow::SKIP_CHILDREN`] to prune the node and its entire subtree (it is never entered). Return
	/// [`VisitFlow::STOP`] to halt the whole traversal.
	fn consider_node(&self, node: VisitNode) -> VisitFlow;

	/// Called on entry to every queryable node.
	///
	/// Receives a [`VisitNode`]; per-node metadata and properties are available via its methods. Return
	/// [`VisitFlow::SKIP_CHILDREN`] to skip the typed `visit_*` call and children.
	fn enter_node(&mut self, node: VisitNode) -> VisitFlow;

	/// Called on exit from every queryable node.
	fn exit_node(&mut self, node: VisitNode) -> VisitFlow;
}

impl NodeVisitor for &mut (dyn NodeVisitor + '_) {
	fn consider_node(&self, node: VisitNode) -> VisitFlow {
		(**self).consider_node(node)
	}

	fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
		(**self).enter_node(node)
	}

	fn exit_node(&mut self, node: VisitNode) -> VisitFlow {
		(**self).exit_node(node)
	}
}

impl Visit for &mut (dyn NodeVisitor + '_) {}

macro_rules! visit_trait {
	( $(
		$name: ident$(<$($gen:tt),+>)?($obj: ty) [$($tp: ident),*],
	)+ ) => {
		pub trait Visit: NodeVisitor + Sized {

			fn enter_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &Declaration<'a, T, CssMetadata>, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_declaration<'a, T: DeclarationValue<'a, CssMetadata>>(&mut self, _rule: &Declaration<'a, T, CssMetadata>, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn visit_bad_declaration<'a>(&mut self, _rule: &BadDeclaration<'a>) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_bad_declaration<'a>(&mut self, _rule: &BadDeclaration<'a>) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn visit_string(&mut self, _str: &token_macros::String) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_string(&mut self, _str: &token_macros::String) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn visit_comparison(&mut self, _comparison: &Comparison) -> VisitFlow {
				VisitFlow::DESCEND
			}
			fn exit_comparison(&mut self, _comparison: &Comparison) -> VisitFlow {
				VisitFlow::DESCEND
			}

			/// Called before entering a node whose type has feature metadata. Return [`VisitFlow::STOP`] to halt the
			/// traversal; any other flow enters the node.
			fn visit_feature<T: FeatureMetadata>(&mut self, _node: &T) -> VisitFlow {
				VisitFlow::DESCEND
			}
			/// Called after leaving a node whose type has feature metadata. Return [`VisitFlow::STOP`] to halt the
			/// traversal.
			fn exit_feature<T: FeatureMetadata>(&mut self, _node: &T) -> VisitFlow {
				VisitFlow::DESCEND
			}

			$(
				fn $name$(<$($gen),+>)?(&mut self, _rule: &$obj) -> VisitFlow where $($tp: Visitable + ToSpan,)* {
					VisitFlow::DESCEND
				}
			)+
		}
	}
}
apply_visit_methods!([visit_trait]);

pub trait VisitableMut<'a> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow;

	/// The queryable node this value stands for, which lists and optional slots offer to [`VisitMut::retain`]. A
	/// derived wrapper with a single visited field stands for that field; containers such as `Option`, `Vec` and
	/// [`CommaSeparated`] stand for nothing, so removal stays with the slot that holds the node.
	fn element_node(&self) -> Option<VisitNode<'_>> {
		None
	}
}

pub trait Visitable {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow;
}

/// Marker trait for AST nodes that can be queried with selectors.
///
/// Implemented by `#[derive(Visitable)]` for queryable nodes, and manually for nodes
/// with `get_property` overrides (named at-rules, declarations).
/// Not part of the `Visit` public API - visitors receive a [`VisitNode`] instead.
pub(crate) trait QueryableNode: ToSpan + NodeWithMetadata<CssMetadata> {
	/// Unique identifier for this node type.
	const NODE_ID: NodeId;

	/// [`NodeId::holds_rule_metadata`] of [`Self::NODE_ID`].
	const RULE_ROOT: bool = Self::NODE_ID.holds_rule_metadata();

	/// Returns a cursor for the given property kind, if the node has that property.
	/// Used by attribute selectors to extract values from nodes.
	///
	/// For `PropertyKind::Name`, returns a cursor to the node's name (e.g., property
	/// name for declarations, animation name for `@keyframes`).
	fn get_property(&self, _kind: PropertyKind) -> Option<Cursor> {
		None
	}

	/// Builds the [`VisitNode`] passed to every `Visit` callback for this node.
	///
	/// Metadata and properties are reached through `&dyn` accessors, so they only cost anything
	/// when a visitor actually reads them - `subtree_metadata` in particular walks the subtree.
	fn visit_node(&self) -> VisitNode<'_>
	where
		Self: Sized,
	{
		VisitNode::new(Self::NODE_ID, Self::RULE_ROOT, self)
	}
}

/// Blanket bridge so any [`QueryableNode`] can be held as a `&dyn` in [`VisitNode`].
impl<T: QueryableNode> QueryNodeData for T {
	#[inline]
	fn span(&self) -> Span {
		ToSpan::to_span(self)
	}

	#[inline]
	fn self_metadata(&self) -> CssMetadata {
		NodeWithMetadata::self_metadata(self)
	}

	#[inline]
	fn subtree_metadata(&self) -> CssMetadata {
		NodeWithMetadata::metadata(self)
	}

	#[inline]
	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		QueryableNode::get_property(self, kind)
	}
}

impl<'a, T> VisitableMut<'a> for Option<T>
where
	T: VisitableMut<'a>,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		let Some(node) = self else { return VisitFlow::DESCEND };
		if node.element_node().is_some_and(|element| !v.retain(element)) {
			*self = None;
			return VisitFlow::DESCEND;
		}
		node.accept_mut(v)
	}
}

macro_rules! impl_optionals {
	($N:ident, $($T:ident),+) => {
		impl<$($T),*> Visitable for $N<$($T),+>
		where
			$($T: Visitable,)+
		{
			#[allow(non_snake_case)]
			#[allow(unused)]
			fn accept<VI: Visit>(&self, v: &mut VI) -> VisitFlow {
				let $N($($T),+) = self;
				$(try_visit!($T.accept(v));)+;
				VisitFlow::DESCEND
			}
		}

		impl<'a, $($T),*> VisitableMut<'a> for $N<$($T),+>
		where
			$($T: VisitableMut<'a>,)+
		{
			#[allow(non_snake_case)]
			#[allow(unused)]
			fn accept_mut<VI: VisitMut<'a>>(&mut self, v: &mut VI) -> VisitFlow {
				let $N($($T),+) = self;
				$(try_visit!($T.accept_mut(v));)+;
				VisitFlow::DESCEND
			}
		}
	};
}

impl_optionals!(Optionals2, T, U);
impl_optionals!(Optionals3, T, U, V);
impl_optionals!(Optionals4, T, U, V, W);
impl_optionals!(Optionals5, T, U, V, W, X);

impl Visitable for token_macros::Ident {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Ident {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::Function {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Function {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::Comma {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Comma {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::LeftParen {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::LeftParen {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::RightParen {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::RightParen {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::Colon {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Colon {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::Semicolon {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Semicolon {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for Comparison {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_comparison(self));
		try_visit!(v.exit_comparison(self));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for Comparison {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_comparison(self));
		try_visit!(v.exit_comparison(self));
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::delim::Dash {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::delim::Dash {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::delim::Slash {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::delim::Slash {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::Number {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Number {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::Any {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::Any {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl Visitable for token_macros::String {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_string(self));
		try_visit!(v.exit_string(self));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for token_macros::String {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_string(self));
		try_visit!(v.exit_string(self));
		VisitFlow::DESCEND
	}
}

impl<T> Visitable for Option<T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		if let Some(node) = self {
			try_visit!(node.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, T: VisitableMut<'a>> VisitableMut<'a> for Box<'a, T> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		(**self).accept_mut(v)
	}

	fn element_node(&self) -> Option<VisitNode<'_>> {
		(**self).element_node()
	}
}

impl<'a, T: Visitable> Visitable for Box<'a, T> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		(**self).accept(v)
	}
}

fn retain_and_visit<'a, T: VisitableMut<'a>, V: VisitMut<'a>>(
	node: &mut T,
	v: &mut V,
	stopped: &mut Option<VisitBreak>,
	removable: bool,
) -> bool {
	if stopped.is_some() {
		return true;
	}
	if removable && node.element_node().is_some_and(|element| !v.retain(element)) {
		return false;
	}
	if let ControlFlow::Break(stop) = node.accept_mut(v) {
		*stopped = Some(stop);
	}
	true
}

impl<'a, T, const MIN: usize> VisitableMut<'a> for CommaSeparated<'a, T, MIN>
where
	T: VisitableMut<'a>,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		let mut stopped = None;
		self.retain_mut(|node, removable| retain_and_visit(node, v, &mut stopped, removable));
		stopped.map_or(VisitFlow::DESCEND, ControlFlow::Break)
	}
}

impl<'a, T, const MIN: usize> Visitable for CommaSeparated<'a, T, MIN>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for (node, _) in self {
			try_visit!(node.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, Left, Right> VisitableMut<'a> for Either<Left, Right>
where
	Left: VisitableMut<'a>,
	Right: VisitableMut<'a>,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		match self {
			Self::Left(t) => t.accept_mut(v),
			Self::Right(t) => t.accept_mut(v),
		}
	}

	fn element_node(&self) -> Option<VisitNode<'_>> {
		match self {
			Self::Left(t) => t.element_node(),
			Self::Right(t) => t.element_node(),
		}
	}
}

impl<Left, Right> Visitable for Either<Left, Right>
where
	Left: Visitable,
	Right: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		match self {
			Self::Left(t) => t.accept(v),
			Self::Right(t) => t.accept(v),
		}
	}
}

impl<'a, T> VisitableMut<'a> for Declaration<'a, T, CssMetadata>
where
	T: VisitableMut<'a> + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(self.visit_node())) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(self.visit_node())) {
			if let VisitAction::Descend = try_visit!(v.enter_declaration(self)) {
				try_visit!(self.value.accept_mut(v));
			}
			try_visit!(v.exit_declaration(self));
		}
		try_visit!(v.exit_node(self.visit_node()));
		VisitFlow::DESCEND
	}

	fn element_node(&self) -> Option<VisitNode<'_>> {
		Some(self.visit_node())
	}
}

impl<'a, T> QueryableNode for Declaration<'a, T, CssMetadata>
where
	T: DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	const NODE_ID: NodeId = NodeId::StyleValue;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.name.into()),
			_ => None,
		}
	}

	fn visit_node(&self) -> VisitNode<'_> {
		// Use T::NODE_ID so each declaration type has its own identity.
		VisitNode::new(T::NODE_ID, T::RULE_ROOT, self)
	}
}

impl<'a, T> Visitable for Declaration<'a, T, CssMetadata>
where
	T: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = self.visit_node();
		if let visit_flow::VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let visit_flow::VisitAction::Descend = visit_flow::try_visit!(v.enter_node(node)) {
			if let visit_flow::VisitAction::Descend = visit_flow::try_visit!(v.enter_declaration::<T>(self, node)) {
				try_visit!(self.value.accept(v));
			}
			try_visit!(v.exit_declaration::<T>(self, node));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a, T> VisitableMut<'a> for DeclarationList<'a, T, CssMetadata>
where
	T: VisitableMut<'a> + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		try_visit!(self.declarations.accept_mut(v));
		VisitFlow::DESCEND
	}
}

impl<'a, T> Visitable for DeclarationList<'a, T, CssMetadata>
where
	T: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for declaration in &self.declarations {
			try_visit!(declaration.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, T, M> VisitableMut<'a> for RuleList<'a, T, M>
where
	T: VisitableMut<'a> + Parse<'a> + ToCursors + ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		self.rules.accept_mut(v)
	}
}

impl<'a, T, M> Visitable for RuleList<'a, T, M>
where
	T: Visitable + Parse<'a> + ToCursors + ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		self.rules.accept(v)
	}
}

impl<'a, P, D, R> VisitableMut<'a> for QualifiedRule<'a, P, D, R, CssMetadata>
where
	P: VisitableMut<'a> + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: VisitableMut<'a> + DeclarationValue<'a, CssMetadata> + QueryableNode,
	R: VisitableMut<'a> + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R, CssMetadata>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		try_visit!(self.prelude.accept_mut(v));
		self.block.accept_mut(v)
	}
}

impl<'a, P, D, R> Visitable for QualifiedRule<'a, P, D, R, CssMetadata>
where
	P: Visitable + Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R, CssMetadata>: Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(self.prelude.accept(v));
		self.block.accept(v)
	}
}

impl<'a, D, R> VisitableMut<'a> for Block<'a, D, R, CssMetadata>
where
	D: VisitableMut<'a> + DeclarationValue<'a, CssMetadata> + QueryableNode,
	R: VisitableMut<'a> + Parse<'a> + ToCursors + ToSpan,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		try_visit!(self.declarations.accept_mut(v));
		try_visit!(self.rules.accept_mut(v));
		VisitFlow::DESCEND
	}
}

impl<'a, D, R> Visitable for Block<'a, D, R, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
	R: Visitable + Parse<'a> + ToCursors + ToSpan,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for declaration in &self.declarations {
			try_visit!(declaration.accept(v));
		}
		for rule in &self.rules {
			try_visit!(rule.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, T> VisitableMut<'a> for Vec<'a, T>
where
	T: VisitableMut<'a>,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		let mut stopped = None;
		self.retain_mut(|node| retain_and_visit(node, v, &mut stopped, true));
		stopped.map_or(VisitFlow::DESCEND, ControlFlow::Break)
	}
}

impl<'a, T> Visitable for Vec<'a, T>
where
	T: Visitable,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for node in self {
			try_visit!(node.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for BadDeclaration<'a> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_bad_declaration(self));
		try_visit!(v.exit_bad_declaration(self));
		VisitFlow::DESCEND
	}
}

impl<'a> Visitable for BadDeclaration<'a> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		try_visit!(v.visit_bad_declaration(self));
		try_visit!(v.exit_bad_declaration(self));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for ComponentValues<'a> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(QueryableNode::visit_node(&*self))) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(QueryableNode::visit_node(&*self))) {
			if let VisitAction::Descend = try_visit!(v.visit_component_values(self)) {
				try_visit!(self.values.accept_mut(v));
			}
			try_visit!(v.exit_component_values(self));
		}
		try_visit!(v.exit_node(QueryableNode::visit_node(&*self)));
		VisitFlow::DESCEND
	}

	fn element_node(&self) -> Option<VisitNode<'_>> {
		Some(QueryableNode::visit_node(self))
	}
}

impl<'a> QueryableNode for ComponentValues<'a> {
	const NODE_ID: NodeId = NodeId::ComponentValues;
}

impl<'a> Visitable for ComponentValues<'a> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = QueryableNode::visit_node(self);
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(node)) {
			if let VisitAction::Descend = try_visit!(v.visit_component_values(self)) {
				for value in &self.values {
					try_visit!(value.accept(v));
				}
			}
			try_visit!(v.exit_component_values(self));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for ComponentValue<'a> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(QueryableNode::visit_node(&*self))) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(QueryableNode::visit_node(&*self))) {
			if let VisitAction::Descend = try_visit!(v.visit_component_value(self)) {
				match self {
					ComponentValue::SimpleBlock(block) => {
						try_visit!(block.values.accept_mut(v));
					}
					ComponentValue::Function(function) => {
						try_visit!(function.params.accept_mut(v));
					}
					_ => {}
				}
			}
			try_visit!(v.exit_component_value(self));
		}
		try_visit!(v.exit_node(QueryableNode::visit_node(&*self)));
		VisitFlow::DESCEND
	}

	fn element_node(&self) -> Option<VisitNode<'_>> {
		Some(QueryableNode::visit_node(self))
	}
}

impl<'a> QueryableNode for ComponentValue<'a> {
	const NODE_ID: NodeId = NodeId::ComponentValue;
}

impl<'a> Visitable for ComponentValue<'a> {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = QueryableNode::visit_node(self);
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(node)) {
			if let VisitAction::Descend = try_visit!(v.visit_component_value(self)) {
				match self {
					ComponentValue::SimpleBlock(block) => {
						try_visit!(block.values.accept(v));
					}
					ComponentValue::Function(function) => {
						try_visit!(function.params.accept(v));
					}
					_ => {}
				}
			}
			try_visit!(v.exit_component_value(self));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for AttributeOperator {
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(QueryableNode::visit_node(&*self))) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(QueryableNode::visit_node(&*self))) {
			try_visit!(v.visit_attribute_operator(self));
			try_visit!(v.exit_attribute_operator(self));
		}
		try_visit!(v.exit_node(QueryableNode::visit_node(&*self)));
		VisitFlow::DESCEND
	}

	fn element_node(&self) -> Option<VisitNode<'_>> {
		Some(QueryableNode::visit_node(self))
	}
}

impl QueryableNode for AttributeOperator {
	const NODE_ID: NodeId = NodeId::AttributeOperator;
}

impl Visitable for AttributeOperator {
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		let node = QueryableNode::visit_node(self);
		if let VisitAction::SkipChildren = try_visit!(v.consider_node(node)) {
			return VisitFlow::DESCEND;
		}
		if let VisitAction::Descend = try_visit!(v.enter_node(node)) {
			try_visit!(v.visit_attribute_operator(self));
			try_visit!(v.exit_attribute_operator(self));
		}
		try_visit!(v.exit_node(node));
		VisitFlow::DESCEND
	}
}

impl<'a> VisitableMut<'a> for Unresolved<'a> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a> Visitable for Unresolved<'a> {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a, D, M> VisitableMut<'a> for NoBlockAllowed<D, M> {
	fn accept_mut<V: VisitMut<'a>>(&mut self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<D, M> Visitable for NoBlockAllowed<D, M> {
	fn accept<V: Visit>(&self, _: &mut V) -> VisitFlow {
		VisitFlow::DESCEND
	}
}

impl<'a, D> VisitableMut<'a> for DeclarationGroup<'a, D, CssMetadata>
where
	D: VisitableMut<'a> + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		self.declarations.accept_mut(v)
	}
}

impl<'a, D> Visitable for DeclarationGroup<'a, D, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		for declaration in &self.declarations {
			try_visit!(declaration.accept(v));
		}
		VisitFlow::DESCEND
	}
}

impl<'a, D> VisitableMut<'a> for DeclarationOrBad<'a, D, CssMetadata>
where
	D: VisitableMut<'a> + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept_mut<V: VisitMut<'a>>(&mut self, v: &mut V) -> VisitFlow {
		match self {
			Self::Declaration(d) => d.accept_mut(v),
			Self::Bad(b) => b.accept_mut(v),
		}
	}
}

impl<'a, D> Visitable for DeclarationOrBad<'a, D, CssMetadata>
where
	D: Visitable + DeclarationValue<'a, CssMetadata> + QueryableNode,
{
	fn accept<V: Visit>(&self, v: &mut V) -> VisitFlow {
		match self {
			Self::Declaration(d) => d.accept(v),
			Self::Bad(b) => b.accept(v),
		}
	}
}

macro_rules! impl_tuple_mut {
    ($($T:ident),*) => {
				impl<'a, $($T),*> VisitableMut<'a> for ($($T),*)
        where
            $($T: VisitableMut<'a>,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
						fn accept_mut<VI: VisitMut<'a>>(&mut self, v: &mut VI) -> VisitFlow {
                let ($($T),*) = self;
                $(try_visit!($T.accept_mut(v));)*
                VisitFlow::DESCEND
            }
        }
    };
}

impl_tuple_mut!(T, U);
impl_tuple_mut!(T, U, V);
impl_tuple_mut!(T, U, V, W);
impl_tuple_mut!(T, U, V, W, X);
impl_tuple_mut!(T, U, V, W, X, Y);
impl_tuple_mut!(T, U, V, W, X, Y, Z);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B, C);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B, C, D);
impl_tuple_mut!(T, U, V, W, X, Y, Z, A, B, C, D, E);

macro_rules! impl_tuple {
    ($($T:ident),*) => {
			impl<$($T),*> Visitable for ($($T),*)
        where
            $($T: Visitable,)*
        {
            #[allow(non_snake_case)]
            #[allow(unused)]
					fn accept<VI: Visit>(&self, v: &mut VI) -> VisitFlow {
                let ($($T),*) = self;
                $(try_visit!($T.accept(v));)*
                VisitFlow::DESCEND
            }
        }
    };
}
impl_tuple!(T, U);
impl_tuple!(T, U, V);
impl_tuple!(T, U, V, W);
impl_tuple!(T, U, V, W, X);
impl_tuple!(T, U, V, W, X, Y);
impl_tuple!(T, U, V, W, X, Y, Z);
impl_tuple!(T, U, V, W, X, Y, Z, A);
impl_tuple!(T, U, V, W, X, Y, Z, A, B);
impl_tuple!(T, U, V, W, X, Y, Z, A, B, C);
impl_tuple!(T, U, V, W, X, Y, Z, A, B, C, D);
impl_tuple!(T, U, V, W, X, Y, Z, A, B, C, D, E);

#[cfg(test)]
mod tests {
	use super::*;
	use css_lexer::Lexer;
	use css_parse::{Arena, CursorWriteSink, Parser};

	#[derive(Default)]
	struct DropCompoundSelectors {
		asked: usize,
		visited: usize,
	}

	impl<'a> VisitMut<'a> for DropCompoundSelectors {
		fn retain(&mut self, node: VisitNode) -> bool {
			let compound = node.node_id == Some(NodeId::CompoundSelector);
			self.asked += usize::from(compound);
			!compound
		}

		fn visit_compound_selector(&mut self, _: &mut CompoundSelector<'a>) -> VisitFlow {
			self.visited += 1;
			VisitFlow::DESCEND
		}
	}

	#[test]
	fn comma_separated_retain_stops_at_minimum() {
		let alloc = Arena::default();
		let source_text = "a,b,c";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let mut list = parser.parse_entirely::<SelectorList>().output.unwrap();
		let mut visitor = DropCompoundSelectors::default();
		let _ = list.accept_mut(&mut visitor);
		assert_eq!(visitor.asked, 2);
		assert_eq!(visitor.visited, 1);
		let mut actual = css_parse::String::new_in(&alloc);
		list.to_cursors(&mut CursorWriteSink::new(source_text, &mut actual));
		assert_eq!(actual.as_str(), "c");
	}

	#[derive(Default)]
	struct SkipThenStop {
		compounds: usize,
		exited: usize,
		classes: usize,
	}

	#[visitor]
	impl<'a> VisitMut<'a> for SkipThenStop {
		fn visit_compound_selector(&mut self, _: &mut CompoundSelector<'a>) -> VisitFlow {
			self.compounds += 1;
			if self.compounds == 1 { VisitFlow::SKIP_CHILDREN } else { VisitFlow::STOP }
		}

		fn exit_compound_selector(&mut self, _: &mut CompoundSelector<'a>) {
			self.exited += 1;
		}

		fn visit_class(&mut self, _: &mut Class) {
			self.classes += 1;
		}
	}

	#[test]
	fn typed_hooks_skip_children_and_stop() {
		let alloc = Arena::default();
		let source_text = ".a,.b,.c";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let mut list = parser.parse_entirely::<SelectorList>().output.unwrap();
		let mut visitor = SkipThenStop::default();
		let result = list.accept_mut(&mut visitor);
		assert!(result.is_break());
		assert_eq!(visitor.compounds, 2);
		assert_eq!(visitor.exited, 1);
		assert_eq!(visitor.classes, 0);
	}

	#[derive(Default)]
	struct FeatureOrder {
		events: std::vec::Vec<&'static str>,
		stop_at_feature: bool,
	}

	#[visitor]
	impl Visit for FeatureOrder {
		fn enter_node(&mut self, _: VisitNode) {
			self.events.push("enter_node");
		}

		fn exit_node(&mut self, _: VisitNode) {
			self.events.push("exit_node");
		}

		fn visit_feature<T: FeatureMetadata>(&mut self, _: &T) -> VisitFlow {
			self.events.push("visit_feature");
			if self.stop_at_feature { VisitFlow::STOP } else { VisitFlow::DESCEND }
		}

		fn exit_feature<T: FeatureMetadata>(&mut self, _: &T) {
			self.events.push("exit_feature");
		}
	}

	#[visitor]
	impl<'a> VisitMut<'a> for FeatureOrder {
		fn enter_node(&mut self, _: VisitNode) {
			self.events.push("enter_node");
		}

		fn exit_node(&mut self, _: VisitNode) {
			self.events.push("exit_node");
		}

		fn visit_feature<T: FeatureMetadata>(&mut self, _: &mut T) -> VisitFlow {
			self.events.push("visit_feature");
			if self.stop_at_feature { VisitFlow::STOP } else { VisitFlow::DESCEND }
		}

		fn exit_feature<T: FeatureMetadata>(&mut self, _: &mut T) {
			self.events.push("exit_feature");
		}
	}

	#[test]
	fn feature_hooks_wrap_the_node_in_both_walks() {
		let alloc = Arena::default();
		let source_text = "(hover:hover)";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let mut feature = parser.parse_entirely::<HoverMediaFeature>().output.unwrap();
		let expected = ["visit_feature", "enter_node", "exit_node", "exit_feature"];
		let mut visitor = FeatureOrder::default();
		let _ = feature.accept(&mut visitor);
		assert_eq!(visitor.events, expected);
		let mut visitor = FeatureOrder::default();
		let _ = feature.accept_mut(&mut visitor);
		assert_eq!(visitor.events, expected);
	}

	#[test]
	fn feature_hook_stop_halts_before_the_node_in_both_walks() {
		let alloc = Arena::default();
		let source_text = "(hover:hover)";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut parser = Parser::new(&alloc, source_text, lexer);
		let mut feature = parser.parse_entirely::<HoverMediaFeature>().output.unwrap();
		let mut visitor = FeatureOrder { stop_at_feature: true, ..Default::default() };
		assert!(feature.accept(&mut visitor).is_break());
		assert_eq!(visitor.events, ["visit_feature"]);
		let mut visitor = FeatureOrder { stop_at_feature: true, ..Default::default() };
		assert!(feature.accept_mut(&mut visitor).is_break());
		assert_eq!(visitor.events, ["visit_feature"]);
	}
}
