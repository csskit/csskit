use super::*;
use css_parse::Arena;
use std::cell::Cell;

/// A part of the walk within which [`FusedVisitor`] calls fewer visitors than outside it.
struct Scope {
	/// The number of scopes open, this one included, when it opened.
	depth: u32,
	/// The visitors receiving calls outside this scope.
	outer: u64,
}

/// Runs several visitors in a single walk, calling each hook on every visitor in list order.
///
/// Each visitor receives the calls it would receive walking the tree alone. If it skips a node's children, it gets no
/// calls from within them while the others carry on, and once it stops it gets no more calls. The walk stops when
/// every visitor has stopped. A list or optional slot drops an element as soon as one visitor's [`VisitMut::retain`]
/// returns `false`; the visitors after it are not asked.
///
/// The visitors share the tree as it is rewritten: on each node a visitor sees the changes made by the visitors
/// before it, and below a node it sees the changes every visitor made on entering that node.
pub struct FusedVisitor<'a, Visitor> {
	visitors: Vec<'a, Visitor>,
	/// One bit per visitor that has not stopped.
	running: Cell<u64>,
	/// One bit per visitor receiving calls in the current scope.
	receiving: u64,
	/// The visitors `consider_node` let into a node, until `visit_feature` or `enter_node` opens the node's scope.
	considered: Cell<Option<u64>>,
	/// The number of scopes open.
	depth: u32,
	/// The open scopes that narrowed the visitors receiving calls, innermost last.
	scopes: Vec<'a, Scope>,
	/// The visitors that received `visit_feature`, for each node with feature metadata being walked, innermost last.
	features: Vec<'a, u64>,
}

/// Returns the index of each set bit in `bits`, lowest first.
fn bit_indices(mut bits: u64) -> impl Iterator<Item = usize> {
	std::iter::from_fn(move || {
		(bits != 0).then(|| {
			let index = bits.trailing_zeros() as usize;
			bits &= bits - 1;
			index
		})
	})
}

impl<'a, Visitor: VisitMut<'a>> FusedVisitor<'a, Visitor> {
	/// Fuses `visitors`, of which there may be at most 64, allocating the walk's state in `alloc`.
	pub fn new_in(alloc: &'a Arena, visitors: Vec<'a, Visitor>) -> Self {
		assert!(visitors.len() <= 64, "a FusedVisitor holds at most 64 visitors");
		let all = if visitors.is_empty() { 0 } else { u64::MAX >> (64 - visitors.len()) };
		Self {
			visitors,
			running: Cell::new(all),
			receiving: all,
			considered: Cell::new(None),
			depth: 0,
			scopes: Vec::new_in(alloc),
			features: Vec::new_in(alloc),
		}
	}

	#[inline]
	fn active(&self) -> u64 {
		self.receiving & self.running.get()
	}

	#[inline]
	fn stop(&self, index: usize) {
		self.running.set(self.running.get() & !(1 << index));
	}

	#[inline]
	fn flow(&self) -> VisitFlow {
		if self.running.get() == 0 { VisitFlow::STOP } else { VisitFlow::DESCEND }
	}

	/// The flow for the walk once the visitors in `descending` are the ones to receive calls.
	#[inline]
	fn narrowed_flow(&self, descending: u64) -> VisitFlow {
		if descending == 0 && self.running.get() != 0 { VisitFlow::SKIP_CHILDREN } else { self.flow() }
	}

	#[inline]
	fn push_scope(&mut self, receiving: u64) {
		self.depth += 1;
		if receiving != self.active() {
			self.scopes.push(Scope { depth: self.depth, outer: self.receiving });
			self.receiving = receiving;
		}
	}

	#[inline]
	fn close(&mut self) {
		match self.scopes.last() {
			Some(scope) if scope.depth == self.depth => {
				self.receiving = scope.outer;
				self.scopes.pop();
			}
			_ => {}
		}
		self.depth -= 1;
	}

	/// Opens the scope of the node `consider_node` last let in, if it is not open yet.
	#[inline]
	fn open_considered(&mut self) {
		if let Some(considered) = self.considered.take() {
			self.push_scope(considered);
		}
	}

	/// Calls `hook` on every active visitor, then opens a scope holding those that descend.
	#[inline]
	fn open(&mut self, mut hook: impl FnMut(&mut Visitor) -> VisitFlow) -> VisitFlow {
		let mut descending = 0;
		for index in bit_indices(self.active()) {
			match hook(&mut self.visitors[index]) {
				ControlFlow::Continue(VisitAction::Descend) => descending |= 1 << index,
				ControlFlow::Continue(VisitAction::SkipChildren) => {}
				ControlFlow::Break(_) => self.stop(index),
			}
		}
		self.push_scope(descending);
		self.narrowed_flow(descending)
	}

	/// Calls `hook` on every visitor in `receivers` that has not stopped.
	#[inline]
	fn deliver(&mut self, receivers: u64, mut hook: impl FnMut(&mut Visitor) -> VisitFlow) -> VisitFlow {
		for index in bit_indices(receivers & self.running.get()) {
			if hook(&mut self.visitors[index]).is_break() {
				self.stop(index);
			}
		}
		self.flow()
	}

	/// Closes the innermost scope, then calls `hook` on every visitor active outside it.
	#[inline]
	fn close_and_deliver(&mut self, hook: impl FnMut(&mut Visitor) -> VisitFlow) -> VisitFlow {
		self.close();
		self.deliver(self.receiving, hook)
	}
}

macro_rules! fused_visit_mut {
	(@pair scoped $visit:ident $exit:ident [$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*]) => {
		fn $visit<$($hgen)*>(&mut self, node: &mut $hobj) -> VisitFlow where $($hwhere)* {
			self.open(|visitor| visitor.$visit(node))
		}
		fn $exit<$($hgen)*>(&mut self, node: &mut $hobj) -> VisitFlow where $($hwhere)* {
			self.close_and_deliver(|visitor| visitor.$exit(node))
		}
	};
	(@pair feature $visit:ident $exit:ident [$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*]) => {
		fn $visit<$($hgen)*>(&mut self, node: &mut $hobj) -> VisitFlow where $($hwhere)* {
			self.open_considered();
			self.features.push(self.receiving);
			self.deliver(self.receiving, |visitor| visitor.$visit(node))
		}
		fn $exit<$($hgen)*>(&mut self, node: &mut $hobj) -> VisitFlow where $($hwhere)* {
			let receivers = self.features.pop().expect("exit_feature follows visit_feature");
			self.deliver(receivers, |visitor| visitor.$exit(node))
		}
	};
	(
		@hooks { $(
			$kind:ident $(#[$visit_attr:meta])* $visit:ident, $(#[$exit_attr:meta])* $exit:ident
			[$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*];
		)+ }
	) => {
		apply_visit_methods!([fused_visit_mut] @methods { $( $kind $visit $exit [$($hgen)*] ($hobj) [$($hwhere)*]; )+ });
	};
	(
		@methods { $( $kind:ident $hvisit:ident $hexit:ident [$($hgen:tt)*] ($hobj:ty) [$($hwhere:tt)*]; )+ }
		$(
			$visit: ident$(<$visit_lt: lifetime $(, $visit_gen: ident)*>)?($visit_obj: ty) [$($visit_tp: ident),*],
			$exit: ident$(<$exit_lt: lifetime $(, $exit_gen: ident)*>)?($exit_obj: ty) [$($exit_tp: ident),*],
		)+
	) => {
		impl<'a, Visitor: VisitMut<'a>> VisitMut<'a> for FusedVisitor<'a, Visitor> {
			fn consider_node(&self, node: VisitNode) -> VisitFlow {
				let mut descending = 0;
				for index in bit_indices(self.active()) {
					match self.visitors[index].consider_node(node) {
						ControlFlow::Continue(VisitAction::Descend) => descending |= 1 << index,
						ControlFlow::Continue(VisitAction::SkipChildren) => {}
						ControlFlow::Break(_) => self.stop(index),
					}
				}
				let flow = self.narrowed_flow(descending);
				if flow == VisitFlow::DESCEND {
					self.considered.set(Some(descending));
				}
				flow
			}
			fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
				self.open_considered();
				self.open(|visitor| visitor.enter_node(node))
			}
			fn exit_node(&mut self, node: VisitNode) -> VisitFlow {
				let flow = self.close_and_deliver(|visitor| visitor.exit_node(node));
				self.close();
				flow
			}
			fn retain(&mut self, node: VisitNode) -> bool {
				bit_indices(self.active()).all(|index| self.visitors[index].retain(node))
			}
			$( fused_visit_mut!(@pair $kind $hvisit $hexit [$($hgen)*] ($hobj) [$($hwhere)*]); )+
			$(
				fn $visit$(<$($visit_gen),*>)?(&mut self, node: &mut $visit_obj) -> VisitFlow
				where $($visit_tp: Visitable + VisitableMut<'a> + ToSpan,)*
				{
					self.open(|visitor| visitor.$visit(node))
				}
				fn $exit$(<$($exit_gen),*>)?(&mut self, node: &mut $exit_obj) -> VisitFlow
				where $($exit_tp: Visitable + VisitableMut<'a> + ToSpan,)*
				{
					self.close_and_deliver(|visitor| visitor.$exit(node))
				}
			)+
		}
	};
}
apply_visit_mut_hooks!([fused_visit_mut]);

#[cfg(test)]
mod tests {
	use super::*;
	use css_lexer::Lexer;
	use css_parse::Parser;

	/// Each hook a recorder received, with the node it was called on where the hook names one.
	type Events = std::vec::Vec<(&'static str, Option<NodeId>)>;

	/// Records the hooks it receives, and skips or stops where configured.
	#[derive(Default)]
	struct Recorder {
		skip_considering: Option<NodeId>,
		skip_entering: Option<NodeId>,
		stop_entering: Option<NodeId>,
		dropping: Option<NodeId>,
		skip_value_slots: bool,
		events: Events,
	}

	impl Recorder {
		fn matches(configured: Option<NodeId>, node: VisitNode) -> bool {
			configured.is_some() && configured == node.node_id
		}
	}

	impl<'a> VisitMut<'a> for Recorder {
		fn consider_node(&self, node: VisitNode) -> VisitFlow {
			if Self::matches(self.skip_considering, node) { VisitFlow::SKIP_CHILDREN } else { VisitFlow::DESCEND }
		}

		fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
			self.events.push(("enter_node", node.node_id));
			if Self::matches(self.stop_entering, node) {
				VisitFlow::STOP
			} else if Self::matches(self.skip_entering, node) {
				VisitFlow::SKIP_CHILDREN
			} else {
				VisitFlow::DESCEND
			}
		}

		fn exit_node(&mut self, node: VisitNode) -> VisitFlow {
			self.events.push(("exit_node", node.node_id));
			VisitFlow::DESCEND
		}

		fn retain(&mut self, node: VisitNode) -> bool {
			self.events.push(("retain", node.node_id));
			!Self::matches(self.dropping, node)
		}

		fn enter_declaration<T: DeclarationValue<'a, CssMetadata>>(
			&mut self,
			_: &mut Declaration<'a, T, CssMetadata>,
		) -> VisitFlow {
			self.events.push(("enter_declaration", None));
			VisitFlow::DESCEND
		}

		fn exit_declaration<T: DeclarationValue<'a, CssMetadata>>(
			&mut self,
			_: &mut Declaration<'a, T, CssMetadata>,
		) -> VisitFlow {
			self.events.push(("exit_declaration", None));
			VisitFlow::DESCEND
		}

		fn visit_feature<T: FeatureMetadata>(&mut self, _: &mut T) -> VisitFlow {
			self.events.push(("visit_feature", None));
			VisitFlow::DESCEND
		}

		fn exit_feature<T: FeatureMetadata>(&mut self, _: &mut T) -> VisitFlow {
			self.events.push(("exit_feature", None));
			VisitFlow::DESCEND
		}

		fn visit_value_slot<S: ValueSlot<'a>>(&mut self, _: &mut S) -> VisitFlow
		where
			S::Literal: Visitable + ToSpan,
		{
			self.events.push(("visit_value_slot", None));
			if self.skip_value_slots { VisitFlow::SKIP_CHILDREN } else { VisitFlow::DESCEND }
		}

		fn exit_value_slot<S: ValueSlot<'a>>(&mut self, _: &mut S) -> VisitFlow
		where
			S::Literal: Visitable + ToSpan,
		{
			self.events.push(("exit_value_slot", None));
			VisitFlow::DESCEND
		}

		fn visit_length(&mut self, _: &mut Length) -> VisitFlow {
			self.events.push(("visit_length", None));
			VisitFlow::DESCEND
		}
	}

	fn configurations() -> [Recorder; 5] {
		[
			Recorder { skip_considering: Some(NodeId::StyleRule), ..Default::default() },
			Recorder { skip_entering: Some(NodeId::MediaRule), ..Default::default() },
			Recorder { stop_entering: Some(NodeId::SelectorList), ..Default::default() },
			Recorder { skip_value_slots: true, ..Default::default() },
			Recorder::default(),
		]
	}

	const SOURCE: &str = "@media (width > 1px) { a { width: calc(1px + 2px); color: red } } .b, .c { margin: 1px 2px }";

	fn parse(alloc: &Arena) -> StyleSheet<'_> {
		Parser::new(alloc, SOURCE, Lexer::new(&CssAtomSet::ATOMS, SOURCE))
			.parse_entirely::<StyleSheet>()
			.output
			.unwrap()
	}

	/// Walks a fresh stylesheet with each recorder alone, then another with all of them fused, asserting each records
	/// the same calls both ways, and returns the fused walk's flow and the recorders' calls when alone.
	fn assert_fused_matches_alone<const N: usize>(
		configurations: impl Fn() -> [Recorder; N],
	) -> (VisitFlow, [Events; N]) {
		let alloc = Arena::default();
		let alone = configurations().map(|mut recorder| {
			let _ = parse(&alloc).accept_mut(&mut recorder);
			recorder.events
		});
		let mut visitors = Vec::new_in(&alloc);
		visitors.extend(configurations());
		let mut fused = FusedVisitor::new_in(&alloc, visitors);
		let flow = parse(&alloc).accept_mut(&mut fused);
		for (index, (fused, alone)) in fused.visitors.iter().zip(&alone).enumerate() {
			assert_eq!(&fused.events, alone, "configuration {index}");
		}
		(flow, alone)
	}

	#[test]
	fn each_visitor_receives_the_calls_it_receives_alone() {
		let (flow, alone) = assert_fused_matches_alone(configurations);
		assert!(flow.is_continue());
		for hook in ["enter_declaration", "visit_feature", "visit_value_slot", "visit_length", "retain"] {
			assert!(alone[4].iter().any(|(name, _)| *name == hook), "the stylesheet exercises no {hook}");
		}
	}

	#[test]
	fn stops_once_every_visitor_stops() {
		let (flow, _) = assert_fused_matches_alone(|| {
			[
				Recorder { stop_entering: Some(NodeId::SelectorList), ..Default::default() },
				Recorder { stop_entering: Some(NodeId::MediaRule), ..Default::default() },
			]
		});
		assert!(flow.is_break());
	}

	#[test]
	fn retain_asks_no_visitor_after_one_drops_the_element() {
		let alloc = Arena::default();
		let mut sheet = parse(&alloc);
		let mut visitors = Vec::new_in(&alloc);
		visitors.push(Recorder::default());
		visitors.push(Recorder { dropping: Some(NodeId::StyleRule), ..Default::default() });
		visitors.push(Recorder::default());
		let mut fused = FusedVisitor::new_in(&alloc, visitors);
		assert!(sheet.accept_mut(&mut fused).is_continue());
		let [before, _, after] = &fused.visitors[..] else { unreachable!() };
		let style_rule = Some(NodeId::StyleRule);
		assert!(before.events.contains(&("retain", style_rule)));
		assert!(!before.events.contains(&("enter_node", style_rule)));
		assert!(!after.events.contains(&("retain", style_rule)));
	}
}
