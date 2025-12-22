use super::*;
use crate::assert_query;

#[test]
fn match_style_rule() {
	let matches = assert_query!("a { color: red; }", "style-rule", 1);
	assert_eq!(matches[0].node_id, NodeId::StyleRule);
	assert!(!matches[0].span.is_empty());
}

#[test]
fn match_selector_list() {
	assert_query!("a, b { color: red; }", "selector-list", 1);
}

#[test]
fn match_multiple_selectors() {
	assert_query!("a { color: red; } @media screen {}", "style-rule, media-rule", 2);
}

#[test]
fn descendant_combinator() {
	let matches = assert_query!("a { color: red; }", "style-rule selector-list", 1);
	assert_eq!(matches[0].node_id, NodeId::SelectorList);
}

#[test]
fn descendant_combinator_no_match() {
	assert_query!("@media screen {}", "style-rule selector-list", 0);
}

#[test]
fn nested_descendant() {
	let matches = assert_query!("@media screen { a { color: red; } }", "media-rule style-rule selector-list", 1);
	assert_eq!(matches[0].node_id, NodeId::SelectorList);
}

#[test]
fn child_combinator() {
	let matches = assert_query!("a { color: red; }", "style-rule > selector-list", 1);
	assert_eq!(matches[0].node_id, NodeId::SelectorList);
}

#[test]
fn child_combinator_no_match() {
	assert_query!("a { color: red; }", "style-sheet > selector-list", 0);
}

#[test]
fn next_sibling_combinator() {
	assert_query!("a {} b {}", "style-rule + style-rule", 1);
}

#[test]
fn next_sibling_combinator_no_match() {
	assert_query!("a {}", "style-rule + style-rule", 0);
}

#[test]
fn next_sibling_combinator_different_types() {
	let matches = assert_query!("@media screen {} a {}", "media-rule + style-rule", 1);
	assert_eq!(matches[0].node_id, NodeId::StyleRule);
}

#[test]
fn next_sibling_combinator_wrong_order() {
	assert_query!("a {} @media screen {}", "media-rule + style-rule", 0);
}

#[test]
fn subsequent_sibling_combinator() {
	assert_query!("a {} b {} c {}", "style-rule ~ style-rule", 2);
}

#[test]
fn subsequent_sibling_combinator_with_gap() {
	let matches = assert_query!("@media screen {} @keyframes foo {} a {}", "media-rule ~ style-rule", 1);
	assert_eq!(matches[0].node_id, NodeId::StyleRule);
}

#[test]
fn subsequent_sibling_combinator_no_match() {
	assert_query!("a {} @media screen {}", "media-rule ~ style-rule", 0);
}

#[test]
fn match_custom_properties() {
	assert_query!("a { --my-color: red; color: blue; --spacing: 10px; }", "*:custom", 2);
}

#[test]
fn no_match_custom_on_regular_properties() {
	assert_query!("a { color: red; background: blue; }", "*:custom", 0);
}

#[test]
fn attribute_name_selector() {
	assert_query!("a { color: red; background: blue; margin: 10px; }", "[name=color]", 1);
}

#[test]
fn attribute_name_selector_multiple() {
	assert_query!("a { color: red; } b { color: blue; background: green; }", "[name=color]", 2);
}

#[test]
fn attribute_name_selector_quoted() {
	assert_query!("a { background-color: red; }", "[name='background-color']", 1);
}

#[test]
fn attribute_name_selector_no_match() {
	assert_query!("a { color: red; background: blue; }", "[name=margin]", 0);
}

#[test]
fn attribute_name_case_insensitive() {
	assert_query!("a { COLOR: red; }", "[name=color]", 1);
}

#[test]
fn first_child() {
	assert_query!("a {} b {} c {}", "style-rule:first-child", 1);
}

#[test]
fn first_child_no_match() {
	assert_query!("@media screen {} a {}", "style-rule:first-child", 0);
}

#[test]
fn nth_child_index() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(2)", 1);
}

#[test]
fn nth_child_odd() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(odd)", 2);
}

#[test]
fn nth_child_even() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(even)", 2);
}

#[test]
fn nth_child_formula() {
	assert_query!("a {} b {} c {} d {} e {} f {}", "style-rule:nth-child(3n)", 2);
}

#[test]
fn nth_child_formula_with_offset() {
	assert_query!("a {} b {} c {} d {} e {} f {}", "style-rule:nth-child(2n+1)", 3);
}

#[test]
fn only_child() {
	assert_query!("a {}", "style-rule:only-child", 1);
}

#[test]
fn only_child_no_match() {
	assert_query!("a {} b {}", "style-rule:only-child", 0);
}

#[test]
fn style_rules_in_media() {
	assert_query!("@media screen { a {} b {} }", "media-rule style-rule", 2);
}

#[test]
fn last_child() {
	assert_query!("a {} b {} c {}", "style-rule:last-child", 1);
}

#[test]
fn last_child_no_match() {
	// style-rule is not last (media-rule is)
	assert_query!("a {} @media screen {}", "style-rule:last-child", 0);
}

#[test]
fn last_child_single() {
	// Single child is both first and last
	assert_query!("a {}", "style-rule:last-child", 1);
}

#[test]
fn nth_last_child_index() {
	assert_query!("a {} b {} c {}", "style-rule:nth-last-child(1)", 1);
}

#[test]
fn nth_last_child_second() {
	assert_query!("a {} b {} c {}", "style-rule:nth-last-child(2)", 1);
}

#[test]
fn nth_last_child_odd() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(odd)", 2);
}

#[test]
fn nth_last_child_even() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(even)", 2);
}

#[test]
fn nth_last_child_formula() {
	assert_query!("a {} b {} c {} d {} e {} f {}", "style-rule:nth-last-child(2n)", 3);
}

#[test]
fn first_of_type() {
	assert_query!("@media screen {} a {} b {}", "style-rule:first-of-type", 1);
}

#[test]
fn first_of_type_is_first() {
	assert_query!("a {} b {} @media screen {}", "style-rule:first-of-type", 1);
}

#[test]
fn last_of_type() {
	assert_query!("a {} b {} @media screen {}", "style-rule:last-of-type", 1);
}

#[test]
fn last_of_type_at_end() {
	assert_query!("@media screen {} a {} b {}", "style-rule:last-of-type", 1);
}

#[test]
fn only_of_type() {
	assert_query!("@media screen {} a {} @keyframes foo {}", "style-rule:only-of-type", 1);
}

#[test]
fn only_of_type_no_match() {
	assert_query!("a {} b {}", "style-rule:only-of-type", 0);
}

#[test]
fn nth_of_type() {
	assert_query!("@media screen {} a {} b {} c {}", "style-rule:nth-of-type(2)", 1);
}

#[test]
fn nth_of_type_odd() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-of-type(odd)", 2);
}

#[test]
fn nth_last_of_type() {
	assert_query!("a {} b {} c {} @media screen {}", "style-rule:nth-last-of-type(2)", 1);
}

#[test]
fn nth_last_of_type_even() {
	assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-of-type(even)", 2);
}

#[test]
fn root() {
	assert_query!("a {}", "style-sheet:root", 1);
}

#[test]
fn root_no_match() {
	assert_query!("a {}", "style-rule:root", 0);
}

#[test]
fn at_rule() {
	assert_query!("@media screen {} a {} @keyframes foo {}", "*:at-rule", 2);
}

#[test]
fn rule() {
	assert_query!("@media screen {} a {}", "*:rule", 2);
}

#[test]
fn function() {
	assert_query!("a { color: rgb(255, 0, 0); }", "*:function", 1);
}

#[test]
fn function_multiple() {
	// linear-gradient and rotate are both functions
	assert_query!("a { background: linear-gradient(red, blue); transform: rotate(45deg); }", "*:function", 2);
}

#[test]
fn at_rule_not_style_rule() {
	assert_query!("a {}", "*:at-rule", 0);
}

#[test]
fn prefixed_declaration() {
	assert_query!("a { -webkit-transform: rotate(45deg); }", "*:prefixed", 1);
}

#[test]
fn prefixed_declaration_multiple() {
	assert_query!("a { -webkit-transform: rotate(45deg); -moz-appearance: none; }", "*:prefixed", 2);
}

#[test]
fn prefixed_declaration_filter_webkit() {
	assert_query!("a { -webkit-transform: rotate(45deg); -moz-appearance: none; }", "*:prefixed(webkit)", 1);
}

#[test]
fn prefixed_declaration_filter_moz() {
	assert_query!("a { -webkit-transform: rotate(45deg); -moz-appearance: none; }", "*:prefixed(moz)", 1);
}

#[test]
fn prefixed_no_match_regular() {
	// Regular properties should not match :prefixed
	assert_query!("a { color: red; margin: 10px; }", "*:prefixed", 0);
}

#[test]
fn prefixed_no_match_custom_properties() {
	// CSS custom properties (--foo) should not match :prefixed
	assert_query!("a { --animate-duration: 1s; --animate-delay: 1s; }", "*:prefixed", 0);
}

#[test]
fn prefixed_unknown_property_filter() {
	assert_query!("a { -webkit-animation-duration: 1s; -moz-unknown: value; }", "*:prefixed(webkit)", 1);
	assert_query!("a { -webkit-animation-duration: 1s; -moz-unknown: value; }", "*:prefixed(moz)", 1);
}

#[test]
fn prefixed_unknown_multiple() {
	assert_query!("a { -webkit-animation-duration: 1s; -webkit-animation-delay: 2s; }", "*:prefixed", 2);
}

#[test]
fn prefixed_node_webkit_keyframes() {
	assert_query!("@-webkit-keyframes spin { to { opacity: 1; } }", "webkit-keyframes-rule:prefixed", 1);
}

#[test]
fn prefixed_node_filter() {
	assert_query!("@-webkit-keyframes spin { to { opacity: 1; } }", "*:prefixed(webkit)", 1);
}

// :shorthand and :longhand tests
#[test]
fn shorthand() {
	assert_query!("a { margin: 10px; }", "*:shorthand", 1);
}

#[test]
fn shorthand_multiple() {
	assert_query!("a { margin: 10px; padding: 5px; border: 1px solid; }", "*:shorthand", 3);
}

#[test]
fn longhand() {
	assert_query!("a { color: red; padding-top: 5px; }", "*:longhand", 2);
}

#[test]
fn longhand_not_shorthand() {
	assert_query!("a { margin: 10px; }", "*:longhand", 0);
}

#[test]
fn property_type_color() {
	assert_query!("a { color: red; margin: 10px; }", "*:property-type(color)", 1);
}

#[test]
fn property_type_sizing() {
	assert_query!("a { width: 100px; height: 50px; color: red; }", "*:property-type(sizing)", 2);
}

#[test]
fn property_type_animation() {
	assert_query!("a { animation-name: spin; animation-duration: 1s; color: red; }", "*:property-type(animation)", 2);
}

#[test]
fn nested_style_rule() {
	assert_query!("a { & b { color: red; } }", "style-rule:nested", 1);
}

#[test]
fn nested_not_top_level() {
	assert_query!("a { color: red; }", "style-rule:nested", 0);
}

#[test]
fn supports_rule() {
	assert_query!("@supports (color: red) { a { color: red; } }", "supports-rule", 1);
}

#[test]
fn supports_condition() {
	assert_query!("@supports (color: red) { a {} }", "supports-condition", 1);
}

#[test]
fn supports_condition_not() {
	assert_query!("@supports not (color: red) { a {} }", "supports-condition", 1);
}

#[test]
fn supports_feature() {
	assert_query!("@supports (color: red) { a {} }", "supports-feature", 1);
}

#[test]
fn supports_descendant() {
	assert_query!("@supports (color: red) { a {} }", "supports-rule style-rule", 1);
}

#[test]
fn supports_nested_media() {
	assert_query!("@supports (color: red) { @media screen { a {} } }", "supports-rule media-rule style-rule", 1);
}

#[test]
fn container_rule() {
	assert_query!("@container (width > 400px) { a { color: red; } }", "container-rule", 1);
}

#[test]
fn container_query() {
	assert_query!("@container (width > 400px) { a {} }", "container-query", 1);
}

#[test]
fn container_query_named() {
	assert_query!("@container sidebar (width > 400px) { a {} }", "container-query", 1);
}

#[test]
fn container_descendant() {
	assert_query!("@container (width > 400px) { a {} }", "container-rule style-rule", 1);
}

#[test]
fn container_nested_supports() {
	assert_query!("@container (width > 400px) { @supports (color: red) { a {} } }", "container-rule supports-rule", 1);
}

#[test]
fn not_type_selector() {
	// :not(media-rule) matches all nodes except MediaRule
	assert_query!("a {} @media screen {} b {}", "*", 13);
	assert_query!("a {} @media screen {} b {}", "media-rule", 1);
	assert_query!("a {} @media screen {} b {}", ":not(media-rule)", 12);
}

#[test]
fn not_excludes_type() {
	// :not(style-rule) matches all nodes except StyleRule
	assert_query!("a {} @media screen {} b {}", "*", 13);
	assert_query!("a {} @media screen {} b {}", "style-rule", 2);
	assert_query!("a {} @media screen {} b {}", ":not(style-rule)", 11);
}

#[test]
fn universal_matches_all() {
	assert_query!("a { color: red; }", "*", 9);
}

#[test]
fn universal_with_pseudo() {
	assert_query!("a {} b {}", "*:first-child", 10);
}

#[test]
fn universal_descendant() {
	assert_query!("a { color: red; }", "style-rule *", 7);
}

#[test]
fn important() {
	assert_query!("a { color: red !important; }", "*:important", 1);
}

#[test]
fn important_multiple() {
	assert_query!("a { color: red !important; margin: 10px; padding: 5px !important; }", "*:important", 2);
}

#[test]
fn important_no_match() {
	assert_query!("a { color: red; margin: 10px; }", "*:important", 0);
}

#[test]
fn important_combined_with_name() {
	assert_query!("a { color: red !important; margin: 10px !important; }", "[name=color]:important", 1);
}

#[test]
fn triple_descendant() {
	assert_query!("@media screen { @supports (color: red) { a {} } }", "media-rule supports-rule style-rule", 1);
}

#[test]
fn mixed_combinators() {
	assert_query!("@media screen { a {} b {} }", "media-rule > style-rule selector-list", 2);
}

#[test]
fn sibling_after_descendant() {
	assert_query!("@media screen { a {} b {} }", "media-rule style-rule + style-rule", 1);
}

#[test]
fn child_chain() {
	assert_query!("a { color: red; }", "style-sheet > style-rule > selector-list", 1);
}

#[test]
fn empty_stylesheet() {
	assert_query!("", "style-rule", 0);
}

#[test]
fn comments_only() {
	assert_query!("/* comment */", "style-rule", 0);
}

#[test]
fn whitespace_only() {
	assert_query!("   \n\t   ", "style-rule", 0);
}

#[test]
fn deeply_nested_media() {
	assert_query!("@media screen { @media print { a {} } }", "media-rule media-rule style-rule", 1);
}

#[test]
fn multiple_selector_list() {
	assert_query!("a {} @media screen {} b {}", "style-rule, media-rule", 3);
}

#[test]
fn attribute_with_pseudo() {
	assert_query!("a { color: red !important; margin: 10px !important; }", "[name=margin]:important", 1);
}

#[test]
fn property_type_backgrounds() {
	assert_query!("a { background-color: red; }", "*:property-type(backgrounds)", 1);
}

#[test]
fn declaration_in_keyframes() {
	assert_query!("@keyframes spin { from { opacity: 0; } to { opacity: 1; } }", "[name=opacity]", 2);
}

#[test]
fn declaration_in_font_face() {
	assert_query!("@font-face { font-family: 'Custom'; src: url('font.woff'); }", "[name=font-family]", 1);
}

#[test]
fn custom_property_in_root() {
	assert_query!(":root { --primary: blue; }", "*:custom", 1);
}

#[test]
fn color_function_rgb() {
	assert_query!("a { color: rgb(255, 0, 0); }", "color-function", 1);
}

#[test]
fn color_function_hsl() {
	assert_query!("a { color: hsl(120, 100%, 50%); }", "color-function", 1);
}

#[test]
fn color_function_multiple() {
	assert_query!("a { color: rgb(255, 0, 0); background-color: hsl(120, 100%, 50%); }", "color-function", 2);
}

#[test]
fn linear_gradient() {
	assert_query!("a { background-image: linear-gradient(red, blue); }", "linear-gradient-function", 1);
}

#[test]
fn url_in_background_image() {
	assert_query!("a { background-image: url('image.png'); }", "url", 1);
}

#[test]
fn computed_with_calc() {
	assert_query!("a { width: calc(100% - 20px); }", "*:computed", 1);
}

#[test]
fn computed_with_var() {
	assert_query!("a { color: var(--primary); }", "*:computed", 1);
}

#[test]
fn computed_no_match() {
	assert_query!("a { color: red; width: 100px; }", "*:computed", 0);
}

#[test]
fn unknown_property() {
	assert_query!("a { not-a-real-property: value; }", "*:unknown", 1);
}

#[test]
fn unknown_no_match() {
	assert_query!("a { color: red; margin: 10px; }", "*:unknown", 0);
}

#[test]
fn important_early_exit() {
	assert_query!("a { color: red; margin: 10px; }", "*:important", 0);
}

#[test]
fn custom_early_exit() {
	assert_query!("a { color: red; }", "*:custom", 0);
}

#[test]
fn shorthand_early_exit() {
	assert_query!("a { margin-top: 10px; }", "*:shorthand", 0);
}

#[test]
fn prefixed_early_exit() {
	assert_query!("a { transform: rotate(45deg); }", "*:prefixed", 0);
}

#[test]
fn multiple_selectors_partial_filter() {
	assert_query!("a { color: red; }", "*:important, style-rule", 1);
}

#[test]
fn all_selectors_filtered_out() {
	assert_query!("a { color: red; }", "*:important, *:custom", 0);
}

#[test]
fn at_rule_early_exit() {
	assert_query!("a { color: red; }", "*:at-rule", 0);
}

#[test]
fn sibling_with_attribute_match() {
	assert_query!("@keyframes spin {} a {}", "keyframes-rule[name=spin] + style-rule", 1);
}

#[test]
fn sibling_with_attribute_no_match() {
	assert_query!("@keyframes other {} a {}", "keyframes-rule[name=spin] + style-rule", 0);
}

#[test]
fn sibling_with_attribute_multiple() {
	assert_query!("@keyframes spin {} a {} @keyframes other {} b {}", "keyframes-rule[name=spin] + style-rule", 1);
}

#[test]
fn subsequent_sibling_with_attribute_match() {
	assert_query!("@keyframes spin {} @keyframes other {} a {}", "keyframes-rule[name=spin] ~ style-rule", 1);
}

#[test]
fn subsequent_sibling_with_attribute_no_match() {
	assert_query!("@keyframes other {} @keyframes another {} a {}", "keyframes-rule[name=spin] ~ style-rule", 0);
}

#[test]
fn sibling_with_pseudo_at_rule() {
	assert_query!("@media screen {} a {}", "*:at-rule + style-rule", 1);
}

#[test]
fn sibling_with_pseudo_at_rule_no_match() {
	assert_query!("a {} b {}", "*:at-rule + style-rule", 0);
}

#[test]
fn sibling_with_pseudo_rule() {
	assert_query!("a {} b {}", "*:rule + style-rule", 1);
	assert_query!("@media screen {} a {}", "*:rule + style-rule", 1);
}

#[test]
fn deferred_last_child_with_at_rule() {
	assert_query!("a {} @media screen {}", "media-rule:last-child:at-rule", 1);
}

#[test]
fn deferred_last_child_with_at_rule_no_match() {
	assert_query!("@media screen {} a {}", "media-rule:last-child:at-rule", 0);
}

#[test]
fn deferred_last_child_with_rule() {
	assert_query!("@media screen {} a {}", "style-rule:last-child:rule", 1);
}

#[test]
fn deferred_only_child_with_at_rule() {
	assert_query!("@media screen {}", "media-rule:only-child:at-rule", 1);
}

#[test]
fn deferred_only_child_with_at_rule_no_match() {
	assert_query!("a {} @media screen {}", "media-rule:only-child:at-rule", 0);
}

#[test]
fn deferred_nth_last_child_with_at_rule() {
	assert_query!("@media screen {} a {}", "media-rule:nth-last-child(2):at-rule", 1);
}

#[test]
fn deferred_first_of_type_with_at_rule() {
	assert_query!("a {} @media screen {} @media print {}", "media-rule:first-of-type:at-rule", 1);
}

#[test]
fn deferred_last_of_type_with_at_rule() {
	assert_query!("@media screen {} @media print {} a {}", "media-rule:last-of-type:at-rule", 1);
}

#[test]
fn deferred_only_of_type_with_at_rule() {
	assert_query!("a {} @media screen {} b {}", "media-rule:only-of-type:at-rule", 1);
}

#[test]
fn not_with_attribute_excludes_match() {
	assert_query!("a { color: red; margin: 10px; }", "[name]:not([name=color])", 1);
}

#[test]
fn not_with_attribute_all_match() {
	assert_query!("a { color: red; margin: 10px; }", "[name]:not([name=padding])", 2);
}

#[test]
fn not_with_pseudo_important() {
	assert_query!("a { color: red !important; margin: 10px; }", "[name]:not(:important)", 1);
}

#[test]
fn not_with_pseudo_custom() {
	assert_query!("a { --color: red; margin: 10px; }", "*:custom", 1);
	assert_query!("a { --color: red; margin: 10px; }", "*:not(:custom)", 13);
}

#[test]
fn not_with_pseudo_shorthand() {
	assert_query!("a { margin: 10px; color: red; }", "*:shorthand", 1);
	assert_query!("a { margin: 10px; color: red; }", "*:longhand", 1);
	assert_query!("a { margin: 10px; color: red; }", "*:not(:shorthand)", 15);
}

#[test]
fn not_at_rule_on_nodes() {
	let total = assert_query!("a {} @media screen {} b {}", "*", 13);
	let at_rules = assert_query!("a {} @media screen {} b {}", "*:at-rule", 1);
	assert_query!("a {} @media screen {} b {}", "*:not(:at-rule)", total.len() - at_rules.len());
}

#[test]
fn not_with_type_and_attribute() {
	assert_query!("@keyframes spin {} @keyframes other {}", "keyframes-rule:not([name=spin])", 1);
}

#[test]
fn not_with_type_and_pseudo() {
	assert_query!("a {} b {} c {}", "style-rule:not(:first-child)", 2);
}

#[test]
fn not_with_nested_pseudo() {
	assert_query!("a { & b { color: red; } }", "style-rule:not(:nested)", 1);
}
