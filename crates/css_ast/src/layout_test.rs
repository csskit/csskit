//! Size tripwire: snapshots `size_of` for every `#[node]` AST type, collected
//! via `inventory`. An accidental size change fails the snapshot loudly. Kept
//! `#[cfg(test)]` so it adds no runtime cost or dependency to shipped builds.

pub(crate) struct LayoutInfo {
	pub name: &'static str,
	pub size: usize,
}
inventory::collect!(LayoutInfo);

fn render() -> String {
	let mut infos: Vec<&LayoutInfo> = inventory::iter::<LayoutInfo>.into_iter().collect();
	infos.sort_by_key(|info| info.name);
	let mut out = String::new();
	for info in infos {
		out.push_str(&format!("{}: {}\n", info.name, info.size));
	}
	out
}

#[test]
fn assert_layouts() {
	insta::assert_snapshot!("assert_layouts", render());
}
