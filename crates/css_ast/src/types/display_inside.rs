use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-display-4/#typedef-display-inside>
	///
	/// ```text,ignore
	/// <display-inside> = flow | flow-root | table | flex | grid | ruby
	/// ```
	pub enum DisplayInside {
		Flow: "flow",
		FlowRoot: "flow-root",
		Table: "table",
		Flex: "flex",
		Grid: "grid",
		Ruby: "ruby",
	}
);
