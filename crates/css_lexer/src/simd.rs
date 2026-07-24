pub(crate) use fearless_simd::{Level, Simd, dispatch, prelude::*, u8x16};
use std::sync::LazyLock;
pub(crate) static LEVEL: LazyLock<Level> = LazyLock::new(Level::new);
