use std::ops::ControlFlow;

/// Controls whether children are visited after a `visit_*` method returns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitAction {
	/// Recurse into children (default).
	Descend,
	/// Skip recursing into children.
	SkipChildren,
}

/// Controls early termination of the entire traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisitBreak {
	/// Halt immediately; no further visits or exits.
	Stop,
}

/// Convenience alias for the visitor control-flow type.
pub type VisitFlow = ControlFlow<VisitBreak, VisitAction>;

/// Named [`VisitFlow`] values.
///
/// `VisitFlow` is a type alias for [`ControlFlow`], so the canonical values are
/// exposed as associated consts via this extension trait rather than inherent
/// consts. Bringing the trait into scope lets you write `VisitFlow::DESCEND`,
/// `VisitFlow::SKIP_CHILDREN`, and `VisitFlow::STOP`.
pub trait VisitFlowExt {
	/// Continue traversal and recurse into children.
	const DESCEND: VisitFlow = ControlFlow::Continue(VisitAction::Descend);
	/// Continue traversal but skip visiting children (`exit_*` still fires).
	const SKIP_CHILDREN: VisitFlow = ControlFlow::Continue(VisitAction::SkipChildren);
	/// Stop traversal immediately.
	const STOP: VisitFlow = ControlFlow::Break(VisitBreak::Stop);
}

impl VisitFlowExt for VisitFlow {}

/// Propagates a [`VisitFlow`]: on `Break`, returns early; on `Continue`, yields the [`VisitAction`].
#[macro_export]
macro_rules! try_visit {
	($e:expr) => {
		match $e {
			::std::ops::ControlFlow::Continue(action) => action,
			::std::ops::ControlFlow::Break(r) => return ::std::ops::ControlFlow::Break(r),
		}
	};
}
