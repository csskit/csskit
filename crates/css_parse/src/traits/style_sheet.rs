use crate::{Cursor, Kind, NodeMetadata, NodeWithMetadata, Parse, Parser, Result, T};
use bumpalo::collections::Vec;

/// This trait provides an implementation for parsing a [StyleSheet][1].
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#parse-stylesheet
///
/// It does not implement [Parse], but provides `parse_stylesheet(&mut Parser<'a>) -> Result<...>`, which can make
/// for a trivial [Parse] implementation. The type [StyleSheet::Rule] must be defined, and represents any Rule allowed
/// in a style sheet, which is the only top level item of the stylesheet.
///
/// StyleSheets are special in that they must discard CdcOrCdo tokens.
///
/// The steps `parse_stylesheet` takes can be defined as:
///
/// ```md
/// <style-sheet>
///  │├─╮─╭─ <ws*> ─╮─╭╮─╭─ <cdcorcdo-token> ─╮─╭─ <rule> ──┤│
///     │ ╰─────────╯ ││ ╰────────────────────╯ │
///     ╰─────────────╯╰────────────────────────╯
/// ```
///
pub trait StyleSheet<'a, M: NodeMetadata>: Sized + Parse<'a> {
	type Rule: Parse<'a> + NodeWithMetadata<M>;

	fn parse_stylesheet<I>(p: &mut Parser<'a, I>) -> Result<(Vec<'a, Self::Rule>, M)>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut rules: Vec<'a, Self::Rule> = Vec::new_in(p.bump());
		let mut meta: M = Default::default();
		loop {
			// While by default the parser will skip whitespace, the Rule type may be a whitespace sensitive
			// node, for example `ComponentValues`. As such whitespace needs to be consumed here, before Declarations and
			// Rules are parsed.
			if p.parse_if_peek::<T![' ']>()?.is_some() || p.parse_if_peek::<T![CdcOrCdo]>()?.is_some() {
				continue;
			}

			// need to peek as last tokens may be whitespace.
			if p.at_end() || p.peek_n(1) == Kind::Eof {
				return Ok((rules, meta));
			}
			let rule = p.parse::<Self::Rule>()?;
			meta = meta.merge(rule.metadata());
			rules.push(rule);
		}
	}
}
