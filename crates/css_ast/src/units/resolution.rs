use super::prelude::*;

/// <https://drafts.csswg.org/css-values/#resolution>
///
/// ```text,ignore
/// <resolution> = <dimension-token>
/// ```
#[node]
#[derive(
	ToCursors, Parse, Peek, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub enum Resolution {
	#[atom(CssAtomSet::Dpi)]
	Dpi(T![Dimension]),
	#[atom(CssAtomSet::Dpcm)]
	Dpcm(T![Dimension]),
	#[atom(CssAtomSet::Dppx)]
	Dppx(T![Dimension]),
	#[atom(CssAtomSet::X)]
	X(T![Dimension]),
}

impl From<Resolution> for f32 {
	fn from(res: Resolution) -> Self {
		match res {
			Resolution::Dpi(r) => r.into(),
			Resolution::Dpcm(r) => r.into(),
			Resolution::Dppx(r) => r.into(),
			Resolution::X(r) => r.into(),
		}
	}
}

impl Resolution {
	pub(crate) fn per_dppx(unit: CssAtomSet) -> Option<f32> {
		match unit {
			CssAtomSet::Dpi => Some(96.0),
			CssAtomSet::Dpcm => Some(96.0 / 2.54),
			CssAtomSet::Dppx | CssAtomSet::X => Some(1.0),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Resolution, "1dppx");
		assert_parse!(CssAtomSet::ATOMS, Resolution, "1x");
	}
}
