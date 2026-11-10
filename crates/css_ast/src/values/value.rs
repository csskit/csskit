use crate::{
	AttrFunction, CssAtomSet, CssMetadata, EnvFunction, FirstValidFunction, IdentFunction, IfFunction, MathFunction,
	ReducedCalc, TreeCountingFunction, Unresolved, VarFunction,
};
use css_lexer::ToSpan;
use css_parse::{
	Box, Cursor, DeclarationValue, NodeWithMetadata, Parse, Parser, Peek, Result, SemanticEq, ToCursors,
	ToNormalisedValue, ToNumberValue, Token,
};
use csskit_derives::*;
use csskit_proc_macro::node;

/// Classifies a function atom as an arbitrary substitution function.
#[inline]
pub(crate) fn is_substitution_function(atom: CssAtomSet) -> bool {
	crate::SubstitutionKinds::try_from(atom).is_ok()
}

/// Generates the `ParseInfallible` impl for a value-slot enum with the shape
/// `Literal(..) | Substituted(Box<Sub>) | Unresolved(Box<Unresolved>) | Bad(Box<Unresolved>)`.
///
/// The slot never fails to parse. When the literal or function cannot be parsed, the parser rewinds,
/// records the diagnostic, and keeps the tokens up to the next stop token as `Bad`.
macro_rules! impl_value_slot_parse {
	($ty:ident, $sub:ident, $lit:ty) => {
		impl<'a, T: ::css_parse::Parse<'a> + ::css_parse::Peek<'a>> ::css_parse::ParseInfallible<'a> for $ty<'a, T> {
			fn parse_infallible<I>(p: &mut ::css_parse::Parser<'a, I>) -> Self
			where
				I: ::std::iter::Iterator<Item = ::css_parse::Cursor> + ::std::clone::Clone,
			{
				let checkpoint = p.checkpoint();
				let result = if p.peek::<$sub<T>>() {
					if !p.enter_substitution() {
						return Self::Unresolved(::css_parse::Box::new_in(
							p.alloc(),
							p.parse_infallible::<Unresolved>(),
						));
					}
					let sub = p.parse::<$sub<T>>();
					p.exit_substitution();
					sub.map(|sub| Self::Substituted(::css_parse::Box::new_in(p.alloc(), sub)))
				} else {
					p.parse::<$lit>().map(Self::Literal)
				};
				result.unwrap_or_else(|diagnostic| {
					p.rewind(checkpoint);
					p.diagnose(diagnostic);
					Self::Bad(::css_parse::Box::new_in(p.alloc(), p.parse_infallible::<Unresolved>()))
				})
			}
		}
	};
}
pub(crate) use impl_value_slot_parse;

/// A value slot whose grammar permits typed math functions, such as [`CalcableValue`], offered to
/// [`VisitMut::visit_value_slot`][crate::VisitMut::visit_value_slot] so a visitor may replace the whole slot.
pub trait ValueSlot<'a>: Parse<'a> + ToCursors + ToSpan + 'a {
	/// The literal type of the slot.
	type Literal;

	/// Returns the math function filling this slot, if one does.
	fn math_function(&self) -> Option<&MathFunction<'a, Self::Literal>>;
}

macro_rules! impl_value_slot {
	($ty:ident, $sub:ident) => {
		impl<'a, T: Parse<'a> + Peek<'a> + ToCursors + ToSpan> ValueSlot<'a> for $ty<'a, T> {
			type Literal = T;

			fn math_function(&self) -> Option<&MathFunction<'a, T>> {
				match self {
					Self::Substituted(func) => match &**func {
						$sub::Math(math) => Some(math),
						_ => None,
					},
					_ => None,
				}
			}
		}
	};
}

impl_value_slot!(CalcableValue, CalcableSubstitutionFunction);
impl_value_slot!(NumericValue, NumericSubstitutionFunction);
impl_value_slot!(QueryValue, QuerySubstitutionFunction);

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
	#[peek(skip)]
	Bad(Box<'a, Unresolved<'a>>),
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
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Function)]
pub enum SubstitutionFunction<'a, T> {
	Var(VarFunction<'a, Value<'a, T>>),
	Env(Box<'a, EnvFunction<'a, Value<'a, T>>>),
	Attr(Box<'a, AttrFunction<'a>>),
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
#[cfg_attr(feature = "visitable", derive(Visitable), visit(value_slot), visit_mut_bound(T: Parse<'a> + Peek<'a> + ToCursors))]
pub enum CalcableValue<'a, T> {
	Literal(T),
	Substituted(Box<'a, CalcableSubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
	#[peek(skip)]
	Bad(Box<'a, Unresolved<'a>>),
}

impl_value_slot_parse!(CalcableValue, CalcableSubstitutionFunction, T);

impl<'a, T: Copy + Into<Token>> CalcableValue<'a, T> {
	/// Reduces a literal or math function; see [`MathFunction::reduce`].
	pub fn reduce(&self) -> Option<ReducedCalc> {
		match self {
			Self::Literal(literal) => ReducedCalc::from_token((*literal).into()),
			Self::Substituted(func) => match &**func {
				CalcableSubstitutionFunction::Math(math) => math.reduce(),
				_ => None,
			},
			Self::Unresolved(_) | Self::Bad(_) => None,
		}
	}
}

/// A substitution or math function appearing in a [`CalcableValue`] slot.
///
/// `Math` covers `calc()`, `min()`, `max()`, `clamp()`, `round()`, `mod()`, `rem()`, the
/// trigonometric/exponential functions, and `abs()`/`sign()` (see [`MathFunction`]). It's
/// parametrized by the same `T` as the surrounding [`CalcableValue`], since most of these
/// functions are "type-transparent" (their arguments and result share `T`'s type).
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children), visit_mut_bound(T: Parse<'a> + Peek<'a> + ToCursors))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Function)]
pub enum CalcableSubstitutionFunction<'a, T> {
	Math(Box<'a, MathFunction<'a, T>>),
	Var(VarFunction<'a, CalcableValue<'a, T>>),
	Env(Box<'a, EnvFunction<'a, CalcableValue<'a, T>>>),
	Attr(Box<'a, AttrFunction<'a>>),
	If(IfFunction<'a, CalcableValue<'a, T>>),
	FirstValid(FirstValidFunction<'a, CalcableValue<'a, T>>),
}

/// Generic wrapper for CSS values whose grammar is an `<integer>` or `<number>`, which permit
/// everything a [`CalcableValue`] does plus the tree-counting functions (`sibling-count()`,
/// `sibling-index()`).
///
/// <https://drafts.csswg.org/css-values-5/#tree-counting>
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(value_slot), visit_mut_bound(T: Parse<'a> + Peek<'a> + ToCursors))]
pub enum NumericValue<'a, T> {
	Literal(T),
	Substituted(Box<'a, NumericSubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
	#[peek(skip)]
	Bad(Box<'a, Unresolved<'a>>),
}

impl_value_slot_parse!(NumericValue, NumericSubstitutionFunction, T);

impl<'a, T: Copy + Into<Token>> NumericValue<'a, T> {
	/// Reduces a literal or math function; see [`MathFunction::reduce`].
	pub fn reduce(&self) -> Option<ReducedCalc> {
		match self {
			Self::Literal(literal) => ReducedCalc::from_token((*literal).into()),
			Self::Substituted(func) => match &**func {
				NumericSubstitutionFunction::Math(math) => math.reduce(),
				_ => None,
			},
			Self::Unresolved(_) | Self::Bad(_) => None,
		}
	}
}

/// A tree-counting, substitution, or math function appearing in a [`NumericValue`] slot.
///
/// Identical to [`CalcableSubstitutionFunction`] except for the `TreeCounting` variant, which is
/// not a substitution function but is resolved just as late.
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children), visit_mut_bound(T: Parse<'a> + Peek<'a> + ToCursors))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Function)]
pub enum NumericSubstitutionFunction<'a, T> {
	TreeCounting(TreeCountingFunction),
	Math(Box<'a, MathFunction<'a, T>>),
	Var(VarFunction<'a, NumericValue<'a, T>>),
	Env(Box<'a, EnvFunction<'a, NumericValue<'a, T>>>),
	Attr(Box<'a, AttrFunction<'a>>),
	If(IfFunction<'a, NumericValue<'a, T>>),
	FirstValid(FirstValidFunction<'a, NumericValue<'a, T>>),
}

/// Generic wrapper for CSS keyword values whose grammar permits arbitrary substitution functions
/// **and** the `ident()` function, which constructs a `<custom-ident>` from several parts and is
/// resolved just as late.
///
/// Used for bare keyword slots, so a substitution function or `ident()` can occupy the keyword
/// position and stay typed to the enclosing style value.
///
/// <https://drafts.csswg.org/css-values-5/#ident-fn>
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
pub enum KeywordValue<'a, T> {
	Literal(T),
	Substituted(Box<'a, KeywordSubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
	#[peek(skip)]
	Bad(Box<'a, Unresolved<'a>>),
}

impl_value_slot_parse!(KeywordValue, KeywordSubstitutionFunction, T);

/// An `ident()` or substitution function appearing in a [`KeywordValue`] slot.
///
/// Identical to [`SubstitutionFunction`] except for the `Ident` variant, which is not a
/// substitution function but is resolved just as late.
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Function)]
pub enum KeywordSubstitutionFunction<'a, T> {
	Ident(IdentFunction<'a>),
	Var(VarFunction<'a, KeywordValue<'a, T>>),
	Env(Box<'a, EnvFunction<'a, KeywordValue<'a, T>>>),
	Attr(Box<'a, AttrFunction<'a>>),
	If(IfFunction<'a, KeywordValue<'a, T>>),
	FirstValid(FirstValidFunction<'a, KeywordValue<'a, T>>),
}

/// Generic wrapper for values in media and container query features, which permit typed math
/// functions (`calc()`, `min()`, `max()`, etc.) and `env()`, but no other substitution functions.
///
/// <https://drafts.csswg.org/mediaqueries-4/#mq-range-context>
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(value_slot), visit_mut_bound(T: Parse<'a> + Peek<'a> + ToCursors))]
pub enum QueryValue<'a, T> {
	Literal(T),
	Substituted(Box<'a, QuerySubstitutionFunction<'a, T>>),
	#[peek(skip)]
	Unresolved(Box<'a, Unresolved<'a>>),
	#[peek(skip)]
	Bad(Box<'a, Unresolved<'a>>),
}

impl_value_slot_parse!(QueryValue, QuerySubstitutionFunction, T);

/// A math function or `env()` appearing in a [`QueryValue`] slot.
#[node]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit(children), visit_mut_bound(T: Parse<'a> + Peek<'a> + ToCursors))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Function)]
pub enum QuerySubstitutionFunction<'a, T> {
	Math(MathFunction<'a, T>),
	Env(EnvFunction<'a, QueryValue<'a, T>>),
}

/// Generates the value-behaviour trait impls (`ToNumberValue`, `ToNormalisedValue`,
/// `NodeWithMetadata`, `DeclarationValue`) shared verbatim by every value-slot enum.
macro_rules! impl_value_slot_traits {
	($ty:ident) => {
		impl<T: ToNumberValue> ToNumberValue for $ty<'_, T> {
			fn to_number_value(&self) -> Option<f32> {
				match self {
					Self::Literal(t) => t.to_number_value(),
					Self::Substituted(_) | Self::Unresolved(_) | Self::Bad(_) => None,
				}
			}
		}

		impl<T: ToNormalisedValue> ToNormalisedValue for $ty<'_, T> {
			fn to_normalised_value(&self) -> Option<f32> {
				match self {
					Self::Literal(t) => t.to_normalised_value(),
					Self::Substituted(_) | Self::Unresolved(_) | Self::Bad(_) => None,
				}
			}
		}

		impl<'a, T: NodeWithMetadata<CssMetadata>> NodeWithMetadata<CssMetadata> for $ty<'a, T> {
			fn self_metadata(&self) -> CssMetadata {
				match self {
					Self::Literal(_) | Self::Bad(_) => CssMetadata::default(),
					Self::Substituted(_) | Self::Unresolved(_) => CssMetadata {
						declaration_kinds: crate::DeclarationKind::Computed,
						..CssMetadata::default()
					},
				}
			}

			fn metadata(&self) -> CssMetadata {
				let mut meta = CssMetadata::default();
				self.merge_metadata(&mut meta);
				meta
			}

			#[inline]
			fn merge_metadata(&self, meta: &mut CssMetadata) {
				match self {
					Self::Literal(t) => t.merge_metadata(meta),
					Self::Substituted(f) => {
						f.merge_metadata(meta);
						*meta = css_parse::NodeMetadata::merge(*meta, self.self_metadata());
					}
					Self::Unresolved(u) => {
						u.merge_metadata(meta);
						*meta = css_parse::NodeMetadata::merge(*meta, self.self_metadata());
					}
					Self::Bad(_) => {}
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
			fn is_revert_rule(&self) -> bool {
				matches!(self, Self::Literal(t) if t.is_revert_rule())
			}
			fn needs_computing(&self) -> bool {
				matches!(self, Self::Substituted(_) | Self::Unresolved(_))
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
impl_value_slot_traits!(NumericValue);
impl_value_slot_traits!(KeywordValue);
impl_value_slot_traits!(QueryValue);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CSSInt, Color, Length};
	use css_parse::{assert_parse, assert_peek_false};

	type ValueColor<'a> = Value<'a, Color<'a>>;
	type CalcLength<'a> = CalcableValue<'a, Length>;
	type NumericInt<'a> = NumericValue<'a, CSSInt>;
	type KeywordIdent<'a> = KeywordValue<'a, css_parse::T![Ident]>;
	type QueryLength<'a> = QueryValue<'a, Length>;

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
	fn numeric_tree_counting_functions() {
		assert_parse!(CssAtomSet::ATOMS, NumericInt, "sibling-index()", |v| {
			let NumericValue::Substituted(sub) = v else { panic!() };
			assert!(matches!(&*sub, NumericSubstitutionFunction::TreeCounting(_)));
		});
		assert_parse!(CssAtomSet::ATOMS, NumericInt, "sibling-count()");
	}

	#[test]
	fn numeric_covers_alternations_containing_number() {
		// <number-percentage> and <alpha-value> both admit a bare <number>, so the
		// tree-counting functions stand in for them too.
		type NumericNumberPercentage<'a> = NumericValue<'a, crate::NumberPercentage>;
		type NumericAlpha<'a> = NumericValue<'a, crate::OpacityValue>;
		assert_parse!(CssAtomSet::ATOMS, NumericNumberPercentage, "sibling-index()");
		assert_parse!(CssAtomSet::ATOMS, NumericNumberPercentage, "50%");
		assert_parse!(CssAtomSet::ATOMS, NumericAlpha, "sibling-count()");
		assert_parse!(CssAtomSet::ATOMS, NumericAlpha, "0.5");
	}

	#[test]
	fn numeric_keeps_literal_and_calcable_behaviour() {
		assert_parse!(CssAtomSet::ATOMS, NumericInt, "3", |v| {
			assert!(matches!(v, NumericValue::Literal(_)));
		});
		assert_parse!(CssAtomSet::ATOMS, NumericInt, "calc(sibling-index() + 1)", |v| {
			let NumericValue::Substituted(sub) = v else { panic!() };
			assert!(matches!(&*sub, NumericSubstitutionFunction::Math(_)));
		});
	}

	#[test]
	fn numeric_fallback_recurses_into_numeric_slot() {
		assert_parse!(CssAtomSet::ATOMS, NumericInt, "var(--i, sibling-index())", |v| {
			let NumericValue::Substituted(sub) = v else { panic!() };
			let NumericSubstitutionFunction::Var(var) = &*sub else { panic!() };
			assert!(matches!(var.fallback.as_deref(), Some(NumericValue::Substituted(_))));
		});
	}

	#[test]
	fn calcable_rejects_tree_counting_functions() {
		assert_peek_false!(CssAtomSet::ATOMS, CalcLength, "sibling-index()");
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

	#[test]
	fn keyword_literal() {
		assert_parse!(CssAtomSet::ATOMS, KeywordIdent, "flex", |v| {
			assert!(matches!(v, KeywordValue::Literal(_)));
		});
	}

	#[test]
	fn keyword_ident_function() {
		assert_parse!(CssAtomSet::ATOMS, KeywordIdent, "ident('vtl-'sibling-index())", |v| {
			let KeywordValue::Substituted(sub) = v else { panic!("expected Substituted") };
			assert!(matches!(&*sub, KeywordSubstitutionFunction::Ident(_)));
		});
	}

	#[test]
	fn keyword_substituted_var_typed_fallback() {
		// Fallback recurses into the keyword slot and stays typed.
		assert_parse!(CssAtomSet::ATOMS, KeywordIdent, "var(--k, flex)", |v| {
			let KeywordValue::Substituted(sub) = v else { panic!("expected Substituted") };
			let KeywordSubstitutionFunction::Var(var) = &*sub else { panic!("expected Var") };
			assert!(matches!(var.fallback.as_deref(), Some(KeywordValue::Literal(_))));
		});
	}

	#[test]
	fn query_math_and_env() {
		assert_parse!(CssAtomSet::ATOMS, QueryLength, "48rem", |v| {
			assert!(matches!(v, QueryValue::Literal(_)));
		});
		assert_parse!(CssAtomSet::ATOMS, QueryLength, "calc(48rem - 0.02px)", |v| {
			let QueryValue::Substituted(sub) = v else { panic!() };
			assert!(matches!(&*sub, QuerySubstitutionFunction::Math(_)));
		});
		assert_parse!(CssAtomSet::ATOMS, QueryLength, "env(safe-area-inset-top, 10px)", |v| {
			let QueryValue::Substituted(sub) = v else { panic!() };
			let QuerySubstitutionFunction::Env(env) = &*sub else { panic!() };
			assert!(matches!(env.fallback.as_deref(), Some(QueryValue::Literal(_))));
		});
	}

	#[test]
	fn query_rejects_other_substitution_functions() {
		assert_peek_false!(CssAtomSet::ATOMS, QueryLength, "var(--w)");
		assert_peek_false!(CssAtomSet::ATOMS, QueryLength, "attr(data-w)");
	}

	#[test]
	fn keyword_rejects_tree_counting_functions() {
		assert_peek_false!(CssAtomSet::ATOMS, KeywordIdent, "sibling-index()");
	}

	fn parse_numeric_int<'a>(
		alloc: &'a css_parse::Arena,
		input: &'a str,
	) -> css_parse::ParserReturn<'a, NumericInt<'a>> {
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = css_parse::Parser::new(alloc, input, lexer);
		p.parse_entirely::<NumericInt>()
	}

	fn written<'a>(alloc: &'a css_parse::Arena, input: &str, node: &impl ToCursors) -> css_parse::String<'a> {
		let mut out = css_parse::String::new_in(alloc);
		let mut write_sink = css_parse::CursorWriteSink::new(input, &mut out);
		let mut ordered_sink = css_parse::CursorOrderedSink::new(alloc, &mut write_sink);
		node.to_cursors(&mut ordered_sink);
		out
	}

	#[test]
	fn numeric_bad_keeps_tokens_and_records_diagnostic() {
		let alloc = css_parse::Arena::new();
		let input = "1.5 junk";
		let result = parse_numeric_int(&alloc, input);
		let Some(node @ NumericValue::Bad(_)) = &result.output else { panic!("expected Bad, got {:?}", result.output) };
		assert_eq!(result.errors.len(), 1);
		assert_eq!(written(&alloc, input, node).as_str(), input);
	}

	#[test]
	fn numeric_bad_stops_at_stop_token() {
		use css_parse::{Kind, KindSet};
		let alloc = css_parse::Arena::new();
		let input = "1.5 junk;2";
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = css_parse::Parser::new(&alloc, input, lexer);
		p.set_stop(KindSet::new(&[Kind::Semicolon]));
		let node = p.parse_infallible::<NumericInt>();
		assert!(matches!(node, NumericValue::Bad(_)));
		assert_eq!(p.peek_n(1), Kind::Semicolon);
		assert_eq!(written(&alloc, input, &node).as_str(), "1.5 junk");
	}

	#[test]
	fn numeric_bad_keeps_unclosed_function() {
		let alloc = css_parse::Arena::new();
		let input = "1.5 foo(bar";
		let result = parse_numeric_int(&alloc, input);
		let Some(node @ NumericValue::Bad(_)) = &result.output else { panic!("expected Bad, got {:?}", result.output) };
		assert_eq!(result.errors.len(), 2);
		assert_eq!(written(&alloc, input, node).as_str(), "1.5 foo(bar)");
	}

	#[test]
	fn numeric_try_parse_rejects_bad_and_rewinds() {
		use css_parse::Kind;
		let alloc = css_parse::Arena::new();
		let input = "1.5";
		let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = css_parse::Parser::new(&alloc, input, lexer);
		assert!(p.try_parse::<NumericInt>().is_err());
		assert_eq!(p.peek_n(1), Kind::Number);
		let result = p.parse_entirely::<NumericInt>();
		assert!(matches!(result.output, Some(NumericValue::Bad(_))));
		assert_eq!(result.errors.len(), 1);
	}

	#[test]
	fn substitution_dispatch_skips_variant_with_bad() {
		use crate::Positive;
		use css_parse::{Kind, T};

		#[derive(Parse, Peek, Debug)]
		enum Slotted<'a> {
			Strict(Value<'a, NumericValue<'a, Positive<CSSInt>>>),
			Loose(Value<'a, T![Number]>),
		}

		// `0` peeks as a positive integer but fails to parse, so `Strict` succeeds with a Bad
		// fallback. The derive must rewind and try `Loose`, which parses cleanly. With `1` the
		// typed slot is clean and keeps priority.
		let alloc = css_parse::Arena::new();
		for (input, loose) in [("var(--a,0)", true), ("var(--a,1)", false)] {
			let lexer = css_lexer::Lexer::new(&CssAtomSet::ATOMS, input);
			let mut p = css_parse::Parser::new(&alloc, input, lexer);
			let checkpoint = p.checkpoint();
			let node = p.parse::<Slotted>().expect("slots never fail");
			match node {
				Slotted::Loose(Value::Substituted(_)) if loose => {}
				Slotted::Strict(Value::Substituted(_)) if !loose => {}
				_ => panic!("{input}: got {node:?}"),
			}
			assert_eq!(p.peek_n(1), Kind::Eof);
			assert!(p.diagnostic_since(&checkpoint).is_none());
		}
	}
}
