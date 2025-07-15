use css_parse::keyword_set;

keyword_set!(
	/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-play-state>
	///
	/// ```text,ignore
	/// <single-animation-play-state> = running | paused
	/// ```
	pub enum SingleAnimationPlayState {
		Running: "running",
		Paused: "paused"
	}
);
