use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Expr, ImplItem, ItemImpl, Result, ReturnType, parse_quote,
	spanned::Spanned,
	visit_mut::{self, VisitMut},
};

/// Rewrites bare `return;` into `return VisitFlow::DESCEND` within a method
/// body, so observer methods can early-out without spelling the flow value.
///
/// Does not descend into nested closures or async blocks: their `return`
/// targets a different (non-`VisitFlow`) context.
struct ReturnRewriter;

impl VisitMut for ReturnRewriter {
	fn visit_expr_mut(&mut self, expr: &mut Expr) {
		match expr {
			// Don't cross into a different return target.
			Expr::Closure(_) | Expr::Async(_) => return,
			Expr::Return(ret) if ret.expr.is_none() => {
				ret.expr =
					Some(Box::new(parse_quote!(<::visit_flow::VisitFlow as ::visit_flow::VisitFlowExt>::DESCEND)));
			}
			_ => {}
		}
		visit_mut::visit_expr_mut(self, expr);
	}
}

fn trait_name(item: &ItemImpl) -> Result<&syn::Ident> {
	let Some((path, _)) = &item.trait_ else {
		return Err(syn::Error::new(item.impl_token.span(), "#[visitor] requires an impl Visit block"));
	};
	path.segments
		.last()
		.map(|segment| &segment.ident)
		.ok_or_else(|| syn::Error::new(path.span(), "#[visitor] requires an impl Visit block"))
}

/// Rewrites a `Visit` impl block so observer methods need no
/// `-> VisitFlow` return type or trailing `VisitFlow::DESCEND`.
///
/// For every method that declares **no** return type, the macro:
/// - sets the return type to `visit_flow::VisitFlow`, and
/// - appends `visit_flow::VisitFlow::DESCEND` as the trailing expression.
///
/// Methods that declare an explicit return type (e.g. `-> VisitFlow`) are left
/// untouched, so flow-controlling visitors keep full control (return
/// `VisitFlow::SKIP_CHILDREN` / `VisitFlow::STOP` as normal).
///
/// `VisitMut` impls are accepted but left untouched; those trait methods return
/// `()` and do not participate in `VisitFlow`.
pub fn expand(item: ItemImpl) -> Result<TokenStream> {
	let mut item = item;
	let name = trait_name(&item)?;
	if name == "VisitMut" {
		return Ok(quote! { #item });
	}
	if name != "Visit" {
		return Err(syn::Error::new(name.span(), "#[visitor] only supports impl Visit or impl VisitMut"));
	}
	for impl_item in &mut item.items {
		let ImplItem::Fn(method) = impl_item else { continue };
		// Only rewrite methods with an elided return type. An explicit return
		// type (including `-> ()`) is an opt-out.
		if !matches!(method.sig.output, ReturnType::Default) {
			continue;
		}
		method.sig.output = parse_quote!(-> ::visit_flow::VisitFlow);
		ReturnRewriter.visit_block_mut(&mut method.block);
		let stmts = &method.block.stmts;
		method.block = parse_quote!({
			#(#stmts)*
			<::visit_flow::VisitFlow as ::visit_flow::VisitFlowExt>::DESCEND
		});
	}

	let mut node_visitor = item.clone();
	node_visitor.trait_ = Some((parse_quote!(NodeVisitor), Default::default()));
	let is_core = |impl_item: &ImplItem| matches!(impl_item, ImplItem::Fn(method) if NODE_VISITOR_METHODS.iter().any(|(name, _)| method.sig.ident == name));
	(node_visitor.items, item.items) = std::mem::take(&mut item.items).into_iter().partition(is_core);
	for (name, receiver) in NODE_VISITOR_METHODS {
		if node_visitor.items.iter().any(|item| matches!(item, ImplItem::Fn(method) if method.sig.ident == name)) {
			continue;
		}
		let name = format_ident!("{name}");
		let receiver: TokenStream = receiver.parse().expect("a receiver parses");
		node_visitor.items.push(parse_quote!(
			fn #name(#receiver, _node: VisitNode) -> ::visit_flow::VisitFlow {
				<::visit_flow::VisitFlow as ::visit_flow::VisitFlowExt>::DESCEND
			}
		));
	}

	Ok(quote! { #node_visitor #item })
}

/// The `NodeVisitor` methods and their receivers. `NodeVisitor` declares no defaults, thus every
/// impl states all three; the attribute writes the ones the visitor leaves out.
const NODE_VISITOR_METHODS: [(&str, &str); 3] =
	[("consider_node", "&self"), ("enter_node", "&mut self"), ("exit_node", "&mut self")];
