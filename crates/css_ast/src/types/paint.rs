#![allow(unused)]
use super::prelude::*;

use crate::Todo;

/// <https://drafts.csswg.org/fill-stroke-3/#typedef-paint>
///
/// ```text,ignore
/// <paint> = none | <image> | <svg-paint>
/// <svg-paint> = child | child( <integer> )
/// ```
pub type Paint = Todo;
