#![allow(unused)]
use super::prelude::*;

use crate::Todo;

/// <https://drafts.csswg.org/css-gcpm-4/#typedef-content-level>
///
/// ```text,ignore
/// <content-level> = element | content | text | attr(<custom-ident>) | <counter()> | <counters()>
/// ```
pub type ContentLevel = Todo;
