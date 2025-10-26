use super::prelude::*;
use crate::KeypressFunction;

/// <https://drafts.csswg.org/css-animations-2/#typedef-event-trigger-event>
///
/// ```text,ignore
/// <event-trigger-event> = <event-trigger-event> = activate | click | touch | dblclick | keypress(<string>)
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum EventTriggerEvent {
	#[atom(CssAtomSet::Activate)]
	Activate(T![Ident]),
	#[atom(CssAtomSet::Click)]
	Click(T![Ident]),
	#[atom(CssAtomSet::Touch)]
	Touch(T![Ident]),
	#[atom(CssAtomSet::Dblclick)]
	Dblclick(T![Ident]),
	KeypressFunction(KeypressFunction),
}
