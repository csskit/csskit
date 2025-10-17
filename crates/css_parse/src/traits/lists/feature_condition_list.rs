use crate::{Cursor, Parse, Parser, Peek, Result, token_macros::Ident};
use bumpalo::collections::Vec;

/// This trait can be used for AST nodes representing a list of "Feature Conditions". This is an amalgamation of
/// [Supports Conditions][1], [Media Conditions][2], and [Container Queries][3]
/// This is an implementation of [`<at-rule-list>`][1].
///
/// Looking at `<supports-condition>` and `<container-query>` we can se almost identical grammars (eliding some tokens
/// for brevity):
///
/// ```md
/// <supports-condition>
///  │├─╮─ <ident-token "not"> ─ <supports-in-parens> ──────────────────────────────╭──┤│
///     ╰─ <supports-in-parens> ─╮─╭─ <ident-token "and"> ─ <supports-in-parens> ─╮─┤
///                              │ ╰──────────────────────────────────────────────╯ │
///                              ├─╭─ <ident-token "or"> ─ <supports-in-parens> ─╮──┤
///                              │ ╰─────────────────────────────────────────────╯  │
///                              ╰──────────────────────────────────────────────────╯
///
/// <container-query>
///  │├─╮─ <ident-token "not"> ─ <query-in-parens> ───────────────────────────╭──┤│
///     ╰─ <supports-in-parens> ─╮─╭─ <ident-token "and"> ─ <supports-in-parens> ─╮─┤
///                              │ ╰──────────────────────────────────────────────╯ │
///                              ├─╭─ <ident-token "or"> ─ <supports-in-parens> ─╮──┤
///                              │ ╰─────────────────────────────────────────────╯  │
///                              ╰──────────────────────────────────────────────────╯
///
/// <media-condition>
///  │├─╮─ <ident-token "not"> ─ <media-in-parens> ───────────────────────────╭──┤│
///     ╰─ <media-in-parens> ─╮─╭─ <ident-token "and"> ─ <media-in-parens> ─╮─┤
///                           │ ╰───────────────────────────────────────────╯ │
///                           │─╭─ <ident-token "or"> ─ <media-in-parens> ─╮──│
///                           │ ╰──────────────────────────────────────────╯  │
///                           ╰───────────────────────────────────────────────╯
/// ```
///
/// The key difference between each of these is their own `<*-in-parens>` tokens. Thus they could all be defined as:
///
/// ```md
/// <condition-prelude-list>
///  │├─╮─ <ident-token "not"> ─ <feature> ───────────────────╭──┤│
///     ╰─ <feature> ─╮─╭─ <ident-token "and"> ─ <feature> ─╮─┤
///                   │ ╰───────────────────────────────────╯ │
///                   │─╭─ <ident-token "or"> ─ <feature> ─╮──│
///                   │ ╰──────────────────────────────────╯  │
///                   ╰───────────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-conditional-3/#typedef-supports-condition
/// [2]: https://drafts.csswg.org/mediaqueries/#media-condition
/// [3]: https://drafts.csswg.org/css-conditional-5/#typedef-container-query
pub trait FeatureConditionList<'a>: Sized + Parse<'a>
where
	Self: 'a,
{
	type FeatureCondition: Sized + Parse<'a>;

	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone;
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone;
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone;

	fn build_is(feature: Self::FeatureCondition) -> Self;
	fn build_not(keyword: Ident, feature: Self::FeatureCondition) -> Self;
	fn build_and(features: Vec<'a, (Self::FeatureCondition, Option<Ident>)>) -> Self;
	fn build_or(features: Vec<'a, (Self::FeatureCondition, Option<Ident>)>) -> Self;

	fn parse_condition<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if Ident::peek(p, c) && Self::keyword_is_not(p, c) {
			return Ok(Self::build_not(p.parse::<Ident>()?, p.parse::<Self::FeatureCondition>()?));
		}
		let mut feature = p.parse::<Self::FeatureCondition>()?;
		let c = p.peek_n(1);
		if Ident::peek(p, c) {
			if Self::keyword_is_and(p, c) {
				let mut features = Vec::new_in(p.bump());
				let mut keyword = p.parse::<Ident>()?;
				loop {
					features.push((feature, Some(keyword)));
					feature = p.parse::<Self::FeatureCondition>()?;
					let c = p.peek_n(1);
					if !(Ident::peek(p, c) && Self::keyword_is_and(p, c)) {
						features.push((feature, None));
						return Ok(Self::build_and(features));
					}
					keyword = p.parse::<Ident>()?
				}
			} else if Self::keyword_is_or(p, c) {
				let mut features = Vec::new_in(p.bump());
				let mut keyword = p.parse::<Ident>()?;
				loop {
					features.push((feature, Some(keyword)));
					feature = p.parse::<Self::FeatureCondition>()?;
					let c = p.peek_n(1);
					if !(Ident::peek(p, c) && Self::keyword_is_or(p, c)) {
						features.push((feature, None));
						return Ok(Self::build_or(features));
					}
					keyword = p.parse::<Ident>()?
				}
			}
		}
		Ok(Self::build_is(feature))
	}
}
