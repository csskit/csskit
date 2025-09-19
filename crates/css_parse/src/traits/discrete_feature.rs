use crate::Cursor;

use crate::{Diagnostic, Parser, Result, T};

use super::Parse;

/// This trait provides an implementation for parsing a ["Media Feature" that has a discrete keyword][1]. This is
/// complementary to the other media features: [BooleanFeature][crate::BooleanFeature] and
/// [DiscreteFeature][crate::DiscreteFeature].
///
/// [1]: https://drafts.csswg.org/mediaqueries/#typedef-mf-plain
///
/// Rather than implementing this trait on an enum, use the [discrete_feature!][crate::discrete_feature] macro which
/// expands to define the enum and necessary traits ([Parse], this trait, and [ToCursors][crate::ToCursors]) in a
/// single macro call.
///
/// It does not implement [Parse], but provides `parse_discrere_feature(&mut Parser<'a>, name: &str) -> Result<Self>`,
/// which can make for a trivial [Parse] implementation. The `name: &str` parameter refers to the `<feature-name>`
/// token, which will be parsed as an Ident. The [DiscreteFeature::Value] type must be implemented, and defines the
/// `<value>` portion. Usually [DiscreteFeature::Value] can be easily defined using
/// [keyword_set!][crate::keyword_set].
///
/// CSS defines the Media Feature generally as:
///
/// ```md
///  │├─ "(" ─╮─ <feature-name> ─ ":" ─ <value> ─╭─ ")" ─┤│
///           ├─ <feature-name> ─────────────────┤
///           ╰─ <ranged-feature> ───────────────╯
///
/// ```
///
/// The [RangedFeature][crate::RangedFeature] trait provides algorithms for parsing `<ranged-feature>` productions, but
/// discrete features use the other two productions.
///
/// Given this, this trait parses as:
///
/// ```md
/// <feature-name>
///  │├─ <ident> ─┤│
///
/// <discrete-feature>
///  │├─ "(" ─╮─ <feature-name> ─ ":" ─ <value> ─╭─ ")" ─┤│
///           ╰─ <feature-name> ─────────────────╯
///
/// ```
///
pub trait DiscreteFeature<'a>: Sized {
	type Value: Parse<'a>;

	#[allow(clippy::type_complexity)] // TODO: simplify types
	fn parse_descrete_feature(
		p: &mut Parser<'a>,
		name: &'static str,
	) -> Result<(T!['('], T![Ident], Option<(T![:], Self::Value)>, T![')'])> {
		let open = p.parse::<T!['(']>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		if !p.eq_ignore_ascii_case(c, name) {
			Err(Diagnostic::new(c, Diagnostic::unexpected_ident))?
		}
		if p.peek::<T![:]>() {
			let colon = p.parse::<T![:]>()?;
			let value = p.parse::<Self::Value>()?;
			let close = p.parse::<T![')']>()?;
			Ok((open, ident, Some((colon, value)), close))
		} else {
			let close = p.parse::<T![')']>()?;
			Ok((open, ident, None, close))
		}
	}
}

/// This macro expands to define an enum which already implements [Parse][crate::Parse] and [DiscreteFeature], for a
/// one-liner definition of a [DiscreteFeature].
///
/// # Example
///
/// ```
/// use css_parse::*;
/// use bumpalo::Bump;
/// use csskit_derives::{ToCursors, ToSpan};
///
/// keyword_set!(
///     /// A keyword that defines text-feature options
///     pub enum FeatureKeywords {
///         Big: "big",
///         Small: "small",
///     }
/// );
///
/// // Define the Discrete Feature.
/// discrete_feature! {
///     /// A discrete media feature: `(test-feature: big)`, `(test-feature: small)`
///     #[derive(ToCursors, ToSpan, Debug)]
///     pub enum TestFeature<"test-feature", FeatureKeywords>
/// }
///
/// // Test!
/// let allocator = Bump::new();
/// let mut p = Parser::new(&allocator, "(test-feature)");
/// let result = p.parse_entirely::<TestFeature>();
/// assert!(matches!(result.output, Some(TestFeature::Bare(open, ident, close))));
///
/// let mut p = Parser::new(&allocator, "(test-feature: big)");
/// let result = p.parse_entirely::<TestFeature>();
/// assert!(matches!(result.output, Some(TestFeature::WithValue(open, ident, colon, any, close))))
/// ```
///
#[macro_export]
macro_rules! discrete_feature {
	($(#[$meta:meta])* $vis:vis enum $feature: ident<$feature_name: tt, $value: ty>) => {
		$(#[$meta])*
		$vis enum $feature {
			WithValue($crate::T!['('], $crate::T![Ident], $crate::T![:], $value, $crate::T![')']),
			Bare($crate::T!['('], $crate::T![Ident], $crate::T![')']),
		}

		impl<'a> $crate::Parse<'a> for $feature {
			fn parse(p: &mut $crate::Parser<'a>) -> $crate::Result<Self> {
				use $crate::DiscreteFeature;
				let (open, ident, opt, close) = Self::parse_descrete_feature(p, $feature_name)?;
				if let Some((colon, value)) = opt {
					Ok(Self::WithValue(open, ident, colon, value, close))
				} else {
					Ok(Self::Bare(open, ident, close))
				}
			}
		}

		impl<'a> $crate::DiscreteFeature<'a> for $feature {
			type Value = $value;
		}
	};
}
