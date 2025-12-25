use crate::assert_query;
use css_ast::NodeId;

#[test]
fn match_basic_types() {
	assert_query!("a { color: red; }", "style-rule", 1, [NodeId::StyleRule]);
	assert_query!("a, b { color: red; }", "selector-list", 1, [NodeId::SelectorList]);
}

#[test]
fn match_multiple_selectors() {
	assert_query!(
		"a { color: red; } @media screen {}",
		"style-rule, media-rule",
		2,
		[NodeId::StyleRule, NodeId::MediaRule]
	);
	assert_query!(
		"a {} @media screen {} b {}",
		"style-rule, media-rule",
		3,
		[NodeId::StyleRule, NodeId::MediaRule, NodeId::StyleRule]
	);
}

#[test]
fn descendant_combinator() {
	assert_query!("a { color: red; }", "style-rule selector-list", 1, [NodeId::SelectorList]);
	assert_query!(
		"@media screen { a { color: red; } }",
		"media-rule style-rule selector-list",
		1,
		[NodeId::SelectorList]
	);
}

#[test]
fn descendant_combinator_no_match() {
	assert_query!("@media screen {}", "style-rule selector-list", 0);
}

#[test]
fn child_combinator() {
	assert_query!("a { color: red; }", "style-rule > selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a { color: red; }", "style-sheet > style-rule > selector-list", 1, [NodeId::SelectorList]);
}

#[test]
fn child_combinator_no_match() {
	assert_query!("a { color: red; }", "style-sheet > selector-list", 0);
}

#[test]
fn next_sibling_combinator() {
	assert_query!("a {} b {}", "style-rule + style-rule", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {} a {}", "media-rule + style-rule", 1, [NodeId::StyleRule]);
}

#[test]
fn next_sibling_combinator_no_match() {
	assert_query!("a {}", "style-rule + style-rule", 0);
	assert_query!("a {} @media screen {}", "media-rule + style-rule", 0);
}

#[test]
fn sibling_with_attribute_match() {
	assert_query!("@keyframes spin {} a {}", "keyframes-rule[name=spin] + style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"@keyframes spin {} a {} @keyframes other {} b {}",
		"keyframes-rule[name=spin] + style-rule",
		1,
		[NodeId::StyleRule]
	);
}

#[test]
fn sibling_with_attribute_no_match() {
	assert_query!("@keyframes other {} a {}", "keyframes-rule[name=spin] + style-rule", 0);
}

#[test]
fn sibling_with_pseudo() {
	assert_query!("@media screen {} a {}", "*:at-rule + style-rule", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {}", "*:rule + style-rule", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {} a {}", "*:rule + style-rule", 1, [NodeId::StyleRule]);
}

#[test]
fn sibling_with_pseudo_at_rule_no_match() {
	assert_query!("a {} b {}", "*:at-rule + style-rule", 0);
}

#[test]
fn subsequent_sibling_combinator() {
	assert_query!("a {} b {} c {}", "style-rule ~ style-rule", 2, [NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("@media screen {} @keyframes foo {} a {}", "media-rule ~ style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"@keyframes spin {} @keyframes other {} a {}",
		"keyframes-rule[name=spin] ~ style-rule",
		1,
		[NodeId::StyleRule]
	);
}

#[test]
fn subsequent_sibling_combinator_no_match() {
	assert_query!("a {} @media screen {}", "media-rule ~ style-rule", 0);
	assert_query!("@keyframes other {} @keyframes another {} a {}", "keyframes-rule[name=spin] ~ style-rule", 0);
}

#[test]
fn chained_combinators() {
	assert_query!("a {} b { color: red; }", "style-rule + style-rule selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a {} b { color: red; }", "style-rule + style-rule > selector-list", 1, [NodeId::SelectorList]);
	assert_query!(
		"a {} @media screen {} b { color: red; }",
		"style-rule ~ style-rule selector-list",
		1,
		[NodeId::SelectorList]
	);
	assert_query!("@media screen { a {} b {} }", "media-rule style-rule + style-rule", 1, [NodeId::StyleRule]);
}

#[test]
fn chained_combinators_no_match() {
	assert_query!("a { color: red; }", "style-rule + style-rule selector-list", 0);
}

#[test]
fn chained_next_sibling_match() {
	assert_query!("a {} b {} c {}", "style-rule + style-rule + style-rule", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {} c {} d {}", "style-rule + style-rule + style-rule + style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"@media screen {} @keyframes foo {} a {}",
		"media-rule + keyframes-rule + style-rule",
		1,
		[NodeId::StyleRule]
	);
}

#[test]
fn chained_next_sibling_gap_no_match() {
	assert_query!("a {} @media screen {} b {} c {}", "style-rule + style-rule + style-rule", 0);
	assert_query!("@keyframes foo {} @media screen {} a {}", "media-rule + keyframes-rule + style-rule", 0);
}

#[test]
fn chained_subsequent_sibling_match() {
	assert_query!("a {} b {} c {}", "style-rule ~ style-rule ~ style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"a {} @media screen {} b {} @media print {} c {}",
		"style-rule ~ style-rule ~ style-rule",
		1,
		[NodeId::StyleRule]
	);
	assert_query!("a {} b {} c {} d {}", "style-rule ~ style-rule ~ style-rule ~ style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"@media screen {} @keyframes foo {} a {}",
		"media-rule ~ keyframes-rule ~ style-rule",
		1,
		[NodeId::StyleRule]
	);
}

#[test]
fn chained_subsequent_sibling_no_match() {
	// Order matters for ~ too - A must appear before B, B before C
	assert_query!("@keyframes foo {} @media screen {} a {}", "media-rule ~ keyframes-rule ~ style-rule", 0);
}

#[test]
fn mixed_sibling_combinators_match() {
	// A + B ~ C: C has B somewhere before, B immediately follows A
	assert_query!("a {} b {} @media screen {} c {}", "style-rule + style-rule ~ style-rule", 1, [NodeId::StyleRule]);
	// A ~ B + C: C immediately follows B, B has A somewhere before
	assert_query!("a {} @media screen {} b {} c {}", "style-rule ~ style-rule + style-rule", 1, [NodeId::StyleRule]);
}

#[test]
fn first_child() {
	assert_query!("a {} b {} c {}", "style-rule:first-child", 1, [NodeId::StyleRule]);
}

#[test]
fn first_child_no_match() {
	assert_query!("@media screen {} a {}", "style-rule:first-child", 0);
}

#[test]
fn last_child() {
	assert_query!("a {} b {} c {}", "style-rule:last-child", 1, [NodeId::StyleRule]);
	assert_query!("a {}", "style-rule:last-child", 1, [NodeId::StyleRule]);
}

#[test]
fn last_child_no_match() {
	assert_query!("a {} @media screen {}", "style-rule:last-child", 0);
}

#[test]
fn only_child() {
	assert_query!("a {}", "style-rule:only-child", 1, [NodeId::StyleRule]);
}

#[test]
fn only_child_no_match() {
	assert_query!("a {} b {}", "style-rule:only-child", 0);
}

#[test]
fn nth_child_index() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(2)", 1, [NodeId::StyleRule]);
}

#[test]
fn nth_child_odd() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(odd)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn nth_child_even() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(even)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn nth_child_formula() {
	assert_query!(
		"a {} b {} c {} d {} e {} f {}",
		"style-rule:nth-child(3n)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!(
		"a {} b {} c {} d {} e {} f {}",
		"style-rule:nth-child(2n+1)",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn nth_child_negative_formula() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(-n+2)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn nth_child_negative_formula_first_three() {
	assert_query!(
		"a {} b {} c {} d {} e {}",
		"style-rule:nth-child(-n+3)",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn nth_child_zero() {
	assert_query!("a {} b {} c {}", "style-rule:nth-child(0)", 0);
}

#[test]
fn nth_child_large_index() {
	assert_query!("a {} b {} c {}", "style-rule:nth-child(100)", 0);
}

#[test]
fn nth_child_n_matches_all() {
	assert_query!(
		"a {} b {} c {}",
		"style-rule:nth-child(n)",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn nth_child_n_plus_one_matches_all() {
	assert_query!(
		"a {} b {} c {}",
		"style-rule:nth-child(n+1)",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn nth_last_child_index() {
	assert_query!("a {} b {} c {}", "style-rule:nth-last-child(1)", 1, [NodeId::StyleRule]);
}

#[test]
fn nth_last_child_second() {
	assert_query!("a {} b {} c {}", "style-rule:nth-last-child(2)", 1, [NodeId::StyleRule]);
}

#[test]
fn nth_last_child_odd() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(odd)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn nth_last_child_even() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(even)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn nth_last_child_formula() {
	assert_query!(
		"a {} b {} c {} d {} e {} f {}",
		"style-rule:nth-last-child(2n)",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn nth_last_child_negative_formula() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(-n+2)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn last_child_and_last_of_type() {
	assert_query!("a {}", "style-rule:last-child:last-of-type", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {} a {}", "style-rule:last-child:last-of-type", 1, [NodeId::StyleRule]);
}

#[test]
fn only_child_and_only_of_type() {
	assert_query!("a {}", "style-rule:only-child:only-of-type", 1, [NodeId::StyleRule]);
}

#[test]
fn first_of_type() {
	assert_query!("@media screen {} a {} b {}", "style-rule:first-of-type", 1, [NodeId::StyleRule]);
}

#[test]
fn first_of_type_is_first() {
	assert_query!("a {} b {} @media screen {}", "style-rule:first-of-type", 1, [NodeId::StyleRule]);
}

#[test]
fn last_of_type() {
	assert_query!("a {} b {} @media screen {}", "style-rule:last-of-type", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {} a {} b {}", "style-rule:last-of-type", 1, [NodeId::StyleRule]);
}

#[test]
fn only_of_type() {
	assert_query!("@media screen {} a {} @keyframes foo {}", "style-rule:only-of-type", 1, [NodeId::StyleRule]);
}

#[test]
fn only_of_type_no_match() {
	assert_query!("a {} b {}", "style-rule:only-of-type", 0);
}

#[test]
fn nth_of_type() {
	assert_query!("@media screen {} a {} b {} c {}", "style-rule:nth-of-type(2)", 1, [NodeId::StyleRule]);
}

#[test]
fn nth_of_type_odd() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-of-type(odd)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn nth_last_of_type() {
	assert_query!("a {} b {} c {} @media screen {}", "style-rule:nth-last-of-type(2)", 1, [NodeId::StyleRule]);
}

#[test]
fn nth_last_of_type_even() {
	assert_query!(
		"a {} b {} c {} d {}",
		"style-rule:nth-last-of-type(even)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn attribute_name_selector() {
	assert_query!("a { color: red; background: blue; margin: 10px; }", "[name=color]", 1, [NodeId::StyleValue]);
	assert_query!(
		"a { color: red; } b { color: blue; background: green; }",
		"[name=color]",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
	assert_query!("a { background-color: red; }", "[name='background-color']", 1, [NodeId::StyleValue]);
	assert_query!("a { COLOR: red; }", "[name=color]", 1, [NodeId::StyleValue]);
	assert_query!("a { color: red; margin: 10px; }", "[name]", 2, [NodeId::StyleValue, NodeId::StyleValue]);
	assert_query!("a { color: red; margin: 10px; }", "[name|=\"\"]", 2, [NodeId::StyleValue, NodeId::StyleValue]);
	assert_query!("a { color: red; margin: 10px; }", "[name*=\"\"]", 2, [NodeId::StyleValue, NodeId::StyleValue]);
	assert_query!("a { color: red; margin: 10px; }", "[name^=\"\"]", 2, [NodeId::StyleValue, NodeId::StyleValue]);
	assert_query!("a { color: red; margin: 10px; }", "[name$=\"\"]", 2, [NodeId::StyleValue, NodeId::StyleValue]);
}

#[test]
fn attribute_name_selector_no_match() {
	assert_query!("a { color: red; background: blue; }", "[name=margin]", 0);
}

#[test]
fn attribute_with_pseudo() {
	assert_query!("a { color: red; margin: 10px; }", "[name=\"\"]", 0);
	assert_query!(
		"a { color: red !important; margin: 10px !important; }",
		"[name=margin]:important",
		1,
		[NodeId::StyleValue]
	);
	assert_query!("a { color: red; margin: 10px; }", "[name~=\"\"]", 0);
}

#[test]
fn custom_properties_pseudo() {
	assert_query!(
		"a { --my-color: red; color: blue; --spacing: 10px; }",
		"*:custom",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
	assert_query!(":root { --primary: blue; }", "*:custom", 1, [NodeId::StyleValue]);
}

#[test]
fn custom_properties_pseudo_no_match() {
	assert_query!("a { color: red; }", "*:custom", 0);
	assert_query!("a { color: red; background: blue; }", "*:custom", 0);
}

#[test]
fn important_pseudo() {
	assert_query!("a { color: red !important; }", "*:important", 1, [NodeId::StyleValue]);
	assert_query!(
		"a { color: red !important; margin: 10px; padding: 5px !important; }",
		"*:important",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
	assert_query!(
		"a { color: red !important; margin: 10px !important; }",
		"[name=color]:important",
		1,
		[NodeId::StyleValue]
	);
}

#[test]
fn important_pseudo_no_match() {
	assert_query!("a { color: red; margin: 10px; }", "*:important", 0);
}

#[test]
fn prefixed_pseudo() {
	assert_query!("a { -webkit-transform: rotate(45deg); }", "*:prefixed", 1, [NodeId::StyleValue]);
	assert_query!(
		"a { -webkit-transform: rotate(45deg); -moz-appearance: none; }",
		"*:prefixed",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
	assert_query!(
		"a { -webkit-transform: rotate(45deg); -moz-appearance: none; }",
		"*:prefixed(webkit)",
		1,
		[NodeId::StyleValue]
	);
	assert_query!(
		"a { -webkit-transform: rotate(45deg); -moz-appearance: none; }",
		"*:prefixed(moz)",
		1,
		[NodeId::StyleValue]
	);
	assert_query!(
		"@-webkit-keyframes spin { to { opacity: 1; } }",
		"*:prefixed(webkit)",
		1,
		[NodeId::WebkitKeyframesRule]
	);
	assert_query!(
		"@-webkit-keyframes spin { to { opacity: 1; } }",
		"webkit-keyframes-rule:prefixed",
		1,
		[NodeId::WebkitKeyframesRule]
	);
}

#[test]
fn prefixed_pseudo_no_match() {
	assert_query!("a { color: red; margin: 10px; }", "*:prefixed", 0);
	assert_query!("a { --animate-duration: 1s; --animate-delay: 1s; }", "*:prefixed", 0);
}

#[test]
fn prefixed_unknown_vendor_string_fallback() {
	assert_query!("a { -webkit-foo: bar; }", ":prefixed(webkit)", 1, [NodeId::StyleValue]);
	assert_query!("a { -moz-foo: bar; }", ":prefixed(moz)", 1, [NodeId::StyleValue]);
	assert_query!("a { -webkit-foo: bar; }", ":prefixed(moz)", 0);
	assert_query!("a { -moz-foo: bar; }", ":prefixed(webkit)", 0);
	assert_query!("a { -webkit-foo: bar; }", ":prefixed(unknown)", 0);
	assert_query!("a { color: red; }", ":prefixed(webkit)", 0);
	assert_query!(
		"a { -webkit-animation-duration: 1s; -moz-unknown: value; }",
		"*:prefixed(webkit)",
		1,
		[NodeId::StyleValue]
	);
	assert_query!(
		"a { -webkit-animation-duration: 1s; -moz-unknown: value; }",
		"*:prefixed(moz)",
		1,
		[NodeId::StyleValue]
	);
	assert_query!(
		"a { -webkit-animation-duration: 1s; -webkit-animation-delay: 2s; }",
		"*:prefixed",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
}

#[test]
fn shorthand_pseudo() {
	assert_query!("a { margin: 10px; }", "*:shorthand", 1, [NodeId::StyleValue]);
	assert_query!(
		"a { margin: 10px; padding: 5px; border: 1px solid; }",
		"*:shorthand",
		3,
		[NodeId::StyleValue, NodeId::StyleValue, NodeId::StyleValue]
	);
}

#[test]
fn shorthand_pseudo_no_match() {
	assert_query!("a { margin-top: 10px; }", "*:shorthand", 0);
}

#[test]
fn longhand_pseudo() {
	assert_query!("a { color: red; padding-top: 5px; }", "*:longhand", 2, [NodeId::StyleValue, NodeId::StyleValue]);
}

#[test]
fn longhand_pseudo_no_match() {
	assert_query!("a { margin: 10px; }", "*:longhand", 0);
}

#[test]
fn property_type_pseudo() {
	assert_query!("a { color: red; margin: 10px; }", "*:property-type(color)", 1, [NodeId::StyleValue]);
	assert_query!(
		"a { width: 100px; height: 50px; color: red; }",
		"*:property-type(sizing)",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
	assert_query!(
		"a { animation-name: spin; animation-duration: 1s; color: red; }",
		"*:property-type(animation)",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
	assert_query!("a { background-color: red; }", "*:property-type(backgrounds)", 1, [NodeId::StyleValue]);
}

#[test]
fn computed_pseudo() {
	assert_query!("a { width: calc(100% - 20px); }", "*:computed", 1, [NodeId::StyleValue]);
	assert_query!("a { color: var(--primary); }", "*:computed", 1, [NodeId::StyleValue]);
}

#[test]
fn computed_pseudo_no_match() {
	assert_query!("a { color: red; width: 100px; }", "*:computed", 0);
}

#[test]
fn unknown_pseudo() {
	assert_query!("a { not-a-real-property: value; }", "*:unknown", 1, [NodeId::StyleValue]);
}

#[test]
fn unknown_pseudo_no_match() {
	assert_query!("a { color: red; margin: 10px; }", "*:unknown", 0);
}

#[test]
fn at_rule_pseudo() {
	assert_query!(
		"@media screen {} a {} @keyframes foo {}",
		"*:at-rule",
		2,
		[NodeId::MediaRule, NodeId::KeyframesRule]
	);
}

#[test]
fn at_rule_pseudo_no_match() {
	assert_query!("a {}", "*:at-rule", 0);
	assert_query!("a { color: red; }", "*:at-rule", 0);
}

#[test]
fn rule_pseudo() {
	assert_query!("@media screen {} a {}", "*:rule", 2, [NodeId::MediaRule, NodeId::StyleRule]);
}

#[test]
fn function_pseudo() {
	assert_query!("a { color: rgb(255, 0, 0); }", "*:function", 1, [NodeId::ColorFunction]);
	assert_query!(
		"a { background: linear-gradient(red, blue); transform: rotate(45deg); }",
		"*:function",
		2,
		[NodeId::TransformFunction, NodeId::RotateFunction]
	);
}

#[test]
fn nested_pseudo() {
	assert_query!("a { & b { color: red; } }", "style-rule:nested", 1, [NodeId::StyleRule]);
}

#[test]
fn nested_pseudo_no_match() {
	assert_query!("a { color: red; }", "style-rule:nested", 0);
}

#[test]
fn root_pseudo() {
	assert_query!("a {}", "style-sheet:root", 1, [NodeId::StyleSheet]);
}

#[test]
fn root_pseudo_no_match() {
	assert_query!("a {}", "style-rule:root", 0);
}

#[test]
fn empty_pseudo() {
	assert_query!("a {}", "style-rule:empty", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {}", "media-rule:empty", 1, [NodeId::MediaRule]);
}

#[test]
fn empty_pseudo_no_match() {
	assert_query!("a { color: red; }", "style-rule:empty", 0);
	assert_query!("@media screen { a {} }", "media-rule:empty", 0);
}

#[test]
fn not_empty() {
	assert_query!("a { color: red; } a {}", "style-rule:not(:empty)", 1, [NodeId::StyleRule]);
}

#[test]
fn universal_matches_all() {
	// Depth-first traversal from root
	assert_query!(
		"a { color: red; }",
		"*",
		10,
		[
			NodeId::StyleSheet,
			NodeId::StyleRule,
			NodeId::SelectorList,
			NodeId::CompoundSelector,
			NodeId::Tag,
			NodeId::HtmlTag,
			NodeId::StyleValue,
			NodeId::StyleValue,
			NodeId::ColorStyleValue,
			NodeId::Color
		]
	);
	assert_query!(
		"a {} b {}",
		"*:first-child",
		10,
		[
			NodeId::StyleSheet,
			NodeId::StyleRule,
			NodeId::SelectorList,
			NodeId::CompoundSelector,
			NodeId::Tag,
			NodeId::HtmlTag,
			NodeId::SelectorList,
			NodeId::CompoundSelector,
			NodeId::Tag,
			NodeId::HtmlTag
		]
	);
	assert_query!(
		"a { color: red; }",
		"style-rule *",
		8,
		[
			NodeId::SelectorList,
			NodeId::CompoundSelector,
			NodeId::Tag,
			NodeId::HtmlTag,
			NodeId::StyleValue,
			NodeId::StyleValue,
			NodeId::ColorStyleValue,
			NodeId::Color
		]
	);
}

#[test]
fn not_pseudo() {
	// :not(media-rule) matches all nodes except MediaRule
	assert_query!("a {} @media screen {} b {}", "*", 13);
	assert_query!("a {} @media screen {} b {}", "media-rule", 1, [NodeId::MediaRule]);
	assert_query!("a {} @media screen {} b {}", ":not(media-rule)", 12);

	// :not(style-rule) matches all nodes except StyleRule
	assert_query!("a {} @media screen {} b {}", "*", 13);
	assert_query!("a {} @media screen {} b {}", "style-rule", 2, [NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("a {} @media screen {} b {}", ":not(style-rule)", 11);
}

#[test]
fn not_with_attribute_excludes_match() {
	assert_query!("a { color: red; margin: 10px; }", "[name]:not([name=color])", 1, [NodeId::StyleValue]);
}

#[test]
fn not_with_attribute_all_match() {
	assert_query!(
		"a { color: red; margin: 10px; }",
		"[name]:not([name=padding])",
		2,
		[NodeId::StyleValue, NodeId::StyleValue]
	);
}

#[test]
fn not_with_type_and_attribute() {
	assert_query!(
		"@keyframes spin {} @keyframes other {}",
		"keyframes-rule:not([name=spin])",
		1,
		[NodeId::KeyframesRule]
	);
}

#[test]
fn not_type_with_attribute() {
	assert_query!(
		"@keyframes spin {} @keyframes other {}",
		"keyframes-rule",
		2,
		[NodeId::KeyframesRule, NodeId::KeyframesRule]
	);
	assert_query!("@keyframes spin {} @keyframes other {}", "keyframes-rule[name=spin]", 1, [NodeId::KeyframesRule]);
	assert_query!(
		"@keyframes spin {} @keyframes other {}",
		"keyframes-rule:not(keyframes-rule[name=spin])",
		1,
		[NodeId::KeyframesRule]
	);
}

#[test]
fn not_with_pseudo_important() {
	assert_query!("a { color: red !important; margin: 10px; }", "[name]:not(:important)", 1, [NodeId::StyleValue]);
}

#[test]
fn not_with_pseudo_custom() {
	// 14 nodes: includes Declaration nodes (visited as StyleValue) + their value children
	let total = assert_query!("a { --color: red; margin: 10px; }", "*", 14);
	let custom = assert_query!("a { --color: red; margin: 10px; }", "*:custom", 1, [NodeId::StyleValue]);
	assert_query!("a { --color: red; margin: 10px; }", "*:not(:custom)", total.len() - custom.len());
}

#[test]
fn not_with_pseudo_shorthand() {
	// 16 nodes: includes Declaration nodes (visited as StyleValue) + their value children
	let total = assert_query!("a { margin: 10px; color: red; }", "*", 16);
	let shorthand = assert_query!("a { margin: 10px; color: red; }", "*:shorthand", 1, [NodeId::StyleValue]);
	let longhand = assert_query!("a { margin: 10px; color: red; }", "*:longhand", 1, [NodeId::StyleValue]);
	assert_query!("a { margin: 10px; color: red; }", "*:not(:shorthand)", total.len() - shorthand.len());
	// Ensure total = shorthand + longhand + non-declaration nodes
	assert!(total.len() > shorthand.len() + longhand.len());
}

#[test]
fn not_at_rule_on_nodes() {
	let total = assert_query!("a {} @media screen {} b {}", "*", 13);
	let at_rules = assert_query!("a {} @media screen {} b {}", "*:at-rule", 1, [NodeId::MediaRule]);
	assert_query!("a {} @media screen {} b {}", "*:not(:at-rule)", total.len() - at_rules.len());
}

#[test]
fn not_with_type_and_pseudo() {
	assert_query!("a {} b {} c {}", "style-rule:not(:first-child)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn not_with_nested_pseudo() {
	assert_query!("a { & b { color: red; } }", "style-rule:not(:nested)", 1, [NodeId::StyleRule]);
}

#[test]
fn not_type_with_filter() {
	assert_query!("a {} b {} c {}", "style-rule", 3, [NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("a {} b {} c {}", "style-rule:first-child", 1, [NodeId::StyleRule]);
	assert_query!(
		"a {} b {} c {}",
		"style-rule:not(style-rule:first-child)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn not_with_child_combinator() {
	assert_query!(
		"a {} @media screen { b {} } c {}",
		"style-rule",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!("a {} @media screen { b {} } c {}", "media-rule > style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"a {} @media screen { b {} } c {}",
		"style-rule:not(media-rule > style-rule)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn not_with_descendant_combinator() {
	assert_query!(
		"a {} @media screen { @supports (color:red) { b {} } } c {}",
		"style-rule",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!(
		"a {} @media screen { @supports (color:red) { b {} } } c {}",
		"media-rule style-rule",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"a {} @media screen { @supports (color:red) { b {} } } c {}",
		"style-rule:not(media-rule style-rule)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn not_with_subsequent_sibling() {
	assert_query!(
		"a {} @media screen {} b {} c {}",
		"style-rule",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!(
		"a {} @media screen {} b {} c {}",
		"media-rule ~ style-rule",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!("a {} @media screen {} b {} c {}", "style-rule:not(media-rule ~ style-rule)", 1, [NodeId::StyleRule]);
}

#[test]
fn not_with_next_sibling() {
	assert_query!(
		"a {} @media screen {} b {} c {}",
		"style-rule",
		3,
		[NodeId::StyleRule, NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!("a {} @media screen {} b {} c {}", "media-rule + style-rule", 1, [NodeId::StyleRule]);
	assert_query!(
		"a {} @media screen {} b {} c {}",
		"style-rule:not(media-rule + style-rule)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn not_last_child() {
	assert_query!("a {} b {} c {}", "style-rule:not(:last-child)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn not_only_child() {
	assert_query!("a {} b {}", "style-rule:not(:only-child)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("a {}", "style-rule:not(:only-child)", 0);
}

#[test]
fn not_nth_last_child() {
	assert_query!("a {} b {} c {}", "style-rule:not(:nth-last-child(1))", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn not_first_of_type() {
	assert_query!("a {} b {} c {}", "style-rule:not(:first-of-type)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn not_deferred_with_type_inner_fails() {
	assert_query!("a {}", "style-rule:not(:last-child:at-rule)", 1, [NodeId::StyleRule]);
}

#[test]
fn not_deferred_with_type_inner_matches() {
	assert_query!("@media screen {}", "media-rule:not(:last-child:at-rule)", 0);
}

#[test]
fn not_deferred_inner_deferred_fails_nondeferred_matches() {
	assert_query!("@media screen {} a {}", "media-rule:not(:last-child:at-rule)", 1, [NodeId::MediaRule]);
}

#[test]
fn multiple_selectors_filtering_smoke() {
	assert_query!("a { color: red; }", "*:important, style-rule", 1, [NodeId::StyleRule]);
	assert_query!("a { color: red; }", "*:important, *:custom", 0);
}

#[test]
fn pseudos_after_tree_pseudos() {
	assert_query!("a {} @media screen {}", "media-rule:last-child:at-rule", 1, [NodeId::MediaRule]);
	assert_query!("@media screen {} a {}", "style-rule:last-child:rule", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {}", "media-rule:only-child:at-rule", 1, [NodeId::MediaRule]);
	assert_query!("@media screen {} a {}", "media-rule:nth-last-child(2):at-rule", 1, [NodeId::MediaRule]);
	assert_query!("a {} @media screen {} @media print {}", "media-rule:first-of-type:at-rule", 1, [NodeId::MediaRule]);
	assert_query!("@media screen {} @media print {} a {}", "media-rule:last-of-type:at-rule", 1, [NodeId::MediaRule]);
	assert_query!("a {} @media screen {} b {}", "media-rule:only-of-type:at-rule", 1, [NodeId::MediaRule]);
	assert_query!("@media screen { a {} b {} }", "media-rule style-rule:last-child", 1, [NodeId::StyleRule]);
	assert_query!("@media screen { a {} b {} }", "media-rule > style-rule:last-child", 1, [NodeId::StyleRule]);
	assert_query!("@media screen { a {} }", "media-rule style-rule:only-of-type", 1, [NodeId::StyleRule]);
	assert_query!("@media screen { a {} }", "media-rule style-rule:only-child", 0);
	assert_query!("a {} @media screen {}", "media-rule:only-child:at-rule", 0);
	assert_query!("@media screen {} a {}", "media-rule:last-child:at-rule", 0);
}

#[test]
fn tree_pseudos_with_comnbinators() {
	assert_query!("a {} b {}", "style-rule:first-child > selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a {} b {}", "style-rule:first-child + style-rule", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {} c {}", "style-rule:first-child ~ style-rule", 2, [NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("a {} b {} c {}", "style-rule:nth-child(2) > selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a {} b {} c {}", "style-rule:last-of-type > selector-list", 1, [NodeId::SelectorList]);
	// With single style-rule, it is both :only-of-type and :only-child
	assert_query!("a {}", "style-rule:only-of-type > selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a {} b {}", "style-rule:last-child > selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a {}", "style-rule:only-child > selector-list", 1, [NodeId::SelectorList]);
	// With 2 style-rules, neither is :only-child
	assert_query!("a {} b {}", "style-rule:only-child > selector-list", 0);
	assert_query!("a {} @media screen {} b {}", "style-rule:last-of-type > selector-list", 1, [NodeId::SelectorList]);
	assert_query!("a {} b {}", "style-rule:last-child selector-list", 1, [NodeId::SelectorList]);
	assert_query!(
		"@media screen { a {} } @media print { b {} }",
		"media-rule:last-child > style-rule > selector-list",
		1,
		[NodeId::SelectorList]
	);
}

#[test]
fn not_with_tree_pseudo() {
	assert_query!("a {} b {}", "style-rule:not(:first-of-type)", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {} a {} b {}", "style-rule:not(:first-of-type)", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {}", "style-rule:not(:last-of-type)", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {} @media screen {}", "style-rule:not(:last-of-type)", 1, [NodeId::StyleRule]);
	assert_query!("@media screen {} a {} @keyframes foo {}", "style-rule:not(:only-of-type)", 0);
	assert_query!("a {} b {}", "style-rule:not(:only-of-type)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("a {} b {} c {}", "style-rule:not(:nth-of-type(2))", 2, [NodeId::StyleRule, NodeId::StyleRule]);
	assert_query!("@media screen {} a {} b {}", "style-rule:not(:nth-of-type(2))", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {} c {}", "style-rule:not(:nth-last-of-type(2))", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn not_with_combinator() {
	assert_query!("a {} @media screen {} b {} c {}", "style-rule:not(media-rule ~ style-rule)", 1, [NodeId::StyleRule]);
}

#[test]
fn not_with_combinator_and_pseudo() {
	assert_query!(
		"a {} b {}",
		"style-rule:not(media-rule > style-rule:last-child)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!(
		"a {} b {}",
		"style-rule:not(media-rule + style-rule:last-child)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!(
		"@media screen { a {} b {} }",
		"style-rule:not(media-rule > style-rule:last-child)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"a {} @media screen { b {} }",
		"style-rule:not(media-rule > style-rule:last-child)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"a {} @media screen { @supports (color:red) { b {} } }",
		"style-rule:not(media-rule style-rule:last-child)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"@media screen { a {} b {} }",
		"style-rule:not(media-rule > style-rule:first-of-type)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"a {} b {}",
		"style-rule:not(media-rule > style-rule:first-of-type)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
}

#[test]
fn not_with_multiple_tree_pseudos() {
	// Test :not() with multiple deferred pseudo-classes
	// :last-child:last-of-type - must be both last child AND last of type
	assert_query!(
		"a {} b {} @media screen {}",
		"style-rule:not(:last-child:last-of-type)",
		2,
		[NodeId::StyleRule, NodeId::StyleRule]
	);
	assert_query!("a {} b {}", "style-rule:not(:last-child:last-of-type)", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {}", "style-rule:not(:last-child:at-rule)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn has_pseudo_match() {
	assert_query!("a { color: red; }", "style-rule:has(selector-list)", 1, [NodeId::StyleRule]);
	assert_query!("@media screen { a { color: red; } }", "media-rule:has(style-rule)", 1, [NodeId::MediaRule]);
	assert_query!(
		"@media screen { a { color: rgb(255, 0, 0); } }",
		"media-rule:has(color-function)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!("a { color: red; }", "style-rule:has(*)", 1, [NodeId::StyleRule]);
	assert_query!("a { color: red; } b { margin: 10px; }", "style-rule:has([name=color])", 1, [NodeId::StyleRule]);
	assert_query!("a {} b {}", "style-rule:has(selector-list)", 2, [NodeId::StyleRule, NodeId::StyleRule]);
}

#[test]
fn has_pseudo_no_match() {
	assert_query!("@media screen {}", "media-rule:has(selector-list)", 0);
}

#[test]
fn has_pseudo_with_inner_pseudo_match() {
	assert_query!(
		"a { color: red !important; } b { margin: 10px; }",
		"style-rule:has(:important)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!("a { color: red !important; }", "style-rule:has(:important)", 1, [NodeId::StyleRule]);
	assert_query!("a { --x: red; }", "style-rule:has(:custom)", 1, [NodeId::StyleRule]);
	assert_query!("a { margin: 10px; }", "style-rule:has([name=margin])", 1, [NodeId::StyleRule]);
	assert_query!("a { color: var(--x); }", "style-rule:has(:computed)", 1, [NodeId::StyleRule]);
	assert_query!("a { margin: 10px; }", "style-rule:has(:shorthand)", 1, [NodeId::StyleRule]);
}

#[test]
fn has_pseudo_with_inner_pseudo_no_match() {
	assert_query!("a { color: red; }", "*:has(:important)", 0);
	assert_query!("a { color: red; }", "*:has([name=margin])", 0);
	assert_query!("a { color: red; }", "*:has(:custom)", 0);
	assert_query!("a { color: red; }", "*:has(:computed)", 0);
	assert_query!("a { margin-top: 10px; }", "*:has(:shorthand)", 0);
}

#[test]
fn has_pseudo_chained_match() {
	assert_query!(
		"a { color: rgb(0,0,0); }",
		"style-rule:has(selector-list):has(color-function)",
		1,
		[NodeId::StyleRule]
	);
}

#[test]
fn has_pseudo_chained_no_match() {
	assert_query!("a { color: red; }", "style-rule:has(selector-list):has(media-rule)", 0);
}

#[test]
fn has_pseudo_descendant_chain_match() {
	assert_query!(
		"@media screen { a { color: rgb(255,0,0); } }",
		"media-rule:has(style-rule color-function)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!(
		"@media screen { a { color: rgb(255,0,0); } }",
		"style-sheet:has(media-rule style-rule color-function)",
		1,
		[NodeId::StyleSheet]
	);
	assert_query!(
		"@media screen { a { color: red; } }",
		"media-rule:has(style-rule [name=color])",
		1,
		[NodeId::MediaRule]
	);
}

#[test]
fn has_pseudo_descendant_chain_no_match() {
	assert_query!("@media screen { a { color: rgb(255,0,0); } }", "media-rule:has(supports-rule color-function)", 0);
}

#[test]
fn has_pseudo_leading_combinator_direct() {
	assert_query!("a { color: red; }", "style-rule:has(> selector-list)", 1, [NodeId::StyleRule]);
	assert_query!("@media screen { a { color: red; } }", "media-rule:has(> style-rule)", 1, [NodeId::MediaRule]);
	assert_query!("a { color: red; }", "style-rule:has(> [name=color])", 1, [NodeId::StyleRule]);
	assert_query!("a { color: red !important; }", "style-rule:has(> :important)", 1, [NodeId::StyleRule]);
	assert_query!(
		"@media screen { a { color: rgb(255,0,0); } }",
		"media-rule:has(> style-rule color-function)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!(
		"@media screen { a { color: red; } }",
		"media-rule:has(> style-rule > selector-list)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!("a {} b {}", "style-rule:has(+ style-rule)", 1, [NodeId::StyleRule]);
	assert_query!("a {} @media screen {} b {}", "style-rule:has(~ style-rule)", 1, [NodeId::StyleRule]);
}

#[test]
fn has_pseudo_leading_combinator_direct_no_mach() {
	assert_query!("@media screen { a { color: red; } }", "media-rule:has(> selector-list)", 0);
	assert_query!("@media screen { a { color: rgb(255,0,0); } }", "style-sheet:has(> style-rule color-function)", 0);
	assert_query!("@media screen { a { color: red; } }", "style-sheet:has(> style-rule > selector-list)", 0);
	assert_query!("@media screen { a { color: red; } }", "media-rule:has(> style-rule selector-list)", 1);
	assert_query!("a {}", "style-rule:has(+ style-rule)", 0);
	assert_query!("a {} @media screen {} b {}", "style-rule:has(+ style-rule)", 0);
	assert_query!("a {} b {}", "style-rule:has(~ media-rule)", 0);
}

#[test]
fn has_pseudo_child_combinators_match() {
	assert_query!(
		"@media screen { a { color: red; } }",
		"media-rule:has(style-rule > selector-list)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!("@media screen { a {} b {} }", "media-rule:has(style-rule + style-rule)", 1, [NodeId::MediaRule]);
	assert_query!(
		"@media screen { a {} @keyframes x {} b {} }",
		"media-rule:has(style-rule ~ style-rule)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!("@media screen { a {} }", "media-rule:has(style-rule selector-list)", 1, [NodeId::MediaRule]);
	assert_query!("@media screen { a {} }", "style-sheet:has(media-rule style-rule)", 1, [NodeId::StyleSheet]);
}

#[test]
fn has_pseudo_child_combinators_no_match() {
	assert_query!("@media screen { a {} }", "style-rule:has(media-rule selector-list)", 0);
	assert_query!("@media screen { a {} }", "media-rule:has(style-rule + style-rule)", 0);
	assert_query!("@media screen { a {} b {} }", "media-rule:has(keyframes-rule + style-rule)", 0);
	assert_query!("@media screen { a {} }", "media-rule:has(style-rule ~ style-rule)", 0);
	assert_query!("@media screen { a {} @keyframes x {} }", "media-rule:has(keyframes-rule ~ style-rule)", 0);
}

#[test]
fn combinators_with_has_pseudo_match() {
	assert_query!(
		"@media screen { a { color: red; } }",
		"media-rule style-rule:has(selector-list)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"@media screen { a { color: red; } }",
		"media-rule > style-rule:has(selector-list)",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"a { color: red; } b { margin: 10px; }",
		"style-rule + style-rule:has([name=margin])",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"a {} b {} c { padding: 5px; }",
		"style-rule ~ style-rule:has([name=padding])",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"@media screen { a { color: rgb(0,0,0); } }",
		"style-sheet media-rule style-rule:has(color-function)",
		1,
		[NodeId::StyleRule]
	);
}

#[test]
fn combinators_with_has_pseudo_no_match() {
	assert_query!("@media screen {}", "media-rule style-rule:has(selector-list)", 0);
	assert_query!("@media screen { a { color: red; } }", "style-sheet > style-rule:has(selector-list)", 0);
	assert_query!("a { color: red; } b { margin: 10px; }", "style-rule + style-rule:has([name=color])", 0);
}

#[test]
fn has_pseudo_with_chained_inner_combinators_match() {
	assert_query!(
		"@media screen { a { color: red; } }",
		"style-sheet:has(> media-rule > style-rule > selector-list)",
		1,
		[NodeId::StyleSheet]
	);
	assert_query!("@media screen { a {} b {} }", "media-rule:has(> style-rule + style-rule)", 1, [NodeId::MediaRule]);
	assert_query!(
		"@media screen { a {} @keyframes x {} b {} }",
		"media-rule:has(> style-rule ~ style-rule)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!(
		"@media screen { a {} @keyframes x {} b {} }",
		"media-rule:has(> style-rule ~ style-rule)",
		1,
		[NodeId::MediaRule]
	);
}

#[test]
fn has_pseudo_with_chained_inner_combinators_no_match() {
	assert_query!("@media screen { a {} }", "media-rule:has(> style-rule + style-rule)", 0);
}

#[test]
fn has_pseudo_with_inner_not_match() {
	assert_query!("a {}", "style-rule:has(:not(media-rule))", 1, [NodeId::StyleRule]);
	assert_query!(
		"a { color: red; } b { color: red !important; }",
		"style-rule:has([name=color]:not(:important))",
		1,
		[NodeId::StyleRule]
	);
	assert_query!("a { color: red; }", "style-rule:has(*:not(media-rule))", 1, [NodeId::StyleRule]);
	assert_query!("a { color: red; }", "style-rule:has(> :not(:important))", 1, [NodeId::StyleRule]);
	assert_query!("a { color: red; }", "style-sheet:has(:not(media-rule style-rule))", 1, [NodeId::StyleSheet]);
	assert_query!("a { color: red; }", "style-sheet:has(:not(media-rule > style-rule))", 1, [NodeId::StyleSheet]);
	assert_query!("a {} b {}", "style-sheet:has(:not(style-rule + style-rule))", 1, [NodeId::StyleSheet]);
	assert_query!("@media screen { a {} }", "style-sheet:has(:not(media-rule style-rule))", 1, [NodeId::StyleSheet]);
	assert_query!("@media screen { a {} }", "style-sheet:has(*:not(media-rule > style-rule))", 1, [NodeId::StyleSheet]);
	assert_query!("@media screen { a {} }", "style-sheet:has(*:not(media-rule style-rule))", 1, [NodeId::StyleSheet]);
	assert_query!(
		"a {} @media screen { b {} }",
		"style-sheet:has(style-rule:not(media-rule style-rule))",
		1,
		[NodeId::StyleSheet]
	);
	assert_query!("a {} b {}", "style-sheet:has(style-rule:not(style-rule + style-rule))", 1, [NodeId::StyleSheet]);
}

#[test]
fn has_pseudo_with_inner_not_no_match() {
	assert_query!("a { color: red !important; }", "style-rule:has(> [name=color]:not(:important))", 0);
	assert_query!("@media screen { @keyframes x {} a {} }", "media-rule:has(> style-rule + keyframes-rule)", 0);
	assert_query!("@media screen { a {} }", "style-sheet:has(style-rule:not(media-rule style-rule))", 0);
	assert_query!("@media screen { a {} }", "style-sheet:has(style-rule:not(media-rule > style-rule))", 0);
}

#[test]
fn has_pseudo_with_wildcard_anchor() {
	assert_query!("a { color: red !important; }", "*:has(:important)", 2, [NodeId::StyleSheet, NodeId::StyleRule]);
	assert_query!("a { color: red !important; }", "style-sheet:has(:important)", 1, [NodeId::StyleSheet]);
	assert_query!("a { --x: red; }", "style-sheet:has(:custom)", 1, [NodeId::StyleSheet]);
	assert_query!("a { margin: 10px; }", "style-sheet:has(:shorthand)", 1, [NodeId::StyleSheet]);
	assert_query!("a { color: red; }", "style-sheet:has([name=color])", 1, [NodeId::StyleSheet]);
}

#[test]
fn has_pseudo_eeply_nested() {
	let deep = "@media a { @media b { @media c { @media d { @media e { x { color: rgb(0,0,0); } } } } } }";
	assert_query!(deep, "style-sheet:has(color-function)", 1, [NodeId::StyleSheet]);
	assert_query!(deep, "style-sheet:has(style-rule)", 1, [NodeId::StyleSheet]);
	assert_query!(deep, "media-rule:has(style-rule)", 5);
	assert_query!(deep, "media-rule:has(color-function)", 5);
}

#[test]
fn has_pseudo_respects_boundaries() {
	assert_query!("a { color: red; } @media screen { b {} }", "media-rule:has(style-rule [name=color])", 0);
	assert_query!(
		"a {} @media screen { b { c {} } }",
		"media-rule:has(> style-rule style-rule)",
		1,
		[NodeId::MediaRule]
	);
	assert_query!("a {} @media screen { b {} c {} }", "media-rule:has(> style-rule style-rule)", 0);
	assert_query!("a {} @media screen { b {} }", "media-rule:has(style-rule + style-rule)", 0);
}

#[test]
fn has_pseudo_nested_anchor_boundaries() {
	assert_query!("@media screen { a:has(selector-list) {} }", "style-rule:has(selector-list)", 1, [NodeId::StyleRule]);
	assert_query!(
		"@media screen { a { color: red; } } b { margin: 10px; }",
		"media-rule style-rule:has([name=color])",
		1,
		[NodeId::StyleRule]
	);
	assert_query!(
		"@media screen { a { color: red; } } b { margin: 10px; }",
		"media-rule style-rule:has([name=margin])",
		0
	);
}

#[test]
fn size_pseudo_match() {
	assert_query!(".a, .b, .c {}", "selector-list:size(3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b {}", "selector-list:size(2)", 1, [NodeId::SelectorList]);
	assert_query!(".a {}", "selector-list:size(1)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {} .x, .y {}", "selector-list:size(3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {} .x, .y {}", "selector-list:size(2)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(>2)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(>1)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b {}", "selector-list:size(>1)", 1, [NodeId::SelectorList]);
	assert_query!(
		".a, .b, .c {} .x, .y, .z {}",
		"selector-list:size(>2)",
		2,
		[NodeId::SelectorList, NodeId::SelectorList]
	);
	assert_query!(".a {}", "selector-list:size(<2)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b {}", "selector-list:size(<3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(<5)", 1, [NodeId::SelectorList]);
	assert_query!(".a {} .x, .y {}", "selector-list:size(<3)", 2, [NodeId::SelectorList, NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(>=3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b {}", "selector-list:size(>=2)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(>=2)", 1, [NodeId::SelectorList]);
	assert_query!(
		".a, .b {} .x, .y, .z {}",
		"selector-list:size(>=2)",
		2,
		[NodeId::SelectorList, NodeId::SelectorList]
	);
	assert_query!(".a {}", "selector-list:size(<=1)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b {}", "selector-list:size(<=2)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(<=3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c {}", "selector-list:size(<=5)", 1, [NodeId::SelectorList]);
	assert_query!(".a {} .x, .y {}", "selector-list:size(<=2)", 2, [NodeId::SelectorList, NodeId::SelectorList]);
}

#[test]
fn size_pseudo_no_match() {
	assert_query!(".a, .b {}", "selector-list:size(3)", 0);
	assert_query!(".a {}", "selector-list:size(2)", 0);
	assert_query!(".a, .b, .c {} .x, .y {}", "selector-list:size(5)", 0);
	assert_query!(".a {}", "selector-list:size(>1)", 0);
	assert_query!(".a, .b {}", "selector-list:size(>2)", 0);
	assert_query!(".a, .b, .c {}", "selector-list:size(>5)", 0);
	assert_query!(".a, .b {}", "selector-list:size(<2)", 0);
	assert_query!(".a, .b, .c {}", "selector-list:size(<3)", 0);
	assert_query!(".a, .b, .c {}", "selector-list:size(<1)", 0);
	assert_query!(".a {}", "selector-list:size(>=2)", 0);
	assert_query!(".a, .b {}", "selector-list:size(>=3)", 0);
	assert_query!(".a, .b {}", "selector-list:size(<=1)", 0);
	assert_query!(".a, .b, .c {}", "selector-list:size(<=2)", 0);
}

#[test]
fn size_pseudo_combinations_match() {
	assert_query!(".a, .b, .c {} .x, .y {}", "style-rule > selector-list:size(3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c { color: red; }", "style-rule > selector-list:size(3)", 1, [NodeId::SelectorList]);
	assert_query!(".a, .b, .c { color: red; }", "style-rule:has(selector-list:size(3))", 1, [NodeId::StyleRule]);
	assert_query!(
		".a {} .b, .c {} .d, .e, .f {}",
		"selector-list:not(:size(2))",
		2,
		[NodeId::SelectorList, NodeId::SelectorList]
	);
	assert_query!(
		"@media screen { .a, .b, .c {} .x {} }",
		"media-rule selector-list:size(3)",
		1,
		[NodeId::SelectorList]
	);
}
