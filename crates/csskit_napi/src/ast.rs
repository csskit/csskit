use css_ast::visit::{ErasedNode, NodeId, NodeKey, NodeVisitor, ParsedRoot, VisitNode, parse_root};
use css_lexer::Lexer;
use css_parse::{Arena, Diagnostic as ParseError, DiagnosticMeta, Parser};
use csskit_ast::{CsskitAtomSet, QuerySelectorList, SelectorMatcher};
use napi_derive::napi;
use self_cell::self_cell;
use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use visit_flow::{VisitFlow, VisitFlowExt};

/// A parse diagnostic.
#[napi(object)]
#[derive(Clone, Debug)]
pub struct Diagnostic {
	pub from: u32,
	pub to: u32,
	pub severity: String,
	pub code: String,
	pub message: String,
	pub help: String,
}

impl Diagnostic {
	/// Owns its strings: the source [`ParseError`] does not outlive the parse borrow.
	fn new(err: &ParseError, source: &str) -> Self {
		let DiagnosticMeta { code, message, help, .. } = err.meta(source);
		let span = err.span();
		Self {
			from: span.start().0,
			to: span.end().0,
			severity: err.severity.to_string(),
			code: code.to_string(),
			message,
			help,
		}
	}
}

/// One selector match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct QueryMatch {
	pub node_id: NodeId,
	pub node_key: NodeKey,
	pub order: usize,
	pub start: u32,
	pub end: u32,
}

#[derive(Clone, Copy)]
struct NodePosition {
	order: usize,
	end: usize,
}

#[derive(Default)]
struct IndexVisitor {
	positions: HashMap<NodeKey, NodePosition>,
	stack: Vec<NodeKey>,
	next: usize,
	root_key: Option<NodeKey>,
}

impl NodeVisitor for IndexVisitor {
	fn consider_node(&self, _node: VisitNode) -> VisitFlow {
		VisitFlow::DESCEND
	}

	fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
		let node_key = node.key().expect("queryable nodes have identity");
		self.root_key.get_or_insert(node_key);
		let order = self.next;
		self.next += 1;
		let old = self.positions.insert(node_key, NodePosition { order, end: self.next });
		debug_assert!(old.is_none(), "node identity is unique within a tree");
		self.stack.push(node_key);
		VisitFlow::DESCEND
	}

	fn exit_node(&mut self, node: VisitNode) -> VisitFlow {
		let node_key = node.key().expect("queryable nodes have identity");
		debug_assert_eq!(self.stack.pop(), Some(node_key));
		self.positions.get_mut(&node_key).expect("entered nodes are indexed").end = self.next;
		VisitFlow::DESCEND
	}
}

struct NodeIndex {
	positions: HashMap<NodeKey, NodePosition>,
	root_key: Option<NodeKey>,
}

impl NodeIndex {
	fn contains(&self, ancestor: NodeKey, node: NodeKey) -> bool {
		let Some(ancestor) = self.positions.get(&ancestor) else { return false };
		let Some(node) = self.positions.get(&node) else { return false };
		ancestor.order <= node.order && node.order < ancestor.end
	}
}

/// What a parse borrows from: the arena it allocates into, and the source text it reads.
struct Input {
	arena: Arena,
	source: String,
}

/// What a parse gives back. The diagnostics are rendered here, where the source is at hand, thus
/// they own their strings and outlive the borrow.
struct Parsed<'a> {
	root: Option<&'a dyn ErasedNode>,
	diagnostics: Vec<Diagnostic>,
}

self_cell!(
	/// Holds the arena and the source alongside the root that borrows them.
	struct ParseCell {
		owner: Input,

		#[covariant]
		dependent: Parsed,
	}
);

/// A parsed root that owns its arena and its source.
pub(crate) struct Ast {
	cell: ParseCell,
	last_query: RefCell<Option<(String, Rc<[QueryMatch]>)>>,
	index: OnceCell<NodeIndex>,
}

impl Ast {
	/// Parses `source` as the given node kind. Gives `None` if the kind has no standalone grammar.
	///
	/// The arena gets a capacity hint four times the source length, in the range `64 KiB ..= 8 MiB`, and adds
	/// chunks if the parse needs more. [`Arena::new`] would reserve 2 GiB of address space for each parse, which a host
	/// process that holds many parses at once cannot afford.
	pub(crate) fn parse(source: String, id: NodeId) -> Option<Self> {
		let arena = Arena::with_capacity(source.len().saturating_mul(4).clamp(64 * 1024, 8 * 1024 * 1024));
		let cell = ParseCell::try_new(Input { arena, source }, |input| -> Result<Parsed<'_>, ()> {
			let ParsedRoot { root, diagnostics } = parse_root(id, &input.arena, input.source.as_str()).ok_or(())?;
			let diagnostics = diagnostics.iter().map(|err| Diagnostic::new(err, &input.source)).collect();
			Ok(Parsed { root, diagnostics })
		})
		.ok()?;
		Some(Self { cell, last_query: RefCell::new(None), index: OnceCell::new() })
	}

	fn root(&self) -> Option<&dyn ErasedNode> {
		self.cell.borrow_dependent().root
	}

	pub(crate) fn source(&self) -> &str {
		&self.cell.borrow_owner().source
	}

	/// The byte range the root covers, or `(0, 0)` where the source does not parse.
	pub(crate) fn span(&self) -> (u32, u32) {
		self.root().map_or((0, 0), |root| {
			let span = root.to_span();
			(span.start().0, span.end().0)
		})
	}

	/// The source text in the byte range `[start, end)`.
	pub(crate) fn text_at(&self, start: u32, end: u32) -> &str {
		self.source().get(start as usize..end as usize).unwrap_or_default()
	}

	pub(crate) fn diagnostics(&self) -> &[Diagnostic] {
		&self.cell.borrow_dependent().diagnostics
	}

	/// Runs `visitor` over each queryable node, in document order.
	pub(crate) fn walk(&self, visitor: &mut dyn NodeVisitor) {
		if let Some(root) = self.root() {
			let _ = root.accept_dyn(visitor);
		}
	}

	fn index(&self) -> &NodeIndex {
		self.index.get_or_init(|| {
			let mut visitor = IndexVisitor::default();
			self.walk(&mut visitor);
			NodeIndex { positions: visitor.positions, root_key: visitor.root_key }
		})
	}

	pub(crate) fn root_key(&self) -> Option<NodeKey> {
		self.index().root_key
	}

	pub(crate) fn contains(&self, ancestor: NodeKey, node: NodeKey) -> bool {
		self.index().contains(ancestor, node)
	}

	/// Every node that matches `selector`, in document order.
	/// [`Ast::descendants_of`] and [`Ast::matches`] do a binary search on that order.
	///
	/// The last result list is kept against its selector, thus asking the same question of many nodes
	/// costs one match run, not one run for each node. A selector that does not parse is not kept.
	pub(crate) fn query(&self, selector: &str) -> Result<Rc<[QueryMatch]>, String> {
		if let Some((kept, matches)) = self.last_query.borrow().as_ref()
			&& kept == selector
		{
			return Ok(Rc::clone(matches));
		}
		let matches = self.run_query(selector)?;
		*self.last_query.borrow_mut() = Some((selector.to_owned(), Rc::clone(&matches)));
		Ok(matches)
	}

	/// Runs `selector` over the document. Sorts matches into structural document order.
	fn run_query(&self, selector: &str) -> Result<Rc<[QueryMatch]>, String> {
		let Some(root) = self.root() else { return Ok(Vec::new().into()) };
		let scratch = Arena::with_capacity(selector.len().saturating_mul(16).clamp(1024, 64 * 1024));
		let lexer = Lexer::new(&CsskitAtomSet::ATOMS, selector);
		let result = Parser::new(&scratch, selector, lexer).parse_entirely::<QuerySelectorList<'_>>();
		let Some(selectors) = result.output else {
			let msg = result
				.errors
				.first()
				.map(|err| Diagnostic::new(err, selector).message)
				.unwrap_or_else(|| "invalid selector".to_owned());
			return Err(msg);
		};
		let index = self.index();
		let mut matches: Vec<QueryMatch> = SelectorMatcher::new(&selectors, selector, self.source())
			.run(root)
			.map(|m| {
				let node_key = m.node_key.expect("selector matches are real nodes");
				let order = index.positions.get(&node_key).expect("selector matches are indexed").order;
				QueryMatch { node_id: m.node_id, node_key, order, start: m.span.start().0, end: m.span.end().0 }
			})
			.collect();
		matches.sort_by_key(|m| m.order);
		Ok(matches.into())
	}

	/// Every match of `selector` structurally below `node_key`. Does not include the scoped node.
	///
	/// The match run stays rooted at the document, thus a descendant selector can use ancestors above
	/// the scope, as `querySelectorAll` does in the DOM.
	pub(crate) fn descendants_of(
		&self,
		node_key: Option<NodeKey>,
		selector: &str,
	) -> Result<impl Iterator<Item = QueryMatch>, String> {
		let matches = self.query(selector)?;
		let range = node_key
			.and_then(|key| self.index().positions.get(&key))
			.map(|scope| {
				let from = matches.partition_point(|m| m.order <= scope.order);
				let to = matches.partition_point(|m| m.order < scope.end);
				from..to
			})
			.unwrap_or(0..0);
		Ok(range.map(move |i| matches[i]))
	}

	/// True if `node_key` matches `selector`.
	pub(crate) fn matches(&self, node_key: Option<NodeKey>, selector: &str) -> Result<bool, String> {
		let Some(node_key) = node_key else { return Ok(false) };
		let matches = self.query(selector)?;
		let Some(position) = self.index().positions.get(&node_key) else { return Ok(false) };
		let from = matches.partition_point(|m| m.order < position.order);
		Ok(matches[from..].first().is_some_and(|m| m.node_key == node_key))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_ast::visit::VisitNode;
	use visit_flow::{VisitFlow, VisitFlowExt};

	fn parse(source: &str, tag: &str) -> Ast {
		let id = NodeId::from_tag_name(tag).expect("known tag");
		Ast::parse(source.to_owned(), id).expect("a root tag")
	}

	fn kinds(ast: &Ast) -> Vec<&'static str> {
		struct Collect(Vec<&'static str>);
		impl NodeVisitor for Collect {
			fn consider_node(&self, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}

			fn enter_node(&mut self, node: VisitNode) -> VisitFlow {
				if let Some(id) = node.node_id {
					self.0.push(id.tag_name());
				}
				VisitFlow::DESCEND
			}

			fn exit_node(&mut self, _node: VisitNode) -> VisitFlow {
				VisitFlow::DESCEND
			}
		}
		let mut c = Collect(Vec::new());
		ast.walk(&mut c);
		c.0
	}

	fn text(ast: &Ast) -> &str {
		let (start, end) = ast.span();
		ast.text_at(start, end)
	}

	#[test]
	fn parses_stylesheet_and_spans_the_source() {
		let ast = parse("a{color:red}", "style-sheet");
		assert_eq!(ast.diagnostics().len(), 0);
		assert_eq!(text(&ast), "a{color:red}");
	}

	#[test]
	fn walk_visits_nodes() {
		let ast = parse("a{color:red}", "style-sheet");
		let visited = kinds(&ast);
		assert!(visited.contains(&"style-sheet"), "got {visited:?}");
		assert!(visited.contains(&"style-rule"), "got {visited:?}");
		assert!(visited.contains(&"color"), "got {visited:?}");
	}

	#[test]
	fn query_finds_nodes() {
		let ast = parse("a{color:red}b{color:blue}", "style-sheet");
		let rules = ast.query("style-rule").unwrap();
		assert_eq!(rules.len(), 2, "two style rules");
		let colors = ast.query("color").unwrap();
		assert_eq!(colors.len(), 2, "two color values");
	}

	#[test]
	fn query_by_attribute() {
		let ast = parse("a{color:red;width:2px}", "style-sheet");
		let color = ast.query("*[name=color]").unwrap();
		assert_eq!(color.len(), 1);
		let named = &color[0];
		assert_eq!(ast.text_at(named.start, named.end), "color:red;");
	}

	#[test]
	fn matches_a_found_node() {
		let ast = parse("a{color:red}", "style-sheet");
		let rules = ast.query("style-rule").unwrap();
		let r = rules[0];
		assert!(ast.matches(Some(r.node_key), "style-rule").unwrap());
		assert!(!ast.matches(Some(r.node_key), "color").unwrap());
	}

	#[test]
	fn invalid_selector_errors() {
		let ast = parse("a{}", "style-sheet");
		assert!(ast.query("!!!nonsense!!!").is_err());
	}

	#[test]
	fn parses_every_kind_of_root() {
		for (source, root) in [
			("#ff0000", "color"),
			("a > b, .c", "selector-list"),
			("a.b", "compound-selector"),
			("a{color:red}", "style-rule"),
			("@media print{a{color:red}}", "media-rule"),
			("1px solid red", "component-values"),
			("100px", "width-style-value"),
			("@scope (.a){b{color:red}}", "scope-rule"),
			("45deg", "angle"),
		] {
			let ast = parse(source, root);
			assert_eq!(ast.diagnostics().len(), 0, "{root} parsed {source} cleanly");
			assert_eq!(text(&ast), source);
		}
	}

	#[test]
	fn roots_resolve_by_tag() {
		let root = |tag: &str| NodeId::from_tag_name(tag).and_then(|id| Ast::parse(String::new(), id).map(|_| id));
		assert_eq!(root("style-sheet"), Some(NodeId::StyleSheet));
		assert_eq!(root("component-values"), Some(NodeId::ComponentValues));
		assert_eq!(root("StyleSheet"), None, "a class name is not a tag");
		assert_eq!(root("bogus"), None);
		assert_eq!(root("style-value"), None, "no standalone grammar");
	}

	#[test]
	fn component_values_is_a_node_like_any_other() {
		let ast = parse("1px solid red", "component-values");
		// Each component value is a node of its own, thus the root and every token in it answer a
		// universal query.
		let all = ast.query("*").unwrap();
		assert_eq!(all[0].node_id, NodeId::ComponentValues);
		assert!(all[1..].iter().all(|n| n.node_id == NodeId::ComponentValue));
		assert_eq!(ast.query("component-value").unwrap().len(), all.len() - 1);
		assert!(ast.matches(Some(all[0].node_key), "component-values").unwrap());
	}

	#[test]
	fn repeated_queries_reuse_one_match_run() {
		let ast = parse("a{color:red}b{color:blue}c{width:2px}", "style-sheet");
		let rules = ast.query("style-rule").unwrap();
		let arena = || {
			let arena = &ast.cell.borrow_owner().arena;
			(arena.capacity(), arena.used_bytes())
		};
		let before = arena();
		for _ in 0..1000 {
			assert!(Rc::ptr_eq(&ast.query("style-rule").unwrap(), &rules), "one run answers the same selector");
			let r = rules[0];
			assert!(ast.matches(Some(r.node_key), "style-rule").unwrap());
		}
		// Only the last selector is kept, thus a new selector displaces the one before it.
		assert_eq!(ast.query("*[name=color]").unwrap().len(), 2);
		assert!(!Rc::ptr_eq(&ast.query("style-rule").unwrap(), &rules));
		assert_eq!(arena(), before, "no query allocates from the parse arena");
	}

	#[test]
	fn descendants_are_contained_and_exclude_the_scope() {
		let ast = parse("a{color:red}b{color:blue}", "style-sheet");
		let rules = ast.query("style-rule").unwrap();
		let outer = rules[0];
		let inside: Vec<QueryMatch> = ast.descendants_of(Some(outer.node_key), "*").unwrap().collect();
		assert!(!inside.is_empty(), "the first rule has queryable descendants");
		assert!(!inside.contains(&outer), "the scoped node is not its own descendant");
		for m in &inside {
			assert!(m.start >= outer.start && m.end <= outer.end, "{m:?} escapes {outer:?}");
		}
		// The second rule and the sheet itself are outside the scope.
		let all = ast.query("*").unwrap();
		assert!(all.len() > inside.len() + 1, "the document-wide query sees more than the scope does");
	}

	#[test]
	fn descendants_exclude_a_same_span_ancestor() {
		let ast = parse("a{color:red}", "style-sheet");
		let sheet = ast.query("style-sheet").unwrap()[0];
		let rule = ast.query("style-rule").unwrap()[0];
		assert_eq!((sheet.start, sheet.end), (rule.start, rule.end), "fixture requires equal spans");
		let inside: Vec<QueryMatch> = ast.descendants_of(Some(rule.node_key), "*").unwrap().collect();
		assert!(!inside.is_empty(), "the rule has queryable descendants");
		assert!(!inside.iter().any(|node| node.node_key == sheet.node_key), "the sheet is an ancestor");
	}

	#[test]
	fn scoped_descendants_still_see_ancestors_above_the_scope() {
		let ast = parse("a{color:red}", "style-sheet");
		let rule = ast.query("style-rule").unwrap()[0];
		let scope =
			|selector| -> Vec<QueryMatch> { ast.descendants_of(Some(rule.node_key), selector).unwrap().collect() };
		// The ancestor part lives *above* the scoped rule; matching the rule's subtree in isolation would find
		// nothing, whereas DOM `querySelectorAll` matches against the whole document and only scopes the results.
		assert_eq!(scope("style-sheet *[name=color]"), ast.query("style-sheet *[name=color]").unwrap().to_vec());
		assert_eq!(scope("style-sheet *[name=color]").len(), 1);
		// ... and the combinator is still honoured: no media rule wraps this declaration.
		assert_eq!(scope("media-rule *[name=color]").len(), 0);
	}

	#[test]
	fn matches_agrees_with_query() {
		let ast = parse("a{color:red;width:2px}@media print{b{color:blue}}", "style-sheet");
		for selector in ["*", "style-rule", "color", "media-rule *", "style-sheet style-rule", "*[name=width]"] {
			let expected = ast.query(selector).unwrap();
			for m in ast.query("*").unwrap().iter() {
				let want = expected.contains(m);
				assert_eq!(ast.matches(Some(m.node_key), selector).unwrap(), want, "{selector} vs {m:?}");
			}
		}
	}

	#[test]
	fn a_root_that_fails_to_parse_reports_diagnostics() {
		let ast = parse("a{}", "pseudo-class");
		assert!(!ast.diagnostics().is_empty());
		assert_eq!(text(&ast), "");
	}
}
