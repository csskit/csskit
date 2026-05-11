#![allow(unused)]
use super::prelude::*;

use crate::Todo;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation>
///
/// ```text,ignore
/// <single-animation> =
///   <'animation-duration'> ||
///   <easing-function> ||
///   <'animation-delay'> ||
///   <single-animation-iteration-count> ||
///   <single-animation-direction> ||
///   <single-animation-fill-mode> ||
///   <single-animation-play-state> ||
///   [ none | <keyframes-name> ] ||
///   <single-animation-timeline>
/// `` `
pub type SingleAnimation = Todo;
