mod helpers;

#[cfg(feature = "serde")]
#[test]
fn postcss_media() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/apply.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_brackets() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/at-rule-brackets.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_decls() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/atrule-decls.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_empty() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/atrule-empty.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_no_params() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/atrule-no-params.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_no_semicolon() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/atrule-no-semicolon.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_params() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/atrule-params.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_atrule_rules() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/atrule-rules.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_between() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/between.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_colon_selector() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/colon-selector.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_comments() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/comments.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_custom_properties() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/custom-properties.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_decls() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/decls.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_empty() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/empty.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_escape() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/escape.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_extends() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/extends.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_function() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/function.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_ie_prodid() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/ie-progid.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_important() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/important.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_inside() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/inside.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_no_selector() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/no-selector.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_prop() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/prop.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_quotes() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/quotes.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_raw_decl() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/raw-decl.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_rule_at() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/rule-at.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_rule_no_semicolon() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/rule-no-semicolon.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_selector() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/selector.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_semicolons() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/semicolons.css");
}

#[cfg(feature = "serde")]
#[test]
fn postcss_tab() {
	assert_snap_tokens!("../../coverage/postcss-parser-tests/cases/tab.css");
}
