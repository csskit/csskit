use super::*;
use bumpalo::Bump;
use css_ast::CssAtomSet;
use css_lexer::Lexer;
use css_parse::Parser;

fn run<'a>(bump: &'a Bump, source: &'a str, css_source: &'a str) -> (Stats, Vec<'a, CollectorDiagnostic>) {
	let sheet = {
		let lexer = Lexer::new(CsskitAtomSet::get_dyn_set(), source);
		let mut parser = Parser::new(bump, source, lexer);
		parser.parse_entirely::<crate::sheet::Sheet>().with_trivia().output.unwrap()
	};
	let stylesheet = {
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css_source);
		let mut parser = Parser::new(bump, css_source, lexer);
		parser.parse_entirely::<StyleSheet>().with_trivia().output.unwrap()
	};
	let mut collector = Collector::new(&sheet, source, bump);
	collector.collect(&stylesheet, css_source);
	let mut diagnostics = bumpalo::vec![in bump];
	for diagnostic in collector.diagnostics(css_source) {
		diagnostics.push(diagnostic);
	}
	(collector.stats, diagnostics)
}

#[test]
fn test_basic() {
	let bump = Bump::new();
	let source = r#"
		@stat --total-rules { type: counter; }
		style-rule { collect: --total-rules; }
	"#;
	let css_source = r#"
		.foo {}
		#bar {}
		.baz {}
	"#;
	let (counters, diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("total-rules");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 0, "No diagnostics should be generated");
}

#[test]
fn test_implicit_counter() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --total-rules; }
	"#;
	let css_source = r#"
		.foo {}
		#bar {}
		.baz {}
	"#;
	let (counters, diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("total-rules");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 0, "No diagnostics should be generated");
}

#[test]
fn test_bytes() {
	let bump = Bump::new();
	let source = r#"
		@stat --rule-bytes { type: bytes; }
		style-rule { collect: --rule-bytes; }
	"#;
	let css_source = ".a{}.bb{}";
	let (counters, diagnostics) = run(&bump, source, css_source);
	// .a{} = 4 bytes, .bb{} = 5 bytes, total = 9 bytes
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("rule-bytes");
	assert_eq!(counters.get(&a), Some(&(StatType::Bytes, 9)));
	assert_eq!(diagnostics.len(), 0, "No diagnostics should be generated");
}

#[test]
fn test_lines() {
	let bump = Bump::new();
	let source = r#"
		@stat --rule-lines { type: lines; }
		style-rule { collect: --rule-lines; }
	"#;
	let css_source = r"
.a{

}
.bb{}";
	let (counters, diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("rule-lines");
	assert_eq!(counters.get(&a), Some(&(StatType::Lines, 4)));
	assert_eq!(diagnostics.len(), 0, "No diagnostics should be generated");
}

#[test]
fn test_diagnostics_with_string() {
	let bump = Bump::new();
	let source = r#"
		style-rule {
			level: warning;
			diagnostic: "Found a style rule";
		}
	"#;
	let css_source = ".foo {} .bar {}";

	let (_, diagnostics) = run(&bump, source, css_source);
	assert_eq!(diagnostics.len(), 2);
	assert_eq!(diagnostics[0].message, "Found a style rule");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Warning);
	assert_eq!(diagnostics[1].message, "Found a style rule");
	assert_eq!(diagnostics[1].severity, ResolvedDiagnosticLevel::Warning);
}

#[test]
fn test_diagnostics_with_attr() {
	let bump = Bump::new();
	let source = r#"
		[name] {
			level: error;
			diagnostic: "Property " attr(name) " found";
		}
	"#;
	let css_source = ".foo { color: red; margin: 0; }";

	let (_, diagnostics) = run(&bump, source, css_source);
	assert_eq!(diagnostics.len(), 2);
	assert_eq!(diagnostics[0].message, "Property color found");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Error);
	assert_eq!(diagnostics[1].message, "Property margin found");
	assert_eq!(diagnostics[1].severity, ResolvedDiagnosticLevel::Error);
}

#[test]
fn test_diagnostics_with_size() {
	let bump = Bump::new();
	let source = r#"
		selector-list {
			level: advice;
			diagnostic: "Selector list has " size() " selectors";
		}
	"#;
	let css_source = ".foo {}";

	let (_, diagnostics) = run(&bump, source, css_source);
	assert_eq!(diagnostics.len(), 1);
	assert_eq!(diagnostics[0].message, "Selector list has 1 selectors");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Advice);
}

#[test]
fn test_diagnostics_with_size_various() {
	let bump = Bump::new();
	let source = r#"
		selector-list {
			level: error;
			diagnostic: size() " selectors";
		}
	"#;
	let css_source = r#"
		.a {}
		.b, .c {}
		.d, .e, .f {}
	"#;

	let (_, diagnostics) = run(&bump, source, css_source);
	assert_eq!(diagnostics.len(), 3);
	assert_eq!(diagnostics[0].message, "1 selectors");
	assert_eq!(diagnostics[1].message, "2 selectors");
	assert_eq!(diagnostics[2].message, "3 selectors");
}

#[test]
fn test_diagnostics_with_stat_reference() {
	let bump = Bump::new();
	let source = r#"
		@stat --total { type: counter; }
		style-rule {
			collect: --total;
			level: warning;
			diagnostic: "Found " --total " rules so far";
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("total");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 3)));

	assert_eq!(diagnostics.len(), 3);
	assert_eq!(diagnostics[0].message, "Found 0 rules so far");
	assert_eq!(diagnostics[1].message, "Found 1 rules so far");
	assert_eq!(diagnostics[2].message, "Found 2 rules so far");
}

#[test]
fn test_when_rule_true_condition() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --rules; }
		@when (--rules > 1) {
			level: error;
			diagnostic: "Too many rules: " --rules;
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 1);
	assert_eq!(diagnostics[0].message, "Too many rules: 3");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Error);
}

#[test]
fn test_when_rule_false_condition() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --rules; }
		@when (--rules > 10) {
			level: error;
			diagnostic: "Too many rules";
		}
	"#;
	let css_source = ".a {} .b {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 2)));
	assert_eq!(diagnostics.len(), 0);
}

#[test]
fn test_when_rule_with_nested_selector() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --rules; }
		@when (--rules > 1) {
			id {
				collect: --id-selectors;
				level: warning;
				diagnostic: "ID selector found";
			}
		}
	"#;
	let css_source = ".a {} #foo {} .b {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let ids = CsskitAtomSet::get_dyn_set().atom_from_str("id-selectors");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&ids), Some(&(StatType::Counter, 1)));
	assert_eq!(diagnostics.len(), 1);
	assert_eq!(diagnostics[0].message, "ID selector found");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Warning);
}

#[test]
fn test_when_rule_equal_comparison() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --rules; }
		@when (--rules = 2) {
			level: advice;
			diagnostic: "Exactly 2 rules";
		}
	"#;
	let css_source = ".a {} .b {}";

	let (_, diagnostics) = run(&bump, source, css_source);
	assert_eq!(diagnostics.len(), 1);
	assert_eq!(diagnostics[0].message, "Exactly 2 rules");
}

#[test]
fn test_nested_if_in_style_rule() {
	let bump = Bump::new();
	let source = r#"
		style-rule {
			collect: --foo;
			@when (--foo > 1) {
				level: warning;
				diagnostic: "bar";
			}
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let foo = CsskitAtomSet::get_dyn_set().atom_from_str("foo");
	assert_eq!(counters.get(&foo), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 1);
	assert_eq!(diagnostics[0].message, "bar");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Warning);
}

#[test]
fn test_nested_selector_in_style_rule() {
	let bump = Bump::new();
	let source = r#"
		@stat --color-decls { type: counter; }
		style-rule {
			[name=color] {
				collect: --color-decls;
				level: warning;
				diagnostic: "color declaration found";
			}
		}
	"#;
	let css_source = ".a { color: red; } .b { background: blue; } .c { color: green; }";

	let (counters, diagnostics) = run(&bump, source, css_source);

	// Verify stats - only color declarations should be counted
	let color_decls = CsskitAtomSet::get_dyn_set().atom_from_str("color-decls");
	assert_eq!(counters.get(&color_decls), Some(&(StatType::Counter, 2)));

	// Verify diagnostics for each color declaration
	assert_eq!(diagnostics.len(), 2);
	assert_eq!(diagnostics[0].message, "color declaration found");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Warning);
}

#[test]
fn test_nested_if_with_combined_conditions() {
	let bump = Bump::new();
	let source = r#"
		@stat --rules { type: counter; }
		@stat --decls { type: counter; }
		style-rule { collect: --rules; }
		style-value[name] { collect: --decls; }
		@when (--rules > 1) {
			@when (--decls > 2) {
				level: error;
				diagnostic: "Hit";
			}
		}
	"#;
	let css_source = ".a { color: red; } .b { background: blue; font-size: 12px; }";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let decls = CsskitAtomSet::get_dyn_set().atom_from_str("decls");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 2)));
	assert_eq!(counters.get(&decls), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 1);
	assert_eq!(diagnostics[0].message, "Hit");
}

#[test]
fn test_conditional_nested_selector() {
	let bump = Bump::new();
	let source = r#"
		@stat --rules { type: counter; }
		@stat --color-decls { type: counter; }
		style-rule { collect: --rules; }
		@when (--rules > 1) {
			style-rule {
				[name=color] {
					collect: --color-decls;
					level: warning;
					diagnostic: "color declaration in conditional context";
				}
			}
		}
	"#;
	let css_source = ".a { color: red; } .b { background: blue; } .c { color: green; }";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let color_decls = CsskitAtomSet::get_dyn_set().atom_from_str("color-decls");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&color_decls), Some(&(StatType::Counter, 2)));
	assert_eq!(diagnostics.len(), 2);
	assert_eq!(diagnostics[0].message, "color declaration in conditional context");
	assert_eq!(diagnostics[0].severity, ResolvedDiagnosticLevel::Warning);
}

#[test]
fn test_reactive_conditional_chain() {
	let bump = Bump::new();
	let source = r#"
		@stat --rules { type: counter; }
		@stat --warnings { type: counter; }
		@stat --critical { type: counter; }

		style-rule { collect: --rules; }

		@when (--rules > 1) {
			style-rule {
				collect: --warnings;
				level: warning;
				diagnostic: "Many rules";
			}
		}

		@when (--warnings > 0) {
			style-rule {
				collect: --critical;
				level: error;
				diagnostic: "Warnings triggered, now critical";
			}
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let warnings = CsskitAtomSet::get_dyn_set().atom_from_str("warnings");
	let critical = CsskitAtomSet::get_dyn_set().atom_from_str("critical");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&warnings), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&critical), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 6);
	let warning_count = diagnostics.iter().filter(|d| d.severity == ResolvedDiagnosticLevel::Warning).count();
	let error_count = diagnostics.iter().filter(|d| d.severity == ResolvedDiagnosticLevel::Error).count();
	assert_eq!(warning_count, 3);
	assert_eq!(error_count, 3);
}

#[test]
fn test_nested_not_condition_preserved() {
	let bump = Bump::new();
	let source = r#"
		@stat --rules { type: counter; }
		@stat --nested-matches { type: counter; }

		style-rule { collect: --rules; }

		@when not (--rules > 10) {
			@when (--rules > 2) {
				style-rule {
					collect: --nested-matches;
					level: warning;
					diagnostic: "Nested match";
				}
			}
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, diagnostics) = run(&bump, source, css_source);

	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let nested = CsskitAtomSet::get_dyn_set().atom_from_str("nested-matches");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&nested), Some(&(StatType::Counter, 3)));
	assert_eq!(diagnostics.len(), 3);
}

#[test]
fn test_nested_not_condition_unmatched() {
	let bump = Bump::new();
	let source = r#"
		@stat --rules { type: counter; }
		@stat --nested-matches { type: counter; }

		style-rule { collect: --rules; }

		@when not (--rules > 1) {
			style-rule {
				collect: --nested-matches;
				level: warning;
				diagnostic: "Nested match";
			}
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let nested = CsskitAtomSet::get_dyn_set().atom_from_str("nested-matches");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&nested), Some(&(StatType::Counter, 0)));
	assert_eq!(diagnostics.len(), 0);
}

#[test]
fn test_cycles_direct() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --a; }
		@when (--a > 0) {
			style-rule { collect: --b; }
		}
		@when (--b > 0) {
			style-rule { collect: --a; }
		}
	"#;
	let css_source = ".x {} .y {} .z {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("a");
	let b = CsskitAtomSet::get_dyn_set().atom_from_str("b");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 6))); // 3 unconditional + 3 from second @when
	assert_eq!(counters.get(&b), Some(&(StatType::Counter, 3))); // 3 from first @when
}

#[test]
fn test_cycle_prevention_indirect() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --a; }
		@when (--a > 0) {
			style-rule { collect: --b; }
		}
		@when (--b > 0) {
			style-rule { collect: --c; }
		}
		@when (--c > 0) {
			style-rule { collect: --a; }
		}
	"#;
	let css_source = ".x {} .y {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let a = CsskitAtomSet::get_dyn_set().atom_from_str("a");
	let b = CsskitAtomSet::get_dyn_set().atom_from_str("b");
	let c = CsskitAtomSet::get_dyn_set().atom_from_str("c");
	assert_eq!(counters.get(&a), Some(&(StatType::Counter, 4)));
	assert_eq!(counters.get(&b), Some(&(StatType::Counter, 2)));
	assert_eq!(counters.get(&c), Some(&(StatType::Counter, 2)));
}

#[test]
fn test_cycle_prevention_self_referential() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --count; }
		@when (--count > 1) {
			style-rule { collect: --count; }
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let count = CsskitAtomSet::get_dyn_set().atom_from_str("count");
	assert_eq!(counters.get(&count), Some(&(StatType::Counter, 6)));
}

#[test]
fn test_equality_vs_range_semantics() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --rules; }
		@when (--rules = 1) {
			style-rule { collect: --equal-triggered; }
		}
		@when (--rules > 1) {
			style-rule { collect: --greater-triggered; }
		}
	"#;
	let css_source = ".a {} .b {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let rules = CsskitAtomSet::get_dyn_set().atom_from_str("rules");
	let equal = CsskitAtomSet::get_dyn_set().atom_from_str("equal-triggered");
	let greater = CsskitAtomSet::get_dyn_set().atom_from_str("greater-triggered");
	assert_eq!(counters.get(&rules), Some(&(StatType::Counter, 2)));
	assert_eq!(counters.get(&equal), Some(&(StatType::Counter, 0))); // Created but never triggered
	assert_eq!(counters.get(&greater), Some(&(StatType::Counter, 2)));
}

#[test]
fn test_equality_intermediate_value_does_not_trigger() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --count; }
		@when (--count = 2) {
			style-rule { collect: --triggered; }
		}
	"#;
	let css_source = ".a {} .b {} .c {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let count = CsskitAtomSet::get_dyn_set().atom_from_str("count");
	let triggered = CsskitAtomSet::get_dyn_set().atom_from_str("triggered");
	assert_eq!(counters.get(&count), Some(&(StatType::Counter, 3)));
	assert_eq!(counters.get(&triggered), Some(&(StatType::Counter, 0))); // Never triggered
}

#[test]
fn test_equality_does_trigger_on_exact_match() {
	let bump = Bump::new();
	let source = r#"
		style-rule { collect: --count; }
		@when (--count = 2) {
			style-rule { collect: --triggered; }
		}
	"#;
	let css_source = ".a {} .b {}"; // Exactly 2 rules

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let count = CsskitAtomSet::get_dyn_set().atom_from_str("count");
	let triggered = CsskitAtomSet::get_dyn_set().atom_from_str("triggered");
	assert_eq!(counters.get(&count), Some(&(StatType::Counter, 2)));
	assert_eq!(counters.get(&triggered), Some(&(StatType::Counter, 2)));
}

#[test]
fn test_equality_triggers_once_then_becomes_false() {
	let bump = Bump::new();
	let source = r#"
		@stat --count { type: counter; }
		@stat --trigger { type: counter; }
		@stat --hit { type: counter; }

		style-rule {
			collect: --count;
			collect: --trigger;
		}

		@when (--trigger = 2) {
			collect: --count;
		}

		@when (--count = 2) {
			diagnostic: "Foo"
			collect: --hit;
		}
	"#;
	let css_source = ".a {} .b {}";

	let (counters, diagnostics) = run(&bump, source, css_source);
	let count = CsskitAtomSet::get_dyn_set().atom_from_str("count");
	let hit = CsskitAtomSet::get_dyn_set().atom_from_str("hit");
	assert_eq!(counters.get(&hit), Some(&(StatType::Counter, 0)));
	assert_eq!(diagnostics.len(), 0);
	assert_eq!(counters.get(&count), Some(&(StatType::Counter, 3)));
}

#[test]
fn test_when_only_rules_with_bytes_and_lines() {
	let bump = Bump::new();
	let source = r#"
		@stat --trigger { type: counter; }
		@stat --byte-count { type: bytes; }
		@stat --line-count { type: lines; }
		@stat --counter { type: counter; }

		style-rule { collect: --trigger; }

		@when (--trigger > 1) {
			collect: --byte-count;
			collect: --line-count;
			collect: --counter;
		}
	"#;
	let css_source = ".a {} .b {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let trigger = CsskitAtomSet::get_dyn_set().atom_from_str("trigger");
	let bytes = CsskitAtomSet::get_dyn_set().atom_from_str("byte-count");
	let lines = CsskitAtomSet::get_dyn_set().atom_from_str("line-count");
	let counter = CsskitAtomSet::get_dyn_set().atom_from_str("counter");

	assert_eq!(counters.get(&trigger), Some(&(StatType::Counter, 2)));
	// For @when-only rules without selectors:
	// - Counter increments by 1 (event counting)
	// - Bytes/lines use the parent span (stylesheet root for top-level @when)
	let css_bytes = css_source.len();
	assert_eq!(counters.get(&bytes), Some(&(StatType::Bytes, css_bytes)));
	assert_eq!(counters.get(&lines), Some(&(StatType::Lines, 1))); // CSS is one line
	assert_eq!(counters.get(&counter), Some(&(StatType::Counter, 1)));
}

#[test]
fn test_invalid_when_threshold_skipped() {
	let bump = Bump::new();
	let source = r#"
		@stat --count { type: counter; }
		@stat --triggered { type: counter; }

		style-rule { collect: --count; }

		@when (--count > invalid) {
			collect: --triggered;
		}
	"#;
	let css_source = ".a {} .b {}";

	let (counters, _diagnostics) = run(&bump, source, css_source);
	let count = CsskitAtomSet::get_dyn_set().atom_from_str("count");
	let triggered = CsskitAtomSet::get_dyn_set().atom_from_str("triggered");

	assert_eq!(counters.get(&count), Some(&(StatType::Counter, 2)));
	// Invalid threshold should skip the @when rule entirely, so --triggered should not exist or be 0
	assert_eq!(counters.get(&triggered), Some(&(StatType::Counter, 0)));
}
