use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-play-state>
	///
	/// ```text,ignore
	/// <single-animation-play-state> = running | paused
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum SingleAnimationPlayState {
		Running: "running",
		Paused: "paused"
	}
);
