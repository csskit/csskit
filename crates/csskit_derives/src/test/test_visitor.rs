use crate::visitor;
use insta::assert_snapshot;
use quote::quote;

fn expand(tokens: proc_macro2::TokenStream) -> String {
	let item = syn::parse2::<syn::ItemImpl>(tokens).unwrap();
	let out = visitor::expand(item).expect("visitor expansion failed");
	let file = syn::parse2::<syn::File>(out).unwrap();
	prettyplease::unparse(&file)
}

#[test]
fn observer_method_gets_return_type_and_descend_tail() {
	let out = expand(quote! {
		impl Visit for V {
			fn visit_color(&mut self, c: &Color) {
				self.seen.push(c);
			}
		}
	});
	assert_snapshot!("visitor_observer", out);
}

#[test]
fn explicit_return_type_is_untouched() {
	let out = expand(quote! {
		impl Visit for V {
			fn consider_node(&self, q: VisitNode) -> VisitFlow {
				VisitFlow::SKIP_CHILDREN
			}
		}
	});
	assert_snapshot!("visitor_explicit_return", out);
}

#[test]
fn bare_return_in_observer_is_rewritten() {
	let out = expand(quote! {
		impl Visit for V {
			fn visit_color(&mut self, c: &Color) {
				if c.is_empty() {
					return;
				}
				self.seen.push(c);
			}
		}
	});
	assert_snapshot!("visitor_bare_return", out);
}

#[test]
fn return_in_nested_closure_is_not_rewritten() {
	let out = expand(quote! {
		impl Visit for V {
			fn visit_color(&mut self, c: &Color) {
				let f = || {
					return;
				};
				f();
			}
		}
	});
	assert_snapshot!("visitor_nested_closure", out);
}

#[test]
fn explicit_unit_return_opts_out() {
	let out = expand(quote! {
		impl VisitMut for V {
			fn visit_color(&mut self, c: &Color) -> () {
				self.seen.push(c);
			}
		}
	});
	assert_snapshot!("visitor_unit_optout", out);
}

#[test]
fn rejects_inherent_impl() {
	let item = syn::parse2::<syn::ItemImpl>(quote! {
		impl V {
			fn visit_color(&mut self, c: &Color) {
				self.seen.push(c);
			}
		}
	})
	.unwrap();

	assert!(visitor::expand(item).is_err());
}
