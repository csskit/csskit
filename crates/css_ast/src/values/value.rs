use crate::{
	AttrFunction, CssAtomSet, CssMetadata, EnvFunction, FirstValidFunction, IfFunction, MathFunction, Unresolved,
	VarFunction,
};
use css_lexer::ToSpan;
use css_parse::{
	Box, Cursor, DeclarationValue, NodeWithMetadata, Parse, Parser, Peek, Result, SemanticEq, ToCursors,
	ToNormalisedValue, ToNumberValue,
};
use csskit_derives::*;
use csskit_proc_macro::node;

/// Classifies a function atom as an arbitrary substitution function.
#[inline]
pub(crate) fn is_substitution_function(atom: CssAtomSet) -> bool {
	matches!(atom, CssAtomSet::Var | CssAtomSet::Env | CssAtomSet::Attr | CssAtomSet::If | CssAtomSet::FirstValid)
}

/// Generates the `Parse` impl for a value-slot enum with the shape
/// `Literal(..) | Substituted(Box<Sub>) | Unresolved(Box<Unresolved>)`.
macro_rules! impl_value_slot_parse {
	($ty:ident, $sub:ident, $lit:ty) => {
		impl<'a, T: ::css_parse::Parse<'a> + ::css_parse::Peek<'a>> ::css_parse::Parse<'a> for $ty<'a, T> {
			fn parse<I>(p: &mut ::css_parse::Parser<'a, I>) -> ::css_parse::Result<Self>
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
			{
				if p.peek::<$sub<T>>() {
					if !p.enter_substitution() {
						return Ok(Self::Unresolved(::css_parse::Box::new_in(p.alloc(), p.parse::<Unresolved>()?)));
					}
					let sub = p.parse::<$sub<T>>();
					p.exit_substitution();
					return Ok(Self::Substituted(::css_parse::Box::new_in(p.alloc(), sub?)));
				}
				Ok(Self::Literal(p.parse::<$lit>()?))
			}
		}
	};
}
pub(crate) use impl_value_slot_parse;

/// Generic wrapper for CSS values whose grammar permits arbitrary substitution functions
/// (`var()`, `env()`, `attr()`, `if()`, `first-valid()`), but **not** typed math functions.
///
/// <https://drafts.csswg.org/css-values-5/#arbitrary-substitution-function>
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
pub enum Value<'a, T> {
	Literal(T),
	Substituted(Box<'a, SubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
}

impl_value_slot_parse!(Value, SubstitutionFunction, T);

/// An arbitrary substitution function appearing in a [`Value`] slot.
///
/// Fallbacks recurse into the slot's own type (`Value<T>`), preserving maximal type information.
/// Parse/Peek are derived: each variant is atom-dispatched by the leading function name.
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
pub enum SubstitutionFunction<'a, T> {
	Var(VarFunction<'a, Value<'a, T>>),
	Env(EnvFunction<'a, Value<'a, T>>),
	Attr(AttrFunction<'a>),
	If(IfFunction<'a, Value<'a, T>>),
	FirstValid(FirstValidFunction<'a, Value<'a, T>>),
}

/// Generic wrapper for numeric CSS values whose grammar permits both arbitrary substitution
/// functions **and** typed math functions (`calc()`, `min()`, `max()`, etc.).
///
/// Used for: `<length>`, `<length-percentage>`, `<number>`, `<percentage>`, `<integer>`,
/// `<time>`, `<angle>`, `<frequency>`, `<flex>`, `<alpha-value>`, etc.
///
/// Structurally identical to [`Value`] except [`CalcableSubstitutionFunction`] adds a `Math` variant.
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
pub enum CalcableValue<'a, T> {
	Literal(T),
	Substituted(Box<'a, CalcableSubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
}

impl_value_slot_parse!(CalcableValue, CalcableSubstitutionFunction, T);

/// A substitution or math function appearing in a [`CalcableValue`] slot.
///
/// `Math` covers `calc()`, `min()`, `max()`, `clamp()`, `round()`, `mod()`, `rem()`, the
/// trigonometric/exponential functions, and `abs()`/`sign()` (see [`MathFunction`]). It's
/// parametrized by the same `T` as the surrounding [`CalcableValue`], since most of these
/// functions are "type-transparent" (their arguments and result share `T`'s type).
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
pub enum CalcableSubstitutionFunction<'a, T> {
	Math(MathFunction<'a, T>),
	Var(VarFunction<'a, CalcableValue<'a, T>>),
	Env(EnvFunction<'a, CalcableValue<'a, T>>),
	Attr(AttrFunction<'a>),
	If(IfFunction<'a, CalcableValue<'a, T>>),
	FirstValid(FirstValidFunction<'a, CalcableValue<'a, T>>),
}

/// Generates the value-behaviour trait impls (`ToNumberValue`, `ToNormalisedValue`,
/// `NodeWithMetadata`, `DeclarationValue`) shared verbatim by every value-slot enum.
macro_rules! impl_value_slot_traits {
	($ty:ident) => {
		impl<T: ToNumberValue> ToNumberValue for $ty<'_, T> {
			fn to_number_value(&self) -> Option<f32> {
				match self {
					Self::Literal(t) => t.to_number_value(),
					Self::Substituted(_) | Self::Unresolved(_) => None,
				}
			}
		}

		impl<T: ToNormalisedValue> ToNormalisedValue for $ty<'_, T> {
			fn to_normalised_value(&self) -> Option<f32> {
				match self {
					Self::Literal(t) => t.to_normalised_value(),
					Self::Substituted(_) | Self::Unresolved(_) => None,
				}
			}
		}

		impl<'a, T: NodeWithMetadata<CssMetadata>> NodeWithMetadata<CssMetadata> for $ty<'a, T> {
			fn metadata(&self) -> CssMetadata {
				match self {
					Self::Literal(t) => t.metadata(),
					Self::Substituted(_) | Self::Unresolved(_) => CssMetadata {
						uses_substitution: true,
						declaration_kinds: crate::DeclarationKind::Computed,
						..CssMetadata::default()
					},
				}
			}
		}

		impl<'a, T> DeclarationValue<'a, CssMetadata> for $ty<'a, T>
		where
			T: Parse<'a>
				+ Peek<'a>
				+ ToCursors
				+ ToSpan
				+ SemanticEq
				+ NodeWithMetadata<CssMetadata>
				+ DeclarationValue<'a, CssMetadata>,
		{
			fn is_computed_declaration_value<I>(p: &Parser<'a, I>, c: Cursor) -> bool
			where
				I: Iterator<Item = Cursor> + Clone,
			{
				<Self as Peek>::peek(p, c)
			}

			fn is_initial(&self) -> bool {
				matches!(self, Self::Literal(t) if t.is_initial())
			}
			fn is_inherit(&self) -> bool {
				matches!(self, Self::Literal(t) if t.is_inherit())
			}
			fn is_unset(&self) -> bool {
				matches!(self, Self::Literal(t) if t.is_unset())
			}
			fn is_revert(&self) -> bool {
				matches!(self, Self::Literal(t) if t.is_revert())
			}
			fn is_revert_layer(&self) -> bool {
				matches!(self, Self::Literal(t) if t.is_revert_layer())
			}
			fn needs_computing(&self) -> bool {
				!matches!(self, Self::Literal(_))
			}
			fn parse_specified_declaration_value<Iter>(p: &mut Parser<'a, Iter>, name: Cursor) -> Result<Self>
			where
				Iter: Iterator<Item = Cursor> + Clone,
			{
				let _ = name;
				p.parse()
			}
		}
	};
}

impl_value_slot_traits!(Value);
impl_value_slot_traits!(CalcableValue);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Color, Length};
	use css_parse::assert_parse;

	type ValueColor<'a> = Value<'a, Color<'a>>;
	type CalcLength<'a> = CalcableValue<'a, Length>;

	#[test]
	fn value_literal() {
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "red", |v| {
			assert!(matches!(v, Value::Literal(_)));
		});
	}

	#[test]
	fn value_substituted_var_no_fallback() {
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "var(--c)", |v| {
			assert!(matches!(v, Value::Substituted(_)));
		});
	}

	#[test]
	fn value_substituted_var_typed_fallback() {
		// Fallback recurses into the slot type (Color) and stays typed.
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "var(--c, red)", |v| {
			let Value::Substituted(sub) = v else { panic!("expected Substituted") };
			let SubstitutionFunction::Var(var) = &*sub else { panic!("expected Var") };
			assert!(matches!(var.fallback.as_deref(), Some(Value::Literal(_))));
		});
	}

	#[test]
	fn value_substituted_env_and_others() {
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "env(my-color, red)");
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "attr(data-color)");
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "first-valid(red, blue)");
	}

	#[test]
	fn value_nested_fallback() {
		// var fallback contains another var, itself with a typed literal fallback.
		assert_parse!(CssAtomSet::ATOMS, ValueColor, "var(--a, var(--b, red))", |v| {
			let Value::Substituted(sub) = v else { panic!() };
			let SubstitutionFunction::Var(outer) = &*sub else { panic!() };
			let Some(inner) = outer.fallback.as_deref() else { panic!("missing outer fallback") };
			assert!(matches!(inner, Value::Substituted(_)));
		});
	}

	#[test]
	fn calcable_literal_and_calc() {
		assert_parse!(CssAtomSet::ATOMS, CalcLength, "10px", |v| {
			assert!(matches!(v, CalcableValue::Literal(_)));
		});
		assert_parse!(CssAtomSet::ATOMS, CalcLength, "calc(1px + 2px)", |v| {
			let CalcableValue::Substituted(sub) = v else { panic!() };
			assert!(matches!(&*sub, CalcableSubstitutionFunction::Math(_)));
		});
	}

	#[test]
	fn calcable_var_typed_fallback() {
		assert_parse!(CssAtomSet::ATOMS, CalcLength, "var(--w, 10px)", |v| {
			let CalcableValue::Substituted(sub) = v else { panic!() };
			let CalcableSubstitutionFunction::Var(var) = &*sub else { panic!() };
			assert!(matches!(var.fallback.as_deref(), Some(CalcableValue::Literal(_))));
		});
	}

	#[test]
	fn depth_limit_rejects_to_unresolved() {
		// Build var(--a, var(--a, ... )) nested past MAX_SUBSTITUTION_DEPTH.
		use css_lexer::Lexer;
		use css_parse::{Arena, Parser};

		let depth = (Parser::<std::vec::IntoIter<css_parse::Cursor>>::MAX_SUBSTITUTION_DEPTH as usize) + 5;
		let mut input = String::new();
		for _ in 0..depth {
			input.push_str("var(--a,");
		}
		input.push_str("red");
		for _ in 0..depth {
			input.push(')');
		}

		let alloc = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, &input);
		let mut p = Parser::new(&alloc, &input, lexer);
		// Must not stack-overflow; deepest level degrades to Unresolved rather than recursing.
		let result = p.parse_entirely::<ValueColor>();
		assert!(result.output.is_some(), "expected parse to succeed via Unresolved degradation");
	}
}
