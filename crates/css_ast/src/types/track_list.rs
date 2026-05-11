#![allow(unused)]
use super::prelude::*;

use crate::Todo;

/// <https://drafts.csswg.org/css-grid-3/#typedef-track-list>
///
/// ```text,ignore
/// <track-list> = [ <line-names>? [ <track-size> | <track-repeat> ] ]+ <line-names>?
/// ```
pub type TrackList = Todo;

/// <https://drafts.csswg.org/css-grid-3/#typedef-auto-track-list>
///
/// ```text,ignore
/// <auto-track-list> = [ <line-names>? [ <fixed-size> | <fixed-repeat> ] ]* <line-names>? <auto-repeat>
///                     [ <line-names>? [ <fixed-size> | <fixed-repeat> ] ]* <line-names>?
/// ```
pub type AutoTrackList = Todo;

/// <https://drafts.csswg.org/css-grid-3/#typedef-line-name-list>
///
/// ```text,ignore
/// <line-name-list> = [ <line-names> | <name-repeat> ]+
/// ```
pub type LineNameList = Todo;
