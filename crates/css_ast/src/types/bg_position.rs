#![allow(unused)]
use super::prelude::*;

use crate::Todo;

/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-position>
///
/// ```text,ignore
/// <bg-position> =  <position> | <position-three>
/// <position-three> = [
///   [ left | center | right ] && [ [ top | bottom ] <length-percentage> ]
/// |
///   [ [ left | right ] <length-percentage> ] && [ top | center | bottom ]
/// ]
/// ```
pub type BgPosition = Todo;
