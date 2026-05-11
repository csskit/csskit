/// <https://drafts.csswg.org/css-masking-1/#typedef-mask-reference>
///
/// ```text,ignore
/// <mask-reference> = none | <image> | <mask-source>
/// <mask-source> = <url>
/// ```
///
/// Since `<image>` already includes `<url>`, this simplifies to `none | <image>`.
pub type MaskReference<'a> = crate::NoneOr<crate::Image<'a>>;
