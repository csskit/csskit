use crate::{
	BadDeclaration, CursorSink, Declaration, DeclarationGroup, DeclarationOrBad, DeclarationValue, Kind, KindSet,
	NodeMetadata, NodeWithMetadata, Parse, Parser, Peek, Result, RuleVariants, SemanticEq, Span, State, T, ToCursors,
	ToSpan, token_macros,
};
use bumpalo::collections::Vec;

/// This trait provides an implementation for ["consuming a blocks contents"][1].
///
/// ```md
/// <block>
///
///  │├─ "{" ─╭──╮─╭─ <ws-*> ─╮─╭─╮─╭─ ";" ─╮─╭─╮─ <R> ─╭─╮─ "}" ─┤│
///           │  │ ╰──────────╯ │ │ ╰───────╯ │ ├─ <D> ─┤ │
///           │  ╰──────────────╯ ╰───────────╯ ╰───────╯ │
///           ╰───────────────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-block-contents
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "D: serde::Serialize, R: serde::Serialize")))]
pub struct Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, D, M>>,
	pub rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub meta: M,
}

impl<'a, D, R, M> NodeWithMetadata<M> for Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		self.meta
	}
}

impl<'a, D, R, M> Peek<'a> for Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, D, R, M> Parse<'a> for Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	R: Parse<'a> + NodeWithMetadata<M> + RuleVariants<'a, DeclarationValue = D, Metadata = M>,
	M: NodeMetadata,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut rules = Vec::new_in(p.bump());
		let mut meta = M::default();

		// Per CSS Syntax spec: maintain a buffer of declarations to flush when we encounter rules.
		// This enables proper interleaving of declarations and rules.
		let mut decls: Vec<'a, DeclarationOrBad<'a, D, M>> = Vec::new_in(p.bump());

		// Flush the decls buffer into the rules list as a DeclarationGroup.
		// Per spec: "If decls is not empty, append it to rules, and set decls to a fresh empty list"
		macro_rules! flush_decls {
			() => {
				if !decls.is_empty() {
					let group = DeclarationGroup { declarations: std::mem::replace(&mut decls, Vec::new_in(p.bump())) };
					if let Some(rule) = R::from_declaration_group(group) {
						meta.merge(&rule.metadata());
						rules.push(rule);
					}
				}
			};
		}

		loop {
			// While by default the parser will skip whitespace, the Declaration or Rule type may be a whitespace sensitive
			// node, for example `ComponentValues`. As such whitespace needs to be consumed here, before Declarations and
			// Rules are parsed.
			if p.parse_if_peek::<T![' ']>()?.is_some() || p.parse_if_peek::<T![;]>()?.is_some() {
				continue;
			}
			if p.at_end() {
				break;
			}
			let c = p.peek_n(1);
			if <T!['}']>::peek(p, c) {
				break;
			}
			let old_state = p.set_state(State::Nested);
			let checkpoint = p.checkpoint();
			if <T![AtKeyword]>::peek(p, c) {
				// At-rule: flush pending declarations and parse the rule
				flush_decls!();
				let rule = p.parse::<R>();
				p.set_state(old_state);
				let rule = rule?;
				meta.merge(&rule.metadata());
				rules.push(rule);
			} else if let Ok(Some(decl)) = p.try_parse_if_peek::<Declaration<'a, D, M>>() {
				// https://drafts.csswg.org/css-syntax-3/#consume-a-blocks-contents
				// Parsing a declaration can result in an error, at which point the parser must be rewound and a Rule parse
				// must be attempted. The CSS spec allows parsers to discard unknown rules as syntax errors, but this parser
				// needs to retain them as unknown declarations, which creates some ambiguity as a Declaration may successfully
				// parse as an unknown. In these cases attempting to parse as a Rule should also be tried so that valid Rules
				// are not accidentally parsed as Unknown Declarations.
				//
				// Only reparse as a rule if:
				// 1. The declaration is unknown (not recognized property/value)
				// 2. The declaration name is invalid (not a known CSS property name)
				// 3. Re-parsing as a rule succeeds AND produces a known rule (not UnknownQualifiedRule/UnknownAtRule)
				//
				// This ensures:
				// - `background: var(--bg);` stays as declaration (valid name, even if unknown value)
				// - `.foo {...}` becomes a rule (invalid declaration name, parses as known StyleRule)
				// - `bad-prop: value;` stays as declaration (both unknown, prefer declaration)
				if decl.is_unknown() && !D::valid_declaration_name(p, decl.name.into()) {
					p.rewind(checkpoint.clone());
					if let Ok(rule) = p.parse::<R>()
						&& !rule.is_unknown()
					{
						// Successfully parsed as a known rule, use it instead of the unknown declaration
						flush_decls!();
						p.set_state(old_state);
						meta.merge(&rule.metadata());
						rules.push(rule);
						continue;
					}
					// Failed to parse as rule or rule was also unknown, re-parse as declaration
					p.rewind(checkpoint);
					p.parse::<Declaration<'a, D, M>>().ok();
				}
				p.set_state(old_state);
				meta.merge(&decl.metadata());
				declarations.push(decl);
			} else {
				// Not an at-rule, not a declaration - try parsing as a qualified rule
				let result = p.parse::<R>();
				p.set_state(old_state);
				match result {
					Ok(rule) => {
						flush_decls!();
						meta.merge(&rule.metadata());
						rules.push(rule);
					}
					Err(_) => {
						// Failed as both declaration and rule - consume as bad declaration for error recovery
						p.rewind(checkpoint);
						p.set_state(State::Nested);
						if let Ok(bad_decl) = p.parse::<BadDeclaration>() {
							p.set_state(old_state);
							decls.push(DeclarationOrBad::Bad(bad_decl));
						}
					}
				}
			}
		}

		// Flush any remaining declarations to rules
		flush_decls!();
		let close_curly = p.parse_if_peek::<T!['}']>()?;
		Ok(Self { open_curly, declarations, rules, close_curly, meta })
	}
}

impl<'a, D, R, M> ToCursors for Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M> + ToCursors,
	R: ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

impl<'a, D, R, M> ToSpan for Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M> + ToSpan,
	R: ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		self.open_curly.to_span()
			+ if self.close_curly.is_some() {
				self.close_curly.to_span()
			} else {
				self.declarations.to_span() + self.rules.to_span() + self.close_curly.to_span()
			}
	}
}

impl<'a, D, R, M> SemanticEq for Block<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	R: SemanticEq,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self) -> bool {
		self.open_curly.semantic_eq(&other.open_curly)
			&& self.close_curly.semantic_eq(&other.close_curly)
			&& self.declarations.semantic_eq(&other.declarations)
			&& self.rules.semantic_eq(&other.rules)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::EmptyAtomSet;
	use crate::{Cursor, test_helpers::*};

	#[derive(Debug)]
	struct Decl(T![Ident]);

	impl<M: NodeMetadata> NodeWithMetadata<M> for Decl {
		fn metadata(&self) -> M {
			M::default()
		}
	}

	impl<'a, M: NodeMetadata> DeclarationValue<'a, M> for Decl {
		type ComputedValue = T![Eof];

		fn is_initial(&self) -> bool {
			false
		}

		fn is_inherit(&self) -> bool {
			false
		}

		fn is_unset(&self) -> bool {
			false
		}

		fn is_revert(&self) -> bool {
			false
		}

		fn is_revert_layer(&self) -> bool {
			false
		}

		fn needs_computing(&self) -> bool {
			false
		}

		fn parse_specified_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _: Cursor) -> Result<Self>
		where
			Iter: Iterator<Item = crate::Cursor> + Clone,
		{
			p.parse::<T![Ident]>().map(Self)
		}
	}

	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	impl ToSpan for Decl {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	impl SemanticEq for Decl {
		fn semantic_eq(&self, other: &Self) -> bool {
			self.0.semantic_eq(&other.0)
		}
	}

	impl NodeWithMetadata<()> for T![Ident] {
		fn metadata(&self) {}
	}

	#[derive(Debug)]
	struct Rule(T![Ident]);

	impl<'a> Parse<'a> for Rule {
		fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
		where
			I: Iterator<Item = Cursor> + Clone,
		{
			Ok(Self(p.parse::<T![Ident]>()?))
		}
	}

	impl ToCursors for Rule {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	impl ToSpan for Rule {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	impl NodeWithMetadata<()> for Rule {
		fn metadata(&self) {}
	}

	impl<'a> crate::RuleVariants<'a> for Rule {
		type DeclarationValue = Decl;
		type Metadata = ();
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Block<Decl, Rule, ()>>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, Block<Decl, Rule, ()>, "{color:black}");
	}
}
