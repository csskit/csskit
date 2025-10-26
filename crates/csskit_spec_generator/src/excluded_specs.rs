/// Specs that should NOT be generated automatically.
///
/// These specs either:
/// - Are not real specs (e.g., easter eggs like css-egg)
/// - Have been superseded by other specs
pub fn is_excluded_spec(spec_name: &str) -> bool {
	EXCLUDED_SPECS.contains(&spec_name)
}

/// List of specs to exclude from automatic generation.
const EXCLUDED_SPECS: &[&str] = &[
	"egg",      // Easter egg spec - not a real spec (https://drafts.csswg.org/css-egg-1/)
	"template", // Template Layout spec - superseded by CSS Grid
];
