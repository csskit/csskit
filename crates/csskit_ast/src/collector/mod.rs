use crate::{
	CsskitAtomSet, StatDeclarationValue, StatTypeValue,
	rule_block::{
		DiagnosticComponent, DiagnosticFunction, DiagnosticLevel as RuleDiagnosticLevel, NestedRule,
		RuleDeclarationValue,
	},
	selector::{MatchOutput, QuerySelectorList, SelectorMatcher},
	sheet::{Rule, Sheet},
	when_rule::{ComparisonOperator, WhenCondition, WhenFeature},
};
use bumpalo::{Bump, collections::Vec, vec};
use css_ast::{AtomSet, PropertyKind, StyleSheet};
use css_lexer::{Atom, Cursor, RegisteredAtomSet, SourceCursor, SourceOffset, Span, ToSpan};
#[cfg(feature = "miette")]
use miette::{LabeledSpan, MietteDiagnostic, Severity as MietteSeverity};
use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};

#[cfg(test)]
mod tests;

/// Diagnostic severity level for collector diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedDiagnosticLevel {
	Error,
	Warning,
	Advice,
}

#[cfg(feature = "miette")]
impl From<ResolvedDiagnosticLevel> for MietteSeverity {
	fn from(value: ResolvedDiagnosticLevel) -> Self {
		match value {
			ResolvedDiagnosticLevel::Error => MietteSeverity::Error,
			ResolvedDiagnosticLevel::Warning => MietteSeverity::Warning,
			ResolvedDiagnosticLevel::Advice => MietteSeverity::Advice,
		}
	}
}

/// A diagnostic produced by the collector for a matched rule.
#[derive(Debug, Clone)]
pub struct CollectorDiagnostic {
	/// Severity of the diagnostic
	pub severity: ResolvedDiagnosticLevel,
	/// Span of the matched node in the CSS source
	pub span: Span,
	/// Resolved diagnostic message
	pub message: String,
}

impl CollectorDiagnostic {
	/// Create a new collector diagnostic.
	pub fn new(severity: ResolvedDiagnosticLevel, span: Span, message: String) -> Self {
		Self { severity, span, message }
	}

	/// Convert to a Miette diagnostic for display.
	#[cfg(feature = "miette")]
	pub fn into_miette(self) -> MietteDiagnostic {
		let label = LabeledSpan::new_with_span(Some(self.message.clone()), self.span);
		MietteDiagnostic::new(self.message).with_severity(self.severity.into()).with_labels(std::vec![label])
	}
}

/// The type of a stat, determining how values are collected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatType {
	/// Counts matching nodes
	Counter,
	/// Sums byte sizes of matching nodes
	Bytes,
	/// Counts lines in matching nodes
	Lines,
}

pub type Stats = HashMap<Atom<CsskitAtomSet>, (StatType, usize)>;

/// Extracted rule data from rule block declarations.
struct ExtractedRuleData<'a> {
	/// Stats to collect when this rule matches
	collectors: SmallVec<[Atom<CsskitAtomSet>; 1]>,
	/// Optional diagnostic to emit (level, message template components)
	diagnostic: Option<(ResolvedDiagnosticLevel, &'a [DiagnosticComponent])>,
	/// Stats referenced in diagnostic message (as atom bits) - need to be snapshotted
	diagnostic_stats: HashSet<u32>,
}

/// A rule that may be a selector-based rule or a standalone conditional diagnostic.
#[derive(Debug, Clone)]
pub struct CollectionRule<'a> {
	/// The selector to match against CSS nodes (None for conditional diagnostics without selectors)
	pub selector: Option<&'a QuerySelectorList<'a>>,
	/// The stat names to collect when nodes match
	pub collectors: SmallVec<[Atom<CsskitAtomSet>; 1]>,
	/// Per-match diagnostics to emit (level, message template components)
	pub diagnostic: Option<(ResolvedDiagnosticLevel, &'a [DiagnosticComponent])>,
	/// Stats referenced in the diagnostic message (as atom bits) - need to be snapshotted
	pub diagnostic_stats: HashSet<u32>,
	/// Conditions that must all be true for this rule to execute (AND conjunction)
	/// Empty vec means unconditional.
	pub conditions: Vec<'a, &'a WhenCondition<'a>>,
	/// Span context for selectorless rules (from parent NodeRule or StyleSheet root)
	/// Used for bytes/lines collection when there's no selector to match against
	pub parent_span: Span,
	/// A collection of matches with stat snapshots for diagnostic message resolution
	matches: Vec<'a, MatchOutput>,
}

/// The main collector that executes collection rules against CSS.
pub struct Collector<'a> {
	/// The Collector sheet source
	source: &'a str,
	/// Stat definitions from @stat rules
	stats: Stats,
	/// All collection rules (both with and without selectors)
	collection_rules: Vec<'a, CollectionRule<'a>>,
	/// Allocator for string parsing
	allocator: &'a Bump,
}

impl<'a> Collector<'a> {
	/// Extract collection rule data from rule block declarations.
	fn extract_rule_data(
		&mut self,
		declarations: &'a [css_parse::Declaration<'a, RuleDeclarationValue<'a>, ()>],
	) -> ExtractedRuleData<'a> {
		let mut collectors = SmallVec::new();
		let mut diagnostic_level = ResolvedDiagnosticLevel::Advice;
		let mut diagnostic_components = None;

		for decl in declarations {
			match &decl.value {
				RuleDeclarationValue::Collect(dashed_ident) => {
					let name = CsskitAtomSet::get_dyn_set().atom_from_bits(Cursor::from(*dashed_ident).atom_bits());
					collectors.push(name);
					self.stats.entry(name).or_insert((StatType::Counter, 0));
				}
				RuleDeclarationValue::Diagnostic(components) => diagnostic_components = Some(components.as_slice()),
				RuleDeclarationValue::Level(level) => {
					diagnostic_level = match level {
						RuleDiagnosticLevel::Warning(_) => ResolvedDiagnosticLevel::Warning,
						RuleDiagnosticLevel::Advice(_) => ResolvedDiagnosticLevel::Advice,
						RuleDiagnosticLevel::Error(_) => ResolvedDiagnosticLevel::Error,
					}
				}
			}
		}

		let diagnostic = diagnostic_components.map(|c| (diagnostic_level, c));
		let mut diagnostic_stats = HashSet::new();
		if let Some((_, components)) = &diagnostic {
			for component in *components {
				if let DiagnosticComponent::DashedIdent(dashed_ident) = component {
					diagnostic_stats.insert(Cursor::from(*dashed_ident).atom_bits());
				}
			}
		}

		ExtractedRuleData { collectors, diagnostic, diagnostic_stats }
	}

	/// Process nested rules and add them to collection rules.
	/// The `conditions` parameter contains all conditions that must be true for nested rules to execute.
	/// The `parent_span` parameter provides span context for selectorless rules (from parent NodeRule or StyleSheet).
	fn process_nested_rules(
		&mut self,
		nested_rules: &'a [NestedRule<'a>],
		conditions: &Vec<'a, &'a WhenCondition<'a>>,
		parent_span: Span,
	) {
		for nested_rule in nested_rules {
			match nested_rule {
				NestedRule::NodeRule(node_rule) => {
					let rule_data = self.extract_rule_data(&node_rule.block.declarations);
					let node_span = node_rule.to_span();

					self.collection_rules.push(CollectionRule {
						selector: Some(&node_rule.selector),
						collectors: rule_data.collectors,
						diagnostic: rule_data.diagnostic,
						diagnostic_stats: rule_data.diagnostic_stats,
						conditions: conditions.clone(),
						parent_span: node_span,
						matches: vec![in self.allocator],
					});

					self.process_nested_rules(&node_rule.block.rules, conditions, node_span);
				}
				NestedRule::WhenRule(when_rule) => {
					if !when_rule.condition.is_valid(self.source) {
						continue;
					}

					let mut combined_conditions = conditions.clone();
					combined_conditions.push(&when_rule.condition);

					let rule_data = self.extract_rule_data(&when_rule.block.declarations);
					if !rule_data.collectors.is_empty() || rule_data.diagnostic.is_some() {
						self.collection_rules.push(CollectionRule {
							selector: None,
							collectors: rule_data.collectors,
							diagnostic: rule_data.diagnostic,
							diagnostic_stats: rule_data.diagnostic_stats,
							conditions: combined_conditions.clone(),
							parent_span,
							matches: vec![in self.allocator],
						});
					}

					self.process_nested_rules(&when_rule.block.rules, &combined_conditions, parent_span);
				}
				NestedRule::Unknown(_) => {}
			}
		}
	}

	/// Create a new Collector from a parsed Sheet.
	pub fn new(sheet: &'a Sheet<'a>, source: &'a str, allocator: &'a Bump) -> Self {
		let stats = HashMap::new();
		let collection_rules = vec![in allocator];
		let mut collector = Self { source, stats, collection_rules, allocator };

		for rule in sheet.rules.iter() {
			match rule {
				Rule::Stat(stat_rule) => {
					let cursor = Cursor::from(stat_rule.name);
					let name = CsskitAtomSet::get_dyn_set().atom_from_bits(cursor.atom_bits());
					let mut stat_type = StatType::Counter;
					for decl in &stat_rule.block.declarations {
						match &decl.value {
							StatDeclarationValue::Type(type_value) => {
								stat_type = match type_value {
									StatTypeValue::Counter(_) => StatType::Counter,
									StatTypeValue::Bytes(_) => StatType::Bytes,
									StatTypeValue::Lines(_) => StatType::Lines,
								};
							}
						}
					}
					collector.stats.insert(name, (stat_type, 0));
				}
				Rule::NodeRule(node_rule) => {
					let rule_data = collector.extract_rule_data(&node_rule.block.declarations);
					let node_span = node_rule.to_span();

					collector.collection_rules.push(CollectionRule {
						selector: Some(&node_rule.selector),
						collectors: rule_data.collectors,
						diagnostic: rule_data.diagnostic,
						diagnostic_stats: rule_data.diagnostic_stats,
						conditions: vec![in allocator],
						parent_span: node_span,
						matches: vec![in allocator],
					});

					collector.process_nested_rules(&node_rule.block.rules, &vec![in allocator], node_span);
				}
				Rule::WhenRule(when_rule) => {
					if !when_rule.condition.is_valid(source) {
						continue;
					}

					let conditions = vec![in allocator; &when_rule.condition];

					let rule_data = collector.extract_rule_data(&when_rule.block.declarations);
					if !rule_data.collectors.is_empty() || rule_data.diagnostic.is_some() {
						collector.collection_rules.push(CollectionRule {
							selector: None,
							collectors: rule_data.collectors,
							diagnostic: rule_data.diagnostic,
							diagnostic_stats: rule_data.diagnostic_stats,
							conditions: conditions.clone(),
							// Use a placeholder span - will be updated with stylesheet span in collect()
							parent_span: Span::new(SourceOffset(0), SourceOffset(0)),
							matches: vec![in allocator],
						});
					}

					collector.process_nested_rules(
						&when_rule.block.rules,
						&conditions,
						Span::new(SourceOffset(0), SourceOffset(0)),
					);
				}
				_ => {}
			}
		}

		collector
	}

	/// Resolve a diagnostic message template for a specific match.
	fn resolve_diagnostic_message(
		&self,
		components: &[DiagnosticComponent],
		match_output: &MatchOutput,
		css_source: &str,
	) -> String {
		let mut message = String::new();
		for component in components {
			match component {
				DiagnosticComponent::String(string_token) => {
					let cursor = Cursor::from(*string_token);
					let source_slice = cursor.str_slice(self.source);
					let source_cursor = SourceCursor::from(cursor, source_slice);
					message.push_str(source_cursor.parse(self.allocator).as_str());
				}
				DiagnosticComponent::DashedIdent(dashed_ident) => {
					let stat_bits = Cursor::from(*dashed_ident).atom_bits();
					let count = match_output
						.stat_snapshot
						.iter()
						.find(|(bits, _)| *bits == stat_bits)
						.map(|(_, count)| *count)
						.unwrap_or(0);
					message.push_str(&count.to_string());
				}
				DiagnosticComponent::Function(func) => match func {
					DiagnosticFunction::Attr(attr_func) => {
						let atom = CsskitAtomSet::from_bits(Cursor::from(attr_func.name).atom_bits());
						let property_kind = atom.to_property_kind();
						let value = match property_kind {
							Some(PropertyKind::Name) => match_output.properties.name,
							_ => None,
						};
						if let Some(cursor) = value {
							message.push_str(cursor.str_slice(css_source));
						}
					}
					DiagnosticFunction::Size(_size_func) => {
						message.push_str(&match_output.size.to_string());
					}
				},
			}
		}
		message
	}

	/// Evaluate a single if condition feature against stats.
	fn evaluate_if_feature(&self, feature: &WhenFeature) -> bool {
		let stat_bits = Cursor::from(feature.stat).atom_bits();
		let stat_name = CsskitAtomSet::get_dyn_set().atom_from_bits(stat_bits);
		let stat_value = self.stats.get(&stat_name).map(|(_, count)| *count).unwrap_or(0);

		let threshold_cursor = Cursor::from(feature.value);
		let threshold_str = threshold_cursor.str_slice(self.source);
		let threshold = threshold_str.parse::<usize>().unwrap_or(0);

		match feature.operator {
			ComparisonOperator::GreaterThan(_) => stat_value > threshold,
			ComparisonOperator::LessThan(_) => stat_value < threshold,
			ComparisonOperator::GreaterThanOrEqual(_) => stat_value >= threshold,
			ComparisonOperator::LessThanOrEqual(_) => stat_value <= threshold,
			ComparisonOperator::Equal(_) => stat_value == threshold,
		}
	}

	/// Evaluate an if condition (supporting and/or/not).
	fn evaluate_if_condition(&self, condition: &WhenCondition) -> bool {
		match condition {
			WhenCondition::Is(feature) => self.evaluate_if_feature(feature),
			WhenCondition::Not(_, feature) => !self.evaluate_if_feature(feature),
			WhenCondition::And(features) => features.iter().all(|(f, _)| self.evaluate_if_feature(f)),
			WhenCondition::Or(features) => features.iter().any(|(f, _)| self.evaluate_if_feature(f)),
		}
	}

	/// Process a single rule, collecting matches and updating stats.
	/// Returns true if rule had any effect (matches or is a non-selector diagnostic).
	fn process_rule(&mut self, rule_idx: usize, stylesheet: &StyleSheet<'_>, css_source: &str) -> bool {
		let (selector, diagnostic_stats, collectors) = {
			let rule = &self.collection_rules[rule_idx];
			(rule.selector, rule.diagnostic_stats.clone(), rule.collectors.clone())
		};

		let Some(selector) = selector else {
			let parent_span = self.collection_rules[rule_idx].parent_span;

			let mut match_output = MatchOutput {
				span: parent_span,
				node_id: css_ast::NodeId::StyleRule,
				properties: Default::default(),
				size: 0,
				stat_snapshot: SmallVec::new(),
			};

			for &stat_bits in &diagnostic_stats {
				if let Some((name, &(_, count))) = self.stats.iter().find(|(name, _)| name.as_bits() == stat_bits) {
					match_output.stat_snapshot.push((name.as_bits(), count));
				}
			}

			self.collection_rules[rule_idx].matches.push(match_output);

			for collector_name in &collectors {
				if let Some(&mut (stat_type, ref mut count)) = self.stats.get_mut(collector_name) {
					match stat_type {
						StatType::Counter => {
							*count += 1;
						}
						StatType::Bytes => {
							*count += (parent_span.end().0 - parent_span.start().0) as usize;
						}
						StatType::Lines => {
							let text = &css_source[parent_span.start().0 as usize..parent_span.end().0 as usize];
							if !text.is_empty() {
								*count += text.lines().count();
							}
						}
					}
				}
			}

			return true;
		};

		let matcher = SelectorMatcher::new(selector, self.source, css_source);
		let mut had_matches = false;

		for mut match_output in matcher.run(stylesheet) {
			had_matches = true;
			let span = match_output.span;

			for &stat_bits in &diagnostic_stats {
				if let Some((name, &(_, count))) = self.stats.iter().find(|(name, _)| name.as_bits() == stat_bits) {
					match_output.stat_snapshot.push((name.as_bits(), count));
				}
			}

			self.collection_rules[rule_idx].matches.push(match_output);
			for collector_name in &collectors {
				if let Some(&mut (stat_type, ref mut count)) = self.stats.get_mut(collector_name) {
					match stat_type {
						StatType::Counter => {
							*count += 1;
						}
						StatType::Bytes => {
							*count += (span.end().0 - span.start().0) as usize;
						}
						StatType::Lines => {
							let text = &css_source[span.start().0 as usize..span.end().0 as usize];
							if !text.is_empty() {
								*count += text.lines().count();
							}
						}
					}
				}
			}
		}

		had_matches
	}

	/// Collect diagnostics & stats on a CSS stylesheet.
	pub fn collect(&mut self, stylesheet: &StyleSheet<'_>, css_source: &str) {
		let stylesheet_span = stylesheet.to_span();
		let zero_span = Span::new(SourceOffset(0), SourceOffset(0));
		for rule in &mut self.collection_rules {
			if rule.parent_span == zero_span {
				rule.parent_span = stylesheet_span;
			}
		}

		let mut processed = vec![in self.allocator; false; self.collection_rules.len()];

		// First, execute all unconditional rules (conditions is empty)
		for idx in 0..self.collection_rules.len() {
			if self.collection_rules[idx].conditions.is_empty() {
				self.process_rule(idx, stylesheet, css_source);
				processed[idx] = true;
			}
		}

		loop {
			let mut made_progress = false;

			for idx in 0..self.collection_rules.len() {
				if processed[idx] {
					continue;
				}
				let rule = &self.collection_rules[idx];
				if rule.conditions.iter().all(|cond| self.evaluate_if_condition(cond)) {
					self.process_rule(idx, stylesheet, css_source);
					processed[idx] = true;
					made_progress = true;
				}
			}

			// If no rules became active, we're done
			if !made_progress {
				break;
			}
		}
	}

	/// Get all diagnostics from matched rules.
	///
	/// This method should be called after `collect()` to retrieve diagnostics
	/// for all rules that had diagnostic templates and matched nodes.
	pub fn diagnostics<'b>(&'b self, css_source: &'b str) -> impl Iterator<Item = CollectorDiagnostic> + 'b {
		self.collection_rules.iter().flat_map(move |rule| {
			let Some((severity, components)) = &rule.diagnostic else {
				return vec![in self.allocator].into_iter();
			};

			let mut diagnostics = vec![in self.allocator];
			for match_output in rule.matches.iter() {
				let message = self.resolve_diagnostic_message(components, match_output, css_source);
				diagnostics.push(CollectorDiagnostic::new(*severity, match_output.span, message));
			}
			diagnostics.into_iter()
		})
	}

	/// Get the collected statistics.
	///
	/// Returns a reference to the statistics map containing stat names and their (type, count) values.
	pub fn stats(&self) -> &Stats {
		&self.stats
	}
}
