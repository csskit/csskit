use crate::{
	AppliesTo, BoxPortion, BoxSide, CssAtomSet, CssMetadata, DeclarationKind, DeclarationMetadata, Inherits, NodeKinds,
	PropertyGroup, PropertyKind, VendorPrefixes, values,
};
use css_lexer::Kind;
use css_parse::{
	AtomSet, ComponentValues, Cursor, Declaration, DeclarationValue, Diagnostic, KindSet, NodeWithMetadata, Parser,
	Peek, Result as ParserResult, SemanticEq as SemanticEqTrait, State, T,
};
use csskit_derives::{Parse, SemanticEq, ToCursors, ToSpan};
use std::{
	collections::{HashMap, HashSet},
	fmt::Debug,
	hash::Hash,
	sync::OnceLock,
};

// The build.rs generates a list of CSS properties from the value mods
include!(concat!(env!("OUT_DIR"), "/css_apply_properties.rs"));

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Custom<'a>(pub ComponentValues<'a>);

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> Peek<'a> for Computed<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& matches!(
				p.to_atom::<CssAtomSet>(c),
				CssAtomSet::Var
					| CssAtomSet::Calc
					| CssAtomSet::Min
					| CssAtomSet::Max
					| CssAtomSet::Clamp
					| CssAtomSet::Round
					| CssAtomSet::Mod
					| CssAtomSet::Rem
					| CssAtomSet::Sin
					| CssAtomSet::Cos
					| CssAtomSet::Tan
					| CssAtomSet::Asin
					| CssAtomSet::Atan
					| CssAtomSet::Atan2
					| CssAtomSet::Pow
					| CssAtomSet::Sqrt
					| CssAtomSet::Hypot
					| CssAtomSet::Log
					| CssAtomSet::Exp
					| CssAtomSet::Abs
					| CssAtomSet::Sign
			)
	}
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Unknown<'a>(pub ComponentValues<'a>);

macro_rules! style_value {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
		pub enum StyleValue<'a> {
			#[cfg_attr(feature = "visitable", visit(skip))]
			Initial(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Inherit(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Unset(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Revert(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			RevertLayer(T![Ident]),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Custom(Custom<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Computed(Computed<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Unknown(Unknown<'a>),
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(values::$ty$(<$a>)?),
			)+
		}
	}
}

apply_properties!(style_value);

impl<'a> NodeWithMetadata<CssMetadata> for StyleValue<'a> {
	fn metadata(&self) -> CssMetadata {
		macro_rules! metadata {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match self {
					Self::Initial(_) |
					Self::Inherit(_)|
					Self::Unset(_)|
					Self::Revert(_)|
					Self::RevertLayer(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::CssWideKeywords,
							..Default::default()
						}
					}
					Self::Custom(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::Custom,
							..Default::default()
						}
					}
					Self::Computed(_) => {
						CssMetadata {
							declaration_kinds: DeclarationKind::Computed,
							..Default::default()
						}
					},
					Self::Unknown(_) => {
						CssMetadata {
							node_kinds: NodeKinds::Unknown,
							..Default::default()
						}
					},
					$(
					Self::$name(_) => {
						let mut declaration_kinds = DeclarationKind::none();
						if values::$ty::is_shorthand() {
							declaration_kinds |= DeclarationKind::Shorthands;
						} else {
							declaration_kinds |= DeclarationKind::Longhands;
						}
						CssMetadata {
							property_groups: values::$ty::property_group(),
							applies_to: values::$ty::applies_to(),
							box_sides: values::$ty::box_side(),
							box_portions: values::$ty::box_portion(),
							declaration_kinds,
							..Default::default()
						}
					}
					)+
				}
			};
		}
		apply_properties!(metadata)
	}
}

impl<'a> StyleValue<'a> {
	/// Returns the initial value string for a given property name.
	/// This is useful when you have `StyleValue::Initial` and need to know what the initial value
	/// should be based on the property name.
	pub fn initial_by_name(property_name: CssAtomSet) -> Option<&'static str> {
		macro_rules! get_initial_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::initial()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_initial_by_name)
	}

	/// Returns the inherits value for a given property name.
	pub fn inherits_by_name(property_name: CssAtomSet) -> Option<Inherits> {
		macro_rules! get_inherits_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::inherits()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_inherits_by_name)
	}

	/// Returns the applies_to value for a given property name.
	pub fn applies_to_by_name(property_name: CssAtomSet) -> Option<AppliesTo> {
		macro_rules! get_applies_to_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::applies_to()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_applies_to_by_name)
	}

	/// Returns the property_group for a given property name.
	pub fn property_group_by_name(property_name: CssAtomSet) -> Option<PropertyGroup> {
		macro_rules! get_property_group_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::property_group()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_property_group_by_name)
	}

	/// Returns the box_side for a given property name.
	pub fn box_side_by_name(property_name: CssAtomSet) -> Option<BoxSide> {
		macro_rules! get_box_side_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::box_side()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_box_side_by_name)
	}

	/// Returns the box_portion for a given property name.
	pub fn box_portion_by_name(property_name: CssAtomSet) -> Option<BoxPortion> {
		macro_rules! get_box_portion_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => Some(values::$ty::box_portion()),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_box_portion_by_name)
	}

	/// Returns the shorthand group for a given property name.
	/// For longhand properties, returns the shorthand they belong to (e.g., MarginTop -> Margin).
	/// For shorthands and non-longhand properties, returns CssAtomSet::_None.
	pub fn shorthand_group_by_name(property_name: CssAtomSet) -> CssAtomSet {
		macro_rules! get_shorthand_group_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => values::$ty::shorthand_group(),
					)+
					_ => CssAtomSet::_None,
				}
			};
		}
		apply_properties!(get_shorthand_group_by_name)
	}

	/// Returns the longhands for a given shorthand property name.
	/// For shorthand properties, returns Some(&[...]) with the list of longhands.
	/// For non-shorthand properties, returns None.
	pub fn longhands_by_name(property_name: CssAtomSet) -> Option<&'static [CssAtomSet]> {
		macro_rules! get_longhands_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => values::$ty::longhands(),
					)+
					_ => None,
				}
			};
		}
		apply_properties!(get_longhands_by_name)
	}

	/// Returns whether a given property name is a shorthand.
	pub fn is_shorthand_by_name(property_name: CssAtomSet) -> bool {
		macro_rules! get_is_shorthand_by_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match property_name {
					$(
					CssAtomSet::$name => values::$ty::is_shorthand(),
					)+
					_ => false,
				}
			};
		}
		apply_properties!(get_is_shorthand_by_name)
	}

	/// Returns all transitive longhands for a given shorthand property name.
	/// For nested shorthands (e.g., `border-width`), this recursively expands to include
	/// all nested longhands (e.g., `border-top-width`, `border-left-width`, etc.).
	/// Results are cached after first computation.
	pub fn expanded_longhands_by_name(property_name: CssAtomSet) -> Option<&'static [CssAtomSet]> {
		// Cache for expanded longhands per property
		static CACHE: OnceLock<HashMap<CssAtomSet, Vec<CssAtomSet>>> = OnceLock::new();

		let cache = CACHE.get_or_init(|| {
			let mut map = HashMap::new();

			// Helper to recursively expand longhands
			fn expand_longhands(atom: CssAtomSet, visited: &mut HashSet<CssAtomSet>, result: &mut Vec<CssAtomSet>) {
				if let Some(direct_longhands) = StyleValue::longhands_by_name(atom) {
					for &longhand in direct_longhands {
						if visited.insert(longhand) {
							result.push(longhand);
							// Recursively expand if this longhand is also a shorthand
							expand_longhands(longhand, visited, result);
						}
					}
				}
			}

			// Pre-compute expanded longhands for all properties that have longhands
			macro_rules! populate_cache {
				( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
					$(
						if values::$ty::is_shorthand() {
							let mut visited = HashSet::new();
							let mut expanded = Vec::new();
							expand_longhands(CssAtomSet::$name, &mut visited, &mut expanded);
							if !expanded.is_empty() {
								map.insert(CssAtomSet::$name, expanded);
							}
						}
					)+
				};
			}
			apply_properties!(populate_cache);

			map
		});

		cache.get(&property_name).map(|v| v.as_slice())
	}
}

impl<'a> DeclarationValue<'a, CssMetadata> for StyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn declaration_metadata(decl: &Declaration<'a, Self, CssMetadata>) -> CssMetadata {
		let mut meta = decl.value.metadata();
		// Mark this node as a declaration
		meta.node_kinds |= NodeKinds::Declaration;
		if decl.important.is_some() {
			meta.declaration_kinds |= DeclarationKind::Important;
		}
		// Check if this is a custom property (dashed ident)
		if decl.name.is_dashed_ident() {
			meta.node_kinds |= NodeKinds::Custom;
		}
		// Check if the value is unknown
		if decl.value.is_unknown() {
			meta.node_kinds |= NodeKinds::Unknown;
		}
		// Extract vendor prefix from property name cursor
		let cursor: Cursor = decl.name.into();
		meta.vendor_prefixes = CssAtomSet::from_bits(cursor.atom_bits()).try_into().unwrap_or(VendorPrefixes::none());
		// Declarations always have a name property
		meta.property_kinds |= PropertyKind::Name;
		meta
	}

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c.token().is_dashed_ident() || crate::property_atoms::CSS_PROPERTY_ATOMS.contains(&p.to_atom::<CssAtomSet>(c))
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn is_custom(&self) -> bool {
		matches!(self, Self::Custom(_))
	}

	fn is_initial(&self) -> bool {
		matches!(self, Self::Initial(_))
	}

	fn is_inherit(&self) -> bool {
		matches!(self, Self::Inherit(_))
	}

	fn is_unset(&self) -> bool {
		matches!(self, Self::Unset(_))
	}

	fn is_revert(&self) -> bool {
		matches!(self, Self::Revert(_))
	}

	fn is_revert_layer(&self) -> bool {
		matches!(self, Self::RevertLayer(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Computed(_))
	}

	fn parse_custom_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Custom>().map(Self::Custom)
	}

	fn parse_computed_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Computed>().map(Self::Computed)
	}

	fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if c == Kind::Ident {
			match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::Initial => return Ok(Self::Initial(p.parse::<T![Ident]>()?)),
				CssAtomSet::Inherit => return Ok(Self::Inherit(p.parse::<T![Ident]>()?)),
				CssAtomSet::Unset => return Ok(Self::Unset(p.parse::<T![Ident]>()?)),
				CssAtomSet::Revert => return Ok(Self::Revert(p.parse::<T![Ident]>()?)),
				CssAtomSet::RevertLayer => return Ok(Self::RevertLayer(p.parse::<T![Ident]>()?)),
				_ => {}
			}
		}
		macro_rules! parse_declaration_value {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $atom: ident,)+ ) => {
				match p.to_atom::<CssAtomSet>(name) {
					$(CssAtomSet::$atom => p.parse::<values::$ty>().map(Self::$name),)+
					_ => Err(Diagnostic::new(name, Diagnostic::unexpected))?,
				}
			}
		}
		apply_properties!(parse_declaration_value)
	}

	fn parse_unknown_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Unknown>().map(Self::Unknown)
	}
}

impl<'a> SemanticEqTrait for crate::StyleValue<'a> {
	fn semantic_eq(&self, other: &Self) -> bool {
		macro_rules! semantic_eq {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match (self, other) {
					(Self::Initial(_), Self::Initial(_)) => true,
					(Self::Inherit(_), Self::Inherit(_)) => true,
					(Self::Unset(_), Self::Unset(_)) => true,
					(Self::Revert(_), Self::Revert(_)) => true,
					(Self::RevertLayer(_), Self::RevertLayer(_)) => true,
					(Self::Custom(a), Self::Custom(b)) => a.semantic_eq(b),
					(Self::Computed(a), Self::Computed(b)) => a.semantic_eq(b),
					(Self::Unknown(a), Self::Unknown(b)) => a.semantic_eq(b),
					$((Self::$name(a), Self::$name(b)) => a.semantic_eq(b),)+
					(_, _) => false,
				}
			};
		}
		apply_properties!(semantic_eq)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, CssMetadata};
	use bumpalo::Bump;
	use css_lexer::Lexer;
	use css_parse::{Declaration, Parser, assert_parse};

	type Property<'a> = Declaration<'a, StyleValue<'a>, CssMetadata>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Property>(), 488);
		assert_eq!(std::mem::size_of::<StyleValue>(), 416);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Property, "width:inherit", Property { value: StyleValue::Inherit(_), .. });
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width:inherit!important",
			Property { value: StyleValue::Inherit(_), important: Some(_), .. }
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width:revert;",
			Property { value: StyleValue::Revert(_), semicolon: Some(_), .. }
		);
		assert_parse!(CssAtomSet::ATOMS, Property, "width:var(--a)", Property { value: StyleValue::Computed(_), .. });

		assert_parse!(CssAtomSet::ATOMS, Property, "float:none!important");
		assert_parse!(CssAtomSet::ATOMS, Property, "width:1px");
		assert_parse!(CssAtomSet::ATOMS, Property, "width:min(1px, 2px)");
		assert_parse!(CssAtomSet::ATOMS, Property, "border:1px solid var(--red)");
		// Should still parse unknown properties
		assert_parse!(CssAtomSet::ATOMS, Property, "dunno:like whatever");
		assert_parse!(CssAtomSet::ATOMS, Property, "rotate:1.21gw");
		assert_parse!(CssAtomSet::ATOMS, Property, "_background:black");
		assert_parse!(CssAtomSet::ATOMS, Property, "--custom:{foo:{bar};baz:(bing);}");
	}

	#[test]
	fn test_property_validation() {
		let bump = Bump::new();

		let input = "width:1px";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&bump, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(!decl.value.is_unknown(), "width should be recognized as a known property");

		let input = "notarealproperty:value";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&bump, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(decl.value.is_unknown(), "notarealproperty should be parsed as unknown");

		let input = "--custom:value";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, input);
		let mut p = Parser::new(&bump, input, lexer);
		let decl = p.parse::<Property>().unwrap();
		assert!(decl.value.is_custom(), "--custom should be parsed as custom property");
	}
}
