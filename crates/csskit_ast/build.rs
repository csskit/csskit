use std::{env, fs, path::Path};

fn main() {
	println!("cargo::rerun-if-changed=build.rs");
	let mut node_type_variants = String::new();
	let mut match_arms = String::new();
	for name in css_ast::NodeId::all_variants() {
		node_type_variants.push_str(&format!("\t{},\n", name));
		match_arms.push_str(&format!("\t\t\tSelf::{} => Some(NodeId::{}),\n", name, name));
	}

	if node_type_variants.is_empty() {
		panic!("node_type_variants is empty!");
	}

	let content = format!(
		r#"
#[derive(AtomSet, Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CsskitAtomSet {{
	#[default]
	_None,

	// Vendor prefixes
	Webkit,
	Moz,
	Ms,
	O,

	// Pseudo-classes
	Important,
	Custom,
	Prefixed,
	Unknown,
	Computed,
	Shorthand,
	Longhand,
	PropertyType,
	Empty,
	Nested,
	Root,
	FirstChild,
	LastChild,
	OnlyChild,
	NthChild,
	NthLastChild,
	FirstOfType,
	LastOfType,
	OnlyOfType,
	NthOfType,
	NthLastOfType,
	Not,
	Has,
	Size,
	Name,
	AtRule,
	Rule,
	Function,
	Block,

	// Property groups
	Align,
	Anchor,
	AnchorPosition,
	Animation,
	Animations,
	Background,
	Backgrounds,
	Border,
	Borders,
	Box,
	Break,
	Cascade,
	ColorAdjust,
	ColorHdr,
	Conditional,
	Contain,
	Content,
	Display,
	Exclusions,
	Flexbox,
	Font,
	Fonts,
	Forms,
	Gap,
	Gaps,
	Gcpm,
	Grid,
	Images,
	Inline,
	LineGrid,
	LinkParams,
	List,
	Lists,
	Logical,
	Mask,
	Masking,
	Multicol,
	Nav,
	Overflow,
	Overscroll,
	Page,
	PageFloats,
	Regions,
	Rhythm,
	RoundDisplay,
	Ruby,
	ScrollAnchoring,
	ScrollSnap,
	Scrollbar,
	Scrollbars,
	Shaders,
	Shape,
	Shapes,
	SizeAdjust,
	Sizing,
	Speech,
	Table,
	Tables,
	Text,
	TextDecor,
	TextDecoration,
	Transform,
	Transforms,
	Transition,
	Transitions,
	Ui,
	Values,
	Variables,
	ViewTransitions,
	Viewport,
	WillChange,
	WritingModes,

	// Node Type Variants
	{node_type_variants}
}}

impl CsskitAtomSet {{
	/// Converts a CsskitAtomSet representing a node type selector to NodeId.
	/// This provides O(1) lookup for ALL queryable node types.
	/// Generated at build time from all NodeId variants in css_ast.
	/// Returns None if the selector is invalid (not a valid NodeId).
	pub fn to_node_id(self) -> Option<NodeId> {{
		match self {{
{match_arms}
			_ => None,
		}}
	}}
}}
"#,
	);

	let out_dir = env::var("OUT_DIR").unwrap();
	fs::write(Path::new(&out_dir).join("csskit_atom_set.rs"), content).unwrap();
}
