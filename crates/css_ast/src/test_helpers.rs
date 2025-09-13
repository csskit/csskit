use crate::visit::apply_visit_methods;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TestVisitor {
	pub visits: Vec<&'static str>,
}

macro_rules! visit_mut_trait {
	( $(
		$name: ident$(<$life:lifetime>)?($obj: ident$(<$olife:lifetime>)?),
	)+ ) => {
		impl $crate::VisitMut for TestVisitor {
			$(
				fn $name$(<$life>)?(&mut self, _rule: &mut $crate::$obj$(<$olife>)?) {
					self.handle_visit(stringify!($obj));
				}
			)+
		}
	}
}
apply_visit_methods!(visit_mut_trait);

impl TestVisitor {
	fn handle_visit(&mut self, type_name: &'static str) {
		self.visits.push(type_name);
	}
}

#[macro_export]
macro_rules! assert_visits {
	($source: expr, $parse_type: ty $(, $visit_type: ty)* $(,)?) => {{
		use bumpalo::Bump;
		use css_parse::Parser;
		use $crate::VisitableMut;

		let bump = Bump::default();
		let source_text = $source;
		let mut parser = Parser::new(&bump, &$crate::CssAtomSet::ATOMS, source_text);
		let result = parser.parse_entirely::<$parse_type>();
		if !result.errors.is_empty() {
			panic!("\n\nParse {:?} failed. Saw error {:?}", source_text, result.errors[0]);
		}
		let mut parsed = result.output.unwrap();

		let mut visitor = $crate::test_helpers::TestVisitor { visits: vec![] };
		parsed.accept_mut(&mut visitor);

		let actual_visits = visitor.visits.as_slice();
		let expected_visits = vec![ stringify!($parse_type), $( stringify!($visit_type) ),* ];

		if actual_visits != expected_visits {
			panic!(
				"\n\nVisit assertion failed for {:?}:\n\nActual visits: {:?}\nExpected visits: {:?}",
				source_text,
				actual_visits,
				expected_visits,
			);
		}
	}};
}

#[cfg(feature = "css_feature_data")]
#[macro_export]
macro_rules! assert_feature_id {
	($source: expr, $ty: ty, $id: literal) => {{
		use bumpalo::Bump;
		use css_parse::Parser;
		let bump = Bump::default();
		let source_text = $source;
		let mut parser = Parser::new(&bump, &$crate::CssAtomSet::ATOMS, source_text);
		let result = parser.parse_entirely::<$ty>();
		if !result.errors.is_empty() {
			panic!("\n\nParse {:?} failed. Saw error {:?}", source_text, result.errors[0]);
		}
		assert_eq!(result.output.unwrap().to_css_feature().unwrap().id, $id);
	}};
}
