use super::prelude::*;
use crate::{Attachment, BgClip, BgImage, BgPositionAndSize, Color, RepeatStyle, VisualBox};

/// Represents `<bg-layer>` and `<final-bg-layer>` from css-backgrounds-3.
///
/// ```text,ignore
/// <bg-layer> =
///   <bg-image> || <bg-position> [ / <bg-size> ]? || <repeat-style>
///   || <attachment> || <visual-box> || <bg-clip>
///
/// <final-bg-layer> = <bg-layer> || <color>?
/// ```
///
/// The `color` field is `None` for non-final layers.
///
/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-bg-layer>
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct BgLayer<'a> {
	pub image: Option<BgImage<'a>>,
	pub position: Option<BgPositionAndSize>,
	pub repeat: Option<RepeatStyle>,
	pub attachment: Option<Attachment>,
	pub origin: Option<VisualBox>,
	pub clip: Option<BgClip>,
	pub color: Option<Color<'a>>,
}

impl<'a> Peek<'a> for BgLayer<'a> {
	const PEEK_KINDSET: KindSet = BgImage::PEEK_KINDSET
		.combine(BgPositionAndSize::PEEK_KINDSET)
		.combine(RepeatStyle::PEEK_KINDSET)
		.combine(Attachment::PEEK_KINDSET)
		.combine(VisualBox::PEEK_KINDSET)
		.combine(BgClip::PEEK_KINDSET)
		.combine(Color::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		BgImage::peek(p, c)
			|| BgPositionAndSize::peek(p, c)
			|| RepeatStyle::peek(p, c)
			|| Attachment::peek(p, c)
			|| VisualBox::peek(p, c)
			|| BgClip::peek(p, c)
			|| Color::peek(p, c)
	}
}

impl<'a> Parse<'a> for BgLayer<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut image: Option<BgImage<'a>> = None;
		let mut position: Option<BgPositionAndSize> = None;
		let mut repeat: Option<RepeatStyle> = None;
		let mut attachment: Option<Attachment> = None;
		let mut origin: Option<VisualBox> = None;
		let mut clip: Option<BgClip> = None;
		let mut color: Option<Color<'a>> = None;

		let mut any = false;
		loop {
			if image.is_none()
				&& let Some(v) = p.parse_if_peek::<BgImage<'a>>()?
			{
				image = Some(v);
				any = true;
				continue;
			}
			if position.is_none()
				&& let Some(v) = p.parse_if_peek::<BgPositionAndSize>()?
			{
				position = Some(v);
				any = true;
				continue;
			}
			if repeat.is_none()
				&& let Some(v) = p.parse_if_peek::<RepeatStyle>()?
			{
				repeat = Some(v);
				any = true;
				continue;
			}
			if attachment.is_none()
				&& let Some(v) = p.parse_if_peek::<Attachment>()?
			{
				attachment = Some(v);
				any = true;
				continue;
			}
			// <visual-box> (origin) before <bg-clip> (superset).
			if origin.is_none()
				&& clip.is_none()
				&& let Some(v) = p.parse_if_peek::<VisualBox>()?
			{
				origin = Some(v);
				any = true;
				continue;
			}
			// <bg-clip> — border-area or text (not covered by VisualBox)
			if clip.is_none()
				&& let Some(v) = p.parse_if_peek::<BgClip>()?
			{
				clip = Some(v);
				any = true;
				continue;
			}
			// <color> — only valid in final-bg-layer but we parse permissively.
			if color.is_none()
				&& let Some(v) = p.parse_if_peek::<Color<'a>>()?
			{
				color = Some(v);
				any = true;
				continue;
			}
			break;
		}

		if !any {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}

		Ok(Self { image, position, repeat, attachment, origin, clip, color })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgLayer>(), 256);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "none");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "url(foo.png)");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "red");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "#fff");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "center");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "0 0");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "center / cover");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "no-repeat");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "fixed");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "url(bg.png) center no-repeat");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "url(bg.png) 0 0 / cover no-repeat fixed");
		assert_parse!(CssAtomSet::ATOMS, BgLayer, "url(bg.png) 0 0 / cover no-repeat fixed red");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BgLayer, "");
	}
}
